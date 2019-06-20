use inkwell::module::Module;
use inkwell::values::*;
use std::collections::HashMap;
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
    let mut vars: HashMap<String, z3::Ast> = HashMap::new();

    assert_eq!(func.count_params(), 2);
    let param1: IntValue = func.get_first_param().unwrap().into_int_value();
    let param2: IntValue = func.get_last_param().unwrap().into_int_value();
    let param1name = get_value_name(&param1);
    let param2name = get_value_name(&param2);
    let z3param1 = z3ctx.named_bitvector_const(&param1name, 32);
    let z3param2 = z3ctx.named_bitvector_const(&param2name, 32);
    vars.insert(param1name.clone(), z3param1);
    vars.insert(param2name.clone(), z3param2);

    let solver = z3::Solver::new(&z3ctx);

    let bb = func.get_entry_basic_block().unwrap();
    let mut inst = bb.get_first_instruction().unwrap();
    let term = bb.get_terminator().unwrap();
    while inst != term {
        let (destname, z3dest) = match inst.get_opcode() {
            InstructionOpcode::Add => {
                assert_eq!(inst.get_num_operands(), 2);
                let firstop = inst.get_operand(0).unwrap().left().unwrap();
                let secondop = inst.get_operand(1).unwrap().left().unwrap();
                if !firstop.is_int_value() {
                    unimplemented!("Add with operands other than integers");
                }
                if !secondop.is_int_value() {
                    unimplemented!("Add with operands other than integers");
                }
                let z3firstop = intval_to_ast(&firstop.into_int_value(), z3ctx, &vars);
                let z3secondop = intval_to_ast(&secondop.into_int_value(), z3ctx, &vars);
                let dest = get_dest_name(&inst);
                let z3dest = z3ctx.named_bitvector_const(&dest, 32);
                solver.assert(&z3dest._eq(&z3firstop.bvadd(&z3secondop)));
                (dest, z3dest)
            }
            _ => {
                unimplemented!("Instruction other than Add");
            }
        };
        vars.insert(destname, z3dest);
        inst = inst.get_next_instruction().unwrap();
    }
    match term.get_opcode() {
        InstructionOpcode::Return => {
            assert_eq!(term.get_num_operands(), 1);
            let rval = term.get_operand(0).unwrap().left().unwrap();
            if !rval.is_int_value() {
                unimplemented!("Returning a non-integer value");
            }
            let z3rval = intval_to_ast(&rval.into_int_value(), z3ctx, &vars);
            let zero = z3::Ast::bitvector_from_u64(z3ctx, 0, 32);
            solver.assert(&z3rval._eq(&zero));
        }
        _ => {
            unimplemented!("Terminator other than Return");
        }
    }

    if solver.check() {
        let model = solver.get_model();
        let z3param1 = vars.get(&param1name).unwrap_or_else(|| {
            let keys: Vec<&String> = vars.keys().collect();
            panic!("failed to find parameter {} in map with keys {:?}", param1name, keys);
        });
        let z3param2 = vars.get(&param2name).unwrap_or_else(|| {
            let keys: Vec<&String> = vars.keys().collect();
            panic!("failed to find parameter {} in map with keys {:?}", param2name, keys);
        });
        let param1 = model.eval(&z3param1).unwrap().as_i64().unwrap();
        let param2 = model.eval(&z3param2).unwrap().as_i64().unwrap();
        Some((param1 as i32, param2 as i32))
    } else {
        None
    }
}

fn get_value_name(v: &IntValue) -> String {
    // get_name() is empty for values like '%0', unfortunately
    v.print_to_string().to_string()
}

fn get_dest_name(inst: &InstructionValue) -> String {
    // seems like there should be a more efficient way?
    let bve: BasicValueEnum = inst.get_first_use().unwrap().get_used_value().left().unwrap();
    if !bve.is_int_value() {
        unimplemented!("Instruction producing value other than integer");
    }
    get_value_name(&bve.into_int_value())
}

// Convert an IntValue to the appropriate z3::Ast, looking it up in the HashMap if necessary
fn intval_to_ast<'a>(v: &IntValue, ctx: &'a z3::Context, vars: &'a HashMap<String, z3::Ast>) -> z3::Ast<'a> {
    if v.is_const() {
        // TODO: don't assume all constants are 32-bit
        z3::Ast::bitvector_from_u64(&ctx, v.get_zero_extended_constant().unwrap(), 32)
    } else {
        vars.get(&get_value_name(v)).unwrap_or_else(|| {
            let keys: Vec<&String> = vars.keys().collect();
            panic!("Failed to find value {:?} with name {} in map with keys {:?}", v, &get_value_name(v), keys);
        }).clone()
    }
}
