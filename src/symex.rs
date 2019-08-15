use llvm_ir::*;
use llvm_ir::instruction::BinaryOp;
use log::{debug, info};
use either::Either;
use reduce::Reduce;
use std::rc::Rc;
use std::cell::RefCell;

pub use crate::state::{State, Callsite, Location, PathEntry};
use crate::backend::*;
use crate::config::*;
use crate::error::*;
use crate::extend::*;
use crate::possible_solutions::*;
use crate::project::Project;
use crate::size::size;

/// Begin symbolic execution of the function named `funcname`, obtaining an
/// `ExecutionManager`. The function's parameters will start completely
/// unconstrained.
///
/// `project`: The `Project` (set of LLVM modules) in which symbolic execution
/// should take place. In the absence of function hooks (see
/// [`Config`](struct.Config.html)), we will try to enter calls to any functions
/// defined in the `Project`.
pub fn symex_function<'ctx, 'p, B>(
    ctx: &'ctx z3::Context,
    funcname: &str,
    project: &'p Project,
    config: Config<'ctx, B>,
) -> ExecutionManager<'ctx, 'p, B> where B: Backend<'ctx> {
    debug!("Symexing function {}", funcname);
    let (func, module) = project.get_func_by_name(funcname).unwrap_or_else(|| panic!("Failed to find function named {:?}", funcname));
    let bb = func.basic_blocks.get(0).expect("Failed to get entry basic block");
    let start_loc = Location {
        module,
        func,
        bbname: bb.name.clone(),
    };
    let mut state = State::new(ctx, project, start_loc, &config);
    let z3params: Vec<_> = func.parameters.iter().map(|param| {
        state.new_bv_with_name(param.name.clone(), size(&param.ty) as u32).unwrap()
    }).collect();
    ExecutionManager::new(state, project, config, z3params, &bb)
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
pub struct ExecutionManager<'ctx, 'p, B> where B: Backend<'ctx> {
    state: State<'ctx, 'p, B>,
    project: &'p Project,
    config: Config<'ctx, B>,
    z3params: Vec<B::BV>,
    start_bb: &'p BasicBlock,
    /// Whether the `ExecutionManager` is "fresh". A "fresh" `ExecutionManager`
    /// has not yet produced its first path, i.e., `next()` has not been called
    /// on it yet.
    fresh: bool,
}

impl<'ctx, 'p, B> ExecutionManager<'ctx, 'p, B> where B: Backend<'ctx> {
    fn new(state: State<'ctx, 'p, B>, project: &'p Project, config: Config<'ctx, B>, z3params: Vec<B::BV>, start_bb: &'p BasicBlock) -> Self {
        Self {
            state,
            project,
            config,
            z3params,
            start_bb,
            fresh: true,
        }
    }

    /// Provides access to the `State` resulting from the end of the most recently
    /// explored path (or, if `next()` has never been called on this `ExecutionManager`,
    /// then simply the initial `State` which was passed in).
    pub fn state(&self) -> &State<'ctx, 'p, B> {
        &self.state
    }

    /// Provides mutable access to the underlying `State` (see notes on `state()`).
    /// Changes made to the initial state (before the first call to `next()`) are
    /// "sticky", and will persist through all executions of the function.
    /// However, changes made to a final state (after a call to `next()`) will be
    /// completely wiped away the next time that `next()` is called.
    pub fn mut_state(&mut self) -> &mut State<'ctx, 'p, B> {
        &mut self.state
    }

    /// Provides access to the `BV` objects representing each of the function's parameters
    pub fn param_bvs(&self) -> &Vec<B::BV> {
        &self.z3params
    }
}

pub enum SymexResult<V> {
    Returned(V),
    ReturnedVoid,
}

impl<'ctx, 'p, B> Iterator for ExecutionManager<'ctx, 'p, B> where B: Backend<'ctx> + 'p {
    type Item = SymexResult<B::BV>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fresh {
            self.fresh = false;
            info!("Beginning symex in function {:?}", self.state.cur_loc.func.name);
            self.symex_from_bb_through_end_of_function(self.start_bb).unwrap_or_else(|e| {
                panic!("Received the following error:\n  {:?}\nat location {:?}", e, self.state.cur_loc);
            })
        } else {
            debug!("ExecutionManager: requesting next path");
            self.backtrack_and_continue().unwrap_or_else(|e| {
                panic!("Received the following error:\n  {:?}\nat location {:?}", e, self.state.cur_loc);
            })
        }
    }
}

impl<'ctx, 'p, B> ExecutionManager<'ctx, 'p, B> where B: Backend<'ctx> + 'p {
    /// Symex from the current `Location` through the rest of the function.
    /// Returns the `SymexResult` representing the return value of the function,
    /// or `Ok(None)` if no possible paths were found.
    fn symex_from_cur_loc_through_end_of_function(&mut self) -> Result<Option<SymexResult<B::BV>>> {
        let bb = self.state.cur_loc.func.get_bb_by_name(&self.state.cur_loc.bbname)
            .unwrap_or_else(|| panic!("Failed to find bb named {:?} in function {:?}", self.state.cur_loc.bbname, self.state.cur_loc.func.name));
        self.symex_from_bb_through_end_of_function(bb)
    }

    /// Symex the given bb, through the rest of the function.
    /// Returns the `SymexResult` representing the return value of the function,
    /// or `Ok(None)` if no possible paths were found.
    fn symex_from_bb_through_end_of_function(&mut self, bb: &'p BasicBlock) -> Result<Option<SymexResult<B::BV>>> {
        self.symex_from_inst_in_bb_through_end_of_function(bb, 0)
    }

