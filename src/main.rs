use inkwell::module::Module;
use inkwell::values::*;
use std::collections::HashMap;
use std::path::Path;
use z3;

fn main() {
    let filepath = Path::new("/Users/craig/pitchfork-rs/c_examples/basic/basic.bc");
    let llvm_mod = Module::parse_bitcode_from_path(&filepath).expect("Failed to parse module");
    let mut o_func = llvm_mod.get_first_function();
    while let Some(func) = o_func {
        println!("Finding zero of function {:?}...", func.get_name());
        let z3cfg = z3::Config::new();
        let z3ctx = z3::Context::new(&z3cfg);
        if let Some((arg1, arg2)) = find_zero_of_func(&z3ctx, func) {
            println!("Function returns zero for arguments {}, {}", arg1, arg2);
        } else {
            println!("Function never returns zero for any values of the arguments");
        }
        o_func = func.get_next_function();
    }
}

// Given a function, find values of its inputs such that it returns zero
// Assumes function takes precisely two i32 arguments and returns an i32
// Returns None if there are no values of the inputs such that the function returns zero
fn find_zero_of_func(z3ctx: &z3::Context, func: FunctionValue) -> Option<(i32, i32)> {
    let mut vars: HashMap<AnyValueEnum, z3::Ast> = HashMap::new();

    assert_eq!(func.count_params(), 2);
    let param1: IntValue = func.get_first_param().unwrap().into_int_value();
    let param2: IntValue = func.get_last_param().unwrap().into_int_value();
    let param1name = get_value_name(&param1);
    let param2name = get_value_name(&param2);
    let z3param1 = z3ctx.named_bitvector_const(&param1name, 32);
    let z3param2 = z3ctx.named_bitvector_const(&param2name, 32);
    vars.insert(param1.as_any_value_enum(), z3param1);
    vars.insert(param2.as_any_value_enum(), z3param2);

    let solver = z3::Solver::new(z3ctx);

    let bb = func.get_entry_basic_block().unwrap();
    let mut inst = bb.get_first_instruction().unwrap();
    let term = bb.get_terminator().unwrap();
    while inst != term {
        if let Some(z3binop) = opcode_to_binop(&inst.get_opcode()) {
            symex_binop(z3ctx, &solver, &inst, &mut vars, z3binop);
        } else {
            unimplemented!("Instruction {:?}", inst.get_opcode());
        }
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

    //println!("Solving constraints\n{}", solver);
    if solver.check() {
        let model = solver.get_model();
        let z3param1 = lookup_ast_for_llvmvalue(&param1, &vars);
        let z3param2 = lookup_ast_for_llvmvalue(&param2, &vars);
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

fn opcode_to_binop<'ctx>(opcode: &InstructionOpcode) -> Option<Box<FnOnce(&z3::Ast<'ctx>, &z3::Ast<'ctx>) -> z3::Ast<'ctx>>> {
    match opcode {
        InstructionOpcode::Add => Some(Box::new(z3::Ast::bvadd)),
        InstructionOpcode::Sub => Some(Box::new(z3::Ast::bvsub)),
        InstructionOpcode::Mul => Some(Box::new(z3::Ast::bvmul)),
        InstructionOpcode::UDiv => Some(Box::new(z3::Ast::bvudiv)),
        InstructionOpcode::SDiv => Some(Box::new(z3::Ast::bvsdiv)),
        InstructionOpcode::URem => Some(Box::new(z3::Ast::bvurem)),
        InstructionOpcode::SRem => Some(Box::new(z3::Ast::bvsrem)),
        InstructionOpcode::And => Some(Box::new(z3::Ast::bvand)),
        InstructionOpcode::Or => Some(Box::new(z3::Ast::bvor)),
        InstructionOpcode::Xor => Some(Box::new(z3::Ast::bvxor)),
        InstructionOpcode::Shl => Some(Box::new(z3::Ast::bvshl)),
        InstructionOpcode::LShr => Some(Box::new(z3::Ast::bvlshr)),
        InstructionOpcode::AShr => Some(Box::new(z3::Ast::bvashr)),
        _ => None,
    }
}

fn symex_binop<'ctx, F>(ctx: &'ctx z3::Context, solver: &z3::Solver, inst: &InstructionValue, vars: &mut HashMap<AnyValueEnum, z3::Ast<'ctx>>, z3op: F)
    where F: FnOnce(&z3::Ast<'ctx>, &z3::Ast<'ctx>) -> z3::Ast<'ctx>
{
    assert_eq!(inst.get_num_operands(), 2);
    let dest = get_dest_name(&inst);
    let z3dest = ctx.named_bitvector_const(&dest, 32);
    let firstop = inst.get_operand(0).unwrap().left().unwrap();
    let secondop = inst.get_operand(1).unwrap().left().unwrap();
    if !firstop.is_int_value() {
        unimplemented!("Add with operands other than integers");
    }
    if !secondop.is_int_value() {
        unimplemented!("Add with operands other than integers");
    }
    let z3firstop = intval_to_ast(&firstop.into_int_value(), ctx, vars);
    let z3secondop = intval_to_ast(&secondop.into_int_value(), ctx, vars);
    solver.assert(&z3dest._eq(&z3op(&z3firstop, &z3secondop)));
    vars.insert(inst.as_any_value_enum(), z3dest);
}

// Convert an IntValue to the appropriate z3::Ast, looking it up in the HashMap if necessary
fn intval_to_ast<'ctx>(v: &IntValue, ctx: &'ctx z3::Context, vars: &HashMap<AnyValueEnum, z3::Ast<'ctx>>) -> z3::Ast<'ctx> {
    if v.is_const() {
        // TODO: don't assume all constants are 32-bit
        z3::Ast::bitvector_from_u64(&ctx, v.get_zero_extended_constant().unwrap(), 32)
    } else {
        lookup_ast_for_llvmvalue(v, vars).clone()
    }
}

fn lookup_ast_for_llvmvalue<'a, 'ctx, V>(v: &V, vars: &'a HashMap<AnyValueEnum, z3::Ast<'ctx>>) -> &'a z3::Ast<'ctx>
    where V: AnyValue
{
    vars.get(&v.as_any_value_enum()).unwrap_or_else(|| {
        let keys: Vec<&AnyValueEnum> = vars.keys().collect();
        panic!("Failed to find value {:?} in map with keys {:?}", v, keys);
    })
}
