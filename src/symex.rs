use llvm_ir::*;
use llvm_ir::instruction::BinaryOp;
use log::debug;
use z3::ast::{Ast, BV, Bool};
use either::Either;
use std::rc::Rc;
use std::cell::RefCell;

pub use crate::state::{State, Callsite, Location, QualifiedBB};
use crate::size::size;

/// Begin symbolic execution of the given function, obtaining an `ExecutionManager`.
/// `loop_bound`: maximum number of times to execute any given line of LLVM IR
/// (so, bounds the number of iterations of loops; for inner loops, this bounds the number
/// of total iterations across all invocations of the loop).
pub fn symex_function<'ctx, 'm>(ctx: &'ctx z3::Context, module: &'m Module, func: &'m Function, loop_bound: usize) -> ExecutionManager<'ctx, 'm> {
    debug!("Symexing function {}", func.name);
    let bb = func.basic_blocks.get(0).expect("Failed to get entry basic block");
    let start_loc = Location {
        module,
        func,
        bbname: bb.name.clone(),
    };
    let mut state = State::new(ctx, start_loc, loop_bound);
    for param in func.parameters.iter() {
        let _ = state.new_bv_with_name(param.name.clone(), size(&param.ty) as u32);
    }
    ExecutionManager::new(state, &bb)
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
pub struct ExecutionManager<'ctx, 'm> {
    state: State<'ctx, 'm>,
    start_bb: &'m BasicBlock,
    /// Whether the `ExecutionManager` is "fresh". A "fresh" `ExecutionManager`
    /// has not yet produced its first path, i.e., `next()` has not been called
    /// on it yet.
    fresh: bool,
}

impl<'ctx, 'm> ExecutionManager<'ctx, 'm> {
    fn new(state: State<'ctx, 'm>, start_bb: &'m BasicBlock) -> Self {
        Self {
            state,
            start_bb,
            fresh: true,
        }
    }

    /// Provides access to the `State` resulting from the end of the most recently
    /// explored path (or, if `next()` has never been called on this `ExecutionManager`,
    /// then simply the initial `State` which was passed in).
    pub fn state(&self) -> &State<'ctx, 'm> {
        &self.state
    }

    /// Provides mutable access to the underlying `State` (see notes on `state()`).
    /// Changes made to the initial state (before the first call to `next()`) are
    /// "sticky", and will persist through all executions of the function.
    /// However, changes made to a final state (after a call to `next()`) will be
    /// completely wiped away the next time that `next()` is called.
    pub fn mut_state(&mut self) -> &mut State<'ctx, 'm> {
        &mut self.state
    }
}

impl<'ctx, 'm> Iterator for ExecutionManager<'ctx, 'm> {
    type Item = SymexResult<'ctx>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fresh {
            self.fresh = false;
            symex_from_bb_through_end_of_function(&mut self.state, self.start_bb)
        } else {
            debug!("ExecutionManager: requesting next path");
            backtrack_and_continue(&mut self.state)
        }
    }
}

pub enum SymexResult<'ctx> {
    Returned(BV<'ctx>),
    ReturnedVoid,
}

/// Symex from the current `Location` through the rest of the function.
/// Returns the `SymexResult` representing the return value of the function, or `None` if no possible paths were found.
fn symex_from_cur_loc_through_end_of_function<'ctx, 'm>(state: &mut State<'ctx, 'm>) -> Option<SymexResult<'ctx>> {
    let bb = state.cur_loc.func.get_bb_by_name(&state.cur_loc.bbname).unwrap_or_else(|| panic!("Failed to find bb named {:?} in function", state.cur_loc.bbname));
    symex_from_bb_through_end_of_function(state, bb)
}

