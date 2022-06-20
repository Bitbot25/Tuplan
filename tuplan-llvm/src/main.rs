use llvm::{
    core::{
        LLVMAddFunction, LLVMAppendBasicBlock, LLVMAppendBasicBlockInContext, LLVMBuildAdd,
        LLVMBuildRet, LLVMContextCreate, LLVMCreateBuilderInContext, LLVMDisposeBuilder,
        LLVMDumpModule, LLVMFunctionType, LLVMGetParam, LLVMInt64TypeInContext,
        LLVMModuleCreateWithNameInContext, LLVMPositionBuilderAtEnd,
    },
    execution_engine::{
        LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine, LLVMGetFunctionAddress,
        LLVMLinkInMCJIT,
    },
    target::{
        LLVM_InitializeAllTargets, LLVM_InitializeNativeAsmParser, LLVM_InitializeNativeTarget,
    },
};
use llvm_sys as llvm;
use std::mem;

// FIXME: The problem when building is that we need to have quotes for cl.exe to link successfully (When we have a path with spaces in it).
// But we cannot use quotes because the resulting path when the llvm-sys crates uses .join the resulting path will be something like "<programs>\LLVM"\include which is NOT a valid path in windows.

fn main() {
    unsafe {
        let ctx = LLVMContextCreate();
        let module = LLVMModuleCreateWithNameInContext(b"sum\0".as_ptr(), ctx);

        let builder = LLVMCreateBuilderInContext(ctx);

        let i64_t = LLVMInt64TypeInContext(ctx);
        let mut args_t = [i64_t, i64_t];
        let fun_t = LLVMFunctionType(i64_t, args_t.as_mut_ptr(), args_t.len(), false);

        let fun = LLVMAddFunction(module, b"sum\0", fun_t);

        let entry_block = LLVMAppendBasicBlockInContext(ctx, fun, b"entry\0".as_ptr());
        LLVMPositionBuilderAtEnd(builder, entry_block);

        let a = LLVMGetParam(fun, 0);
        let b = LLVMGetParam(fun, 1);

        let sum = LLVMBuildAdd(builder, a, b, b"sum_result");
        LLVMBuildRet(builder, sum);
        LLVMDisposeBuilder(builder);
        LLVMDumpModule(module);

        LLVMLinkInMCJIT();
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmParser();

        let mut execution_engine = mem::uninitialized();
        let mut error = mem::uninitialized();
        LLVMCreateExecutionEngineForModule(&mut execution_engine, module, &mut error);

        let addr = LLVMGetFunctionAddress(execution_engine, b"sum\0".as_ptr());
        let sum_jit: extern "C" fn(i64, i64) -> i64 = mem::transmute(addr);

        LLVMDisposeExecutionEngine(execution_engine);
        LLVMContextDispose(ctx);
    }

    eprintln!("Exited without errors.");
}
