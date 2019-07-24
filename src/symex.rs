use llvm_ir::*;
use llvm_ir::instruction::BinaryOp;
use log::debug;
use z3::ast::{Ast, BV, Bool};
use either::Either;
use std::rc::Rc;
use std::cell::RefCell;

use crate::state::State;
use crate::size::size;

/// Begin symbolic execution of the given function, obtaining an `ExecutionManager`.
/// `loop_bound`: maximum number of times to execute any given line of LLVM IR
/// (so, bounds the number of iterations of loops; for inner loops, this bounds the number
/// of total iterations across all invocations of the loop).
pub fn symex_function<'ctx, 'func>(ctx: &'ctx z3::Context, func: &'func Function, loop_bound: usize) -> ExecutionManager<'ctx, 'func> {
    let mut state = State::new(ctx, loop_bound);
    for param in func.parameters.iter() {
        let _ = state.new_bv_with_name(param.name.clone(), size(&param.ty) as u32);
    }
    symex_function_from_initial_state(func, state)
}

/// Like `symex_function`, but starting from the specified initial `State`.
/// `symex_function_from_initial_state()` assumes that the function's parameters
/// are already added to the initial `State`.
pub fn symex_function_from_initial_state<'ctx, 'func>(func: &'func Function, state: State<'ctx, 'func>) -> ExecutionManager<'ctx, 'func> {
    debug!("Symexing function {}", func.name);
    ExecutionManager::new(state, func)
}

/// An `ExecutionManager` allows you to symbolically explore executions of a function.
/// Conceptually, it is an `Iterator` over possible paths through the function.
/// Calling `next()` on an `ExecutionManager` explores another possible path,
/// returning a `BV` (AST) representing the function's symbolic return value at
/// the end of that path, or `None` if the function returns void.
/// Importantly, after any call to `next()`, you can access the `State` resulting
/// from the end of that path using the `state()` or `mut_state()` methods.
/// When `next()` returns `None`, there are no more possible paths through the
/// function.
pub struct ExecutionManager<'ctx, 'func> {
    state: State<'ctx, 'func>,
    func: &'func Function,
    executed_first_time: bool,
}

impl<'ctx, 'func> ExecutionManager<'ctx, 'func> {
    fn new(state: State<'ctx, 'func>, func: &'func Function) -> Self {
        Self {
            state,
            func,
            executed_first_time: false,
        }
    }

    /// Provides access to the `State` resulting from the end of the most recently
    /// explored path (or, if `next()` has never been called on this `ExecutionManager`,
    /// then simply the initial `State` which was passed in).
    pub fn state(&self) -> &State<'ctx, 'func> {
        &self.state
    }

    /// Provides mutable access to the underlying `State` (see notes on `state()`).
    /// Changes made to the initial state (before the first call to `next()`) are
    /// "sticky", and will persist through all executions of the function.
    /// However, changes made to a final state (after a call to `next()`) will be
    /// completely wiped away the next time that `next()` is called.
    pub fn mut_state(&mut self) -> &mut State<'ctx, 'func> {
        &mut self.state
    }
}

impl<'ctx, 'func> Iterator for ExecutionManager<'ctx, 'func> {
    type Item = Option<BV<'ctx>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.executed_first_time {
            if let Some((func, bb, prev_bb)) = self.state.revert_to_backtracking_point() {
                Some(symex_from_bb_by_name(&mut self.state, &bb, func, Some(prev_bb)))
            } else {
                None
            }
        } else {
            self.executed_first_time = true;
            let bb = self.func.basic_blocks.get(0).expect("Failed to get entry basic block");
            Some(symex_from_bb(&mut self.state, bb, self.func, None))
        }
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
    let dest_size = size(&load.get_type());
    state.record_bv_result(load, state.read(&z3addr, dest_size as u32))
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
    let z3base = state.operand_to_bv(&gep.address);
    let offset = get_offset(state, gep.indices.iter(), &gep.address.get_type(), z3base.get_size());
    state.record_bv_result(gep, z3base.bvadd(&offset).simplify())
}

