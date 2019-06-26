use inkwell::module::Module;
use inkwell::values::*;
use std::path::Path;

mod iterators;
use iterators::*;

mod state;
use state::State;

mod symex;
use symex::{symex_function, symex_again};

mod utils;
use utils::get_value_name;

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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum IntOfSomeWidth {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl IntOfSomeWidth {
    fn unwrap_to_i8(self) -> i8 {
        match self {
            IntOfSomeWidth::I8(i) => i,
            _ => panic!("unwrap_to_i8 on {:?}", self),
        }
    }

    fn unwrap_to_i16(self) -> i16 {
        match self {
            IntOfSomeWidth::I16(i) => i,
            _ => panic!("unwrap_to_i16 on {:?}", self),
        }
    }

    fn unwrap_to_i32(self) -> i32 {
        match self {
            IntOfSomeWidth::I32(i) => i,
            _ => panic!("unwrap_to_i32 on {:?}", self),
        }
    }

    fn unwrap_to_i64(self) -> i64 {
        match self {
            IntOfSomeWidth::I64(i) => i,
            _ => panic!("unwrap_to_i64 on {:?}", self),
        }
    }
}

// Given a function, find values of its inputs such that it returns zero
// Assumes function takes (some number of) integer arguments and returns an integer
// Returns None if there are no values of the inputs such that the function returns zero
fn find_zero_of_func(func: FunctionValue) -> Option<Vec<IntOfSomeWidth>> {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let mut state = State::new(&ctx);

    let params: Vec<BasicValueEnum> = ParamsIterator::new(func).collect();
    for &param in params.iter() {
        assert!(param.is_int_value());
        let width = param.as_int_value().get_type().get_bit_width();
        let z3param = ctx.named_bitvector_const(&get_value_name(param), width);
        state.add_var(param, z3param);
    }

    let returnwidth = func.get_type()
        .get_return_type()
        .expect("Expected function to have return type")
        .into_int_type()
        .get_bit_width();
    let zero = z3::Ast::bitvector_from_u64(&ctx, 0, returnwidth);

    let mut optionz3rval = Some(symex_function(&mut state, func));
    loop {
        let z3rval = optionz3rval.clone().expect("optionz3rval should always be Some at this point in the loop");
        state.assert(&z3rval._eq(&zero));
        if state.check() { break; }
        optionz3rval = symex_again(&mut state);
        if optionz3rval.is_none() { break; }
    }

    if optionz3rval.is_some() {
        // in this case state.check() must have passed
        let model = state.get_model();
        let z3params = params.iter().map(|&p| state.lookup_var(p));
        Some(z3params.map(|p| {
            let param_as_i64 = model.eval(&p).unwrap().as_i64().unwrap();
            match returnwidth {
                8 => IntOfSomeWidth::I8(param_as_i64 as i8),
                16 => IntOfSomeWidth::I16(param_as_i64 as i16),
                32 => IntOfSomeWidth::I32(param_as_i64 as i32),
                64 => IntOfSomeWidth::I64(param_as_i64 as i64),
                _ => unimplemented!("Return type with bitwidth {}", returnwidth),
            }
        }).collect())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::Wrapping;

    fn get_module() -> Module {
        Module::parse_bitcode_from_path(&Path::new("c_examples/basic/basic.bc"))
            .expect("Failed to parse module")
    }

    #[test]
    fn no_args_nozero() {
        let module = get_module();
        let func = module.get_function("no_args_nozero").expect("Failed to find function");
        assert_eq!(find_zero_of_func(func), None);
    }

    #[test]
    fn no_args_zero() {
        let module = get_module();
        let func = module.get_function("no_args_zero").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn one_arg() {
        let module = get_module();
        let func = module.get_function("one_arg").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 1);
        let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn two_args() {
        let module = get_module();
        let func = module.get_function("two_args").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn three_args() {
        let module = get_module();
        let func = module.get_function("three_args").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 3);
        let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn four_args() {
        let module = get_module();
        let func = module.get_function("four_args").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 4);
        let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn five_args() {
        let module = get_module();
        let func = module.get_function("five_args").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 5);
        let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn binops() {
        let module = get_module();
        let func = module.get_function("binops").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let a = Wrapping(args[0].unwrap_to_i32());
        let b = Wrapping(args[1].unwrap_to_i32());
        println!("a = {}, b = {}", a, b);
        let c = a + b - (Wrapping(77) * a) + Wrapping(1);
        let d = (c & Wrapping(23)) / (a | Wrapping(99));
        let e = (d ^ a) % (c << 3);
        assert_eq!((e >> (d.0 as usize)).0, 0);
    }

    #[test]
    fn conditional_true() {
        let module = get_module();
        let func = module.get_function("conditional_true").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let a = Wrapping(args[0].unwrap_to_i32());
        let b = Wrapping(args[1].unwrap_to_i32());
        println!("a = {}, b = {}", a, b);
        let c = if a > b { (a - Wrapping(1)) * (b - Wrapping(1)) } else { (a + b) % Wrapping(3) + Wrapping(10) };
        assert_eq!(c.0, 0);
    }

    #[test]
    fn conditional_false() {
        let module = get_module();
        let func = module.get_function("conditional_false").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let a = Wrapping(args[0].unwrap_to_i32());
        let b = Wrapping(args[1].unwrap_to_i32());
        println!("a = {}, b = {}", a, b);
        let c = if a > b { (a + b) % Wrapping(3) + Wrapping(10) } else { (a - Wrapping(1)) * (b - Wrapping(1)) };
        assert_eq!(c.0, 0);
    }


    #[test]
    fn conditional_nozero() {
        let module = get_module();
        let func = module.get_function("conditional_nozero").expect("Failed to find function");
        assert_eq!(find_zero_of_func(func), None);
    }

    #[test]
    fn int8t() {
        let module = get_module();
        let func = module.get_function("int8t").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let sum: i8 = args.iter().map(|a| a.unwrap_to_i8()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn int16t() {
        let module = get_module();
        let func = module.get_function("int16t").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let sum: i16 = args.iter().map(|a| a.unwrap_to_i16()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn int32t() {
        let module = get_module();
        let func = module.get_function("int32t").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let sum: i32 = args.iter().map(|a| a.unwrap_to_i32()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn int64t() {
        let module = get_module();
        let func = module.get_function("int64t").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let sum: i64 = args.iter().map(|a| a.unwrap_to_i64()).sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn mixed_bitwidths() {
        let module = get_module();
        let func = module.get_function("mixed_bitwidths").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 4);
        let sum: i64 = (args[0].unwrap_to_i8() as i32 + args[1].unwrap_to_i16() as i32 + args[2].unwrap_to_i32()) as i64 + args[3].unwrap_to_i64();
        assert_eq!(sum, 3);
    }
}