    /// Symex starting from the given `inst` index in the given bb, through the rest of the function.
    /// Returns the `SymexResult` representing the return value of the function,
    /// or `Ok(None)` if no possible paths were found.
    fn symex_from_inst_in_bb_through_end_of_function(&mut self, bb: &'p BasicBlock, inst: usize) -> Result<Option<SymexResult<B::BV>>> {
        assert_eq!(bb.name, self.state.cur_loc.bbname);
        debug!("Symexing basic block {:?} in function {}", bb.name, self.state.cur_loc.func.name);
        self.state.record_in_path(PathEntry {
            modname: self.state.cur_loc.module.name.clone(),
            funcname: self.state.cur_loc.func.name.clone(),
            bbname: bb.name.clone(),
        });
        for (instnum, inst) in bb.instrs.iter().skip(inst).enumerate() {
            let result = if let Some((binop, z3binop)) = Self::inst_to_binop(&inst) {
                self.symex_binop(&binop, z3binop)
            } else {
                match inst {
                    Instruction::ICmp(icmp) => self.symex_icmp(icmp),
                    Instruction::Load(load) => self.symex_load(load),
                    Instruction::Store(store) => self.symex_store(store),
                    Instruction::GetElementPtr(gep) => self.symex_gep(gep),
                    Instruction::Alloca(alloca) => self.symex_alloca(alloca),
                    Instruction::ZExt(zext) => self.symex_zext(zext),
                    Instruction::SExt(sext) => self.symex_sext(sext),
                    Instruction::Trunc(trunc) => self.symex_trunc(trunc),
                    Instruction::PtrToInt(pti) => self.symex_cast_op(pti),
                    Instruction::IntToPtr(itp) => self.symex_cast_op(itp),
                    Instruction::BitCast(bitcast) => self.symex_cast_op(bitcast),
                    Instruction::Phi(phi) => self.symex_phi(phi),
                    Instruction::Select(select) => self.symex_select(select),
                    Instruction::Call(call) => match self.symex_call(call, instnum) {
                        Err(e) => Err(e),
                        Ok(None) => Ok(()),
                        Ok(Some(symexresult)) => return Ok(Some(symexresult)),
                    },
                    _ => return Err(Error::UnsupportedInstruction(format!("instruction {:?}", inst))),
                }
            };
            match result {
                Ok(_) => {},  // no error, we can continue
                Err(Error::Unsat) | Err(Error::LoopBoundExceeded) => {
                    // we can't continue down this path anymore
                    return self.backtrack_and_continue();
                }
                Err(e) => return Err(e),  // propagate any other errors
            };
        }
        match &bb.term {
            Terminator::Ret(ret) => self.symex_return(ret).map(Some),
            Terminator::Br(br) => self.symex_br(br),
            Terminator::CondBr(condbr) => self.symex_condbr(condbr),
            term => Err(Error::UnsupportedInstruction(format!("terminator {:?}", term))),
        }
    }

    /// Revert to the most recent backtrack point, then continue execution from that point.
    /// Will continue not just to the end of the function containing the backtrack point,
    /// but (using the saved callstack) all the way back to the end of the top-level function.
    ///
    /// Returns the `SymexResult` representing the final return value, or
    /// `Ok(None)` if no possible paths were found.
    fn backtrack_and_continue(&mut self) -> Result<Option<SymexResult<B::BV>>> {
        if self.state.revert_to_backtracking_point() {
            info!("Reverted to backtrack point; {} more backtrack points available", self.state.count_backtracking_points());
            self.symex_from_inst_in_cur_loc(0)
        } else {
            // No backtrack points (and therefore no paths) remain
            Ok(None)
        }
    }

    /// Symex starting from the given `inst` index in the current bb, returning
    /// (using the saved callstack) all the way back to the end of the top-level
    /// function.
    ///
    /// Returns the `SymexResult` representing the final return value, or
    /// `Ok(None)` if no possible paths were found.
    fn symex_from_inst_in_cur_loc(&mut self, inst: usize) -> Result<Option<SymexResult<B::BV>>> {
        let bb = self.state.cur_loc.func.get_bb_by_name(&self.state.cur_loc.bbname)
            .unwrap_or_else(|| panic!("Failed to find bb named {:?} in function {:?}", self.state.cur_loc.bbname, self.state.cur_loc.func.name));
        self.symex_from_inst_in_bb(&bb, inst)
    }

