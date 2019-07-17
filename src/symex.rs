use llvm_ir::*;
use llvm_ir::instruction::BinaryOp;
use log::debug;
use z3::ast::{Ast, BV, Bool};
use either::Either;

use crate::state::State;
use crate::size::size;

// Symex the given function and return the new BV (AST) representing its return value, or None if the function returns void.
// Assumes that the function's parameters are already added to the state.
pub fn symex_function<'ctx, 'func>(state: &mut State<'ctx, 'func>, func: &'func Function) -> Option<BV<'ctx>> {
    debug!("Symexing function {}", func.name);
    let bb = func.basic_blocks.get(0).expect("Failed to get entry basic block");
    symex_from_bb(state, bb, func, None)
}

// TODO: Feels hacky, make better
// After calling symex_function() on the State once, caller can then call
// symex_again() to get a different solution (a different State and a
// different returned Option<BV>), or None if there are no more possibilities.
pub fn symex_again<'ctx, 'func>(state: &mut State<'ctx, 'func>) -> Option<Option<BV<'ctx>>> {
    if let Some((func, bb, prev_bb)) = state.revert_to_backtracking_point() {
        Some(symex_from_bb_by_name(state, &bb, func, Some(prev_bb)))
    } else {
        None
    }
}

// Like `symex_from_bb`, but looks up the bb by name
fn symex_from_bb_by_name<'ctx, 'func>(state: &mut State<'ctx, 'func>, bbname: &Name, cur_func: &'func Function, prev_bb: Option<Name>) -> Option<BV<'ctx>> {
    let bb = cur_func.get_bb_by_name(bbname).unwrap_or_else(|| panic!("Failed to find bb named {:?} in function", bbname));
    symex_from_bb(state, bb, cur_func, prev_bb)
}

// Symex the given bb, through the rest of the function.
// Returns the new BV representing the return value of the function, or None if the function returns void.
fn symex_from_bb<'ctx, 'func>(state: &mut State<'ctx, 'func>, bb: &BasicBlock, cur_func: &'func Function, prev_bb: Option<Name>) -> Option<BV<'ctx>> {
    debug!("Symexing basic block {:?}", bb.name);
    for inst in bb.instrs.iter() {
        let result = if let Some((binop, z3binop)) = inst_to_binop(&inst) {
            symex_binop(state, &binop, z3binop)
        } else {
            match inst {
                Instruction::ICmp(icmp) => symex_icmp(state, &icmp),
                Instruction::Load(load) => symex_load(state, &load),
                Instruction::Store(store) => symex_store(state, &store),
                Instruction::GetElementPtr(gep) => symex_gep(state, &gep),
                Instruction::Alloca(alloca) => symex_alloca(state, &alloca),
                Instruction::ZExt(zext) => symex_zext(state, &zext),
                Instruction::SExt(sext) => symex_sext(state, &sext),
                Instruction::Trunc(trunc) => symex_trunc(state, &trunc),
                Instruction::BitCast(bitcast) => symex_bitcast(state, &bitcast),
                Instruction::Phi(phi) => symex_phi(state, &phi, prev_bb.as_ref()),
                Instruction::Select(select) => symex_select(state, &select),
                Instruction::Call(call) => symex_call(state, &call),
                _ => unimplemented!("instruction {:?}", inst),
            }
        };
        if result.is_err() {
            // Having an `Err` here indicates we can't continue down this path,
            // for instance because we're unsat, or because loop bound was exceeded, etc
            if let Some((func, bb, prev_bb)) = state.revert_to_backtracking_point() {
                return symex_from_bb_by_name(state, &bb, func, Some(prev_bb));
            } else {
                // can't continue on this path due to error, and we have nowhere to backtrack to
                panic!("All possible paths seem to be unsat");
            }
        }
    }
    match &bb.term {
        Terminator::Ret(ret) => symex_return(state, ret),
        Terminator::Br(br) => symex_br(state, br, cur_func, bb.name.clone()),
        Terminator::CondBr(condbr) => symex_condbr(state, condbr, cur_func, bb.name.clone()),
        term => unimplemented!("terminator {:?}", term),
    }
}

fn inst_to_binop<'ctx>(inst: &Instruction) -> Option<(instruction::groups::BinaryOp, Box<FnOnce(&BV<'ctx>, &BV<'ctx>) -> BV<'ctx>>)> {
    match inst {
        // TODO: how to not clone the inner instruction here
        Instruction::Add(i) => Some((i.clone().into(), Box::new(BV::bvadd))),
        Instruction::Sub(i) => Some((i.clone().into(), Box::new(BV::bvsub))),
        Instruction::Mul(i) => Some((i.clone().into(), Box::new(BV::bvmul))),
        Instruction::UDiv(i) => Some((i.clone().into(), Box::new(BV::bvudiv))),
        Instruction::SDiv(i) => Some((i.clone().into(), Box::new(BV::bvsdiv))),
        Instruction::URem(i) => Some((i.clone().into(), Box::new(BV::bvurem))),
        Instruction::SRem(i) => Some((i.clone().into(), Box::new(BV::bvsrem))),
        Instruction::And(i) => Some((i.clone().into(), Box::new(BV::bvand))),
        Instruction::Or(i) => Some((i.clone().into(), Box::new(BV::bvor))),
        Instruction::Xor(i) => Some((i.clone().into(), Box::new(BV::bvxor))),
        Instruction::Shl(i) => Some((i.clone().into(), Box::new(BV::bvshl))),
        Instruction::LShr(i) => Some((i.clone().into(), Box::new(BV::bvlshr))),
        Instruction::AShr(i) => Some((i.clone().into(), Box::new(BV::bvashr))),
        _ => None,
    }
}

fn intpred_to_z3pred<'ctx>(pred: IntPredicate) -> Box<FnOnce(&BV<'ctx>, &BV<'ctx>) -> Bool<'ctx>> {
    match pred {
        IntPredicate::EQ => Box::new(|a,b| BV::_eq(a,b)),
        IntPredicate::NE => Box::new(|a,b| Bool::not(&BV::_eq(a,b))),
        IntPredicate::UGT => Box::new(BV::bvugt),
        IntPredicate::UGE => Box::new(BV::bvuge),
        IntPredicate::ULT => Box::new(BV::bvult),
        IntPredicate::ULE => Box::new(BV::bvule),
        IntPredicate::SGT => Box::new(BV::bvsgt),
        IntPredicate::SGE => Box::new(BV::bvsge),
        IntPredicate::SLT => Box::new(BV::bvslt),
        IntPredicate::SLE => Box::new(BV::bvsle),
    }
}

fn symex_binop<'ctx, 'func, F>(state: &mut State<'ctx, 'func>, bop: &instruction::groups::BinaryOp, z3op: F) -> Result<(), &'static str>
    where F: FnOnce(&BV<'ctx>, &BV<'ctx>) -> BV<'ctx>
{
    debug!("Symexing binop {:?}", bop);
    let z3firstop = state.operand_to_bv(&bop.get_operand0());
    let z3secondop = state.operand_to_bv(&bop.get_operand1());
    state.record_bv_result(bop, z3op(&z3firstop, &z3secondop))
}

fn symex_icmp(state: &mut State, icmp: &instruction::ICmp) -> Result<(), &'static str> {
    debug!("Symexing icmp {:?}", icmp);
    let z3firstop = state.operand_to_bv(&icmp.operand0);
    let z3secondop = state.operand_to_bv(&icmp.operand1);
    let z3pred = intpred_to_z3pred(icmp.predicate);
    state.record_bool_result(icmp, z3pred(&z3firstop, &z3secondop))
}

