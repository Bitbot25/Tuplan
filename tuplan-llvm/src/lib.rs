use llvm::{
    analysis::{LLVMVerifierFailureAction::LLVMAbortProcessAction, LLVMVerifyModule},
    core::{
        LLVMAddFunction, LLVMAppendBasicBlock, LLVMBuildAdd, LLVMBuildRet, LLVMContextCreate,
        LLVMContextDispose, LLVMCreateBuilder, LLVMDisposeBuilder, LLVMDisposeMessage,
        LLVMDumpModule, LLVMFloatTypeInContext, LLVMFunctionType,
        LLVMGetBasicBlockName, LLVMGetModuleIdentifier, LLVMGetParam,
        LLVMGetValueName2, LLVMInt64TypeInContext,
        LLVMModuleCreateWithNameInContext, LLVMPositionBuilderAtEnd,
    },
    execution_engine::{
        LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine, LLVMExecutionEngineRef,
        LLVMGetFunctionAddress, LLVMLinkInMCJIT,
    },
    prelude::{
        LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef,
    },
    target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget},
};
use llvm_sys as llvm;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem, ptr,
};

#[repr(C)]
pub struct LLVMType {
    pub(crate) inner: LLVMTypeRef,
}

impl LLVMType {
    pub unsafe fn copy_ref(&self) -> LLVMType {
        LLVMType { inner: self.inner }
    }
}

#[repr(C)]
pub struct LLVMValue {
    pub(crate) inner: LLVMValueRef,
}

impl LLVMValue {
    pub fn name(&self) -> &CStr {
        unsafe {
            let ptr: *const libc::c_char = LLVMGetValueName2(self.inner, &mut mem::uninitialized());
            CStr::from_ptr(ptr)
        }
    }

    // Unsafe because you can get multiple references to the same value.
    pub unsafe fn get_param(&self, n: usize) -> LLVMValue {
        LLVMValue {
            inner: LLVMGetParam(self.inner, n as libc::c_uint),
        }
    }

    pub fn append_basic_block<S: Into<Vec<u8>>>(&mut self, name: S) -> LLVMBasicBlock {
        unsafe {
            let cstring = CString::new(name).unwrap();
            LLVMBasicBlock {
                inner: LLVMAppendBasicBlock(self.inner, cstring.as_ptr()),
            }
        }
    }
}

#[repr(C)]
pub struct LLVMContext {
    pub(crate) inner: LLVMContextRef,
    _marker: PhantomData<llvm::LLVMContext>,
}

impl LLVMContext {
    pub fn new() -> LLVMContext {
        LLVMContext {
            inner: unsafe { LLVMContextCreate() },
            _marker: PhantomData,
        }
    }

    pub fn i64_t(&self) -> LLVMType {
        LLVMType {
            inner: unsafe { LLVMInt64TypeInContext(self.inner) },
        }
    }

    pub fn float_t(&self) -> LLVMType {
        LLVMType {
            inner: unsafe { LLVMFloatTypeInContext(self.inner) },
        }
    }
}

impl Drop for LLVMContext {
    fn drop(&mut self) {
        unsafe { LLVMContextDispose(self.inner) }
    }
}

#[repr(C)]
pub struct LLVMModule {
    pub(crate) inner: LLVMModuleRef,
    _marker: PhantomData<llvm::LLVMModule>,
}

impl LLVMModule {
    pub fn create_with_name_in_ctx<N: Into<Vec<u8>>>(
        name: N,
        context: &mut LLVMContext,
    ) -> LLVMModule {
        let cstring = CString::new(name).unwrap();
        Self {
            inner: unsafe { LLVMModuleCreateWithNameInContext(cstring.as_ptr(), context.inner) },
            _marker: PhantomData,
        }
    }

    pub fn add_function<N: Into<Vec<u8>>>(&mut self, name: N, function_ty: LLVMType) -> LLVMValue {
        let cstring = CString::new(name).unwrap();
        unsafe {
            LLVMValue {
                inner: LLVMAddFunction(self.inner, cstring.as_ptr(), function_ty.inner),
            }
        }
    }