    /// Symex starting from the given `inst` index in the given bb, returning
    /// (using the saved callstack) all the way back to the end of the top-level
    /// function.
    ///
    /// Returns the `SymexResult` representing the final return value, or
    /// `Ok(None)` if no possible paths were found.
    fn symex_from_inst_in_bb(&mut self, bb: &'p BasicBlock, inst: usize) -> Result<Option<SymexResult<B::BV>>> {
        match self.symex_from_inst_in_bb_through_end_of_function(bb, inst)? {
            Some(symexresult) => match self.state.pop_callsite() {
                Some(callsite) => {
                    // Return to callsite
                    info!("Leaving function {:?}", self.state.cur_loc.func.name);
                    self.state.cur_loc = callsite.loc.clone();
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
                            if self.state.assign_bv_to_name(call.dest.as_ref().unwrap().clone(), bv).is_err() {
                                // This path is dead, try backtracking again
                                return self.backtrack_and_continue();
                            };
                        },
                        SymexResult::ReturnedVoid => { },
                    };
                    // Continue execution in caller, with the instruction after the call instruction
                    self.symex_from_inst_in_cur_loc(callsite.inst + 1)
                },
                None => {
                    // No callsite to return to, so we're done
                    Ok(Some(symexresult))
                }
            },
            None => {
                // This path is dead, try backtracking again
                self.backtrack_and_continue()
            },
        }
    }

    fn inst_to_binop<V>(inst: &Instruction) -> Option<(instruction::groups::BinaryOp, Box<FnOnce(&V, &V) -> V + 'ctx>)> where V: BV<'ctx> + 'ctx {
        match inst {
            // TODO: how to not clone the inner instruction here
            Instruction::Add(i) => Some((i.clone().into(), Box::new(<V as BV<'ctx>>::add))),
            Instruction::Sub(i) => Some((i.clone().into(), Box::new(V::sub))),
            Instruction::Mul(i) => Some((i.clone().into(), Box::new(V::mul))),
            Instruction::UDiv(i) => Some((i.clone().into(), Box::new(V::udiv))),
            Instruction::SDiv(i) => Some((i.clone().into(), Box::new(V::sdiv))),
            Instruction::URem(i) => Some((i.clone().into(), Box::new(V::urem))),
            Instruction::SRem(i) => Some((i.clone().into(), Box::new(V::srem))),
            Instruction::And(i) => Some((i.clone().into(), Box::new(V::and))),
            Instruction::Or(i) => Some((i.clone().into(), Box::new(V::or))),
            Instruction::Xor(i) => Some((i.clone().into(), Box::new(V::xor))),
            Instruction::Shl(i) => Some((i.clone().into(), Box::new(V::shl))),
            Instruction::LShr(i) => Some((i.clone().into(), Box::new(V::lshr))),
            Instruction::AShr(i) => Some((i.clone().into(), Box::new(V::ashr))),
            _ => None,
        }
    }

    fn intpred_to_z3pred(pred: IntPredicate) -> Box<FnOnce(&B::BV, &B::BV) -> B::Bool + 'ctx> {
        match pred {
            IntPredicate::EQ => Box::new(|a,b| B::BV::_eq(a,b)),
            IntPredicate::NE => Box::new(|a,b| B::Bool::not(&B::BV::_eq(a,b))),
            IntPredicate::UGT => Box::new(B::BV::ugt),
            IntPredicate::UGE => Box::new(B::BV::uge),
            IntPredicate::ULT => Box::new(B::BV::ult),
            IntPredicate::ULE => Box::new(B::BV::ule),
            IntPredicate::SGT => Box::new(B::BV::sgt),
            IntPredicate::SGE => Box::new(B::BV::sge),
            IntPredicate::SLT => Box::new(B::BV::slt),
            IntPredicate::SLE => Box::new(B::BV::sle),
        }
    }

    fn symex_binop<F>(&mut self, bop: &instruction::groups::BinaryOp, z3op: F) -> Result<()>
        where F: FnOnce(&B::BV, &B::BV) -> B::BV
    {
        debug!("Symexing binop {:?}", bop);
        let z3firstop = self.state.operand_to_bv(&bop.get_operand0())?;
        let z3secondop = self.state.operand_to_bv(&bop.get_operand1())?;
        self.state.record_bv_result(bop, z3op(&z3firstop, &z3secondop))
    }

    fn symex_icmp(&mut self, icmp: &instruction::ICmp) -> Result<()> {
        debug!("Symexing icmp {:?}", icmp);
        let z3firstop = self.state.operand_to_bv(&icmp.operand0)?;
        let z3secondop = self.state.operand_to_bv(&icmp.operand1)?;
        let z3pred = Self::intpred_to_z3pred(icmp.predicate);
        self.state.record_bool_result(icmp, z3pred(&z3firstop, &z3secondop))
    }

    fn symex_zext(&mut self, zext: &instruction::ZExt) -> Result<()> {
        debug!("Symexing zext {:?}", zext);
        let z3op = self.state.operand_to_bv(&zext.operand)?;
        let source_size = z3op.get_size();
        let dest_size = match zext.get_type() {
            Type::IntegerType { bits } => bits,
            Type::VectorType { .. } => return Err(Error::UnsupportedInstruction("ZExt on a vector".to_owned())),
            ty => return Err(Error::MalformedInstruction(format!("Expected ZExt result type to be integer or vector of integers; got {:?}", ty))),
        };
        self.state.record_bv_result(zext, z3op.zero_ext(dest_size - source_size))
    }

    fn symex_sext(&mut self, sext: &instruction::SExt) -> Result<()> {
        debug!("Symexing sext {:?}", sext);
        let z3op = self.state.operand_to_bv(&sext.operand)?;
        let source_size = z3op.get_size();
        let dest_size = match sext.get_type() {
            Type::IntegerType { bits } => bits,
            Type::VectorType { .. } => return Err(Error::UnsupportedInstruction("SExt on a vector".to_owned())),
            ty => return Err(Error::MalformedInstruction(format!("Expected SExt result type to be integer or vector of integers; got {:?}", ty))),
        };
        self.state.record_bv_result(sext, z3op.sign_ext(dest_size - source_size))
    }

    fn symex_trunc(&mut self, trunc: &instruction::Trunc) -> Result<()> {
        debug!("Symexing trunc {:?}", trunc);
        let z3op = self.state.operand_to_bv(&trunc.operand)?;
        let dest_size = match trunc.get_type() {
            Type::IntegerType { bits } => bits,
            Type::VectorType { .. } => return Err(Error::UnsupportedInstruction("Trunc on a vector".to_owned())),
            ty => return Err(Error::MalformedInstruction(format!("Expected Trunc result type to be integer or vector of integers; got {:?}", ty))),
        };
        self.state.record_bv_result(trunc, z3op.extract(dest_size-1, 0))
    }

    /// Use this for any unary operation that can be treated as a cast
    fn symex_cast_op(&mut self, cast: &impl instruction::UnaryOp) -> Result<()> {
        debug!("Symexing cast op {:?}", cast);
        let z3op = self.state.operand_to_bv(&cast.get_operand())?;
        self.state.record_bv_result(cast, z3op)  // from Z3's perspective a cast is simply a no-op; the bit patterns are equal
    }

    fn symex_load(&mut self, load: &instruction::Load) -> Result<()> {
        debug!("Symexing load {:?}", load);
        let z3addr = self.state.operand_to_bv(&load.address)?;
        let dest_size = size(&load.get_type());
        self.state.record_bv_result(load, self.state.read(&z3addr, dest_size as u32))
    }

    fn symex_store(&mut self, store: &instruction::Store) -> Result<()> {
        debug!("Symexing store {:?}", store);
        let z3val = self.state.operand_to_bv(&store.value)?;
        let z3addr = self.state.operand_to_bv(&store.address)?;
        self.state.write(&z3addr, z3val);
        Ok(())
    }

    fn symex_gep(&mut self, gep: &'p instruction::GetElementPtr) -> Result<()> {
        debug!("Symexing gep {:?}", gep);
        let z3base = self.state.operand_to_bv(&gep.address)?;
        let offset = Self::get_offset(&self.state, gep.indices.iter(), &gep.address.get_type(), z3base.get_size())?;
        self.state.record_bv_result(gep, z3base.add(&offset).simplify())
    }

    /// Get the offset of the element (in bytes, as a `BV` of `result_bits` bits)
    fn get_offset(state: &State<'ctx, 'p, B>, mut indices: impl Iterator<Item = &'p Operand>, base_type: &Type, result_bits: u32) -> Result<B::BV> {
        let index = indices.next();
        if index.is_none() {
            return Ok(BV::from_u64(state.ctx, 0, result_bits));
        }
        let index = index.unwrap();  // we just handled the `None` case, so now it must have been `Some`
        match base_type {
            Type::PointerType { pointee_type, .. }
            | Type::ArrayType { element_type: pointee_type, .. }
            | Type::VectorType { element_type: pointee_type, .. }
            => {
                let el_size_bits = size(pointee_type) as u64;
                if el_size_bits % 8 != 0 {
                    return Err(Error::UnsupportedInstruction(format!("GetElementPtr involving a type with size {} bits", el_size_bits)));
                }
                let el_size_bytes = el_size_bits / 8;
                Ok(zero_extend_to_bits(state.operand_to_bv(index)?, result_bits)
                    .mul(&B::BV::from_u64(state.ctx, el_size_bytes, result_bits))
                    .add(&Self::get_offset(state, indices, pointee_type, result_bits)?))
            },
            Type::StructType { element_types, .. } => match index {
                Operand::ConstantOperand(Constant::Int { value: index, .. }) => {
                    let mut offset_bits = 0;
                    for ty in element_types.iter().take(*index as usize) {
                        offset_bits += size(ty) as u64;
                    }
                    if offset_bits % 8 != 0 {
                        return Err(Error::UnsupportedInstruction(format!("Struct offset of {} bits", offset_bits)));
                    }
                    let offset_bytes = offset_bits / 8;
                    Ok(B::BV::from_u64(state.ctx, offset_bytes, result_bits)
                        .add(&Self::get_offset(state, indices, &element_types[*index as usize], result_bits)?))
                },
                _ => Err(Error::MalformedInstruction(format!("Expected index into struct type to be constant, but got index {:?}", index))),
            },
            Type::NamedStructType { ty, .. } => {
                let rc: Rc<RefCell<Type>> = ty.as_ref()
                    .ok_or_else(|| Error::MalformedInstruction("get_offset on an opaque struct type".to_owned()))?
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
                                return Err(Error::UnsupportedInstruction(format!("Struct offset of {} bits", offset_bits)));
                            }
                            let offset_bytes = offset_bits / 8;
                            Ok(B::BV::from_u64(state.ctx, offset_bytes, result_bits)
                                .add(&Self::get_offset(state, indices, &element_types[*index as usize], result_bits)?))
                        },
                        _ => Err(Error::MalformedInstruction(format!("Expected index into struct type to be constant, but got index {:?}", index))),
                    }
                } else {
                    Err(Error::MalformedInstruction(format!("Expected NamedStructType inner type to be a StructType, but got {:?}", actual_ty)))
                }
            }
            _ => panic!("get_offset with base type {:?}", base_type),
        }
    }

    fn symex_alloca(&mut self, alloca: &instruction::Alloca) -> Result<()> {
        debug!("Symexing alloca {:?}", alloca);
        let allocation_size = size(&alloca.allocated_type);
        let allocated = self.state.allocate(allocation_size as u64);
        self.state.record_bv_result(alloca, allocated)
    }

    /// `instnum`: the index in the current `BasicBlock` of the given `Call` instruction.
    /// If the returned value is `Ok(Some(_))`, then this is the final return value of the top-level function (we had backtracking and finished on a different path).
    /// If the returned value is `Ok(None)`, then we finished the call normally, and execution should continue from here.
    fn symex_call(&mut self, call: &'p instruction::Call, instnum: usize) -> Result<Option<SymexResult<B::BV>>> {
        debug!("Symexing call {:?}", call);
        let funcname: &str = match &call.function {
            // the first two cases are really just optimizations for the third case; things should still work without the first two lines
            Either::Right(Operand::ConstantOperand(Constant::GlobalReference { name: Name::Name(name), .. })) => name,
            Either::Right(Operand::ConstantOperand(Constant::GlobalReference { name, .. })) => panic!("Function with a numbered name: {:?}", name),
            Either::Right(operand) => {
                match self.state.interpret_as_function_ptr(self.state.operand_to_bv(&operand)?, 1)? {
                    PossibleSolutions::MoreThanNPossibleSolutions(1) => unimplemented!("calling a function pointer which has multiple possible targets"),
                    PossibleSolutions::MoreThanNPossibleSolutions(n) => panic!("Expected n==1 since we passed in n==1, but got n=={:?}", n),
                    PossibleSolutions::PossibleSolutions(v) => match v.len() {
                        0 => return Err(Error::Unsat),  // no valid solutions for the function pointer
                        1 => &v[0].name,
                        n => panic!("Got {} solutions even though we asked for a maximum of one", n),
                    }
                }
            },
            Either::Left(_) => return Err(Error::UnsupportedInstruction("inline assembly".to_owned())),
        };
        if let Some(hook) = self.config.function_hooks.get(funcname) {
            match hook.call_hook(&mut self.state, call)? {
                ReturnValue::ReturnVoid => {
                    if call.get_type() != Type::VoidType {
                        return Err(Error::OtherError(format!("Hook for {:?} returned void but call needs a return value", funcname)));
                    }
                },
                ReturnValue::Return(retval) => {
                    if call.get_type() == Type::VoidType {
                        return Err(Error::OtherError(format!("Hook for {:?} returned a value but call is void-typed", funcname)));
                    } else {
                        // can't quite use `state.record_bv_result(call, retval)?` because Call is not HasResult
                        self.state.assign_bv_to_name(call.dest.as_ref().unwrap().clone(), retval)?;
                    }
                }
            }
            Ok(None)
        } else if let Some((callee, callee_mod)) = self.project.get_func_by_name(funcname) {
            assert_eq!(call.arguments.len(), callee.parameters.len());
            let z3args: Vec<B::BV> = call.arguments.iter()
                .map(|arg| self.state.operand_to_bv(&arg.0))  // have to do this before changing state.cur_loc, so that the lookups happen in the caller function
                .collect::<Result<Vec<B::BV>>>()?;
            let saved_loc = self.state.cur_loc.clone();  // don't need to save prev_bb because there can't be any more Phi instructions in this block (they all have to come before any Call instructions)
            self.state.push_callsite(instnum);
            let bb = callee.basic_blocks.get(0).expect("Failed to get entry basic block");
            self.state.cur_loc = Location {
                module: callee_mod,
                func: callee,
                bbname: bb.name.clone(),
            };
            for (z3arg, param) in z3args.into_iter().zip(callee.parameters.iter()) {
                self.state.assign_bv_to_name(param.name.clone(), z3arg)?;  // have to do the assign_bv_to_name calls after changing state.cur_loc, so that the variables are created in the callee function
            }
            info!("Entering function {:?} in module {:?}", funcname, &callee_mod.name);
            let returned_bv = self.symex_from_bb_through_end_of_function(&bb)?.ok_or(Error::Unsat)?;  // if symex_from_bb_through_end_of_function() returns `None`, this path is unsat
            match self.state.pop_callsite() {
                None => Ok(Some(returned_bv)),  // if there was no callsite to pop, then we finished elsewhere. See notes on `symex_call()`
                Some(Callsite { ref loc, inst }) if loc == &saved_loc && inst == instnum => {
                    self.state.cur_loc = saved_loc;
                    self.state.record_in_path(PathEntry {
                        modname: self.state.cur_loc.module.name.clone(),
                        funcname: self.state.cur_loc.func.name.clone(),
                        bbname: self.state.cur_loc.bbname.clone(),
                    });
                    match returned_bv {
                        SymexResult::Returned(bv) => {
                            // can't quite use `state.record_bv_result(call, bv)?` because Call is not HasResult
                            self.state.assign_bv_to_name(call.dest.as_ref().unwrap().clone(), bv)?;
                        },
                        SymexResult::ReturnedVoid => assert_eq!(call.dest, None),
                    };
                    debug!("Completed ordinary return to caller");
                    info!("Leaving function {:?}, continuing in caller {:?} in module {:?}", funcname, &self.state.cur_loc.func.name, &self.state.cur_loc.module.name);
                    Ok(None)
                },
                Some(callsite) => panic!("Received unexpected callsite {:?}", callsite),
            }
        } else if funcname.starts_with("llvm.memset") {
            symex_memset(&mut self.state, call)?;
            Ok(None)
        } else if funcname.starts_with("llvm.memcpy") || funcname.starts_with("llvm.memmove") {
            // Our memcpy implementation also works for memmove
            symex_memcpy(&mut self.state, call)?;
            Ok(None)
        } else if funcname.starts_with("llvm.lifetime")
            || funcname.starts_with("llvm.invariant")
            || funcname.starts_with("llvm.launder.invariant")
            || funcname.starts_with("llvm.strip.invariant")
        {
            Ok(None) // these are all safe to ignore
        } else {
            unimplemented!("Call of a function named {:?}", funcname)
        }
    }

    // Returns the `SymexResult` representing the return value
    fn symex_return(&self, ret: &terminator::Ret) -> Result<SymexResult<B::BV>> {
        debug!("Symexing return {:?}", ret);
        Ok(ret.return_operand
            .as_ref()
            .map(|op| self.state.operand_to_bv(op))
            .transpose()?  // turns Option<Result<_>> into Result<Option<_>>, then ?'s away the Result
            .map(SymexResult::Returned)
            .unwrap_or(SymexResult::ReturnedVoid))
    }

    // Continues to the target of the `Br` and eventually returns the new `SymexResult`
    // representing the return value of the function (when it reaches the end of the
    // function), or `Ok(None)` if no possible paths were found.
    fn symex_br(&mut self, br: &'p terminator::Br) -> Result<Option<SymexResult<B::BV>>> {
        debug!("Symexing br {:?}", br);
        self.state.prev_bb_name = Some(self.state.cur_loc.bbname.clone());
        self.state.cur_loc.bbname = br.dest.clone();
        self.symex_from_cur_loc_through_end_of_function()
    }

    // Continues to the target(s) of the `CondBr` (saving a backtracking point if
    // necessary) and eventually returns the new `SymexResult` representing the
    // return value of the function (when it reaches the end of the function), or
    // `Ok(None)` if no possible paths were found.
    fn symex_condbr(&mut self, condbr: &'p terminator::CondBr) -> Result<Option<SymexResult<B::BV>>> {
        debug!("Symexing condbr {:?}", condbr);
        let z3cond = self.state.operand_to_bool(&condbr.condition)?;
        let true_feasible = self.state.check_with_extra_constraints(std::iter::once(&z3cond))?;
        let false_feasible = self.state.check_with_extra_constraints(std::iter::once(&z3cond.not()))?;
        if true_feasible && false_feasible {
            // for now we choose to explore true first, and backtrack to false if necessary
            self.state.save_backtracking_point(condbr.false_dest.clone(), z3cond.not());
            self.state.assert(&z3cond);
            self.state.prev_bb_name = Some(self.state.cur_loc.bbname.clone());
            self.state.cur_loc.bbname = condbr.true_dest.clone();
            self.symex_from_cur_loc_through_end_of_function()
        } else if true_feasible {
            self.state.assert(&z3cond);  // unnecessary, but may help Z3 more than it hurts?
            self.state.prev_bb_name = Some(self.state.cur_loc.bbname.clone());
            self.state.cur_loc.bbname = condbr.true_dest.clone();
            self.symex_from_cur_loc_through_end_of_function()
        } else if false_feasible {
            self.state.assert(&z3cond.not());  // unnecessary, but may help Z3 more than it hurts?
            self.state.prev_bb_name = Some(self.state.cur_loc.bbname.clone());
            self.state.cur_loc.bbname = condbr.false_dest.clone();
            self.symex_from_cur_loc_through_end_of_function()
        } else {
            self.backtrack_and_continue()
        }
    }

    fn symex_phi(&mut self, phi: &instruction::Phi) -> Result<()> {
        debug!("Symexing phi {:?}", phi);
        let prev_bb = self.state.prev_bb_name.as_ref().expect("not yet implemented: starting in a block with Phi instructions. or error: didn't expect a Phi in function entry block");
        let mut chosen_value = None;
        for (op, bbname) in phi.incoming_values.iter() {
            if bbname == prev_bb {
                chosen_value = Some(op);
                break;
            }
        }
        let chosen_value = chosen_value.ok_or_else(|| Error::OtherError(format!("Failed to find a Phi member matching previous BasicBlock. Phi incoming_values are {:?} but we were looking for {:?}", phi.incoming_values, prev_bb)))?;
        self.state.record_bv_result(phi, self.state.operand_to_bv(&chosen_value)?)
    }

    fn symex_select(&mut self, select: &instruction::Select) -> Result<()> {
        debug!("Symexing select {:?}", select);
        let z3cond = self.state.operand_to_bool(&select.condition)?;
        let z3trueval = self.state.operand_to_bv(&select.true_value)?;
        let z3falseval = self.state.operand_to_bv(&select.false_value)?;
        let true_feasible = self.state.check_with_extra_constraints(std::iter::once(&z3cond))?;
        let false_feasible = self.state.check_with_extra_constraints(std::iter::once(&z3cond.not()))?;
        if true_feasible && false_feasible {
            self.state.record_bv_result(select, B::Bool::bvite(&z3cond, &z3trueval, &z3falseval))
        } else if true_feasible {
            self.state.assert(&z3cond);  // unnecessary, but may help Z3 more than it hurts?
            self.state.record_bv_result(select, z3trueval)
        } else if false_feasible {
            self.state.assert(&z3cond.not());  // unnecessary, but may help Z3 more than it hurts?
            self.state.record_bv_result(select, z3falseval)
        } else {
            // this path is unsat
            Err(Error::Unsat)
        }
    }
}

