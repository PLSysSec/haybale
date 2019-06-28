use inkwell::module::Module;
use inkwell::values::*;
use pitchfork_rs::*;
use std::path::Path;

fn main() {
    env_logger::init();
    let firstarg = std::env::args().nth(1);
    let filepath = Path::new("c_examples/basic/basic.bc");
    let llvm_mod = Module::parse_bitcode_from_path(&filepath).expect("Failed to parse module");
    let functions: Box<Iterator<Item = FunctionValue>>;
    if let Some(funcname) = firstarg {
        functions = Box::new(std::iter::once(llvm_mod.get_function(&funcname).unwrap_or_else(|| panic!("Failed to find function named {}", funcname))));
    } else {
        functions = Box::new(FunctionIterator::new(&llvm_mod));
    }
    for func in functions {
        println!("Finding zero of function {:?}...", func.get_name());
        if let Some(args) = find_zero_of_func(func) {
            assert_eq!(args.len(), func.count_params() as usize);
            match func.count_params() {
                0 => println!("Function returns zero when passed no arguments\n"),
                1 => println!("Function returns zero when passed the argument {:?}\n", args[0]),
                _ => println!("Function returns zero when passed arguments {:?}\n", args),
            }
        } else {
            println!("Function never returns zero for any values of the arguments\n");
        }
    }
}
