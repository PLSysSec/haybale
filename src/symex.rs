use inkwell::basic_block::BasicBlock;
use inkwell::values::*;
use log::debug;

use crate::iterators::*;
use crate::state::State;
use crate::utils::*;

// Symex the given function and return the new AST representing its return value.
// Assumes that the function's parameters are already added to the state.
pub fn symex_function<'ctx>(state: &mut State<'ctx>, func: FunctionValue) -> z3::Ast<'ctx> {
    debug!("Symexing function {}", get_func_name(func));
    let bb = func.get_entry_basic_block().expect("Failed to get entry basic block");
    symex_from_bb(state, bb, None)
}

// TODO: Feels hacky, make better
// After calling symex_function() on the State once, caller can then call
// symex_again() to get a different solution (a different State and a
// different returned AST), or None if there are no more possibilities.
pub fn symex_again<'ctx>(state: &mut State<'ctx>) -> Option<z3::Ast<'ctx>> {
    if let Some((bb, prev_bb)) = state.revert_to_backtracking_point() {
        Some(symex_from_bb(state, bb, Some(prev_bb)))
    } else {
        None
    }
}

// Symex the given bb, through the rest of the function.
// Returns the new AST representing the return value of the function.
fn symex_from_bb<'ctx>(state: &mut State<'ctx>, bb: BasicBlock, prev_bb: Option<BasicBlock>) -> z3::Ast<'ctx> {
    debug!("Symexing basic block {}", get_bb_name(bb));
    let insts = InstructionIterator::new(&bb);
    for inst in insts {
        let opcode = inst.get_opcode();
        if let Some(z3binop) = opcode_to_binop(&opcode) {
            symex_binop(state, inst, z3binop);
        } else if opcode == InstructionOpcode::ICmp {
            symex_icmp(state, inst);
        } else if opcode == InstructionOpcode::Phi {
            symex_phi(state, inst, prev_bb);
        } else if opcode == InstructionOpcode::Select {
            symex_select(state, inst);
        } else if opcode == InstructionOpcode::Return {
            return symex_return(state, inst);
        } else if opcode == InstructionOpcode::Br {
            return symex_br(state, inst, bb);
        } else {
            unimplemented!("instruction {:?}", opcode);
        }
    }
    panic!("No terminator found in function");
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
        inkwell::IntPredicate::EQ => Box::new(z3::Ast::_eq),
        inkwell::IntPredicate::NE => Box::new(|a,b| z3::Ast::not(&z3::Ast::_eq(a,b))),
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
    debug!("Symexing binop {}", &get_value_name(inst));
    assert_eq!(inst.get_num_operands(), 2);
    let firstop = inst.get_operand(0).unwrap().left().unwrap();
    let secondop = inst.get_operand(1).unwrap().left().unwrap();
    let z3firstop = state.operand_to_ast(firstop);
    let z3secondop = state.operand_to_ast(secondop);
    let width = firstop.get_type().as_int_type().get_bit_width();
    assert_eq!(width, secondop.get_type().as_int_type().get_bit_width());
    let dest = get_dest_name(inst);
    let z3dest = state.ctx.named_bitvector_const(&dest, width);
    state.assert(&z3dest._eq(&z3op(&z3firstop, &z3secondop)));
    state.add_var(inst, z3dest);
}

fn symex_icmp(state: &mut State, inst: InstructionValue) {
    debug!("Symexing icmp {}", &get_value_name(inst));
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
    debug!("Symexing return {}", &get_value_name(inst));
    assert_eq!(inst.get_num_operands(), 1);
    let rval = inst.get_operand(0).unwrap().left().unwrap();
    state.operand_to_ast(rval)
}

// Continues to the target of the Br (saving a backtracking point if necessary)
// and eventually returns the new AST representing the return value of the function
// (when it reaches the end of the function)
fn symex_br<'ctx>(state: &mut State<'ctx>, inst: InstructionValue, cur_bb: BasicBlock) -> z3::Ast<'ctx> {
    debug!("Symexing branch {}", &get_value_name(inst));
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
            // From empirical evidence, I guess get_operand(1) is the false branch and get_operand(2) is the true branch?
            let false_branch = 1;
            let true_branch = 2;
            if true_feasible && false_feasible {
                // for now we choose to explore true first, and backtrack to false if necessary
                state.save_backtracking_point(inst.get_operand(false_branch).unwrap().right().unwrap(), cur_bb, z3cond.not());
                state.assert(&z3cond);
                symex_from_bb(state, inst.get_operand(true_branch).unwrap().right().unwrap(), Some(cur_bb))
            } else if true_feasible {
                state.assert(&z3cond);  // unnecessary, but may help Z3 more than it hurts?
                symex_from_bb(state, inst.get_operand(true_branch).unwrap().right().unwrap(), Some(cur_bb))
            } else if false_feasible {
                state.assert(&z3cond.not());  // unnecessary, but may help Z3 more than it hurts?
                symex_from_bb(state, inst.get_operand(false_branch).unwrap().right().unwrap(), Some(cur_bb))
            } else if let Some((bb, prev_bb)) = state.revert_to_backtracking_point() {
                symex_from_bb(state, bb, Some(prev_bb))
            } else {
                panic!("All possible paths seem to be unsat");
            }
        },
        n => { unimplemented!("Br with {} operands", n); },
    }
}

fn symex_phi(state: &mut State, inst: InstructionValue, prev_bb: Option<BasicBlock>) {
    let inst: PhiValue = unsafe { std::mem::transmute(inst) };  // This InstructionValue is actually a PhiValue, but the current inkwell type system doesn't express this (?) so this seems to be the way to do it (?)
    debug!("Symexing phi {}", &get_value_name(inst));
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

fn symex_select(state: &mut State, inst: InstructionValue) {
    debug!("Symexing select {}", &get_value_name(inst));
    assert_eq!(inst.get_num_operands(), 3);
    let cond = inst.get_operand(0).unwrap().left().unwrap();
    let firstop = inst.get_operand(1).unwrap().left().unwrap();
    let secondop = inst.get_operand(2).unwrap().left().unwrap();
    let z3cond = state.operand_to_ast(cond);
    let z3firstop = state.operand_to_ast(firstop);
    let z3secondop = state.operand_to_ast(secondop);
    let width = firstop.get_type().as_int_type().get_bit_width();
    assert_eq!(width, secondop.get_type().as_int_type().get_bit_width());
    let dest = get_dest_name(inst);
    let z3dest = state.ctx.named_bitvector_const(&dest, width);
    let true_feasible = state.check_with_extra_constraints(&[&z3cond]);
    let false_feasible = state.check_with_extra_constraints(&[&z3cond.not()]);
    if true_feasible && false_feasible {
        state.assert(&z3dest._eq(&z3::Ast::ite(&z3cond, &z3firstop, &z3secondop)));
    } else if true_feasible {
        state.assert(&z3cond);  // unnecessary, but may help Z3 more than it hurts?
        state.assert(&z3dest._eq(&z3firstop));
    } else if false_feasible {
        state.assert(&z3cond.not());  // unnecessary, but may help Z3 more than it hurts?
        state.assert(&z3dest._eq(&z3secondop));
    } else {
        unimplemented!("discovered we're unsat while checking a switch condition");
    }
    state.add_var(inst, z3dest);
}
