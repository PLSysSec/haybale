use inkwell::basic_block::BasicBlock;
use inkwell::module::Module;
use inkwell::values::*;
use std::path::Path;

mod iterators;
use iterators::*;

mod state;
use state::State;

fn main() {
    let filepath = Path::new("c_examples/basic/basic.bc");
    let llvm_mod = Module::parse_bitcode_from_path(&filepath).expect("Failed to parse module");
    let functions = FunctionIterator::new(&llvm_mod);
    for func in functions {
        println!("Finding zero of function {:?}...", func.get_name());
        if let Some(args) = find_zero_of_func(func) {
            assert_eq!(args.len(), func.count_params() as usize);
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
fn find_zero_of_func(func: FunctionValue) -> Option<Vec<i32>> {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let mut state = State::new(&ctx);

    let params: Vec<BasicValueEnum> = ParamsIterator::new(func).collect();
    for &param in params.iter() {
        // TODO: don't assume all parameters are 32-bit
        let z3param = ctx.named_bitvector_const(&get_value_name(param), 32);
        state.add_var(param, z3param);
    }

    let z3rval = symex_function(&mut state, func);
    let zero = z3::Ast::bitvector_from_u64(&ctx, 0, 32);
    state.assert(&z3rval._eq(&zero));

    //println!("Solving constraints:");
    //state.prettyprint_constraints();
    if state.check() {
        let model = state.get_model();
        let z3params = params.iter().map(|&p| state.lookup_var(p));
        Some(z3params.map(|p| model.eval(&p).unwrap().as_i64().unwrap() as i32).collect())
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
        let sum: i32 = args.iter().sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn two_args() {
        let module = get_module();
        let func = module.get_function("two_args").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let sum: i32 = args.iter().sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn three_args() {
        let module = get_module();
        let func = module.get_function("three_args").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 3);
        let sum: i32 = args.iter().sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn four_args() {
        let module = get_module();
        let func = module.get_function("four_args").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 4);
        let sum: i32 = args.iter().sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn five_args() {
        let module = get_module();
        let func = module.get_function("five_args").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 5);
        let sum: i32 = args.iter().sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn binops() {
        let module = get_module();
        let func = module.get_function("binops").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let a = Wrapping(args[0]);
        let b = Wrapping(args[1]);
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
        let a = Wrapping(args[0]);
        let b = Wrapping(args[1]);
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
        let a = Wrapping(args[0]);
        let b = Wrapping(args[1]);
        println!("a = {}, b = {}", a, b);
        let c = if a > b { (a + b) % Wrapping(3) + Wrapping(10) } else { (a - Wrapping(1)) * (b - Wrapping(1)) };
        assert_eq!(c.0, 0);
    }


    #[test]
    fn conditional_nozero() {
        let module = get_module();
        let func = module.get_function("conditional_nozero").expect("Failed to find function");
        let args = find_zero_of_func(func).expect("Failed to find zero of the function");
        assert_eq!(args.len(), 2);
        let a = Wrapping(args[0]);
        let b = Wrapping(args[1]);
        let c = if a < Wrapping(2) { a } else if b == Wrapping(0) { b + Wrapping(3) } else { a * b };
        assert_eq!(c.0, 0);
    }
}

// Symex the given function and return the new AST representing its return value.
// Assumes that the function's parameters are already added to the state.
fn symex_function<'ctx>(state: &mut State<'ctx>, func: FunctionValue) -> z3::Ast<'ctx> {
    let bb = func.get_entry_basic_block().unwrap();
    symex_from_bb(state, bb, None)
}

// Symex the given bb, through the rest of the function.
// Returns the new AST representing the return value of the function.
fn symex_from_bb<'ctx>(state: &mut State<'ctx>, bb: BasicBlock, prev_bb: Option<BasicBlock>) -> z3::Ast<'ctx> {
    let insts = InstructionIterator::new(&bb);
    for inst in insts {
        let opcode = inst.get_opcode();
        if let Some(z3binop) = opcode_to_binop(&opcode) {
            symex_binop(state, inst, z3binop);
        } else if opcode == InstructionOpcode::ICmp {
            symex_icmp(state, inst);
        } else if opcode == InstructionOpcode::Phi {
            symex_phi(state, inst, prev_bb);
        } else if opcode == InstructionOpcode::Return {
            return symex_return(state, inst);
        } else if opcode == InstructionOpcode::Br {
            return symex_br(state, inst, bb);
        } else {
            unimplemented!("Instruction {:?}", opcode);
        }
    }
    panic!("No terminator found in function");
}

// In most cases get_name() would seem appropriate instead of this function,
// but unfortunately, get_name() is empty for values like '%0'
fn get_value_name(v: impl AnyValue) -> String {
    match v.as_any_value_enum() {
        AnyValueEnum::ArrayValue(av) => {
            av.print_to_string().to_string()
        }
        AnyValueEnum::IntValue(iv) => {
            iv.print_to_string().to_string()
        },
        AnyValueEnum::FloatValue(fv) => {
            fv.print_to_string().to_string()
        },
        AnyValueEnum::PhiValue(pv) => {
            pv.print_to_string().to_string()
        },
        AnyValueEnum::FunctionValue(fv) => {
            let rval = fv.get_name().to_str().expect("Failed to convert from CStr").to_owned();
            assert_ne!(rval, "");
            rval
        },
        AnyValueEnum::PointerValue(pv) => {
            pv.print_to_string().to_string()
        },
        AnyValueEnum::StructValue(sv) => {
            sv.print_to_string().to_string()
        },
        AnyValueEnum::VectorValue(vv) => {
            vv.print_to_string().to_string()
        },
        AnyValueEnum::InstructionValue(_) => {
            unimplemented!("get_value_name() for InstructionValue");
        }
    }
}

fn get_dest_name(inst: InstructionValue) -> String {
    // seems like there should be a more efficient way?
    let bve: BasicValueEnum = inst.get_first_use().unwrap().get_used_value().left().unwrap();
    get_value_name(bve)
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

fn symex_binop<'ctx, F>(state: &mut State<'ctx>, inst: InstructionValue, z3op: F)
    where F: FnOnce(&z3::Ast<'ctx>, &z3::Ast<'ctx>) -> z3::Ast<'ctx>
{
    assert_eq!(inst.get_num_operands(), 2);
    let dest = get_dest_name(inst);
    let z3dest = state.ctx.named_bitvector_const(&dest, 32);
    let firstop = inst.get_operand(0).unwrap().left().unwrap();
    let secondop = inst.get_operand(1).unwrap().left().unwrap();
    let z3firstop = state.operand_to_ast(firstop);
    let z3secondop = state.operand_to_ast(secondop);
    state.assert(&z3dest._eq(&z3op(&z3firstop, &z3secondop)));
    state.add_var(inst, z3dest);
}

fn symex_icmp<'ctx>(state: &mut State<'ctx>, inst: InstructionValue) {
    assert_eq!(inst.get_num_operands(), 2);
    let dest = get_dest_name(inst);
    let z3dest = state.ctx.named_bool_const(&dest);
    let firstop = inst.get_operand(0).unwrap().left().unwrap();
    let secondop = inst.get_operand(1).unwrap().left().unwrap();
    let z3firstop = state.operand_to_ast(firstop);
    let z3secondop = state.operand_to_ast(secondop);
    let z3pred = intpred_to_z3pred(inst.get_icmp_predicate().unwrap());
    state.assert(&z3dest._eq(&z3pred(&z3firstop, &z3secondop)));
    state.add_var(inst, z3dest);
}