fn symex_zext(state: &mut State, zext: &instruction::ZExt) -> Result<(), &'static str> {
    debug!("Symexing zext {:?}", zext);
    let z3op = state.operand_to_bv(&zext.operand);
    let source_size = z3op.get_size();
    let dest_size = match zext.get_type() {
        Type::IntegerType { bits } => bits,
        Type::VectorType { .. } => unimplemented!("ZExt on vectors"),
        ty => panic!("ZExt result should be integer or vector of integers; got {:?}", ty),
    };
    state.record_bv_result(zext, z3op.zero_ext(dest_size - source_size))
}

fn symex_sext(state: &mut State, sext: &instruction::SExt) -> Result<(), &'static str> {
    debug!("Symexing sext {:?}", sext);
    let z3op = state.operand_to_bv(&sext.operand);
    let source_size = z3op.get_size();
    let dest_size = match sext.get_type() {
        Type::IntegerType { bits } => bits,
        Type::VectorType { .. } => unimplemented!("SExt on vectors"),
        ty => panic!("SExt result should be integer or vector of integers; got {:?}", ty),
    };
    state.record_bv_result(sext, z3op.sign_ext(dest_size - source_size))
}

fn symex_trunc(state: &mut State, trunc: &instruction::Trunc) -> Result<(), &'static str> {
    debug!("Symexing trunc {:?}", trunc);
    let z3op = state.operand_to_bv(&trunc.operand);
    let dest_size = match trunc.get_type() {
        Type::IntegerType { bits } => bits,
        Type::VectorType { .. } => unimplemented!("Trunc on vectors"),
        ty => panic!("Trunc result should be integer or vector of integers; got {:?}", ty),
    };
    state.record_bv_result(trunc, z3op.extract(dest_size-1, 0))
}