/// Symex the given bb, through the rest of the function.
/// Returns the `SymexResult` representing the return value of the function, or `None` if no possible paths were found.
fn symex_from_bb_through_end_of_function<'ctx, 'm>(state: &mut State<'ctx, 'm>, bb: &'m BasicBlock) -> Option<SymexResult<'ctx>> {
    symex_from_inst_in_bb_through_end_of_function(state, bb, 0)
}

/// Symex starting from the given `inst` index in the given bb, through the rest of the function.
/// Returns the `SymexResult` representing the return value of the function, or `None` if no possible paths were found.
fn symex_from_inst_in_bb_through_end_of_function<'ctx, 'm>(state: &mut State<'ctx, 'm>, bb: &'m BasicBlock, inst: usize) -> Option<SymexResult<'ctx>> {
    assert_eq!(bb.name, state.cur_loc.bbname);
    debug!("Symexing basic block {:?} in function {}", bb.name, state.cur_loc.func.name);
    state.record_in_path(QualifiedBB {
        funcname: state.cur_loc.func.name.clone(),
        bbname: bb.name.clone(),
    });
    for (instnum, inst) in bb.instrs.iter().skip(inst).enumerate() {
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
                Instruction::Phi(phi) => symex_phi(state, &phi),
                Instruction::Select(select) => symex_select(state, &select),
                Instruction::Call(call) => match symex_call(state, &call, instnum) {
                    Err(e) => Err(e),
                    Ok(None) => Ok(()),
                    Ok(Some(symexresult)) => return Some(symexresult),
                },
                _ => unimplemented!("instruction {:?}", inst),
            }
        };
        if result.is_err() {
            // Having an `Err` here indicates we can't continue down this path,
            // for instance because we're unsat, or because loop bound was exceeded, etc
            return backtrack_and_continue(state);
        }
    }
    match &bb.term {
        Terminator::Ret(ret) => Some(symex_return(state, ret)),
        Terminator::Br(br) => symex_br(state, br),
        Terminator::CondBr(condbr) => symex_condbr(state, condbr),
        term => unimplemented!("terminator {:?}", term),
    }
}

/// Revert to the most recent backtrack point, then continue execution from that point.
/// Will continue not just to the end of the function containing the backtrack point,
/// but (using the saved callstack) all the way back to the end of the top-level function.
///
/// Returns the `SymexResult` representing the final return value, or `None` if
/// no possible paths were found.
fn backtrack_and_continue<'ctx, 'm>(state: &mut State<'ctx, 'm>) -> Option<SymexResult<'ctx>> {
    if state.revert_to_backtracking_point() {
        debug!("Reverted to backtrack point and continuing");
        symex_from_inst_in_cur_loc(state, 0)
    } else {
        // No backtrack points (and therefore no paths) remain
        None
    }
}

/// Symex starting from the given `inst` index in the current bb, returning
/// (using the saved callstack) all the way back to the end of the top-level
/// function.
///
/// Returns the `SymexResult` representing the final return value, or `None` if
/// no possible paths were found.
fn symex_from_inst_in_cur_loc<'ctx, 'm>(state: &mut State<'ctx, 'm>, inst: usize) -> Option<SymexResult<'ctx>> {
    let bb = state.cur_loc.func.get_bb_by_name(&state.cur_loc.bbname).unwrap_or_else(|| panic!("Failed to find bb named {:?} in function {:?}", state.cur_loc.bbname, state.cur_loc.func.name));
    symex_from_inst_in_bb(state, &bb, inst)
}