// Built-in "hooks" for LLVM intrinsics

fn symex_memset<'ctx, 'p, B>(state: &mut State<'ctx, 'p, B>, call: &'p instruction::Call) -> Result<()> where B: Backend<'ctx> {
    assert_eq!(call.arguments.len(), 4);
    let addr = &call.arguments[0].0;
    let val = &call.arguments[1].0;
    let num_bytes = &call.arguments[2].0;
    assert_eq!(addr.get_type(), Type::pointer_to(Type::i8()));
    assert_eq!(val.get_type(), Type::i8());
    assert_eq!(call.get_type(), Type::VoidType);
    let num_bytes = state.operand_to_bv(num_bytes)?;
    let num_bytes = match state.get_possible_solutions_for_bv(&num_bytes, 1)? {
        PossibleSolutions::MoreThanNPossibleSolutions(_) => return Err(Error::OtherError(format!("LLVM memset with non-constant num_bytes {:?}", num_bytes))),
        PossibleSolutions::PossibleSolutions(v) => v[0],
    };
    if num_bytes == 0 {
        debug!("Ignoring an LLVM memset of 0 num_bytes");
    } else {
        let addr = state.operand_to_bv(&addr)?;
        let val = state.operand_to_bv(&val)?;

        // Do this as just one large write; let the memory choose the most efficient way to implement that.
        state.write(&addr, std::iter::repeat(val).take(num_bytes as usize).reduce(|a,b| a.concat(&b)).unwrap().simplify());
    }
    Ok(())
}

fn symex_memcpy<'ctx, 'p, B>(state: &mut State<'ctx, 'p, B>, call: &'p instruction::Call) -> Result<()> where B: Backend<'ctx> {
    assert_eq!(call.arguments.len(), 4);
    let dest = &call.arguments[0].0;
    let src = &call.arguments[1].0;
    let num_bytes = &call.arguments[2].0;
    assert_eq!(dest.get_type(), Type::pointer_to(Type::i8()));
    assert_eq!(src.get_type(), Type::pointer_to(Type::i8()));
    assert_eq!(call.get_type(), Type::VoidType);
    let num_bytes = state.operand_to_bv(num_bytes)?;
    let num_bytes = match state.get_possible_solutions_for_bv(&num_bytes, 1)? {
        PossibleSolutions::MoreThanNPossibleSolutions(_) => return Err(Error::OtherError(format!("LLVM memcpy or memmove with non-constant num_bytes {:?}", num_bytes))),
        PossibleSolutions::PossibleSolutions(v) => v[0],
    };
    if num_bytes == 0 {
        debug!("Ignoring an LLVM memcpy or memmove of 0 num_bytes");
    } else {
        let dest = state.operand_to_bv(&dest)?;
        let src = state.operand_to_bv(&src)?;

        // Do the operation as just one large read and one large write; let the memory choose the most efficient way to implement these.
        let val = state.read(&src, num_bytes as u32 * 8);
        state.write(&dest, val);
    }
    Ok(())
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

    type Path = Vec<PathEntry>;

    fn path_from_bbnames(modname: &str, funcname: &str, bbnames: impl IntoIterator<Item = Name>) -> Path {
        let mut vec = vec![];
        for bbname in bbnames {
            vec.push(PathEntry { modname: modname.to_owned(), funcname: funcname.to_owned(), bbname });
        }
        vec
    }

    fn path_from_bbnums(modname: &str, funcname: &str, bbnums: impl IntoIterator<Item = usize>) -> Path {
        path_from_bbnames(modname, funcname, bbnums.into_iter().map(Name::Number))
    }

    fn path_from_func_and_bbname_pairs<'a>(modname: &str, pairs: impl IntoIterator<Item = (&'a str, Name)>) -> Path {
        let mut vec = vec![];
        for (funcname, bbname) in pairs {
            vec.push(PathEntry { modname: modname.to_owned(), funcname: funcname.to_owned(), bbname });
        }
        vec
    }

    fn path_from_func_and_bbnum_pairs<'a>(modname: &str, pairs: impl IntoIterator<Item = (&'a str, usize)>) -> Path {
        path_from_func_and_bbname_pairs(modname, pairs.into_iter().map(|(f, num)| (f, Name::Number(num))))
    }

    /// Build a path from (modname, funcname, bbnum) triples
    fn path_from_triples<'a>(triples: impl IntoIterator<Item = (&'a str, &'a str, usize)>) -> Path {
        let mut vec = vec![];
        for (modname, funcname, bbnum) in triples {
            vec.push(PathEntry { modname: modname.to_owned(), funcname: funcname.to_owned(), bbname: Name::Number(bbnum) });
        }
        vec
    }

    /// Iterator over the paths through a function
    struct PathIterator<'ctx, 'p, B> where B: Backend<'ctx> {
        em: ExecutionManager<'ctx, 'p, B>,
    }

    impl<'ctx, 'p, B> PathIterator<'ctx, 'p, B> where B: Backend<'ctx> {
        /// For argument descriptions, see notes on `symex_function`
        pub fn new(
            ctx: &'ctx z3::Context,
            funcname: &str,
            project: &'p Project,
            config: Config<'ctx, B>,
        ) -> Self {
            Self { em: symex_function(ctx, funcname, project, config) }
        }
    }

    impl<'ctx, 'p, B> Iterator for PathIterator<'ctx, 'p, B> where B: Backend<'ctx> + 'p {
        type Item = Path;

        fn next(&mut self) -> Option<Self::Item> {
            self.em.next().map(|_| self.em.state().get_path().clone())
        }
    }

    #[test]
    fn one_block() {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "one_arg";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1]));
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn two_paths() {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "conditional_true";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![2, 4, 12]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![2, 8, 12]));
        assert_eq!(paths.len(), 2);  // ensure there are no more paths
    }

    #[test]
    fn four_paths() {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "conditional_nozero";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![2, 4, 6, 14]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![2, 4, 8, 10, 14]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![2, 4, 8, 12, 14]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![2, 14]));
        assert_eq!(paths.len(), 4);  // ensure there are no more paths
    }

    #[test]
    fn while_loop() {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "while_loop";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 6, 6, 6, 6, 6, 12]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 6, 6, 6, 6, 12]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 6, 6, 6, 12]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 6, 6, 12]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 6, 12]));
        assert_eq!(paths.len(), 5);  // ensure there are no more paths
    }

    #[test]
    fn for_loop() {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "for_loop";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 6]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 9, 6]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 9, 9, 6]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 9, 9, 9, 6]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 9, 9, 9, 9, 6]));
        assert_eq!(paths[5], path_from_bbnums(modname, funcname, vec![1, 9, 9, 9, 9, 9, 6]));
        assert_eq!(paths.len(), 6);  // ensure there are no more paths
    }

    #[test]
    fn loop_more_blocks() {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "loop_zero_iterations";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 5, 8, 18]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 5, 11, 8, 18]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 5, 11, 11, 8, 18]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 5, 11, 11, 11, 8, 18]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 5, 11, 11, 11, 11, 8, 18]));
        assert_eq!(paths[5], path_from_bbnums(modname, funcname, vec![1, 5, 11, 11, 11, 11, 11, 8, 18]));
        assert_eq!(paths[6], path_from_bbnums(modname, funcname, vec![1, 18]));
        assert_eq!(paths.len(), 7);  // ensure there are no more paths
    }

    #[test]
    fn loop_more_blocks_in_body() {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "loop_with_cond";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 6, 13, 16,
                                                                         6, 10, 16,
                                                                         6, 10, 16,
                                                                         6, 13, 16,
                                                                         6, 10, 16, 20]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 6, 13, 16,
                                                                         6, 10, 16,
                                                                         6, 10, 16,
                                                                         6, 13, 16, 20]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 6, 13, 16,
                                                                         6, 10, 16,
                                                                         6, 10, 16, 20]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 6, 13, 16,
                                                                         6, 10, 16, 20]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 6, 13, 16, 20]));
        assert_eq!(paths.len(), 5);  // ensure there are no more paths
    }

    #[test]
    fn two_loops() {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "sum_of_array";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 30, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 4,  4,  4,  4,  4,  4,  4,  4,  4,  4,
                                                                         11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 9]));
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn nested_loop() {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "nested_loop";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 30, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                                     10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                                     10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                                     10, 7]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                                     10, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                                     10, 7]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 5, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                                                                     10, 7]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 7]));
        assert_eq!(paths.len(), 4);  // ensure there are no more paths
    }

    #[test]
    fn simple_call() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "simple_caller";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_func_and_bbnum_pairs(&modname, vec![
            ("simple_caller", 1),
            ("simple_callee", 2),
            ("simple_caller", 1),
        ]));
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn cross_module_simple_call() {
        let callee_modname = "tests/bcfiles/call.bc";
        let caller_modname = "tests/bcfiles/crossmod.bc";
        let funcname = "cross_module_simple_caller";
        init_logging();
        let proj = Project::from_bc_paths(vec![callee_modname, caller_modname].into_iter().map(std::path::Path::new))
            .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_triples(vec![
            (caller_modname, "cross_module_simple_caller", 1),
            (callee_modname, "simple_callee", 2),
            (caller_modname, "cross_module_simple_caller", 1),
        ]));
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn conditional_call() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "conditional_caller";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_func_and_bbnum_pairs(modname, vec![
            ("conditional_caller", 2),
            ("conditional_caller", 4),
            ("simple_callee", 2),
            ("conditional_caller", 4),
            ("conditional_caller", 8),
        ]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![2, 6, 8]));
        assert_eq!(paths.len(), 2);  // ensure there are no more paths
    }

    #[test]
    fn call_twice() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "twice_caller";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_func_and_bbnum_pairs(modname, vec![
            ("twice_caller", 1),
            ("simple_callee", 2),
            ("twice_caller", 1),
            ("simple_callee", 2),
            ("twice_caller", 1),
        ]));
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn cross_module_call_twice() {
        let callee_modname = "tests/bcfiles/call.bc";
        let caller_modname = "tests/bcfiles/crossmod.bc";
        let funcname = "cross_module_twice_caller";
        init_logging();
        let proj = Project::from_bc_paths(vec![callee_modname, caller_modname].into_iter().map(std::path::Path::new))
            .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_triples(vec![
            (caller_modname, "cross_module_twice_caller", 1),
            (callee_modname, "simple_callee", 2),
            (caller_modname, "cross_module_twice_caller", 1),
            (callee_modname, "simple_callee", 2),
            (caller_modname, "cross_module_twice_caller", 1),
        ]));
        assert_eq!(paths.len(), 1);  // enusre there are no more paths
    }

    #[test]
    fn nested_call() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "nested_caller";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_func_and_bbnum_pairs(modname, vec![
            ("nested_caller", 2),
            ("simple_caller", 1),
            ("simple_callee", 2),
            ("simple_caller", 1),
            ("nested_caller", 2),
        ]));
        assert_eq!(paths.len(), 1);  // ensure there are no more paths
    }

    #[test]
    fn cross_module_nested_near_call() {
        let callee_modname = "tests/bcfiles/call.bc";
        let caller_modname = "tests/bcfiles/crossmod.bc";
        let funcname = "cross_module_nested_near_caller";
        init_logging();
        let proj = Project::from_bc_paths(vec![callee_modname, caller_modname].into_iter().map(std::path::Path::new))
            .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_triples(vec![
            (caller_modname, "cross_module_nested_near_caller", 2),
            (caller_modname, "cross_module_simple_caller", 1),
            (callee_modname, "simple_callee", 2),
            (caller_modname, "cross_module_simple_caller", 1),
            (caller_modname, "cross_module_nested_near_caller", 2),
        ]));
        assert_eq!(paths.len(), 1);  // enusre there are no more paths
    }

    #[test]
    fn cross_module_nested_far_call() {
        let callee_modname = "tests/bcfiles/call.bc";
        let caller_modname = "tests/bcfiles/crossmod.bc";
        let funcname = "cross_module_nested_far_caller";
        init_logging();
        let proj = Project::from_bc_paths(vec![callee_modname, caller_modname].into_iter().map(std::path::Path::new))
            .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_triples(vec![
            (caller_modname, "cross_module_nested_far_caller", 2),
            (callee_modname, "simple_caller", 1),
            (callee_modname, "simple_callee", 2),
            (callee_modname, "simple_caller", 1),
            (caller_modname, "cross_module_nested_far_caller", 2),
        ]));
        assert_eq!(paths.len(), 1);  // enusre there are no more paths
    }

    #[test]
    fn call_of_loop() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "caller_of_loop";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_of_loop", 1),
            ("callee_with_loop", 2),
            ("callee_with_loop", 9),
            ("caller_of_loop", 1),
        ]));
        assert_eq!(paths[1], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_of_loop", 1),
            ("callee_with_loop", 2),
            ("callee_with_loop", 13),
            ("callee_with_loop", 9),
            ("caller_of_loop", 1),
        ]));
        assert_eq!(paths[2], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_of_loop", 1),
            ("callee_with_loop", 2),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 9),
            ("caller_of_loop", 1),
        ]));
        assert_eq!(paths[3], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_of_loop", 1),
            ("callee_with_loop", 2),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 9),
            ("caller_of_loop", 1),
        ]));
        assert_eq!(paths[4], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_of_loop", 1),
            ("callee_with_loop", 2),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 9),
            ("caller_of_loop", 1),
        ]));
        assert_eq!(paths[5], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_of_loop", 1),
            ("callee_with_loop", 2),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 13),
            ("callee_with_loop", 9),
            ("caller_of_loop", 1),
        ]));
        assert_eq!(paths.len(), 6);  // ensure there are no more paths
    }

    #[test]
    fn call_in_loop() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "caller_with_loop";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 3, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_with_loop", 1),
            ("caller_with_loop", 8),
        ]));
        assert_eq!(paths[1], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_with_loop", 1),
            ("caller_with_loop", 10),
            ("simple_callee", 2),
            ("caller_with_loop", 10),
            ("caller_with_loop", 6),
            ("caller_with_loop", 8),
        ]));
        assert_eq!(paths[2], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_with_loop", 1),
            ("caller_with_loop", 10),
            ("simple_callee", 2),
            ("caller_with_loop", 10),
            ("caller_with_loop", 10),
            ("simple_callee", 2),
            ("caller_with_loop", 10),
            ("caller_with_loop", 6),
            ("caller_with_loop", 8),
        ]));
        assert_eq!(paths[3], path_from_func_and_bbnum_pairs(modname, vec![
            ("caller_with_loop", 1),
            ("caller_with_loop", 10),
            ("simple_callee", 2),
            ("caller_with_loop", 10),
            ("caller_with_loop", 10),
            ("simple_callee", 2),
            ("caller_with_loop", 10),
            ("caller_with_loop", 10),
            ("simple_callee", 2),
            ("caller_with_loop", 10),
            ("caller_with_loop", 6),
            ("caller_with_loop", 8),
        ]));
        assert_eq!(paths.len(), 4);  // ensure there are no more paths
    }

    #[test]
    fn recursive_simple() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_simple";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 4, 6, 1, 4, 6, 1, 4, 6, 1, 4, 6, 1, 4, 9, 6, 6, 6, 6]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 4, 6, 1, 4, 6, 1, 4, 6, 1, 4, 6, 1, 9, 6, 6, 6, 6]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 4, 6, 1, 4, 6, 1, 4, 6, 1, 4, 9, 6, 6, 6]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 4, 6, 1, 4, 6, 1, 4, 6, 1, 9, 6, 6, 6]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 4, 6, 1, 4, 6, 1, 4, 9, 6, 6]));
        assert_eq!(paths[5], path_from_bbnums(modname, funcname, vec![1, 4, 6, 1, 4, 6, 1, 9, 6, 6]));
        assert_eq!(paths[6], path_from_bbnums(modname, funcname, vec![1, 4, 6, 1, 4, 9, 6]));
        assert_eq!(paths[7], path_from_bbnums(modname, funcname, vec![1, 4, 6, 1, 9, 6]));
        assert_eq!(paths[8], path_from_bbnums(modname, funcname, vec![1, 4, 9]));
        assert_eq!(paths[9], path_from_bbnums(modname, funcname, vec![1, 9]));
        assert_eq!(paths.len(), 10);  // ensure there are no more paths
    }

    #[test]
    fn recursive_double() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_double";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 4, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 4, 6, 8, 1, 4, 6, 8, 1, 4, 6, 8, 1, 4, 20, 8, 8, 8]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 4, 6, 8, 1, 4, 6, 8, 1, 4, 20, 8, 8]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 4, 6, 8, 1, 4, 20, 8]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 4, 6, 12, 14, 1, 4, 6, 8, 1, 4, 6, 8, 1, 4, 20, 8, 8, 14]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 4, 6, 12, 14, 1, 4, 6, 8, 1, 4, 20, 8, 14]));
        assert_eq!(paths[5], path_from_bbnums(modname, funcname, vec![1, 4, 6, 12, 14, 1, 4, 6, 12, 18, 20, 14]));
        assert_eq!(paths[6], path_from_bbnums(modname, funcname, vec![1, 4, 6, 12, 14, 1, 4, 20, 14]));
        assert_eq!(paths[7], path_from_bbnums(modname, funcname, vec![1, 4, 6, 12, 18, 20]));
        assert_eq!(paths[8], path_from_bbnums(modname, funcname, vec![1, 4, 20]));
        assert_eq!(paths[9], path_from_bbnums(modname, funcname, vec![1, 20]));
        assert_eq!(paths.len(), 10);  // ensure there are no more paths
    }

    #[test]
    fn recursive_not_tail() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_not_tail";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 3, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 3, 15]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 5, 1, 3, 15, 5, 10, 15]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 5, 1, 3, 15, 5, 12, 15]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 5, 1, 5, 1, 3, 15, 5, 10, 15, 5, 10, 15]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 5, 1, 5, 1, 3, 15, 5, 10, 15, 5, 12, 15]));
        assert_eq!(paths[5], path_from_bbnums(modname, funcname, vec![1, 5, 1, 5, 1, 3, 15, 5, 12, 15, 5, 10, 15]));
        assert_eq!(paths[6], path_from_bbnums(modname, funcname, vec![1, 5, 1, 5, 1, 3, 15, 5, 12, 15, 5, 12, 15]));
        assert_eq!(paths.len(), 7);  // ensure there are no more paths
    }

    #[test]
    fn recursive_and_normal_call() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_and_normal_caller";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 3, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_func_and_bbnum_pairs(modname, vec![
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 3),
            ("simple_callee", 2),
            ("recursive_and_normal_caller", 3),
            ("recursive_and_normal_caller", 7),
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 3),
            ("simple_callee", 2),
            ("recursive_and_normal_caller", 3),
            ("recursive_and_normal_caller", 7),
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 3),
            ("simple_callee", 2),
            ("recursive_and_normal_caller", 3),
            ("recursive_and_normal_caller", 10),
            ("recursive_and_normal_caller", 7),
            ("recursive_and_normal_caller", 7),
        ]));
        assert_eq!(paths[1], path_from_func_and_bbnum_pairs(modname, vec![
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 3),
            ("simple_callee", 2),
            ("recursive_and_normal_caller", 3),
            ("recursive_and_normal_caller", 7),
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 3),
            ("simple_callee", 2),
            ("recursive_and_normal_caller", 3),
            ("recursive_and_normal_caller", 10),
            ("recursive_and_normal_caller", 7),
        ]));
        assert_eq!(paths[2], path_from_func_and_bbnum_pairs(modname, vec![
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 3),
            ("simple_callee", 2),
            ("recursive_and_normal_caller", 3),
            ("recursive_and_normal_caller", 7),
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 10),
            ("recursive_and_normal_caller", 7),
        ]));
        assert_eq!(paths[3], path_from_func_and_bbnum_pairs(modname, vec![
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 3),
            ("simple_callee", 2),
            ("recursive_and_normal_caller", 3),
            ("recursive_and_normal_caller", 10),
        ]));
        assert_eq!(paths[4], path_from_func_and_bbnum_pairs(modname, vec![
            ("recursive_and_normal_caller", 1),
            ("recursive_and_normal_caller", 10),
        ]));
        assert_eq!(paths.len(), 5);  // ensure there are no more paths
    }

    #[test]
    fn mutually_recursive_functions() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "mutually_recursive_a";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let ctx = z3::Context::new(&z3::Config::new());
        let config = Config { loop_bound: 3, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<Z3Backend>::new(&ctx, funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_func_and_bbnum_pairs(modname, vec![
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
        ]));
        assert_eq!(paths[1], path_from_func_and_bbnum_pairs(modname, vec![
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 7),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
        ]));
        assert_eq!(paths[2], path_from_func_and_bbnum_pairs(modname, vec![
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
        ]));
        assert_eq!(paths[3], path_from_func_and_bbnum_pairs(modname, vec![
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 7),
            ("mutually_recursive_b", 3),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
        ]));
        assert_eq!(paths[4], path_from_func_and_bbnum_pairs(modname, vec![
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_b", 1),
            ("mutually_recursive_b", 7),
            ("mutually_recursive_a", 3),
            ("mutually_recursive_a", 7),
        ]));
        assert_eq!(paths[5], path_from_func_and_bbnum_pairs(modname, vec![
            ("mutually_recursive_a", 1),
            ("mutually_recursive_a", 7),
        ]));
        assert_eq!(paths.len(), 6);  // ensure there are no more paths
    }
}