fn symex_bitcast(state: &mut State, bitcast: &instruction::BitCast) -> Result<(), &'static str> {
    debug!("Symexing bitcast {:?}", bitcast);
    let z3op = state.operand_to_bv(&bitcast.operand);
    state.record_bv_result(bitcast, z3op)  // from Z3's perspective the bitcast is simply a no-op; the bit patterns are equal
}

fn symex_load(state: &mut State, load: &instruction::Load) -> Result<(), &'static str> {
    debug!("Symexing load {:?}", load);
    let z3addr = state.operand_to_bv(&load.address);
    let dest_size = match load.get_type() {
        Type::IntegerType { bits } => bits,
        ty => unimplemented!("Load with non-integer result type {:?}", ty),
    };
    state.record_bv_result(load, state.read(&z3addr, dest_size))
}

fn symex_store(state: &mut State, store: &instruction::Store) -> Result<(), &'static str> {
    debug!("Symexing store {:?}", store);
    let z3val = state.operand_to_bv(&store.value);
    let z3addr = state.operand_to_bv(&store.address);
    state.write(&z3addr, z3val);
    Ok(())
}

fn symex_gep(state: &mut State, gep: &instruction::GetElementPtr) -> Result<(), &'static str> {
    debug!("Symexing gep {:?}", gep);
    let indices = gep.indices.iter().map(|i| match i {
        Operand::ConstantOperand(c) => c,
        _ => unimplemented!("Non-constant GEP index {:?}", i),
    });
    let offset = get_offset(indices, &gep.address.get_type());
    let z3base = state.operand_to_bv(&gep.address);
    state.record_bv_result(gep, z3base.bvadd(&BV::from_u64(state.ctx, offset as u64, z3base.get_size())))
}

// Get the offset (in bytes) of the element
fn get_offset<'m>(mut indices: impl Iterator<Item = &'m Constant>, base_type: &'m Type) -> u64 {
    match indices.next() {
        None => 0,
        Some(Constant::Int { value: cur_index, .. }) => {
            match base_type {
                Type::PointerType { pointee_type, .. }
                | Type::ArrayType { element_type: pointee_type, .. }
                | Type::VectorType { element_type: pointee_type, .. }
                => {
                    let el_size_bits = size(pointee_type) as u64;
                    if el_size_bits % 8 != 0 {
                        unimplemented!("Type with size {} bits", el_size_bits);
                    }
                    let el_size_bytes = el_size_bits / 8;
                    cur_index * el_size_bytes + get_offset(indices, pointee_type)
                },
                Type::StructType { element_types, .. } => {
                    let mut offset_bits = 0;
                    for ty in element_types.iter().take(*cur_index as usize) {
                        offset_bits += size(&ty) as u64;
                    }
                    if offset_bits % 8 != 0 {
                        unimplemented!("Struct offset of {} bits", offset_bits);
                    }
                    let offset_bytes = offset_bits / 8;
                    offset_bytes + get_offset(indices, &element_types[*cur_index as usize])
                },
                _ => panic!("Can't get_offset with {:?} base", base_type),
            }
        },
        idx => unimplemented!("get_offset with non-integer index {:?}", idx),
    }
}

fn symex_alloca(state: &mut State, alloca: &instruction::Alloca) -> Result<(), &'static str> {
    debug!("Symexing alloca {:?}", alloca);
    let allocation_size = size(&alloca.allocated_type);
    let allocated = state.allocate(allocation_size as u64);
    state.record_bv_result(alloca, allocated)
}

fn symex_call(state: &mut State, call: &instruction::Call) -> Result<(), &'static str> {
    debug!("Symexing call {:?}", call);
    let funcname = match call.function {
        Either::Right(Operand::ConstantOperand(Constant::GlobalReference { ref name, .. })) => name,
        Either::Left(_) => unimplemented!("inline assembly"),
        _ => unimplemented!("{:?}", call),
    };
    let errorfuncname = funcname.clone();  // just for possible error reporting
    if let Name::Name(s) = funcname {
        if s.starts_with("llvm.") {
            return Ok(())  // We ignore these llvm-internal functions
        }
    }
    unimplemented!("Call of a function named {:?}", errorfuncname);
}