/// Symex starting from the given `inst` index in the given bb, returning
/// (using the saved callstack) all the way back to the end of the top-level
/// function.
///
/// Returns the `SymexResult` representing the final return value, or `None` if
/// no possible paths were found.
fn symex_from_inst_in_bb<'ctx, 'm>(state: &mut State<'ctx, 'm>, bb: &'m BasicBlock, inst: usize) -> Option<SymexResult<'ctx>> {
    match symex_from_inst_in_bb_through_end_of_function(state, bb, inst) {
        Some(symexresult) => match state.pop_callsite() {
            Some(callsite) => {
                // Return to callsite
                state.cur_loc = callsite.loc.clone();
                // Assign the returned value as the result of the caller's call instruction
                match symexresult {
                    SymexResult::Returned(bv) => {
                        let call: &Instruction = callsite.loc.func
                            .get_bb_by_name(&callsite.loc.bbname)
                            .expect("Malformed callsite (bb not found)")
                            .instrs
                            .get(callsite.inst)
                            .expect("Malformed callsite (inst out of range)");
                        let call: &instruction::Call = match call {
                            Instruction::Call(call) => call,
                            _ => panic!("Malformed callsite: expected a Call, got {:?}", call),
                        };
                        match state.new_bv_with_name(call.dest.as_ref().unwrap().clone(), size(&call.get_type()) as u32) {
                            Ok(result) => state.assert(&result._eq(&bv)),
                            Err(_) => {
                                // This path is dead, try backtracking again
                                return backtrack_and_continue(state);
                            }
                        };
                    },
                    SymexResult::ReturnedVoid => { },
                };
                // Continue execution in caller, with the instruction after the call instruction
                symex_from_inst_in_cur_loc(state, callsite.inst + 1)
            },
            None => {
                // No callsite to return to, so we're done
                Some(symexresult)
            }
        },
        None => {
            // This path is dead, try backtracking again
            backtrack_and_continue(state)
        },
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

fn symex_binop<'ctx, 'm, F>(state: &mut State<'ctx, 'm>, bop: &instruction::groups::BinaryOp, z3op: F) -> Result<(), &'static str>
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

/// `instnum`: the index in the current `BasicBlock` of the given `Call` instruction.
/// If the returned value is `Ok(Some(_))`, then this is the final return value of the top-level function (we had backtracking and finished on a different path).
/// If the returned value is `Ok(None)`, then we finished the call normally, and execution should continue from here.
fn symex_call<'ctx, 'm>(state: &mut State<'ctx, 'm>, call: &'m instruction::Call, instnum: usize) -> Result<Option<SymexResult<'ctx>>, &'static str> {
    debug!("Symexing call {:?}", call);
    let funcname = match call.function {
        Either::Right(Operand::ConstantOperand(Constant::GlobalReference { ref name, .. })) => name,
        Either::Left(_) => unimplemented!("inline assembly"),
        _ => unimplemented!("{:?}", call),
    };
    let errorfuncname = funcname.clone();  // just for possible error reporting
    if let Name::Name(s) = funcname {
        if let Some(callee) = state.cur_loc.module.get_func_by_name(s) {
            assert_eq!(call.arguments.len(), callee.parameters.len());
            let z3args: Vec<_> = call.arguments.iter().map(|arg| state.operand_to_bv(&arg.0)).collect();  // have to do this before changing state.cur_loc, so that the lookups happen in the caller function
            let saved_loc = state.cur_loc.clone();  // don't need to save prev_bb because there can't be any more Phi instructions in this block (they all have to come before any Call instructions)
            state.push_callsite(instnum);
            let bb = callee.basic_blocks.get(0).expect("Failed to get entry basic block");
            state.cur_loc.func = callee;
            state.cur_loc.bbname = bb.name.clone();
            for (z3arg, param) in z3args.into_iter().zip(callee.parameters.iter()) {
                let z3param = state.new_bv_with_name(param.name.clone(), size(&param.get_type()) as u32)?;  // have to do the new_bv_with_name calls after changing state.cur_loc, so that the variables are created in the callee function
                state.assert(&z3param._eq(&z3arg));
            }
            let returned_bv = symex_from_bb_through_end_of_function(state, &bb).ok_or("No more valid paths through callee")?;
            match state.pop_callsite() {
                None => Ok(Some(returned_bv)),  // if there was no callsite to pop, then we finished elsewhere. See notes on `symex_call()`
                Some(Callsite { ref loc, inst }) if loc == &saved_loc && inst == instnum => {
                    state.cur_loc = saved_loc;
                    state.record_in_path(QualifiedBB {
                        funcname: state.cur_loc.func.name.clone(),
                        bbname: state.cur_loc.bbname.clone(),
                    });
                    match returned_bv {
                        SymexResult::Returned(bv) => {
                            // can't quite use `state.record_bv_result(call, bv)?` because Call is not HasResult
                            let result = state.new_bv_with_name(call.dest.as_ref().unwrap().clone(), size(&call.get_type()) as u32)?;
                            state.assert(&result._eq(&bv));
                        },
                        SymexResult::ReturnedVoid => assert_eq!(call.dest, None),
                    };
                    debug!("Completed ordinary return to caller");
                    Ok(None)
                },
                Some(callsite) => panic!("Received unexpected callsite {:?}", callsite),
            }
        } else if s.starts_with("llvm.memset") {
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
                Ok(None)
            } else {
                unimplemented!("LLVM memset with non-constant-int num_bytes {:?}", call.arguments[2])
            }
        } else if s.starts_with("llvm.memcpy") {
            unimplemented!("LLVM memcpy")
        } else if s.starts_with("llvm.memmove") {
            unimplemented!("LLVM memmove")
        } else if s.starts_with("llvm.lifetime")
            || s.starts_with("llvm.invariant")
            || s.starts_with("llvm.launder.invariant")
            || s.starts_with("llvm.strip.invariant")
        {
            Ok(None) // these are all safe to ignore
        } else {
            unimplemented!("Call of a function named {:?}", errorfuncname)
        }
    } else {
        panic!("Function with a numbered name, {:?}", funcname)
    }
}

// Returns the `SymexResult` representing the return value
fn symex_return<'ctx, 'm>(state: &State<'ctx, 'm>, ret: &terminator::Ret) -> SymexResult<'ctx> {
    debug!("Symexing return {:?}", ret);
    ret.return_operand
        .as_ref()
        .map(|op| SymexResult::Returned(state.operand_to_bv(op)))
        .unwrap_or(SymexResult::ReturnedVoid)
}

// Continues to the target of the `Br` and eventually returns the new `SymexResult`
// representing the return value of the function (when it reaches the end of the
// function), or `None` if no possible paths were found.
fn symex_br<'ctx, 'm>(state: &mut State<'ctx, 'm>, br: &'m terminator::Br) -> Option<SymexResult<'ctx>> {
    debug!("Symexing br {:?}", br);
    state.prev_bb_name = Some(state.cur_loc.bbname.clone());
    state.cur_loc.bbname = br.dest.clone();
    symex_from_cur_loc_through_end_of_function(state)
}

// Continues to the target(s) of the `CondBr` (saving a backtracking point if
// necessary) and eventually returns the new `SymexResult` representing the
// return value of the function (when it reaches the end of the function), or
// `None` if no possible paths were found.
fn symex_condbr<'ctx, 'm>(state: &mut State<'ctx, 'm>, condbr: &'m terminator::CondBr) -> Option<SymexResult<'ctx>> {
    debug!("Symexing condbr {:?}", condbr);
    let z3cond = state.operand_to_bool(&condbr.condition);
    let true_feasible = state.check_with_extra_constraints(&[&z3cond]);
    let false_feasible = state.check_with_extra_constraints(&[&z3cond.not()]);
    if true_feasible && false_feasible {
        // for now we choose to explore true first, and backtrack to false if necessary
        state.save_backtracking_point(condbr.false_dest.clone(), z3cond.not());
        state.assert(&z3cond);
        state.prev_bb_name = Some(state.cur_loc.bbname.clone());
        state.cur_loc.bbname = condbr.true_dest.clone();
        symex_from_cur_loc_through_end_of_function(state)
    } else if true_feasible {
        state.assert(&z3cond);  // unnecessary, but may help Z3 more than it hurts?
        state.prev_bb_name = Some(state.cur_loc.bbname.clone());
        state.cur_loc.bbname = condbr.true_dest.clone();
        symex_from_cur_loc_through_end_of_function(state)
    } else if false_feasible {
        state.assert(&z3cond.not());  // unnecessary, but may help Z3 more than it hurts?
        state.prev_bb_name = Some(state.cur_loc.bbname.clone());
        state.cur_loc.bbname = condbr.false_dest.clone();
        symex_from_cur_loc_through_end_of_function(state)
    } else {
        backtrack_and_continue(state)
    }
}

fn symex_phi(state: &mut State, phi: &instruction::Phi) -> Result<(), &'static str> {
    debug!("Symexing phi {:?}", phi);
    let prev_bb = state.prev_bb_name.as_ref().expect("not yet implemented: starting in a block with Phi instructions. or error: didn't expect a Phi in function entry block");
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

#[cfg(test)]
mod tests {
    //! These tests check that the correct set of _paths_ are generated for various
    //! functions. In contrast, the integration tests in the tests/ folder test for
    //! specific solutions for function parameters and return values.

    use llvm_ir::*;
    use super::*;

    fn init_logging() {
        // capture log messages with test harness
        let _ = env_logger::builder().is_test(true).try_init();
    }

    type Path<'m> = Vec<QualifiedBB>;

    fn path_from_bbnames(funcname: &str, bbnames: impl IntoIterator<Item = Name>) -> Path {
        let mut vec = vec![];
        for bbname in bbnames {
            vec.push(QualifiedBB { funcname: funcname.to_string(), bbname });
        }
        vec
    }

    fn path_from_bbnums(funcname: &str, bbnums: impl IntoIterator<Item = usize>) -> Path {
        path_from_bbnames(funcname, bbnums.into_iter().map(Name::Number))
    }

    /// Iterator over the paths through a function
    struct PathIterator<'ctx, 'm> {
        em: ExecutionManager<'ctx, 'm>,
    }

    impl<'ctx, 'm> PathIterator<'ctx, 'm> {
        pub fn new(ctx: &'ctx z3::Context, module: &'m Module, func: &'m Function, loop_bound: usize) -> Self {
            Self { em: symex_function(ctx, module, func, loop_bound) }
        }
    }

    impl<'ctx, 'm> Iterator for PathIterator<'ctx, 'm> {
        type Item = Path<'m>;

        fn next(&mut self) -> Option<Self::Item> {
            self.em.next().map(|_| self.em.state().path.clone())
        }
    }

    #[test]
    fn one_block() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/basic.bc"))
            .expect("Failed to parse basic.bc module");
        let func = module.get_func_by_name("one_arg").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = PathIterator::new(&ctx, &module, func, 5).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1]));
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn two_paths() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/basic.bc"))
            .expect("Failed to parse basic.bc module");
        let func = module.get_func_by_name("conditional_true").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![2, 4, 12]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![2, 8, 12]));
        assert_eq!(paths.len(), 2);  // ensure there are no more paths
    }

    #[test]
    fn four_paths() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/basic.bc"))
            .expect("Failed to parse basic.bc module");
        let func = module.get_func_by_name("conditional_nozero").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![2, 4, 6, 14]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![2, 4, 8, 10, 14]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![2, 4, 8, 12, 14]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![2, 14]));
        assert_eq!(paths.len(), 4);  // ensure there are no more paths
    }

    #[test]
    fn while_loop() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
            .expect("Failed to parse loop.bc module");
        let func = module.get_func_by_name("while_loop").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 6, 6, 6, 6, 6, 12]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 6, 6, 6, 6, 12]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 6, 6, 6, 12]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 6, 6, 12]));
        assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 6, 12]));
        assert_eq!(paths.len(), 5);  // ensure there are no more paths
    }

    #[test]
    fn for_loop() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
            .expect("Failed to parse loop.bc module");
        let func = module.get_func_by_name("for_loop").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 6]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 9, 6]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 9, 9, 6]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 9, 9, 9, 6]));
        assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 9, 9, 9, 9, 6]));
        assert_eq!(paths[5], path_from_bbnums(&func.name, vec![1, 9, 9, 9, 9, 9, 6]));
        assert_eq!(paths.len(), 6);  // ensure there are no more paths
    }

    #[test]
    fn loop_more_blocks() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
            .expect("Failed to parse loop.bc module");
        let func = module.get_func_by_name("loop_zero_iterations").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 5, 8, 18]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 5, 11, 8, 18]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 5, 11, 11, 8, 18]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 5, 11, 11, 11, 8, 18]));
        assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 5, 11, 11, 11, 11, 8, 18]));
        assert_eq!(paths[5], path_from_bbnums(&func.name, vec![1, 5, 11, 11, 11, 11, 11, 8, 18]));
        assert_eq!(paths[6], path_from_bbnums(&func.name, vec![1, 18]));
        assert_eq!(paths.len(), 7);  // ensure there are no more paths
    }

    #[test]
    fn loop_more_blocks_in_body() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
            .expect("Failed to parse loop.bc module");
        let func = module.get_func_by_name("loop_with_cond").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 6, 13, 16,
                                                                6, 10, 16,
                                                                6, 10, 16,
                                                                6, 13, 16,
                                                                6, 10, 16, 20]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 6, 13, 16,
                                                                6, 10, 16,
                                                                6, 10, 16,
                                                                6, 13, 16, 20]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 6, 13, 16,
                                                                6, 10, 16,
                                                                6, 10, 16, 20]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 6, 13, 16,
                                                                6, 10, 16, 20]));
        assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 6, 13, 16, 20]));
        assert_eq!(paths.len(), 5);  // ensure there are no more paths
    }

    #[test]
    fn two_loops() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
            .expect("Failed to parse loop.bc module");
        let func = module.get_func_by_name("sum_of_array").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 30)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                                                            11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 9]));
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn nested_loop() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/loop.bc"))
            .expect("Failed to parse loop.bc module");
        let func = module.get_func_by_name("nested_loop").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 30)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                            10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                            10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                            10, 7]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                            10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                            10, 7]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                            10, 7]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 7]));
        assert_eq!(paths.len(), 4);  // ensure there are no more paths
    }

    #[test]
    fn simple_call() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("simple_caller").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], vec![
            QualifiedBB { funcname: "simple_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "simple_caller".to_owned(), bbname: Name::Number(1) },
        ]);
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn conditional_call() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("conditional_caller").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], vec![
            QualifiedBB { funcname: "conditional_caller".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "conditional_caller".to_owned(), bbname: Name::Number(4) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "conditional_caller".to_owned(), bbname: Name::Number(4) },
            QualifiedBB { funcname: "conditional_caller".to_owned(), bbname: Name::Number(8) },
        ]);
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![2, 6, 8]));
        assert_eq!(paths.len(), 2);  // ensure there are no more paths
    }

    #[test]
    fn call_twice() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("twice_caller").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], vec![
            QualifiedBB { funcname: "twice_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "twice_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "twice_caller".to_owned(), bbname: Name::Number(1) },
        ]);
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn nested_call() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("nested_caller").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], vec![
            QualifiedBB { funcname: "nested_caller".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "simple_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "simple_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "nested_caller".to_owned(), bbname: Name::Number(2) },
        ]);
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn call_of_loop() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("caller_of_loop").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], vec![
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(9) },
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
        ]);
        assert_eq!(paths[1], vec![
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(9) },
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
        ]);
        assert_eq!(paths[2], vec![
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(9) },
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
        ]);
        assert_eq!(paths[3], vec![
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(9) },
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
        ]);
        assert_eq!(paths[4], vec![
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(9) },
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
        ]);
        assert_eq!(paths[5], vec![
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(13) },
            QualifiedBB { funcname: "callee_with_loop".to_owned(), bbname: Name::Number(9) },
            QualifiedBB { funcname: "caller_of_loop".to_owned(), bbname: Name::Number(1) },
        ]);
        assert_eq!(paths.len(), 6);  // ensure there are no more paths
    }

    #[test]
    fn call_in_loop() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("caller_with_loop").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 3)).collect();
        assert_eq!(paths[0], vec![
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(8) },
        ]);
        assert_eq!(paths[1], vec![
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(8) },
        ]);
        assert_eq!(paths[2], vec![
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(8) },
        ]);
        assert_eq!(paths[3], vec![
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(10) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "caller_with_loop".to_owned(), bbname: Name::Number(8) },
        ]);
        assert_eq!(paths.len(), 4);  // ensure there are no more paths
    }

    #[test]
    fn recursive_simple() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("recursive_simple").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 5)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 4, 1, 4, 1, 4, 1, 4, 1, 7, 4, 4, 4, 4]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 4, 1, 4, 1, 4, 1, 7, 4, 4, 4]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 4, 1, 4, 1, 7, 4, 4]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 4, 1, 7, 4]));
        assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 7]));
        assert_eq!(paths.len(), 5);  // ensure there are no more paths
    }

    #[test]
    fn recursive_more_complicated() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("recursive_more_complicated").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 4)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 4, 1, 4, 1, 4, 1, 8, 14, 4, 4, 4]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 4, 1, 4, 1, 8, 10, 1, 8, 14, 10, 4, 4]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 4, 1, 4, 1, 8, 14, 4, 4]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 4, 1, 8, 10, 1, 4, 1, 8, 14, 4, 10, 4]));
        assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 4, 1, 8, 10, 1, 8, 10, 1, 8, 14, 10, 10, 4]));
        assert_eq!(paths[5], path_from_bbnums(&func.name, vec![1, 4, 1, 8, 10, 1, 8, 14, 10, 4]));
        assert_eq!(paths[6], path_from_bbnums(&func.name, vec![1, 4, 1, 8, 14, 4]));
        assert_eq!(paths[7], path_from_bbnums(&func.name, vec![1, 8, 10, 1, 4, 1, 4, 1, 8, 14, 4, 4, 10]));
        assert_eq!(paths[8], path_from_bbnums(&func.name, vec![1, 8, 10, 1, 4, 1, 8, 10, 1, 8, 14, 10, 4, 10]));
        assert_eq!(paths[9], path_from_bbnums(&func.name, vec![1, 8, 10, 1, 4, 1, 8, 14, 4, 10]));
        assert_eq!(paths[10], path_from_bbnums(&func.name, vec![1, 8, 10, 1, 8, 10, 1, 4, 1, 8, 14, 4, 10, 10]));
        assert_eq!(paths[11], path_from_bbnums(&func.name, vec![1, 8, 10, 1, 8, 10, 1, 8, 10, 1, 8, 14, 10, 10, 10]));
        assert_eq!(paths[12], path_from_bbnums(&func.name, vec![1, 8, 10, 1, 8, 10, 1, 8, 14, 10, 10]));
        assert_eq!(paths[13], path_from_bbnums(&func.name, vec![1, 8, 10, 1, 8, 14, 10]));
        assert_eq!(paths[14], path_from_bbnums(&func.name, vec![1, 8, 14]));
        assert_eq!(paths.len(), 15);  // ensure there are no more paths
    }

    #[test]
    fn recursive_not_tail() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("recursive_not_tail").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 3)).collect();
        assert_eq!(paths[0], path_from_bbnums(&func.name, vec![1, 3, 15]));
        assert_eq!(paths[1], path_from_bbnums(&func.name, vec![1, 5, 1, 3, 15, 5, 10, 15]));
        assert_eq!(paths[2], path_from_bbnums(&func.name, vec![1, 5, 1, 3, 15, 5, 13, 15]));
        assert_eq!(paths[3], path_from_bbnums(&func.name, vec![1, 5, 1, 5, 1, 3, 15, 5, 10, 15, 5, 10, 15]));
        assert_eq!(paths[4], path_from_bbnums(&func.name, vec![1, 5, 1, 5, 1, 3, 15, 5, 10, 15, 5, 13, 15]));
        assert_eq!(paths[5], path_from_bbnums(&func.name, vec![1, 5, 1, 5, 1, 3, 15, 5, 13, 15, 5, 10, 15]));
        assert_eq!(paths[6], path_from_bbnums(&func.name, vec![1, 5, 1, 5, 1, 3, 15, 5, 13, 15, 5, 13, 15]));
        assert_eq!(paths.len(), 7);  // ensure there are no more paths
    }

    #[test]
    fn recursive_and_normal_call() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("recursive_and_normal_caller").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 3)).collect();
        assert_eq!(paths[0], vec![
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(8) },
        ]);
        assert_eq!(paths[1], vec![
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(5) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(8) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(5) },
        ]);
        assert_eq!(paths[2], vec![
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(5) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(5) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "simple_callee".to_owned(), bbname: Name::Number(2) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(8) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(5) },
            QualifiedBB { funcname: "recursive_and_normal_caller".to_owned(), bbname: Name::Number(5) },
        ]);
        assert_eq!(paths.len(), 3);  // ensure there are no more paths
    }

    #[test]
    fn mutually_recursive_functions() {
        init_logging();
        let module = Module::from_bc_path(&std::path::Path::new("tests/bcfiles/call.bc"))
            .expect("Failed to parse call.bc module");
        let func = module.get_func_by_name("mutually_recursive_a").expect("Failed to find function");
        let ctx = z3::Context::new(&z3::Config::new());
        let paths: Vec<Path> = itertools::sorted(PathIterator::new(&ctx, &module, func, 3)).collect();
        assert_eq!(paths[0], vec![
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
        ]);
        assert_eq!(paths[1], vec![
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
        ]);
        assert_eq!(paths[2], vec![
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
        ]);
        assert_eq!(paths[3], vec![
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_b".to_owned(), bbname: Name::Number(6) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(3) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
        ]);
        assert_eq!(paths[4], vec![
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(1) },
            QualifiedBB { funcname: "mutually_recursive_a".to_owned(), bbname: Name::Number(6) },
        ]);
        assert_eq!(paths.len(), 5);  // ensure there are no more paths
    }
}