// Returns the z3::Ast representing the return value
fn symex_return<'ctx>(state: &State<'ctx>, inst: InstructionValue) -> z3::Ast<'ctx> {
    assert_eq!(inst.get_num_operands(), 1);
    let rval = inst.get_operand(0).unwrap().left().unwrap();
    state.operand_to_ast(rval)
}

// Continues to the target of the Br (saving a backtracking point if necessary)
// and eventually returns the new AST representing the return value of the function
// (when it reaches the end of the function)
fn symex_br<'ctx>(state: &mut State<'ctx>, inst: InstructionValue, cur_bb: BasicBlock) -> z3::Ast<'ctx> {
    match inst.get_num_operands() {
        1 => {
            // unconditional branch
            let bb = inst.get_operand(0).unwrap().right().expect("Single-operand Br but operand is not a BasicBlock");
            symex_from_bb(state, bb, Some(cur_bb))
        },
        3 => {
            // conditional branch
            let cond = inst.get_operand(0).unwrap().left().unwrap();
            let z3cond = state.operand_to_ast(cond);
            let true_feasible = state.check_with_extra_constraints(&[&z3cond]);
            let false_feasible = state.check_with_extra_constraints(&[&z3cond.not()]);
            if true_feasible && false_feasible {
                // for now we choose to explore true first, and backtrack to false if necessary
                state.save_backtracking_point(inst.get_operand(2).unwrap().right().unwrap(), cur_bb, z3cond.not());
                symex_from_bb(state, inst.get_operand(1).unwrap().right().unwrap(), Some(cur_bb))
            } else if true_feasible {
                symex_from_bb(state, inst.get_operand(1).unwrap().right().unwrap(), Some(cur_bb))
            } else if false_feasible {
                symex_from_bb(state, inst.get_operand(2).unwrap().right().unwrap(), Some(cur_bb))
            } else if let Some((bb, prev_bb)) = state.revert_to_backtracking_point() {
                symex_from_bb(state, bb, Some(prev_bb))
            } else {
                panic!("All possible paths seem to be unsat");
            }
        },
        n => { unimplemented!("Br with {} operands", n); },
    }
}

fn symex_phi<'ctx>(state: &mut State<'ctx>, inst: InstructionValue, prev_bb: Option<BasicBlock>) {
    let inst: PhiValue = unsafe { std::mem::transmute(inst) };  // This InstructionValue is actually a PhiValue, but the current inkwell type system doesn't express this (?) so this seems to be the way to do it (?)
    let prev_bb = prev_bb.expect("not yet implemented: starting in a block with Phi instructions");
    let pairs = PhiIterator::new(inst);
    let mut chosen_value = None;
    for (bve, bb) in pairs {
        if bb == prev_bb {
            chosen_value = Some(bve);
        }
    }
    let chosen_value = chosen_value.expect("Failed to find a Phi member matching previous BasicBlock");
    let z3value = state.operand_to_ast(chosen_value);
    state.add_var(inst, z3value);
}
