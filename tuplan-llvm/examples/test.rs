use tuplan_llvm::{
    llvm_get_function_ty, llvm_init_jit, llvm_init_jit_with_printer, LLVMBuilder, LLVMContext,
    LLVMExecutionEngine, LLVMModule,
};

fn main() {
    let mut ctx = LLVMContext::new();
    let mut module = LLVMModule::create_with_name_in_ctx("my_module", &mut ctx);

    let i64_t = ctx.i64_t();
    unsafe {
        let function_ty = llvm_get_function_ty(&i64_t, &mut [i64_t.copy_ref(), i64_t.copy_ref()]);
        let mut sum = module.add_function("sum", function_ty);
        let entry = sum.append_basic_block("entry");
        let mut builder = LLVMBuilder::new();
        builder.position_at_end(&entry);

        let add = builder.build_add("add_result", &sum.get_param(0), &sum.get_param(1));
        builder.build_ret(&add);

        module.dump_ir_to_stdout();

        llvm_init_jit_with_printer();

        let ee = LLVMExecutionEngine::new_for_module(&module).unwrap();

        let function: extern "C" fn(i64, i64) -> i64 = ee.get_function_as("sum");

        println!("{} + {} = {}", 1, 2, function(1, 2));
    }
}
