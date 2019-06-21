use inkwell::module::Module;
use inkwell::values::*;
use std::collections::HashMap;
use std::path::Path;
use z3;

mod iterators;
use iterators::*;

type VarMap<'ctx> = HashMap<AnyValueEnum, z3::Ast<'ctx>>;

fn main() {
    let filepath = Path::new("/Users/craig/pitchfork-rs/c_examples/basic/basic.bc");
    let llvm_mod = Module::parse_bitcode_from_path(&filepath).expect("Failed to parse module");
    let functions = FunctionIterator::new(&llvm_mod);
    for func in functions {
        println!("Finding zero of function {:?}...", func.get_name());
        let z3cfg = z3::Config::new();
        let z3ctx = z3::Context::new(&z3cfg);
        if let Some(args) = find_zero_of_func(&z3ctx, func) {
            match func.count_params() {
                0 => println!("Function returns zero when passed no arguments\n"),
                1 => println!("Function returns zero when passed the argument {}\n", args[0]),
                _ => println!("Function returns zero when passed arguments {:?}\n", args),
            }
        } else {
            println!("Function never returns zero for any values of the arguments\n");
        }
    }
}

// Given a function, find values of its inputs such that it returns zero
// Assumes function takes (some number of) i32 arguments and returns an i32
// Returns None if there are no values of the inputs such that the function returns zero
fn find_zero_of_func(z3ctx: &z3::Context, func: FunctionValue) -> Option<Vec<i32>> {
    let mut vars: VarMap = HashMap::new();

    let params: Vec<IntValue> = ParamsIterator::new(func)
        .map(|p| p.into_int_value())
        .collect();
    let paramnames = params.iter().cloned().map(get_value_name);  // 'cloned()' can become 'copied()' once Rust 1.36 hits stable
    let z3params = paramnames.map(|s| z3ctx.named_bitvector_const(&s, 32));
    for (param, z3ast) in Iterator::zip(params.iter(), z3params) {
        vars.insert(param.as_any_value_enum(), z3ast);
    }

    let solver = z3::Solver::new(z3ctx);

    let z3rval = symex_function(z3ctx, &solver, func, &mut vars);
    let zero = z3::Ast::bitvector_from_u64(z3ctx, 0, 32);
    solver.assert(&z3rval._eq(&zero));

    //println!("Solving constraints\n{}", solver);
    if solver.check() {
        let model = solver.get_model();
        let z3params = params.iter().map(|p| lookup_ast_for_llvmvalue(*p, &vars));
        let params = z3params.map(|p| model.eval(&p).unwrap().as_i64().unwrap() as i32);
        Some(params.collect())
    } else {
        None
    }
}

// Symex the given function and return the new AST representing its return value.
// Assumes that the function's parameters are already inserted into the VarMap.
fn symex_function<'ctx>(z3ctx: &'ctx z3::Context, solver: &z3::Solver, func: FunctionValue, vars: &mut VarMap<'ctx>) -> z3::Ast<'ctx> {
    let bb = func.get_entry_basic_block().unwrap();
    let insts = InstructionIterator::new(&bb);
    for inst in insts {
        let opcode = inst.get_opcode();
        if let Some(z3binop) = opcode_to_binop(&opcode) {
            symex_binop(z3ctx, &solver, inst, vars, z3binop);
        } else if opcode == InstructionOpcode::ICmp {
            symex_icmp(z3ctx, &solver, inst, vars);
        } else if opcode == InstructionOpcode::Return {
            return symex_return(z3ctx, inst, vars);
        } else {
            unimplemented!("Instruction {:?}", opcode);
        }
    }
    panic!("No terminator found in function");
}

fn get_value_name(v: IntValue) -> String {
    // get_name() is empty for values like '%0', unfortunately
    v.print_to_string().to_string()
}

