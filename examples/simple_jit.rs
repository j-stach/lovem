
extern crate lovem;

use lovem::{ir, exec, support as sup};

use ir::{context::Context, types as typ};
use exec::engine::ExecutionEngine as ee;
use sup::target as tgt;

fn main() -> Result<(), anyhow::Error>{
    let context = Context::global();
    let module = context.create_module("Addition");
    let builder = context.create_builder();

    let function_type = typ::Function::new(
        Box::new(typ::Int32::new()),            // Return type
        vec![
            Box::new(typ::Int32::new()),        // Argument types
            Box::new(typ::Int32::new())
        ],
        false       // Variable args? TODO Not sure
    );
    let function = module.add_function(function_type, "add");
    let block = context.append_block(function.clone(), "add");
    builder.position_at_end(block);

    let a = function.param(0);
    let b = function.param(1);

    let i0 = Box::new(builder.build_add(a, b, "AddAB"));
    builder.build_ret(i0);

    module.dump();


    ee::link_in_mcjit();
    tgt::native_target()?;
    tgt::native_asm_printer()?;

    let engine = ee::new_for_module(&module)?;
    let llvm_function = engine.find_function("add")?;

    Ok( unsafe {
        let add: extern "C" fn(u8, u8) -> u8 = std::mem::transmute(llvm_function);
        let a = 2;
        let b = 2;

        let sum = add(a, b);
        println!("{} + {} = {}", a, b, sum);
    })
}

/*
 * fn main() {
    let context = Context::new();
    let mut module = context.module_create_with_name("sum");
    let mut builder = context.create_builder();

    let function_type = llvm::types::Function::new(
        i64::get_type_in_context(&context),
        &[
            i64::get_type_in_context(&context),
            i64::get_type_in_context(&context),
            i64::get_type_in_context(&context)
        ],
        false);
    let mut func = module.add_function(function_type, "fname");
    let bb = context.append_basic_block(&mut func, "fname");
    builder.position_at_end(bb);

    // get the function's arguments
    let x = func.get_param(0).unwrap();
    let y = func.get_param(1).unwrap();
    let z = func.get_param(2).unwrap();

    let b = context.cons(20i64);

    let s1 = builder.build_add(x, b, "s1");
    let s2 = builder.build_add(y, s1, "s2");
    let s3 = builder.build_add(z, s2, "s3");
    builder.build_ret(s3);

    module.dump();

    llvm::link_in_mcjit();
    llvm::initialize_native_target();
    llvm::initialize_native_asm_printer();

    let ee = llvm::ExecutionEngine::create_for_module(&module).unwrap();
    let addr = ee.get_function_address("fname").unwrap();

    unsafe {
        let f: extern "C" fn(u64, u64, u64) -> u64 = mem::transmute(addr);

        let x: u64 = 1;
        let y: u64 = 2;
        let z: u64 = 3;
        let res = f(x, y, z);

        println!("{} + {} + {} = {}", x, y, z, res);
    }
*/