// Returns the BV representing the return value, or None if the function returns void
fn symex_return<'ctx, 'func>(state: &State<'ctx, 'func>, ret: &terminator::Ret) -> Option<BV<'ctx>> {
    debug!("Symexing return {:?}", ret);
    ret.return_operand.as_ref().map(|op| state.operand_to_bv(op))
}

// Continues to the target of the Br and eventually returns the new Option<BV>
//   representing the return value of the function
// (when it reaches the end of the function)
fn symex_br<'ctx, 'func>(state: &mut State<'ctx, 'func>, br: &terminator::Br, cur_func: &'func Function, cur_bb: Name) -> Option<BV<'ctx>> {
    debug!("Symexing branch {:?}", br);
    symex_from_bb_by_name(state, &br.dest, cur_func, Some(cur_bb))
}

// Continues to the target(s) of the CondBr (saving a backtracking point if necessary)
//   and eventually returns the new Option<BV> representing the return value of the function
// (when it reaches the end of the function)
fn symex_condbr<'ctx, 'func>(state: &mut State<'ctx, 'func>, condbr: &terminator::CondBr, cur_func: &'func Function, cur_bb: Name) -> Option<BV<'ctx>> {
    // conditional branch
    let z3cond = state.operand_to_bool(&condbr.condition);
    let true_feasible = state.check_with_extra_constraints(&[&z3cond]);
    let false_feasible = state.check_with_extra_constraints(&[&z3cond.not()]);
    if true_feasible && false_feasible {
        // for now we choose to explore true first, and backtrack to false if necessary
        state.save_backtracking_point(cur_func, condbr.false_dest.clone(), cur_bb.clone(), z3cond.not());
        state.assert(&z3cond);
        symex_from_bb_by_name(state, &condbr.true_dest, cur_func, Some(cur_bb))
    } else if true_feasible {
        state.assert(&z3cond);  // unnecessary, but may help Z3 more than it hurts?
        symex_from_bb_by_name(state, &condbr.true_dest, cur_func, Some(cur_bb))
    } else if false_feasible {
        state.assert(&z3cond.not());  // unnecessary, but may help Z3 more than it hurts?
        symex_from_bb_by_name(state, &condbr.false_dest, cur_func, Some(cur_bb))
    } else if let Some((func, bb, prev_bb)) = state.revert_to_backtracking_point() {
        symex_from_bb_by_name(state, &bb, func, Some(prev_bb))
    } else {
        // both branches are unsat, and we have nowhere to backtrack to
        panic!("All possible paths seem to be unsat");
    }
}

fn symex_phi(state: &mut State, phi: &instruction::Phi, prev_bb: Option<&Name>) -> Result<(), &'static str> {
    debug!("Symexing phi {:?}", phi);
    let prev_bb = prev_bb.expect("not yet implemented: starting in a block with Phi instructions");
    let mut chosen_value = None;
    for (op, bbname) in phi.incoming_values.iter() {
        if bbname == prev_bb {
            chosen_value = Some(op);
            break;
        }
    }
    let chosen_value = chosen_value.expect("Failed to find a Phi member matching previous BasicBlock");
    state.record_bv_result(phi, state.operand_to_bv(&chosen_value))
}

fn symex_select(state: &mut State, select: &instruction::Select) -> Result<(), &'static str> {
    debug!("Symexing select {:?}", select);
    let z3cond = state.operand_to_bool(&select.condition);
    let z3trueval = state.operand_to_bv(&select.true_value);
    let z3falseval = state.operand_to_bv(&select.false_value);
    let true_feasible = state.check_with_extra_constraints(&[&z3cond]);
    let false_feasible = state.check_with_extra_constraints(&[&z3cond.not()]);
    if true_feasible && false_feasible {
        state.record_bv_result(select, Bool::ite(&z3cond, &z3trueval, &z3falseval))
    } else if true_feasible {
        state.assert(&z3cond);  // unnecessary, but may help Z3 more than it hurts?
        state.record_bv_result(select, z3trueval)
    } else if false_feasible {
        state.assert(&z3cond.not());  // unnecessary, but may help Z3 more than it hurts?
        state.record_bv_result(select, z3falseval)
    } else {
        // returning `Err` marks us unsat and will cause us to backtrack
        Err("discovered we're unsat while checking a switch condition")
    }
}
