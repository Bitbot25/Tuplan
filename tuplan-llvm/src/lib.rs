use llvm::{core::{LLVMContextCreate, LLVMModuleCreateWithNameInContext, LLVMInt64Type, LLVMInt64TypeInContext, LLVMFloatTypeInContext, LLVMFunctionType, LLVMGetModuleIdentifier, LLVMAddFunction, LLVMGetValueName2, LLVMGetValueName, LLVMAppendBasicBlock, LLVMGetBasicBlockName}, prelude::{LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef, LLVMBasicBlockRef}, LLVMValueKind};
use llvm_sys as llvm;
use std::{mem::{self, ManuallyDrop}, ffi::{CString, CStr}};

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

    pub fn append_basic_block<S: Into<Vec<u8>>>(&mut self, name: S) -> LLVMBasicBlock {
        unsafe {
            let cstring = CString::new(name).unwrap();
            LLVMBasicBlock { inner: LLVMAppendBasicBlock(self.inner, cstring.as_ptr()) }
        }
    }
}

#[repr(C)]
pub struct LLVMContext {
    pub(crate) inner: LLVMContextRef,
}

impl LLVMContext {
    pub fn new() -> LLVMContext {
        LLVMContext { inner: unsafe { LLVMContextCreate() } }
    }

    pub fn i64_t(&self) -> LLVMType {
        LLVMType { inner: unsafe { LLVMInt64TypeInContext(self.inner) } }
    }

    pub fn float_t(&self) -> LLVMType {
        LLVMType { inner: unsafe { LLVMFloatTypeInContext(self.inner) } }
    }
}

#[repr(C)]
pub struct LLVMModule {
    pub(crate) inner: LLVMModuleRef,
}

impl LLVMModule {
    pub fn create_with_name_in_ctx<N: Into<Vec<u8>>>(name: N, context: &mut LLVMContext) -> LLVMModule {
        let cstring = CString::new(name).unwrap();
        Self { inner: unsafe { LLVMModuleCreateWithNameInContext(cstring.as_ptr(), context.inner) } }
    }

    pub fn add_function<N: Into<Vec<u8>>>(&mut self, name: N, function_ty: LLVMType) -> LLVMValue {
        let cstring = CString::new(name).unwrap();
        unsafe { LLVMValue { inner: LLVMAddFunction(self.inner, cstring.as_ptr(), function_ty.inner) } }
    }

    pub fn ident(&self) -> &CStr {
        unsafe {
            let chars: *const libc::c_char = LLVMGetModuleIdentifier(self.inner, &mut mem::uninitialized());
            CStr::from_ptr(chars)
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

pub fn llvm_get_function_ty(ret: &LLVMType, params: &mut [LLVMType]) -> LLVMType {
    unsafe { LLVMType { inner: LLVMFunctionType(ret.inner, params.as_mut_ptr() as *mut _, params.len() as u32, 0) } }
}