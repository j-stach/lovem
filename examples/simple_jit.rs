
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
