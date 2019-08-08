use llvm_ir::{Function, Module};
use haybale::*;
use std::path::Path;

fn main() {
    env_logger::init();
    // With 0 args, finds zeroes of all the functions in "basic"
    // With 1 arg (a file name), finds zeroes of all the functions in that file
    // With 2 args (file then function), finds zero of that function in that file
    let firstarg = std::env::args().nth(1);
    let secondarg = std::env::args().nth(2);
    let modname = firstarg.unwrap_or_else(|| "basic".to_owned());
    let filepath = Path::new("tests")
        .join(Path::new("bcfiles"))
        .join(Path::new(&modname))
        .with_extension("bc");
    let llvm_mod = Module::from_bc_path(&filepath).unwrap_or_else(|e| panic!("Failed to parse module at path {}: {}", filepath.display(), e));
    let functions: Box<Iterator<Item = &Function>>;
    if let Some(funcname) = secondarg {
        functions = Box::new(std::iter::once(llvm_mod.get_func_by_name(&funcname).unwrap_or_else(|| panic!("Failed to find function named {}", funcname))));
    } else {
        functions = Box::new(llvm_mod.functions.iter());
    }
    let ctx = z3::Context::new(&z3::Config::new());
    for func in functions {
        println!("Finding zero of function {:?}...", func.name);
        if let Some(args) = find_zero_of_func(&ctx, func, &llvm_mod, &Config::default()) {
            assert_eq!(args.len(), func.parameters.len());
            match func.parameters.len() {
                0 => println!("Function returns zero when passed no arguments\n"),
                1 => println!("Function returns zero when passed the argument {:?}\n", args[0]),
                _ => println!("Function returns zero when passed arguments {:?}\n", args),
            }
        } else {
            println!("Function never returns zero for any values of the arguments\n");
        }
    }
}
