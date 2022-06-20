use tuplan_llvm::{LLVMContext, LLVMModule, llvm_get_function_ty};

fn main() {
    let mut ctx = LLVMContext::new();
    let mut module = LLVMModule::create_with_name_in_ctx("my_module", &mut ctx);

    let i64_t = ctx.i64_t();
    unsafe {
        let function_ty = llvm_get_function_ty(&i64_t, &mut [i64_t.copy_ref(), i64_t.copy_ref()]);
        let mut sum = module.add_function("sum", function_ty);
        let entry = sum.append_basic_block("entry");
        println!("{:?}", entry.name());
        println!("{:?}", sum.name());
    }

    println!("{:?}", module.ident());
}