fn get_dest_name(inst: InstructionValue) -> String {
    // seems like there should be a more efficient way?
    let bve: BasicValueEnum = inst.get_first_use().unwrap().get_used_value().left().unwrap();
    if !bve.is_int_value() {
        unimplemented!("Instruction producing value other than integer");
    }
    get_value_name(bve.into_int_value())
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

fn intpred_to_z3pred<'ctx>(pred: inkwell::IntPredicate) -> Box<FnOnce(&z3::Ast<'ctx>, &z3::Ast<'ctx>) -> z3::Ast<'ctx>> {
    match pred {
        inkwell::IntPredicate::EQ => Box::new(|a,b| z3::Ast::not(&z3::Ast::_eq(a,b))),
        inkwell::IntPredicate::NE => Box::new(z3::Ast::_eq),
        inkwell::IntPredicate::UGT => Box::new(z3::Ast::bvugt),
        inkwell::IntPredicate::UGE => Box::new(z3::Ast::bvuge),
        inkwell::IntPredicate::ULT => Box::new(z3::Ast::bvult),
        inkwell::IntPredicate::ULE => Box::new(z3::Ast::bvule),
        inkwell::IntPredicate::SGT => Box::new(z3::Ast::bvsgt),
        inkwell::IntPredicate::SGE => Box::new(z3::Ast::bvsge),
        inkwell::IntPredicate::SLT => Box::new(z3::Ast::bvslt),
        inkwell::IntPredicate::SLE => Box::new(z3::Ast::bvsle),
    }
}

fn symex_binop<'ctx, F>(ctx: &'ctx z3::Context, solver: &z3::Solver, inst: InstructionValue, vars: &mut VarMap<'ctx>, z3op: F)
    where F: FnOnce(&z3::Ast<'ctx>, &z3::Ast<'ctx>) -> z3::Ast<'ctx>
{
    assert_eq!(inst.get_num_operands(), 2);
    let dest = get_dest_name(inst);
    let z3dest = ctx.named_bitvector_const(&dest, 32);
    let firstop = inst.get_operand(0).unwrap().left().unwrap();
    let secondop = inst.get_operand(1).unwrap().left().unwrap();
    if !firstop.is_int_value() {
        unimplemented!("Binop with operands other than integers");
    }
    if !secondop.is_int_value() {
        unimplemented!("Binop with operands other than integers");
    }
    let z3firstop = intval_to_ast(firstop.into_int_value(), ctx, vars);
    let z3secondop = intval_to_ast(secondop.into_int_value(), ctx, vars);
    solver.assert(&z3dest._eq(&z3op(&z3firstop, &z3secondop)));
    vars.insert(inst.as_any_value_enum(), z3dest);
}

fn symex_icmp<'ctx>(ctx: &'ctx z3::Context, solver: &z3::Solver, inst: InstructionValue, vars: &mut VarMap<'ctx>) {
    assert_eq!(inst.get_num_operands(), 2);
    let dest = get_dest_name(inst);
    let z3dest = ctx.named_bool_const(&dest);
    let firstop = inst.get_operand(0).unwrap().left().unwrap();
    let secondop = inst.get_operand(1).unwrap().left().unwrap();
    if !firstop.is_int_value() {
        unimplemented!("ICmp with operands other than integers");
    }
    if !secondop.is_int_value() {
        unimplemented!("ICmp with operands other than integers");
    }
    let z3firstop = intval_to_ast(firstop.into_int_value(), ctx, vars);
    let z3secondop = intval_to_ast(secondop.into_int_value(), ctx, vars);
    let z3pred = intpred_to_z3pred(inst.get_icmp_predicate().unwrap());
    solver.assert(&z3dest._eq(&z3pred(&z3firstop, &z3secondop)));
    vars.insert(inst.as_any_value_enum(), z3dest);
}

// Returns the z3::Ast representing the return value
fn symex_return<'ctx>(ctx: &'ctx z3::Context, inst: InstructionValue, vars: &mut VarMap<'ctx>) -> z3::Ast<'ctx> {
    assert_eq!(inst.get_num_operands(), 1);
    let rval = inst.get_operand(0).unwrap().left().unwrap();
    if !rval.is_int_value() {
        unimplemented!("Returning a non-integer value");
    }
    intval_to_ast(rval.into_int_value(), ctx, &vars)
}

// Convert an IntValue to the appropriate z3::Ast, looking it up in the VarMap if necessary
fn intval_to_ast<'ctx>(v: IntValue, ctx: &'ctx z3::Context, vars: &VarMap<'ctx>) -> z3::Ast<'ctx> {
    if v.is_const() {
        // TODO: don't assume all constants are 32-bit
        z3::Ast::bitvector_from_u64(&ctx, v.get_zero_extended_constant().unwrap(), 32)
    } else {
        lookup_ast_for_llvmvalue(v, vars).clone()
    }
}

fn lookup_ast_for_llvmvalue<'a, 'ctx>(v: impl AnyValue, vars: &'a VarMap<'ctx>) -> &'a z3::Ast<'ctx>
{
    vars.get(&v.as_any_value_enum()).unwrap_or_else(|| {
        let keys: Vec<&AnyValueEnum> = vars.keys().collect();
        panic!("Failed to find value {:?} in map with keys {:?}", v, keys);
    })
}
