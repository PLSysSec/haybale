use inkwell::module::Module;
use inkwell::values::*;
use std::path::Path;
use z3;

fn main() {
    let filepath = Path::new("/Users/craig/pitchfork-rs/c_examples/basic/basic.bc");
    let llvm_mod = Module::parse_bitcode_from_path(&filepath).expect("Failed to parse module");
    if let Some(func) = llvm_mod.get_first_function() {
        println!("Finding zero of function {:?}...", func.get_name());
        let z3cfg = z3::Config::new();
        let z3ctx = z3::Context::new(&z3cfg);
        if let Some((arg1, arg2)) = find_zero_of_func(&z3ctx, func) {
            println!("Function returns zero for arguments {}, {}", arg1, arg2);
        } else {
            println!("Function never returns zero for any values of the arguments");
        }
    } else {
        println!("No functions found in the LLVM file");
    }
}

// Given a function, find values of its inputs such that it returns zero
// Assumes function takes precisely two i32 arguments and returns an i32
// Returns None if there are no values of the inputs such that the function returns zero
fn find_zero_of_func(z3ctx: &z3::Context, func: FunctionValue) -> Option<(i32, i32)> {
    assert_eq!(func.count_params(), 2);
    let param1: IntValue = func.get_first_param().unwrap().into_int_value();
    let param2: IntValue = func.get_last_param().unwrap().into_int_value();
    let param1name = param1.get_name().to_str().expect("Bad parameter name");
    let param2name = param2.get_name().to_str().expect("Bad parameter name");
    let eqnames = param1name == param2name;
    let param1name = if param1name == "" || eqnames { "param1" } else { param1name };
    let param2name = if param2name == "" || eqnames { "param2" } else { param2name };
    let z3param1 = z3ctx.named_bitvector_const(param1name, 32);
    let z3param2 = z3ctx.named_bitvector_const(param2name, 32);
    let solver = z3::Solver::new(&z3ctx);

    // TODO: this simple (arbitrary) assertion is placeholder for actually symbolically executing the function
    println!("[warning: using placeholder computation; not producing real answers yet]");
    solver.assert(&z3param1.bvsgt(&z3param2));

    if solver.check() {
        let model = solver.get_model();
        let param1 = model.eval(&z3param1).unwrap().as_i64().unwrap();
        let param2 = model.eval(&z3param2).unwrap().as_i64().unwrap();
        Some((param1 as i32, param2 as i32))
    } else {
        None
    }
}