/// Get the offset of the element (in bytes, as a `BV` of `result_bits` bits)
fn get_offset<'ctx, 'm>(state: &mut State<'ctx, '_>, mut indices: impl Iterator<Item = &'m Operand>, base_type: &Type, result_bits: u32) -> BV<'ctx> {
    let index = indices.next();
    if index.is_none() {
        return BV::from_u64(state.ctx, 0, result_bits);
    }
    let index = index.unwrap();  // we just handled the `None` case, so now it must have been `Some`
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
            zero_extend_to_bits(state.operand_to_bv(index), result_bits)
                .bvmul(&BV::from_u64(state.ctx, el_size_bytes, result_bits))
                .bvadd(&get_offset(state, indices, pointee_type, result_bits))
        },
        Type::StructType { element_types, .. } => match index {
            Operand::ConstantOperand(Constant::Int { value: index, .. }) => {
                let mut offset_bits = 0;
                for ty in element_types.iter().take(*index as usize) {
                    offset_bits += size(ty) as u64;
                }
                if offset_bits % 8 != 0 {
                    unimplemented!("Struct offset of {} bits", offset_bits);
                }
                let offset_bytes = offset_bits / 8;
                BV::from_u64(state.ctx, offset_bytes, result_bits)
                    .bvadd(&get_offset(state, indices, &element_types[*index as usize], result_bits))
            },
            _ => panic!("Can't get_offset from struct type with index {:?}", index),
        },
        Type::NamedStructType { ty, .. } => {
            let rc: Rc<RefCell<Type>> = ty.as_ref()
                .expect("get_offset on an opaque struct type")
                .upgrade()
                .expect("Failed to upgrade weak reference");
            let actual_ty: &Type = &rc.borrow();
            if let Type::StructType { ref element_types, .. } = actual_ty {
                // this code copied from the StructType case, unfortunately
                match index {
                    Operand::ConstantOperand(Constant::Int { value: index, .. }) => {
                        let mut offset_bits = 0;
                        for ty in element_types.iter().take(*index as usize) {
                            offset_bits += size(ty) as u64;
                        }
                        if offset_bits % 8 != 0 {
                            unimplemented!("Struct offset of {} bits", offset_bits);
                        }
                        let offset_bytes = offset_bits / 8;
                        BV::from_u64(state.ctx, offset_bytes, result_bits)
                            .bvadd(&get_offset(state, indices, &element_types[*index as usize], result_bits))
                    },
                    _ => panic!("Can't get_offset from struct type with index {:?}", index),
                }
            } else {
                panic!("Expected NamedStructType inner type to be a StructType, but got {:?}", actual_ty)
            }
        }
        _ => panic!("get_offset with base type {:?}", base_type),
    }
}

/// Zero-extend a `BV` to the specified number of bits.
/// The input `BV` can be already the desired size (in which case this function is a no-op)
/// or smaller (in which case this function will extend),
/// but not larger (in which case this function will panic).
fn zero_extend_to_bits(bv: BV, bits: u32) -> BV {
    let cur_bits = bv.get_size();
    if cur_bits == bits {
        bv
    } else if cur_bits < bits {
        bv.zero_ext(bits - cur_bits)
    } else {
        panic!("tried to zero-extend to {} bits, but already had {} bits", bits, cur_bits)
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
        if s.starts_with("llvm.memset") {
            assert_eq!(call.arguments.len(), 4);
            assert_eq!(call.arguments[0].0.get_type(), Type::pointer_to(Type::i8()));
            assert_eq!(call.arguments[1].0.get_type(), Type::i8());
            assert_eq!(call.get_type(), Type::VoidType);
            if let Operand::ConstantOperand(Constant::Int { value: num_bytes, .. }) = call.arguments[2].0 {
                let addr = state.operand_to_bv(&call.arguments[0].0);
                let val = state.operand_to_bv(&call.arguments[1].0);
                // TODO: this isn't necessarily efficient. But without knowing alignment of addr we can't do better
                for i in 0 .. num_bytes {
                    state.write(&addr.bvadd(&BV::from_u64(state.ctx, i, addr.get_size())), val.clone());
                }
                return Ok(());
            } else {
                unimplemented!("LLVM memset with non-constant-int num_bytes {:?}", call.arguments[2]);
            }
        } else if s.starts_with("llvm.") {
            return Ok(()); // We ignore other llvm-internal functions
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