    pub fn ident(&self) -> &CStr {
        unsafe {
            let chars: *const libc::c_char =
                LLVMGetModuleIdentifier(self.inner, &mut mem::uninitialized());
            CStr::from_ptr(chars)
        }
    }

    pub fn dump_ir_to_stdout(&self) {
        unsafe { LLVMDumpModule(self.inner) }
    }
}

impl Drop for LLVMModule {
    fn drop(&mut self) {
        let mut error: *mut libc::c_char = ptr::null_mut();
        unsafe {
            LLVMVerifyModule(self.inner, LLVMAbortProcessAction, &mut error as *mut _);
            LLVMDisposeMessage(error);
        }
    }
}

#[repr(C)]
pub struct LLVMBasicBlock {
    pub(crate) inner: LLVMBasicBlockRef,
}

impl LLVMBasicBlock {
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetBasicBlockName(self.inner)) }
    }
}

#[repr(C)]
pub struct LLVMBuilder {
    pub(crate) inner: LLVMBuilderRef,
    _marker: PhantomData<llvm::LLVMBuilder>,
}

impl LLVMBuilder {
    pub fn new() -> LLVMBuilder {
        LLVMBuilder {
            inner: unsafe { LLVMCreateBuilder() },
            _marker: PhantomData,
        }
    }

    pub fn position_at_end(&mut self, block: &LLVMBasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.inner, block.inner);
        }
    }

    pub fn build_add<S: Into<Vec<u8>>>(
        &mut self,
        name: S,
        lhs: &LLVMValue,
        rhs: &LLVMValue,
    ) -> LLVMValue {
        unsafe {
            let cstring = CString::new(name).unwrap();
            LLVMValue {
                inner: LLVMBuildAdd(self.inner, lhs.inner, rhs.inner, cstring.as_ptr()),
            }
        }
    }

    pub fn build_ret(&mut self, value: &LLVMValue) -> LLVMValue {
        unsafe {
            LLVMValue {
                inner: LLVMBuildRet(self.inner, value.inner),
            }
        }
    }
}

impl Drop for LLVMBuilder {
    fn drop(&mut self) {
        unsafe { LLVMDisposeBuilder(self.inner) }
    }
}

#[repr(C)]
pub struct LLVMExecutionEngine {
    pub(crate) inner: LLVMExecutionEngineRef,
    _marker: PhantomData<llvm::execution_engine::LLVMOpaqueExecutionEngine>,
}

impl LLVMExecutionEngine {
    pub fn new_for_module(module: &LLVMModule) -> Result<LLVMExecutionEngine, CString> {
        unsafe {
            let mut ee = mem::uninitialized();
            let mut error = mem::uninitialized();
            if LLVMCreateExecutionEngineForModule(&mut ee, module.inner, &mut error) != 0 {
                return Err(CString::from_raw(error));
            }
            Ok(LLVMExecutionEngine {
                inner: ee,
                _marker: PhantomData,
            })
        }
    }

    pub unsafe fn get_function_as<S: Into<Vec<u8>>, F>(&self, name: S) -> F {
        let cstring = CString::new(name).unwrap();
        mem::transmute_copy(&LLVMGetFunctionAddress(self.inner, cstring.as_ptr()))
    }
}

pub fn llvm_get_function_ty(ret: &LLVMType, params: &mut [LLVMType]) -> LLVMType {
    unsafe {
        LLVMType {
            inner: LLVMFunctionType(
                ret.inner,
                params.as_mut_ptr() as *mut _,
                params.len() as u32,
                0,
            ),
        }
    }
}

impl Drop for LLVMExecutionEngine {
    fn drop(&mut self) {
        unsafe { LLVMDisposeExecutionEngine(self.inner) }
    }
}

pub fn llvm_init_jit() {
    unsafe {
        LLVMLinkInMCJIT();
        LLVM_InitializeNativeTarget();
    }
}

pub fn llvm_init_jit_with_printer() {
    unsafe {
        llvm_init_jit();
        LLVM_InitializeNativeAsmPrinter();
    }
}
