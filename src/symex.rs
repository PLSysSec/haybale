use either::Either;
use itertools::Itertools;
use llvm_ir::instruction::{BinaryOp, InlineAssembly};
use llvm_ir::types::NamedStructDef;
use llvm_ir::*;
use log::{debug, info};
use reduce::Reduce;
use std::convert::TryInto;
use std::fmt;

use crate::backend::*;
use crate::config::*;
use crate::error::*;
use crate::function_hooks::*;
use crate::parameter_val::ParameterVal;
use crate::project::Project;
use crate::return_value::*;
use crate::solver_utils::PossibleSolutions;
pub use crate::state::{BBInstrIndex, Location, LocationDescription, PathEntry, State};

/// Begin symbolic execution of the function named `funcname`, obtaining an
/// `ExecutionManager`.
///
/// `project`: The `Project` (set of LLVM modules) in which symbolic execution
/// should take place. In the absence of function hooks (see
/// [`Config`](struct.Config.html)), we will try to enter calls to any functions
/// defined in the `Project`.
///
/// `params`: a `ParameterVal` for each parameter to the function, indicating
/// what the initial value of that parameter should be, or if the parameter
/// should be unconstrained (so that the analysis considers all possible values
/// for the parameter).
/// `None` here is equivalent to supplying a `Vec` with all
/// `ParameterVal::Unconstrained` entries.
pub fn symex_function<'p, B: Backend>(
    funcname: &str,
    project: &'p Project,
    config: Config<'p, B>,
    params: Option<Vec<ParameterVal>>,
) -> Result<ExecutionManager<'p, B>> {
    debug!("Symexing function {}", funcname);
    let (func, module) = project
        .get_func_by_name(funcname)
        .unwrap_or_else(|| panic!("Failed to find function named {:?}", funcname));
    let start_loc = Location {
        module,
        func,
        bb: func
            .basic_blocks
            .get(0)
            .expect("Failed to get entry basic block"),
        instr: BBInstrIndex::Instr(0),
        source_loc: None, // this will be updated once we get there and begin symex of the instruction
    };
    let squash_unsats = config.squash_unsats;
    let mut state = State::new(project, start_loc, config);
    let params = params.unwrap_or_else(|| {
        std::iter::repeat(ParameterVal::Unconstrained)
            .take(func.parameters.len())
            .collect()
    });
    let bvparams: Vec<_> = func
        .parameters
        .iter()
        .zip_eq(params.into_iter())
        .map(|(param, paramval)| {
            let param_size = state
                .size_in_bits(&param.ty)
                .expect("Parameter type is a struct opaque in the entire Project");
            assert_ne!(param_size, 0, "Parameter {} shouldn't have size 0 bits", &param.name);
            let bvparam = state
                .new_bv_with_name(param.name.clone(), param_size)
                .unwrap();
            match paramval {
                ParameterVal::Unconstrained => {}, // nothing to do
                ParameterVal::ExactValue(val) => {
                    bvparam._eq(&state.bv_from_u64(val, param_size)).assert()?;
                },
                ParameterVal::Range(low, high) => {
                    debug_assert!(low <= high);
                    bvparam.ugte(&state.bv_from_u64(low, param_size)).assert()?;
                    bvparam.ulte(&state.bv_from_u64(high, param_size)).assert()?;
                },
                ParameterVal::NonNullPointer => {
                    match param.ty.as_ref() {
                        Type::PointerType { .. } => {
                            bvparam._ne(&state.zero(param_size)).assert()?;
                        },
                        ty => panic!("ParameterVal::NonNullPointer used for non-pointer parameter {} (which has type {:?})", &param.name, ty),
                    }
                }
                ParameterVal::PointerToAllocated(allocbytes) => {
                    match param.ty.as_ref() {
                        Type::PointerType { .. } => {
                            let allocbits = allocbytes * 8;
                            let allocated = state.allocate(allocbits);
                            bvparam._eq(&allocated).assert()?;
                        },
                        ty => panic!("ParameterVal::PointerToAllocated used for non-pointer parameter {} (which has type {:?})", &param.name, ty),
                    }
                }
            }
            Ok(bvparam)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(ExecutionManager::new(
        state,
        project,
        bvparams,
        squash_unsats,
    ))
}

/// An `ExecutionManager` allows you to symbolically explore executions of a
/// function. Conceptually, it is an `Iterator` over possible paths through the
/// function. Calling `next()` on an `ExecutionManager` explores another possible
/// path, returning either an `Ok` with a [`ReturnValue`](enum.ReturnValue.html)
/// representing the function's symbolic return value at the end of that path, or
/// an `Err` if an error was encountered while processing the path.
///
/// Importantly, after any call to `next()` (whether it results in an `Ok` or an
/// `Err`), you can access the `State` resulting from the end of that path using
/// the `state()` or `mut_state()` methods.
///
/// To get detailed information about an `Err` returned from a path, you can use
/// `state().full_error_message_with_context()`.
///
/// When `next()` returns `None`, there are no more possible paths through the
/// function.
pub struct ExecutionManager<'p, B: Backend> {
    state: State<'p, B>,
    project: &'p Project,
    func: &'p Function,
    bvparams: Vec<B::BV>,
    /// Whether the `ExecutionManager` is "fresh". A "fresh" `ExecutionManager`
    /// has not yet produced its first path, i.e., `next()` has not been called
    /// on it yet.
    fresh: bool,
    /// The `squash_unsats` setting from `Config`
    squash_unsats: bool,
}

impl<'p, B: Backend> ExecutionManager<'p, B> {
    fn new(
        state: State<'p, B>,
        project: &'p Project,
        bvparams: Vec<B::BV>,
        squash_unsats: bool,
    ) -> Self {
        let func = state.cur_loc.func;
        Self {
            state,
            project,
            func,
            bvparams,
            fresh: true,
            squash_unsats,
        }
    }

    /// Reference to the `Function` which the `ExecutionManager` is managing
    /// symbolic execution of. (This is the top-level function, i.e., the
    /// function we started the analysis in.)
    pub fn func(&self) -> &'p Function {
        self.func
    }

    /// Provides access to the `State` resulting from the end of the most recently
    /// explored path (or, if `next()` has never been called on this `ExecutionManager`,
    /// then simply the initial `State` which was passed in).
    pub fn state(&self) -> &State<'p, B> {
        &self.state
    }

    /// Provides mutable access to the underlying `State` (see notes on `state()`).
    /// Changes made to the initial state (before the first call to `next()`) are
    /// "sticky", and will persist through all executions of the function.
    /// However, changes made to a final state (after a call to `next()`) will be
    /// completely wiped away the next time that `next()` is called.
    pub fn mut_state(&mut self) -> &mut State<'p, B> {
        &mut self.state
    }

    /// Provides access to the `BV` objects representing each of the function's parameters
    pub fn param_bvs(&self) -> &Vec<B::BV> {
        &self.bvparams
    }
}

impl<'p, B: Backend> Iterator for ExecutionManager<'p, B>
where
    B: 'p,
{
    type Item = Result<ReturnValue<B::BV>>;

    fn next(&mut self) -> Option<Self::Item> {
        let retval = if self.fresh {
            self.fresh = false;
            info!(
                "Beginning symex in function {:?}",
                self.state.cur_loc.func.name
            );
            self.symex_from_cur_loc_through_end_of_function()
        } else {
            debug!("ExecutionManager: requesting next path");
            self.backtrack_and_continue()
        };
        retval.transpose()
    }
}

impl<'p, B: Backend> ExecutionManager<'p, B>
where
    B: 'p,
{
    /// Symex from the current `Location` through the rest of the function.
    /// Returns the `ReturnValue` representing the return value of the function,
    /// or `Ok(None)` if no possible paths were found.
    ///
    /// The current instruction index (`self.state.cur_loc.instr`) must be a
    /// valid instruction index for the current bb, with the exception that if
    /// the current bb contains no instructions (only a terminator),
    /// `BBInstrIndex::Instr(0)` will still be considered valid, and be treated
    /// equivalently to `BBInstrIndex::Terminator`.
    fn symex_from_cur_loc_through_end_of_function(&mut self) -> Result<Option<ReturnValue<B::BV>>> {
        debug!(
            "Symexing basic block {:?} in function {}",
            self.state.cur_loc.bb.name, self.state.cur_loc.func.name
        );
        let num_insts = self.state.cur_loc.bb.instrs.len();
        let insts_to_skip = match self.state.cur_loc.instr {
            BBInstrIndex::Instr(0) if num_insts == 0 => 0, // considered valid, see notes above
            BBInstrIndex::Instr(i) => {
                assert!(
                    i < num_insts,
                    "Invalid current instruction index: got (0-indexed) instruction {}, but current bb ({} in function {:?}) has only {} instructions plus a terminator",
                    i,
                    self.state.cur_loc.bb.name,
                    self.state.cur_loc.func.name,
                    num_insts,
                );
                i
            },
            BBInstrIndex::Terminator => num_insts, // skip all the instructions, go right to the terminator
        };
        let mut first_iter = true; // is it the first iteration of the for loop
        for (instnum, inst) in self
            .state
            .cur_loc
            .bb
            .instrs
            .iter()
            .enumerate()
            .skip(insts_to_skip)
        {
            self.state.cur_loc.instr = BBInstrIndex::Instr(instnum);
            self.state.cur_loc.source_loc = inst.get_debug_loc().as_ref();
            if first_iter {
                first_iter = false;
                self.state.record_path_entry(); // do this only on the first iteration
            }
            for callback in &self.state.config.callbacks.instruction_callbacks {
                callback(inst, &self.state)?;
            }
            let result = if let Ok(binop) = inst.clone().try_into() {
                self.symex_binop(&binop)
            } else {
                match inst {
                    Instruction::ICmp(icmp) => self.symex_icmp(icmp),
                    Instruction::Load(load) => self.symex_load(load),
                    Instruction::Store(store) => self.symex_store(store),
                    Instruction::GetElementPtr(gep) => self.symex_gep(gep),
                    Instruction::Alloca(alloca) => self.symex_alloca(alloca),
                    Instruction::ExtractElement(ee) => self.symex_extractelement(ee),
                    Instruction::InsertElement(ie) => self.symex_insertelement(ie),
                    Instruction::ShuffleVector(sv) => self.symex_shufflevector(sv),
                    Instruction::ExtractValue(ev) => self.symex_extractvalue(ev),
                    Instruction::InsertValue(iv) => self.symex_insertvalue(iv),
                    Instruction::ZExt(zext) => self.symex_zext(zext),
                    Instruction::SExt(sext) => self.symex_sext(sext),
                    Instruction::Trunc(trunc) => self.symex_trunc(trunc),
                    Instruction::PtrToInt(pti) => self.symex_cast_op(pti),
                    Instruction::IntToPtr(itp) => self.symex_cast_op(itp),
                    Instruction::BitCast(bitcast) => self.symex_cast_op(bitcast),
                    #[cfg(feature = "llvm-10-or-greater")]
                    Instruction::Freeze(freeze) => self.symex_cast_op(freeze), // since our BVs are never undef or poison, freeze is the identity operation for us
                    Instruction::Phi(phi) => self.symex_phi(phi),
                    Instruction::Select(select) => self.symex_select(select),
                    Instruction::CmpXchg(cmpxchg) => self.symex_cmpxchg(cmpxchg),
                    #[cfg(feature = "llvm-9-or-lower")]
                    Instruction::AtomicRMW(_) => return Err(Error::UnsupportedInstruction("LLVM `AtomicRMW` instruction is not supported for the LLVM 9 version of Haybale; see Haybale issue #12".into())),
                    #[cfg(feature = "llvm-10-or-greater")]
                    Instruction::AtomicRMW(armw) => self.symex_atomicrmw(armw),
                    Instruction::Call(call) => match self.symex_call(call) {
                        Err(e) => Err(e),
                        Ok(None) => Ok(()),
                        Ok(Some(symexresult)) => return Ok(Some(symexresult)),
                    },
                    Instruction::LandingPad(_) => return Err(Error::UnsupportedInstruction("Encountered an LLVM `LandingPad` instruction, but wasn't expecting it (there is no inflight exception)".to_owned())),
                    _ => return Err(Error::UnsupportedInstruction(format!("instruction {:?}", inst))),
                }
            };
            match result {
                Ok(_) => {}, // no error, we can continue
                Err(Error::Unsat) if self.squash_unsats => {
                    // we can't continue down this path anymore; try another
                    info!("Path is unsat");
                    return self.backtrack_and_continue();
                },
                Err(e) => return Err(e), // propagate any other errors
            };
        }
        let term = &self.state.cur_loc.bb.term;
        self.state.cur_loc.instr = BBInstrIndex::Terminator;
        self.state.cur_loc.source_loc = term.get_debug_loc().as_ref();
        if first_iter {
            // in this case, we did 0 iterations of the for loop, and still need to record the path entry
            self.state.record_path_entry();
        }
        for callback in &self.state.config.callbacks.terminator_callbacks {
            callback(term, &self.state)?;
        }
        match term {
            Terminator::Ret(ret) => self.symex_return(ret).map(Some),
            Terminator::Br(br) => self.symex_br(br),
            Terminator::CondBr(condbr) => self.symex_condbr(condbr),
            Terminator::Switch(switch) => self.symex_switch(switch),
            Terminator::Invoke(invoke) => self.symex_invoke(invoke),
            Terminator::Resume(resume) => self.symex_resume(resume),
            Terminator::Unreachable(_) => Err(Error::UnreachableInstruction),
            _ => Err(Error::UnsupportedInstruction(format!(
                "terminator {:?}",
                term
            ))),
        }
    }

    /// Revert to the most recent backtrack point, then continue execution from that point.
    /// Will continue not just to the end of the function containing the backtrack point,
    /// but (using the saved callstack) all the way back to the end of the top-level function.
    ///
    /// Returns the `ReturnValue` representing the final return value, or
    /// `Ok(None)` if no possible paths were found.
    fn backtrack_and_continue(&mut self) -> Result<Option<ReturnValue<B::BV>>> {
        if self.state.revert_to_backtracking_point()? {
            info!(
                "Reverted to backtrack point; {} more backtrack points available",
                self.state.count_backtracking_points()
            );
            info!(
                "Continuing in bb {} in function {:?}{}",
                self.state.cur_loc.bb.name,
                self.state.cur_loc.func.name,
                if self.state.config.print_module_name {
                    format!(", module {:?}", self.state.cur_loc.module.name)
                } else {
                    String::new()
                }
            );
            self.symex_from_cur_loc()
        } else {
            // No backtrack points (and therefore no paths) remain
            Ok(None)
        }
    }

    /// Symex starting from the current location, returning (using the saved
    /// callstack) all the way back to the end of the top-level function.
    ///
    /// The current instruction index (`self.state.cur_loc.instr`) must be a
    /// valid instruction index for the current bb, with the exception that if
    /// the current bb contains no instructions (only a terminator),
    /// `BBInstrIndex::Instr(0)` will still be considered valid, and be treated
    /// equivalently to `BBInstrIndex::Terminator`.
    ///
    /// Returns the `ReturnValue` representing the final return value, or
    /// `Ok(None)` if no possible paths were found.
    fn symex_from_cur_loc(&mut self) -> Result<Option<ReturnValue<B::BV>>> {
        match self.symex_from_cur_loc_through_end_of_function()? {
            Some(ReturnValue::Throw(bvptr)) => {
                // pop callsites until we find an `invoke` instruction that can direct us to a catch block
                loop {
                    match self.state.pop_callsite() {
                        Some(callsite) => match callsite.instr {
                            Either::Left(_call) => {
                                // a normal callsite, not an `invoke` instruction
                                info!("Caller {:?} (bb {}){} is not prepared to catch the exception, rethrowing",
                                    callsite.loc.func.name,
                                    callsite.loc.bb.name,
                                    if self.state.config.print_module_name {
                                        format!(" in module {:?}", callsite.loc.module.name)
                                    } else {
                                        String::new()
                                    },
                                );
                                continue;
                            },
                            Either::Right(invoke) => {
                                // catch the thrown value
                                info!(
                                    "Caller {:?} (bb {}){} catching the thrown value at bb {}",
                                    callsite.loc.func.name,
                                    callsite.loc.bb.name,
                                    if self.state.config.print_module_name {
                                        format!(" in module {:?}", callsite.loc.module.name)
                                    } else {
                                        String::new()
                                    },
                                    invoke.exception_label,
                                );
                                self.state.cur_loc = callsite.loc.clone();
                                return self
                                    .catch_at_exception_label(&bvptr, &invoke.exception_label);
                            },
                        },
                        None => {
                            // no callsite to return to, so we're done; exception was uncaught
                            return Ok(Some(ReturnValue::Throw(bvptr)));
                        },
                    }
                }
            },
            Some(ReturnValue::Abort) => Ok(Some(ReturnValue::Abort)),
            Some(symexresult) => match self.state.pop_callsite() {
                Some(callsite) => match callsite.instr {
                    Either::Left(call) => {
                        // Return to normal callsite
                        info!(
                            "Leaving function {:?}, continuing in caller {:?} (bb {}){}",
                            self.state.cur_loc.func.name,
                            callsite.loc.func.name,
                            callsite.loc.bb.name,
                            if self.state.config.print_module_name {
                                format!(" in module {:?}", callsite.loc.module.name)
                            } else {
                                String::new()
                            },
                        );
                        self.state.cur_loc = callsite.loc.clone();
                        // Assign the returned value as the result of the caller's call instruction
                        match symexresult {
                            ReturnValue::Return(bv) => {
                                if self
                                    .state
                                    .assign_bv_to_name(call.dest.as_ref().unwrap().clone(), bv)
                                    .is_err()
                                {
                                    // This path is dead, try backtracking again
                                    return self.backtrack_and_continue();
                                };
                            },
                            ReturnValue::ReturnVoid => {},
                            ReturnValue::Throw(_) => {
                                panic!("This case should have been handled above")
                            },
                            ReturnValue::Abort => {
                                panic!("This case should have been handled above")
                            },
                        };
                        // Continue execution in caller, with the instruction after the call instruction
                        self.state.cur_loc.inc(); // advance past the call instruction itself before recording the path entry. `saved_loc` must have been a call instruction, so can't be a terminator, so the call to `inc()` is safe.
                        self.symex_from_cur_loc()
                    },
                    Either::Right(invoke) => {
                        // Normal return to an `Invoke` instruction
                        info!("Leaving function {:?}, continuing in caller {:?}{} (finished invoke in bb {}, now in bb {})",
                            self.state.cur_loc.func.name,
                            callsite.loc.func.name,
                            if self.state.config.print_module_name {
                                format!(" in module {:?}", callsite.loc.module.name)
                            } else {
                                String::new()
                            },
                            callsite.loc.bb.name,
                            invoke.return_label,
                        );
                        self.state.cur_loc = callsite.loc.clone();
                        // Assign the returned value as the result of the `Invoke` instruction
                        match symexresult {
                            ReturnValue::Return(bv) => {
                                if self
                                    .state
                                    .assign_bv_to_name(invoke.result.clone(), bv)
                                    .is_err()
                                {
                                    // This path is dead, try backtracking again
                                    return self.backtrack_and_continue();
                                };
                            },
                            ReturnValue::ReturnVoid => {},
                            ReturnValue::Throw(_) => {
                                panic!("This case should have been handled above")
                            },
                            ReturnValue::Abort => {
                                panic!("This case should have been handled above")
                            },
                        };
                        // Continue execution in caller, at the normal-return label of the `Invoke` instruction
                        self.state
                            .cur_loc
                            .move_to_start_of_bb_by_name(&invoke.return_label);
                        self.symex_from_cur_loc()
                    },
                },
                None => {
                    // No callsite to return to, so we're done
                    Ok(Some(symexresult))
                },
            },
            None => {
                // This path is dead, try backtracking again
                self.backtrack_and_continue()
            },
        }
    }

    #[allow(clippy::type_complexity)]
    fn binop_to_bvbinop<'a, V: BV + 'a>(
        bop: &instruction::groups::BinaryOp,
    ) -> Result<Box<dyn for<'b> Fn(&'b V, &'b V) -> V + 'a>> {
        match bop {
            // TODO: how to not clone the inner instruction here
            instruction::groups::BinaryOp::Add(_) => Ok(Box::new(V::add)),
            instruction::groups::BinaryOp::Sub(_) => Ok(Box::new(V::sub)),
            instruction::groups::BinaryOp::Mul(_) => Ok(Box::new(V::mul)),
            instruction::groups::BinaryOp::UDiv(_) => Ok(Box::new(V::udiv)),
            instruction::groups::BinaryOp::SDiv(_) => Ok(Box::new(V::sdiv)),
            instruction::groups::BinaryOp::URem(_) => Ok(Box::new(V::urem)),
            instruction::groups::BinaryOp::SRem(_) => Ok(Box::new(V::srem)),
            instruction::groups::BinaryOp::And(_) => Ok(Box::new(V::and)),
            instruction::groups::BinaryOp::Or(_) => Ok(Box::new(V::or)),
            instruction::groups::BinaryOp::Xor(_) => Ok(Box::new(V::xor)),
            instruction::groups::BinaryOp::Shl(_) => Ok(Box::new(V::sll)),
            instruction::groups::BinaryOp::LShr(_) => Ok(Box::new(V::srl)),
            instruction::groups::BinaryOp::AShr(_) => Ok(Box::new(V::sra)),
            _ => Err(Error::UnsupportedInstruction(format!("BinaryOp {:?}", bop))),
        }
    }

    #[allow(clippy::type_complexity)]
    fn intpred_to_bvpred(pred: IntPredicate) -> Box<dyn Fn(&B::BV, &B::BV) -> B::BV + 'p> {
        match pred {
            IntPredicate::EQ => Box::new(B::BV::_eq),
            IntPredicate::NE => Box::new(B::BV::_ne),
            IntPredicate::UGT => Box::new(B::BV::ugt),
            IntPredicate::UGE => Box::new(B::BV::ugte),
            IntPredicate::ULT => Box::new(B::BV::ult),
            IntPredicate::ULE => Box::new(B::BV::ulte),
            IntPredicate::SGT => Box::new(B::BV::sgt),
            IntPredicate::SGE => Box::new(B::BV::sgte),
            IntPredicate::SLT => Box::new(B::BV::slt),
            IntPredicate::SLE => Box::new(B::BV::slte),
        }
    }

    fn symex_binop(&mut self, bop: &instruction::groups::BinaryOp) -> Result<()> {
        debug!("Symexing binop {:?}", bop);
        // We expect these binops to only operate on integers or vectors of integers
        let op0 = bop.get_operand0();
        let op1 = bop.get_operand1();
        let op0_type = self.state.type_of(op0);
        let op1_type = self.state.type_of(op1);
        if op0_type != op1_type {
            return Err(Error::MalformedInstruction(format!("Expected binary op to have two operands of same type, but have types {:?} and {:?}", op0_type, op1_type)));
        }
        let op_type = op0_type;
        let bvop0 = self.state.operand_to_bv(op0)?;
        let bvop1 = self.state.operand_to_bv(op1)?;
        let bvoperation = Self::binop_to_bvbinop(bop)?;
        match op_type.as_ref() {
            Type::IntegerType { .. } => {
                self.state.record_bv_result(bop, bvoperation(&bvop0, &bvop1))
            },
            #[cfg(feature = "llvm-11-or-greater")]
            Type::VectorType { scalable: true, .. } => {
                return Err(Error::UnsupportedInstruction("operation on scalable vectors".into()));
            }
            Type::VectorType { element_type, num_elements, .. } => {
                match element_type.as_ref() {
                    Type::IntegerType { .. } => {
                        self.state.record_bv_result(bop, binary_on_vector(&bvop0, &bvop1, *num_elements as u32, bvoperation)?)
                    },
                    ty => Err(Error::MalformedInstruction(format!("Expected binary operation's vector operands to have integer elements, but elements are type {:?}", ty))),
                }
            }
            ty => Err(Error::MalformedInstruction(format!("Expected binary operation to have operands of type integer or vector of integers, but got type {:?}", ty))),
        }
    }

    fn symex_icmp(&mut self, icmp: &'p instruction::ICmp) -> Result<()> {
        debug!("Symexing icmp {:?}", icmp);
        let bvfirstop = self.state.operand_to_bv(&icmp.operand0)?;
        let bvsecondop = self.state.operand_to_bv(&icmp.operand1)?;
        let bvpred = Self::intpred_to_bvpred(icmp.predicate);
        let op0_type = self.state.type_of(&icmp.operand0);
        let op1_type = self.state.type_of(&icmp.operand1);
        if op0_type != op1_type {
            return Err(Error::MalformedInstruction(format!(
                "Expected icmp to compare two operands of same type, but have types {:?} and {:?}",
                op0_type, op1_type
            )));
        }
        match self.state.type_of(icmp).as_ref() {
            Type::IntegerType { bits } if *bits == 1 => match op0_type.as_ref() {
                Type::IntegerType { .. } | Type::VectorType { .. } | Type::PointerType { .. } => {
                    self.state.record_bv_result(icmp, bvpred(&bvfirstop, &bvsecondop))
                },
                ty => Err(Error::MalformedInstruction(format!("Expected ICmp to have operands of type integer, pointer, or vector of integers, but got type {:?}", ty))),
            },
            #[cfg(feature = "llvm-11-or-greater")]
            Type::VectorType { scalable: true, .. } => {
                return Err(Error::UnsupportedInstruction("icmp on scalable vectors".into()));
            }
            Type::VectorType { element_type, num_elements, .. } => match element_type.as_ref() {
                Type::IntegerType { bits } if *bits == 1 => match op0_type.as_ref() {
                    Type::IntegerType { .. } | Type::VectorType { .. } | Type::PointerType { .. } => {
                        let zero = self.state.zero(1);
                        let one = self.state.one(1);
                        let final_bv = binary_on_vector(&bvfirstop, &bvsecondop, *num_elements as u32, |a,b| bvpred(a,b).cond_bv(&one, &zero))?;
                        self.state.record_bv_result(icmp, final_bv)
                    },
                    ty => Err(Error::MalformedInstruction(format!("Expected ICmp to have operands of type integer, pointer, or vector of integers, but got type {:?}", ty))),
                },
                ty => Err(Error::MalformedInstruction(format!("Expected ICmp result type to be i1 or vector of i1; got vector of {:?}", ty))),
            }
            ty => Err(Error::MalformedInstruction(format!("Expected ICmp result type to be i1 or vector of i1; got {:?}", ty))),
        }
    }

    fn symex_zext(&mut self, zext: &'p instruction::ZExt) -> Result<()> {
        debug!("Symexing zext {:?}", zext);
        match self.state.type_of(&zext.operand).as_ref() {
            Type::IntegerType { bits } => {
                let bvop = self.state.operand_to_bv(&zext.operand)?;
                let source_size = bits;
                let dest_size = self
                    .state
                    .size_in_bits(&self.state.type_of(zext))
                    .ok_or_else(|| {
                        Error::MalformedInstruction(
                            "ZExt return type is an opaque struct type".into(),
                        )
                    })?;
                self.state
                    .record_bv_result(zext, bvop.zext(dest_size - source_size))
            },
            #[cfg(feature = "llvm-11-or-greater")]
            Type::VectorType { scalable: true, .. } => {
                return Err(Error::UnsupportedInstruction("zext on a scalable vector".into()));
            }
            Type::VectorType {
                element_type,
                num_elements,
                ..
            } => {
                let in_vector = self.state.operand_to_bv(&zext.operand)?;
                let in_el_size = self.state.size_in_bits(&element_type).ok_or_else(|| {
                    Error::MalformedInstruction(
                        "ZExt operand type is a vector whose elements are opaque struct type"
                            .into(),
                    )
                })?;
                let out_el_size = match self.state.type_of(zext).as_ref() {
                    #[cfg(feature = "llvm-11-or-greater")]
                    Type::VectorType { scalable: true, .. } => {
                        return Err(Error::MalformedInstruction("ZExt result type is a scalable vector, but its operand is not".into()));
                    }
                    Type::VectorType {
                        element_type: out_el_type,
                        num_elements: out_num_els,
                        ..
                    } => {
                        if out_num_els != num_elements {
                            return Err(Error::MalformedInstruction(format!("ZExt operand is a vector of {} elements but output is a vector of {} elements", num_elements, out_num_els)));
                        }
                        self.state.size_in_bits(&out_el_type)
                            .ok_or_else(|| Error::MalformedInstruction("ZExt return type is a vector whose elements are opaque struct type".into()))?
                    },
                    ty => {
                        return Err(Error::MalformedInstruction(format!(
                            "ZExt operand is a vector type, but output is not: it is {:?}",
                            ty
                        )))
                    },
                };
                let final_bv = unary_on_vector(&in_vector, *num_elements as u32, |el| {
                    Ok(el.zext(out_el_size - in_el_size))
                })?;
                self.state.record_bv_result(zext, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!(
                "Expected ZExt operand type to be integer or vector of integers; got {:?}",
                ty
            ))),
        }
    }

    fn symex_sext(&mut self, sext: &'p instruction::SExt) -> Result<()> {
        debug!("Symexing sext {:?}", sext);
        match self.state.type_of(&sext.operand).as_ref() {
            Type::IntegerType { bits } => {
                let bvop = self.state.operand_to_bv(&sext.operand)?;
                let source_size = bits;
                let dest_size = self
                    .state
                    .size_in_bits(&self.state.type_of(sext))
                    .ok_or_else(|| {
                        Error::MalformedInstruction(
                            "SExt return type is an opaque struct type".into(),
                        )
                    })?;
                self.state
                    .record_bv_result(sext, bvop.sext(dest_size - source_size))
            },
            #[cfg(feature = "llvm-11-or-greater")]
            Type::VectorType { scalable: true, .. } => {
                return Err(Error::UnsupportedInstruction("sext on a scalable vector".into()));
            }
            Type::VectorType {
                element_type,
                num_elements,
                ..
            } => {
                let in_vector = self.state.operand_to_bv(&sext.operand)?;
                let in_el_size = self.state.size_in_bits(&element_type).ok_or_else(|| {
                    Error::MalformedInstruction(
                        "SExt operand type is a vector whose elements are opaque struct type"
                            .into(),
                    )
                })?;
                let out_el_size = match self.state.type_of(sext).as_ref() {
                    #[cfg(feature = "llvm-11-or-greater")]
                    Type::VectorType { scalable: true, .. } => {
                        return Err(Error::MalformedInstruction("SExt result type is a scalable vector, but its operand is not".into()));
                    }
                    Type::VectorType {
                        element_type: out_el_type,
                        num_elements: out_num_els,
                        ..
                    } => {
                        if out_num_els != num_elements {
                            return Err(Error::MalformedInstruction(format!("SExt operand is a vector of {} elements but output is a vector of {} elements", num_elements, out_num_els)));
                        }
                        self.state.size_in_bits(&out_el_type)
                            .ok_or_else(|| Error::MalformedInstruction("SExt return type is a vector whose elements are opaque struct type".into()))?
                    },
                    ty => {
                        return Err(Error::MalformedInstruction(format!(
                            "SExt operand is a vector type, but output is not: it is {:?}",
                            ty
                        )))
                    },
                };
                let final_bv = unary_on_vector(&in_vector, *num_elements as u32, |el| {
                    Ok(el.sext(out_el_size - in_el_size))
                })?;
                self.state.record_bv_result(sext, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!(
                "Expected SExt operand type to be integer or vector of integers; got {:?}",
                ty
            ))),
        }
    }

    fn symex_trunc(&mut self, trunc: &'p instruction::Trunc) -> Result<()> {
        debug!("Symexing trunc {:?}", trunc);
        match self.state.type_of(&trunc.operand).as_ref() {
            Type::IntegerType { .. } => {
                let bvop = self.state.operand_to_bv(&trunc.operand)?;
                let dest_size = self
                    .state
                    .size_in_bits(&self.state.type_of(trunc))
                    .ok_or_else(|| {
                        Error::MalformedInstruction(
                            "Trunc return type is an opaque struct type".into(),
                        )
                    })?;
                self.state
                    .record_bv_result(trunc, bvop.slice(dest_size - 1, 0))
            },
            #[cfg(feature = "llvm-11-or-greater")]
            Type::VectorType { scalable: true, .. } => {
                return Err(Error::UnsupportedInstruction("trunc on a scalable vector".into()));
            }
            Type::VectorType { num_elements, .. } => {
                let in_vector = self.state.operand_to_bv(&trunc.operand)?;
                let dest_el_size = match self.state.type_of(trunc).as_ref() {
                    #[cfg(feature = "llvm-11-or-greater")]
                    Type::VectorType { scalable: true, .. } => {
                        return Err(Error::MalformedInstruction("Trunc result type is a scalable vector, but its operand is not".into()));
                    },
                    Type::VectorType {
                        element_type: out_el_type,
                        num_elements: out_num_els,
                        ..
                    } => {
                        if out_num_els != num_elements {
                            return Err(Error::MalformedInstruction(format!("Trunc operand is a vector of {} elements but output is a vector of {} elements", num_elements, out_num_els)));
                        }
                        self.state.size_in_bits(&out_el_type)
                            .ok_or_else(|| Error::MalformedInstruction("Trunc return type is a vector whose elements are opaque struct type".into()))?
                    },
                    ty => {
                        return Err(Error::MalformedInstruction(format!(
                            "Trunc operand is a vector type, but output is not: it is {:?}",
                            ty
                        )))
                    },
                };
                let final_bv = unary_on_vector(&in_vector, *num_elements as u32, |el| {
                    Ok(el.slice(dest_el_size - 1, 0))
                })?;
                self.state.record_bv_result(trunc, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!(
                "Expected Trunc operand type to be integer or vector of integers; got {:?}",
                ty
            ))),
        }
    }

    /// Use this for any unary operation that can be treated as a cast
    fn symex_cast_op(&mut self, cast: &'p impl instruction::UnaryOp) -> Result<()> {
        debug!("Symexing cast op {:?}", cast);
        let bvop = self.state.operand_to_bv(&cast.get_operand())?;
        self.state.record_bv_result(cast, bvop) // from Boolector's perspective a cast is simply a no-op; the bit patterns are equal
    }

    fn symex_load(&mut self, load: &'p instruction::Load) -> Result<()> {
        debug!("Symexing load {:?}", load);
        let bvaddr = self.state.operand_to_bv(&load.address)?;
        let dest_size = self
            .state
            .size_in_bits(&self.state.type_of(load))
            .ok_or_else(|| {
                Error::MalformedInstruction("Load result type is an opaque struct type".into())
            })?;
        if dest_size == 0 {
            return Err(Error::MalformedInstruction(
                "Shouldn't be loading a value of size 0 bits".into(),
            ));
        }
        self.state
            .record_bv_result(load, self.state.read(&bvaddr, dest_size)?)
    }

    fn symex_store(&mut self, store: &'p instruction::Store) -> Result<()> {
        debug!("Symexing store {:?}", store);
        let bvval = self.state.operand_to_bv(&store.value)?;
        let bvaddr = self.state.operand_to_bv(&store.address)?;
        self.state.write(&bvaddr, bvval)
    }

    fn symex_gep(&mut self, gep: &'p instruction::GetElementPtr) -> Result<()> {
        debug!("Symexing gep {:?}", gep);
        match self.state.type_of(gep).as_ref() {
            Type::PointerType { .. } => {
                let bvbase = self.state.operand_to_bv(&gep.address)?;
                let offset = Self::get_offset_recursive(
                    &self.state,
                    gep.indices.iter(),
                    &self.state.type_of(&gep.address),
                    bvbase.get_width(),
                )?;
                self.state.record_bv_result(gep, bvbase.add(&offset))
            },
            Type::VectorType { .. } => Err(Error::UnsupportedInstruction(
                "GEP calculating a vector of pointers".to_owned(),
            )),
            ty => Err(Error::MalformedInstruction(format!(
                "Expected GEP result type to be pointer or vector of pointers; got {:?}",
                ty
            ))),
        }
    }

    /// Get the offset of the element (in bytes, as a `BV` of `result_bits` bits)
    ///
    /// If `base_type` is a `NamedStructType`, the struct should be defined in the `state`'s current module.
    fn get_offset_recursive(
        state: &State<'p, B>,
        mut indices: impl Iterator<Item = &'p Operand>,
        base_type: &Type,
        result_bits: u32,
    ) -> Result<B::BV> {
        if let Type::NamedStructType { name } = base_type {
            match state.cur_loc.module.types.named_struct_def(name) {
                None => {
                    return Err(Error::MalformedInstruction(format!(
                        "get_offset on a struct type not found in the current module (name {:?})",
                        name
                    )));
                },
                Some(NamedStructDef::Opaque) => {
                    return Err(Error::MalformedInstruction(format!(
                        "get_offset on an opaque struct type (name {:?})",
                        name
                    )));
                },
                Some(NamedStructDef::Defined(ty)) => {
                    return Self::get_offset_recursive(state, indices, &ty, result_bits);
                },
            }
        }
        match indices.next() {
            None => Ok(state.zero(result_bits)),
            Some(index) => match base_type {
                Type::PointerType { .. } | Type::ArrayType { .. } | Type::VectorType { .. } => {
                    let index = state.operand_to_bv(index)?.zero_extend_to_bits(result_bits);
                    let (offset, nested_ty) =
                        state.get_offset_bv_index(base_type, &index, state.solver.clone())?;
                    Self::get_offset_recursive(state, indices, nested_ty, result_bits)
                        .map(|bv| bv.add(&offset))
                },
                Type::StructType { .. } => match index {
                    Operand::ConstantOperand(cref) => match cref.as_ref() {
                        Constant::Int { value: index, .. } => {
                            let (offset, nested_ty) =
                                state.get_offset_constant_index(base_type, *index as usize)?;
                            Self::get_offset_recursive(state, indices, &nested_ty, result_bits)
                                .map(|bv| bv.add(&state.bv_from_u32(offset, result_bits)))
                        },
                        c => Err(Error::MalformedInstruction(format!(
                            "Expected index into struct type to be constant int, but got index {:?}",
                            c
                        )))
                    },
                    _ => Err(Error::MalformedInstruction(format!(
                        "Expected index into struct type to be constant int, but got index {:?}",
                        index
                    ))),
                },
                Type::NamedStructType { .. } => {
                    panic!("NamedStructType case should have been handled above")
                },
                _ => panic!("get_offset_recursive with base type {:?}", base_type),
            },
        }
    }

    fn symex_alloca(&mut self, alloca: &'p instruction::Alloca) -> Result<()> {
        debug!("Symexing alloca {:?}", alloca);
        match &alloca.num_elements {
            Operand::ConstantOperand(cref) => match cref.as_ref() {
                Constant::Int { value: num_elements, .. } => {
                    let allocation_size_bits = {
                        let element_size_bits = self
                            .state
                            .size_in_bits(&alloca.allocated_type)
                            .ok_or_else(|| {
                                Error::MalformedInstruction("Alloca with opaque struct type".into())
                            })?;
                        element_size_bits as u64 * *num_elements
                    };
                    let allocation_size_bits = if allocation_size_bits == 0 {
                        debug!("Alloca of 0 bits; we'll give it 8 bits anyway");
                        8
                    } else {
                        allocation_size_bits
                    };
                    let allocated = self.state.allocate(allocation_size_bits);
                    self.state.record_bv_result(alloca, allocated)
                },
                c => Err(Error::UnsupportedInstruction(format!(
                    "Alloca with num_elements not a constant int: {:?}",
                    c
                ))),
            },
            op => Err(Error::UnsupportedInstruction(format!(
                "Alloca with num_elements not a constant int: {:?}",
                op
            ))),
        }
    }

    fn symex_extractelement(&mut self, ee: &'p instruction::ExtractElement) -> Result<()> {
        debug!("Symexing extractelement {:?}", ee);
        let vector = self.state.operand_to_bv(&ee.vector)?;
        match &ee.index {
            Operand::ConstantOperand(cref) => match cref.as_ref() {
                Constant::Int { value: index, .. } => {
                    let index = *index as u32;
                    match self.state.type_of(&ee.vector).as_ref() {
                        Type::VectorType {
                            element_type,
                            num_elements,
                            ..
                        } => {
                            if index >= *num_elements as u32 {
                                Err(Error::MalformedInstruction(format!(
                                    "ExtractElement index out of range: index {} with {} elements", // or, (in LLVM 11+) trying to extract from a scalable vector, at an index which is not _guaranteed_ to exist
                                    index, num_elements
                                )))
                            } else {
                                let el_size = self.state.size_in_bits(&element_type)
                                    .ok_or_else(|| Error::MalformedInstruction("ExtractElement vector whose elements are opaque struct type".into()))?;
                                self.state.record_bv_result(
                                    ee,
                                    vector.slice((index + 1) * el_size - 1, index * el_size),
                                )
                            }
                        },
                        ty => Err(Error::MalformedInstruction(format!(
                            "Expected ExtractElement vector to be a vector type, got {:?}",
                            ty
                        ))),
                    }
                },
                c => Err(Error::UnsupportedInstruction(format!(
                    "ExtractElement with index not a constant int: {:?}",
                    c
                ))),
            },
            op => Err(Error::UnsupportedInstruction(format!(
                "ExtractElement with index not a constant int: {:?}",
                op
            ))),
        }
    }

    fn symex_insertelement(&mut self, ie: &'p instruction::InsertElement) -> Result<()> {
        debug!("Symexing insertelement {:?}", ie);
        let vector = self.state.operand_to_bv(&ie.vector)?;
        let element = self.state.operand_to_bv(&ie.element)?;
        match &ie.index {
            Operand::ConstantOperand(cref) => match cref.as_ref() {
                Constant::Int { value: index, .. } => {
                    let index = *index as u32;
                    match self.state.type_of(&ie.vector).as_ref() {
                        Type::VectorType {
                            element_type,
                            num_elements,
                            ..
                        } => {
                            if index >= *num_elements as u32 {
                                Err(Error::MalformedInstruction(format!(
                                    "InsertElement index out of range: index {} with {} elements", // or, (in LLVM 11+) trying to insert into a scalable vector, at an index which is not _guaranteed_ to exist
                                    index, num_elements
                                )))
                            } else {
                                let vec_size = vector.get_width();
                                let el_size = self.state.size_in_bits(&element_type)
                                    .ok_or_else(|| Error::MalformedInstruction("InsertElement element is an opaque named struct type".into()))?;
                                assert_eq!(vec_size, el_size * *num_elements as u32);
                                let insertion_bitindex_low = index * el_size; // lowest bit number in the vector which will be overwritten
                                let insertion_bitindex_high = (index + 1) * el_size - 1; // highest bit number in the vector which will be overwritten

                                let with_insertion = Self::overwrite_bv_segment(
                                    &mut self.state,
                                    &vector,
                                    element,
                                    insertion_bitindex_low,
                                    insertion_bitindex_high,
                                );

                                self.state.record_bv_result(ie, with_insertion)
                            }
                        },
                        ty => Err(Error::MalformedInstruction(format!(
                            "Expected InsertElement vector to be a vector type, got {:?}",
                            ty
                        ))),
                    }
                },
                c => Err(Error::UnsupportedInstruction(format!(
                    "InsertElement with index not a constant int: {:?}",
                    c
                ))),
            },
            op => Err(Error::UnsupportedInstruction(format!(
                "InsertElement with index not a constant int: {:?}",
                op
            ))),
        }
    }

    fn symex_shufflevector(&mut self, sv: &'p instruction::ShuffleVector) -> Result<()> {
        debug!("Symexing shufflevector {:?}", sv);
        let op_type = {
            let op0_type = self.state.type_of(&sv.operand0);
            let op1_type = self.state.type_of(&sv.operand1);
            if op0_type != op1_type {
                return Err(Error::MalformedInstruction(format!("Expected ShuffleVector operands to be exactly the same type, but they are {:?} and {:?}", op0_type, op1_type)));
            }
            op0_type
        };
        match op_type.as_ref() {
            #[cfg(feature = "llvm-11-or-greater")]
            Type::VectorType { scalable: true, .. } => {
                return Err(Error::UnsupportedInstruction("shufflevector on scalable vectors".into()));
            }
            Type::VectorType {
                element_type,
                num_elements,
                ..
            } => {
                let mask: Vec<u32> = match sv.mask.as_ref() {
                    Constant::Vector(mask) => mask.iter()
                        .map(|c| match c.as_ref() {
                            Constant::Int { value: idx, .. } => Ok(*idx as u32),
                            Constant::Undef(_) => Ok(0),
                            _ => Err(Error::UnsupportedInstruction(format!("ShuffleVector with a mask entry which is not a Constant::Int or Constant::Undef but instead {:?}", c))),
                        })
                        .collect::<Result<Vec<u32>>>()?,
                    Constant::AggregateZero(ty) | Constant::Undef(ty) => match ty.as_ref() {
                        Type::VectorType { num_elements, .. } => itertools::repeat_n(0, *num_elements).collect(),
                        _ => return Err(Error::MalformedInstruction(format!("Expected ShuffleVector mask (which is an AggregateZero or Undef) to have vector type, but its type is {:?}", ty))),
                    },
                    c => return Err(Error::MalformedInstruction(format!("Expected ShuffleVector mask to be a Constant::Vector, Constant::AggregateZero, or Constant::Undef, but got {:?}", c))),
                };
                let op0 = self.state.operand_to_bv(&sv.operand0)?;
                let op1 = self.state.operand_to_bv(&sv.operand1)?;
                if op0.get_width() != op1.get_width() {
                    return Err(Error::OtherError(format!("ShuffleVector operands are the same type, but somehow we got two different sizes: {} bits and {} bits", op0.get_width(), op1.get_width())));
                }
                let el_size = self.state.size_in_bits(&element_type).ok_or_else(|| {
                    Error::MalformedInstruction(
                        "ShuffleVector element type is an opaque struct type".into(),
                    )
                })?;
                let num_elements = *num_elements as u32;
                assert_eq!(op0.get_width(), el_size * num_elements);
                let final_bv = mask
                    .into_iter()
                    .map(|idx| {
                        if idx < num_elements {
                            op0.slice((idx + 1) * el_size - 1, idx * el_size)
                        } else {
                            let idx = idx - num_elements;
                            op1.slice((idx + 1) * el_size - 1, idx * el_size)
                        }
                    })
                    .reduce(|a, b| b.concat(&a))
                    .ok_or_else(|| {
                        Error::MalformedInstruction("ShuffleVector mask had 0 elements".to_owned())
                    })?;
                self.state.record_bv_result(sv, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!(
                "Expected ShuffleVector operands to be vectors, got {:?}",
                ty
            ))),
        }
    }

    fn symex_extractvalue(&mut self, ev: &'p instruction::ExtractValue) -> Result<()> {
        debug!("Symexing extractvalue {:?}", ev);
        let aggregate = self.state.operand_to_bv(&ev.aggregate)?;
        let (offset_bytes, size_bits) = self.get_offset_recursive_const_indices(
            ev.indices.iter().map(|i| *i as usize),
            &self.state.type_of(&ev.aggregate),
        )?;
        let low_offset_bits = offset_bytes * 8; // inclusive
        let high_offset_bits = low_offset_bits + size_bits; // exclusive
        assert!(aggregate.get_width() >= high_offset_bits, "Trying to extractvalue from an aggregate with total size {} bits, extracting offset {} bits to {} bits (inclusive) is out of bounds", aggregate.get_width(), low_offset_bits, high_offset_bits - 1);
        self.state
            .record_bv_result(ev, aggregate.slice(high_offset_bits - 1, low_offset_bits))
    }

    fn symex_insertvalue(&mut self, iv: &'p instruction::InsertValue) -> Result<()> {
        debug!("Symexing insertvalue {:?}", iv);
        let aggregate = self.state.operand_to_bv(&iv.aggregate)?;
        let element = self.state.operand_to_bv(&iv.element)?;
        let (offset_bytes, size_bits) = self.get_offset_recursive_const_indices(
            iv.indices.iter().map(|i| *i as usize),
            &self.state.type_of(&iv.aggregate),
        )?;
        let low_offset_bits = offset_bytes * 8; // inclusive
        let high_offset_bits = low_offset_bits + size_bits - 1; // inclusive
        assert!(aggregate.get_width() >= high_offset_bits, "Trying to insertvalue into an aggregate with total size {} bits, inserting offset {} bits to {} bits (inclusive) is out of bounds", aggregate.get_width(), low_offset_bits, high_offset_bits);

        let new_aggregate = Self::overwrite_bv_segment(
            &mut self.state,
            &aggregate,
            element,
            low_offset_bits,
            high_offset_bits,
        );

        self.state.record_bv_result(iv, new_aggregate)
    }

    /// Like `get_offset_recursive()` above, but with constant indices rather than `Operand`s.
    ///
    /// Returns the start offset (in bytes) of the indicated element, and the size (in bits) of the indicated element.
    fn get_offset_recursive_const_indices(
        &self,
        mut indices: impl Iterator<Item = usize>,
        base_type: &Type,
    ) -> Result<(u32, u32)> {
        if let Type::NamedStructType { name } = base_type {
            match self.project.get_named_struct_def(name) {
                Err(e) => {
                    return Err(Error::OtherError(format!("error during get_offset: {}", e)));
                },
                Ok((NamedStructDef::Opaque, _)) => {
                    return Err(Error::MalformedInstruction(format!(
                        "get_offset on an opaque struct type ({:?})",
                        name
                    )));
                },
                Ok((NamedStructDef::Defined(ty), _)) => {
                    return self.get_offset_recursive_const_indices(indices, &ty);
                },
            }
        }
        match indices.next() {
            None => Ok((
                0,
                self.state.size_in_bits(base_type).expect(
                    "base_type can't be a NamedStructType here because we handled that case above",
                ),
            )),
            Some(index) => match base_type {
                Type::PointerType { .. }
                | Type::ArrayType { .. }
                | Type::VectorType { .. }
                | Type::StructType { .. } => {
                    let (offset, nested_ty) =
                        self.state.get_offset_constant_index(base_type, index)?;
                    self.get_offset_recursive_const_indices(indices, &nested_ty)
                        .map(|(val, size)| (val + offset, size))
                },
                Type::NamedStructType { .. } => {
                    panic!("NamedStructType case should have been handled above")
                },
                _ => panic!(
                    "get_offset_recursive_const_indices with base type {:?}",
                    base_type
                ),
            },
        }
    }

    /// Helper function which overwrites a particular segment of a BV, returning the new BV
    ///
    /// Specifically, offsets `low_bitindex` to `high_bitindex` of
    /// `original_bv` (inclusive) will be overwritten with the data in
    /// `overwrite_data`, which must be exactly the correct length
    fn overwrite_bv_segment(
        state: &mut State<B>,
        original_bv: &B::BV,
        overwrite_data: B::BV,
        low_bitindex: u32,
        high_bitindex: u32,
    ) -> B::BV {
        let full_width = original_bv.get_width();
        let highest_bit_index = full_width - 1;
        assert!(high_bitindex <= highest_bit_index, "overwrite_bv_segment: high_bitindex {} is larger than highest valid bit index {} for an original_bv of width {}", high_bitindex, highest_bit_index, full_width);
        assert!(
            high_bitindex >= low_bitindex,
            "overwrite_bv_segment: high_bitindex {} is lower than low_bitindex {}",
            high_bitindex,
            low_bitindex
        );
        let overwrite_width = overwrite_data.get_width();
        assert_eq!(overwrite_width, high_bitindex - low_bitindex + 1, "overwrite_bv_segment: indicated a segment from bit {} to bit {} (width {}), but provided overwrite_data has width {}", low_bitindex, high_bitindex, high_bitindex - low_bitindex + 1, overwrite_width);

        // mask_clear is 0's in the bit positions that will be written, 1's elsewhere
        let zeroes = state.zero(overwrite_width);
        let mask_clear = if high_bitindex == highest_bit_index {
            if low_bitindex == 0 {
                zeroes
            } else {
                zeroes.concat(&state.ones(low_bitindex))
            }
        } else {
            let top = state
                .ones(highest_bit_index - high_bitindex)
                .concat(&zeroes);
            if low_bitindex == 0 {
                top
            } else {
                top.concat(&state.ones(low_bitindex))
            }
        };

        // mask_overwrite is the overwrite data in the appropriate bit positions, 0's elsewhere
        let top = overwrite_data.zero_extend_to_bits(full_width - low_bitindex);
        let mask_overwrite = if low_bitindex == 0 {
            top
        } else {
            top.concat(&state.zero(low_bitindex))
        };

        original_bv
            .and(&mask_clear) // zero out the segment we'll be writing
            .or(&mask_overwrite) // write the data into the appropriate position
    }

    /// If the returned value is `Ok(Some(_))`, then this is the final return value of the
    /// _current function_ (the function containing the call instruction), because either:
    ///     - we had backtracking and finished on a different path, and this is the final return value of the top-level function
    ///     - the called function threw an exception which the current function isn't set up to catch, so this is a `ReturnValue::Throw` which should be thrown from the current function
    ///
    /// If the returned value is `Ok(None)`, then we finished the call normally, and execution should continue from here.
    fn symex_call(&mut self, call: &'p instruction::Call) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing call {:?}", call);
        match self.resolve_function(&call.function)? {
            ResolvedFunction::HookActive { hook, hooked_thing } => {
                let pretty_hookedthing = hooked_thing.to_string();
                let quiet = if let HookedThing::Intrinsic(_) = hooked_thing {
                    true // executing the built-in hook of an intrinsic is relatively unimportant from a logging standpoint
                } else {
                    false // executing a hook for an actual function call is relatively important from a logging standpoint
                };
                match self.symex_hook(call, &hook, &pretty_hookedthing, quiet)? {
                    // Assume that `symex_hook()` has taken care of validating the hook return value as necessary
                    ReturnValue::Return(retval) => {
                        // can't quite use `state.record_bv_result(call, retval)?` because Call is not HasResult
                        self.state
                            .assign_bv_to_name(call.dest.as_ref().unwrap().clone(), retval)?;
                    },
                    ReturnValue::ReturnVoid => {},
                    ReturnValue::Throw(bvptr) => {
                        debug!("Hook threw an exception, but caller isn't inside a try block; rethrowing upwards");
                        return Ok(Some(ReturnValue::Throw(bvptr)));
                    },
                    ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                }
                let log_level = if quiet {
                    log::Level::Debug
                } else {
                    log::Level::Info
                };
                log::log!(
                    log_level,
                    "Done processing hook for {}; continuing in bb {} in function {:?}{}",
                    pretty_hookedthing,
                    self.state.cur_loc.bb.name,
                    self.state.cur_loc.func.name,
                    if self.state.config.print_module_name {
                        format!(", module {:?}", self.state.cur_loc.module.name)
                    } else {
                        String::new()
                    }
                );
                Ok(None)
            },
            ResolvedFunction::NoHookActive { called_funcname } => {
                let at_max_callstack_depth = match self.state.config.max_callstack_depth {
                    Some(max_depth) => self.state.current_callstack_depth() >= max_depth,
                    None => false,
                };
                if at_max_callstack_depth {
                    info!("Ignoring a call to function {:?} due to max_callstack_len setting (current callstack depth is {}, max is {})", called_funcname, self.state.current_callstack_depth(), self.state.config.max_callstack_depth.unwrap());
                    match self.state.type_of(call).as_ref() {
                        Type::VoidType => {},
                        ty => {
                            let width = self.state.size_in_bits(&ty).ok_or_else(|| {
                                Error::MalformedInstruction(
                                    "Call return type is an opaque struct type".into(),
                                )
                            })?;
                            assert_ne!(width, 0, "Function return type has size 0 bits but isn't void type"); // void type was handled above
                            let bv = self.state.new_bv_with_name(
                                Name::from(format!("{}_retval", called_funcname)),
                                width,
                            )?;
                            self.state
                                .assign_bv_to_name(call.dest.as_ref().unwrap().clone(), bv)?;
                        },
                    }
                    Ok(None)
                } else if let Some((callee, callee_mod)) =
                    self.state.get_func_by_name(called_funcname)
                {
                    if call.arguments.len() != callee.parameters.len() {
                        if callee.is_var_arg {
                            return Err(Error::UnsupportedInstruction(format!(
                                "Call of a function named {:?} which is variadic",
                                callee.name
                            )));
                        } else {
                            return Err(Error::MalformedInstruction(format!("Call of a function named {:?} which has {} parameters, but {} arguments were given", callee.name, callee.parameters.len(), call.arguments.len())));
                        }
                    }
                    let bvargs: Vec<B::BV> = call
                        .arguments
                        .iter()
                        .map(|arg| self.state.operand_to_bv(&arg.0)) // have to do this before changing state.cur_loc, so that the lookups happen in the caller function
                        .collect::<Result<Vec<B::BV>>>()?;
                    let saved_loc = self.state.cur_loc.clone();
                    self.state.push_callsite(call);
                    self.state.cur_loc = Location {
                        module: callee_mod,
                        func: callee,
                        bb: callee
                            .basic_blocks
                            .get(0)
                            .expect("Failed to get entry basic block"),
                        instr: BBInstrIndex::Instr(0),
                        source_loc: None, // this will be updated once we get there and begin symex of the instruction
                    };
                    for (bvarg, param) in bvargs.into_iter().zip_eq(callee.parameters.iter()) {
                        self.state.assign_bv_to_name(param.name.clone(), bvarg)?;
                        // have to do the assign_bv_to_name calls after changing state.cur_loc, so that the variables are created in the callee function
                    }
                    info!(
                        "Entering function {:?}{}",
                        called_funcname,
                        if self.state.config.print_module_name {
                            format!("in module {:?}", &callee_mod.name)
                        } else {
                            String::new()
                        },
                    );
                    let returned_bv = self
                        .symex_from_cur_loc_through_end_of_function()?
                        .ok_or(Error::Unsat)?; // if symex_from_cur_loc_through_end_of_function() returns `None`, this path is unsat
                    match self.state.pop_callsite() {
                        None => Ok(Some(returned_bv)), // if there was no callsite to pop, then we finished elsewhere. See notes on `symex_call()`
                        Some(ref callsite)
                            if callsite.loc == saved_loc && callsite.instr.is_left() =>
                        {
                            self.state.cur_loc = saved_loc;
                            self.state.cur_loc.inc(); // advance past the call instruction itself before recording the path entry. `saved_loc` must have been a call instruction, so can't be a terminator, so the call to `inc()` is safe.
                            self.state.record_path_entry();
                            match returned_bv {
                                ReturnValue::Return(bv) => {
                                    // can't quite use `state.record_bv_result(call, bv)?` because Call is not HasResult
                                    self.state.assign_bv_to_name(
                                        call.dest.as_ref().unwrap().clone(),
                                        bv,
                                    )?;
                                },
                                ReturnValue::ReturnVoid => assert_eq!(call.dest, None),
                                ReturnValue::Throw(bvptr) => {
                                    debug!("Callee threw an exception, but caller isn't inside a try block; rethrowing upwards");
                                    return Ok(Some(ReturnValue::Throw(bvptr)));
                                },
                                ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                            };
                            debug!("Completed ordinary return to caller");
                            info!(
                                "Leaving function {:?}, continuing in caller {:?} (bb {}){}",
                                called_funcname,
                                self.state.cur_loc.func.name,
                                self.state.cur_loc.bb.name,
                                if self.state.config.print_module_name {
                                    format!(" in module {:?}", self.state.cur_loc.module.name)
                                } else {
                                    String::new()
                                },
                            );
                            Ok(None)
                        }
                        Some(callsite) => panic!("Received unexpected callsite {:?}", callsite),
                    }
                } else {
                    match self.state.config.function_hooks.get_default_hook() {
                        None => Err(Error::FunctionNotFound(
                            self.state.demangle(called_funcname),
                        )),
                        Some(hook) => {
                            let hook = hook.clone(); // end the implicit borrow of `self` that arose from `get_default_hook()`. The `clone` is just an `Rc` and a `usize`, as of this writing
                            let pretty_funcname = self.state.demangle(called_funcname);
                            info!(
                                "Using default hook for a function named {:?}",
                                pretty_funcname
                            );
                            match self.symex_hook(call, &hook.clone(), &pretty_funcname, true)? {
                                // Assume that `symex_hook()` has taken care of validating the hook return value as necessary
                                ReturnValue::Return(retval) => {
                                    // can't quite use `state.record_bv_result(call, retval)?` because Call is not HasResult
                                    self.state.assign_bv_to_name(
                                        call.dest.as_ref().unwrap().clone(),
                                        retval,
                                    )?;
                                },
                                ReturnValue::ReturnVoid => {},
                                ReturnValue::Throw(bvptr) => {
                                    debug!("Hook threw an exception, but caller isn't inside a try block; rethrowing upwards");
                                    return Ok(Some(ReturnValue::Throw(bvptr)));
                                },
                                ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                            }
                            Ok(None)
                        },
                    }
                }
            },
        }
    }

    #[allow(clippy::if_same_then_else)] // in this case, having some identical `if` blocks actually improves readability, I think
    fn resolve_function(
        &mut self,
        function: &'p Either<InlineAssembly, Operand>,
    ) -> Result<ResolvedFunction<'p, B>> {
        use crate::global_allocations::Callable;
        let funcname_or_hook: Either<&str, FunctionHook<B>> = match function {
            // the first case is really just an optimization for the second case; things should still work if the first case was omitted
            Either::Right(Operand::ConstantOperand(cref)) if is_global_reference(cref) => match cref.as_ref() {
                Constant::GlobalReference { name: Name::Name(name), .. } => Either::Left(name),
                Constant::GlobalReference { name, .. } => panic!("Function with a numbered name: {:?}", name),
                _ => panic!("Expected only a GlobalReference here because of earlier check"),
            },
            Either::Right(operand) => {
                match self.state.interpret_as_function_ptr(self.state.operand_to_bv(&operand)?, 1)? {
                    PossibleSolutions::AtLeast(_) => return Err(Error::OtherError("calling a function pointer which has multiple possible targets".to_owned())),  // there must be at least 2 targets since we passed n==1 to `interpret_as_function_ptr`
                    PossibleSolutions::Exactly(v) => match v.iter().next() {
                        None => return Err(Error::Unsat),  // no valid solutions for the function pointer
                        Some(Callable::LLVMFunction(f)) => Either::Left(&f.name),
                        Some(Callable::FunctionHook(h)) => Either::Right(h.clone()),
                    }
                }
            },
            Either::Left(_) => match self.state.config.function_hooks.get_inline_asm_hook() {
                Some(hook) => return Ok(ResolvedFunction::HookActive {
                    hook: hook.clone(),
                    hooked_thing: HookedThing::InlineAsm,
                }),
                None => return Err(Error::OtherError("Encountered a call to inline assembly, but we have no inline assembly hook. Perhaps you want to add an inline assembly hook (see the documentation on FunctionHooks)?".to_owned())),
            },
        };
        match funcname_or_hook {
            Either::Left(funcname) => match self.state.config.function_hooks.get_hook_for(funcname)
            {
                Some(hook) => Ok(ResolvedFunction::HookActive {
                    hook: hook.clone(),
                    hooked_thing: HookedThing::Function(funcname),
                }),
                None => {
                    // No hook currently defined for this function, check if any intrinsic hooks apply
                    // (see notes on function resolution in function_hooks.rs)
                    if funcname.starts_with("llvm.memset") || funcname.starts_with("__memset") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.memset")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic memset hook"),
                            hooked_thing: HookedThing::Function(funcname),
                        })
                    } else if funcname.starts_with("llvm.memcpy")
                        || funcname.starts_with("llvm.memmove")
                        || funcname.starts_with("__memcpy")
                    {
                        // Our memcpy implementation also works for memmove
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.memcpy/memmove")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic memcpy/memmove hook"),
                            hooked_thing: HookedThing::Function(funcname),
                        })
                    } else if funcname.starts_with("llvm.bswap") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.bswap")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic bswap hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.ctlz") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.ctlz")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic ctlz hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.cttz") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.cttz")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic cttz hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.objectsize") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.objectsize")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic objectsize hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname == "llvm.assume" {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.assume")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic assume hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.uadd.with.overflow") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.uadd.with.overflow")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic uadd.with.overflow hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.sadd.with.overflow") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.sadd.with.overflow")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic sadd.with.overflow hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.usub.with.overflow") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.usub.with.overflow")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic usub.with.overflow hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.ssub.with.overflow") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.ssub.with.overflow")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic ssub.with.overflow hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.umul.with.overflow") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.umul.with.overflow")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic umul.with.overflow hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.smul_with_overflow") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.smul.with.overflow")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic smul.with.overflow hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.uadd.sat") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.uadd.sat")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic uadd.sat hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.sadd.sat") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.sadd.sat")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic sadd.sat hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.usub.sat") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.usub.sat")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic usub.sat hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.ssub.sat") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: llvm.ssub.sat")
                                .cloned()
                                .expect("Failed to find LLVM intrinsic ssub.sat hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.read_register")
                        || funcname.starts_with("llvm.write_register")
                    {
                        // These can just ignore their arguments and return unconstrained data, as appropriate
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: generic_stub_hook")
                                .cloned()
                                .expect("Failed to find intrinsic generic stub hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else if funcname.starts_with("llvm.lifetime")
                        || funcname.starts_with("llvm.invariant")
                        || funcname.starts_with("llvm.launder.invariant")
                        || funcname.starts_with("llvm.strip.invariant")
                        || funcname.starts_with("llvm.dbg")
                        || funcname.starts_with("llvm.expect")
                    {
                        // these are all safe to ignore
                        Ok(ResolvedFunction::HookActive {
                            hook: self
                                .state
                                .intrinsic_hooks
                                .get_hook_for("intrinsic: generic_stub_hook")
                                .cloned()
                                .expect("Failed to find intrinsic generic stub hook"),
                            hooked_thing: HookedThing::Intrinsic(funcname),
                        })
                    } else {
                        // No hook currently defined for this function, and none of our intrinsic hooks apply
                        Ok(ResolvedFunction::NoHookActive {
                            called_funcname: funcname,
                        })
                    }
                },
            },
            Either::Right(hook) => Ok(ResolvedFunction::HookActive {
                hook,
                hooked_thing: HookedThing::FunctionPtr,
            }),
        }
    }

    /// Execute the hook `hook` hooking the call `call`, returning the hook's `ReturnValue`.
    ///
    /// `hooked_funcname`: Name of the hooked function, used only for logging and error messages
    ///
    /// `quiet`: if `true`, then non-error log messages will be logged at `DEBUG`
    /// level; if `false`, then at `INFO` level. Callers should decide how
    /// important it is to point out to the user that a hook is being processed
    /// in this case.
    fn symex_hook(
        &mut self,
        call: &'p impl IsCall,
        hook: &FunctionHook<'p, B>,
        hooked_funcname: &str,
        quiet: bool,
    ) -> Result<ReturnValue<B::BV>> {
        let log_level = if quiet {
            log::Level::Debug
        } else {
            log::Level::Info
        };
        log::log!(log_level, "Processing hook for {}", hooked_funcname);
        match hook.call_hook(&mut self.state, call)? {
            ReturnValue::ReturnVoid => {
                if self.state.type_of(call).as_ref() == &Type::VoidType {
                    Ok(ReturnValue::ReturnVoid)
                } else {
                    Err(Error::HookReturnValueMismatch(format!(
                        "Hook for {:?} returned void but call needs a return value",
                        hooked_funcname
                    )))
                }
            },
            ReturnValue::Return(retval) => {
                let ret_type = self.state.type_of(call);
                if ret_type.as_ref() == &Type::VoidType {
                    Err(Error::HookReturnValueMismatch(format!(
                        "Hook for {:?} returned a value but call is void-typed",
                        hooked_funcname
                    )))
                } else {
                    let retwidth = self.state.size_in_bits(&ret_type).ok_or_else(|| {
                        Error::MalformedInstruction(
                            "Call return type is an opaque struct type".into(),
                        )
                    })?;
                    if retval.get_width() != retwidth {
                        Err(Error::HookReturnValueMismatch(format!("Hook for {:?} returned a {}-bit value but call's return type requires a {}-bit value", hooked_funcname, retval.get_width(), retwidth)))
                    } else {
                        Ok(ReturnValue::Return(retval))
                    }
                }
            },
            ReturnValue::Throw(bvptr) => Ok(ReturnValue::Throw(bvptr)), // throwing is always OK and doesn't need to be checked against function type
            ReturnValue::Abort => Ok(ReturnValue::Abort), // aborting is always OK and doesn't need to be checked against function type
        }
    }

    /// Returns the `ReturnValue` representing the return value
    fn symex_return(&self, ret: &'p terminator::Ret) -> Result<ReturnValue<B::BV>> {
        debug!("Symexing return {:?}", ret);
        Ok(ret
            .return_operand
            .as_ref()
            .map(|op| self.state.operand_to_bv(op))
            .transpose()? // turns Option<Result<_>> into Result<Option<_>>, then ?'s away the Result
            .map(ReturnValue::Return)
            .unwrap_or(ReturnValue::ReturnVoid))
    }

    /// Continues to the target of the `Br` and eventually returns the new `ReturnValue`
    /// representing the return value of the function (when it reaches the end of the
    /// function), or `Ok(None)` if no possible paths were found.
    fn symex_br(&mut self, br: &'p terminator::Br) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing br {:?}", br);
        self.state.cur_loc.move_to_start_of_bb_by_name(&br.dest);
        self.symex_from_cur_loc_through_end_of_function()
    }

    /// Continues to the target(s) of the `CondBr` (saving a backtracking point if
    /// necessary) and eventually returns the new `ReturnValue` representing the
    /// return value of the function (when it reaches the end of the function), or
    /// `Ok(None)` if no possible paths were found.
    fn symex_condbr(
        &mut self,
        condbr: &'p terminator::CondBr,
    ) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing condbr {:?}", condbr);
        let bvcond = self.state.operand_to_bv(&condbr.condition)?;
        let true_feasible = self
            .state
            .sat_with_extra_constraints(std::iter::once(&bvcond))?;
        let false_feasible = self
            .state
            .sat_with_extra_constraints(std::iter::once(&bvcond.not()))?;
        if true_feasible && false_feasible {
            debug!("both true and false branches are feasible");
            // for now we choose to explore true first, and backtrack to false if necessary
            self.state
                .save_backtracking_point(&condbr.false_dest, bvcond.not());
            bvcond.assert()?;
            self.state
                .cur_loc
                .move_to_start_of_bb_by_name(&condbr.true_dest);
            self.symex_from_cur_loc_through_end_of_function()
        } else if true_feasible {
            debug!("only the true branch is feasible");
            bvcond.assert()?; // unnecessary, but may help Boolector more than it hurts?
            self.state
                .cur_loc
                .move_to_start_of_bb_by_name(&condbr.true_dest);
            self.symex_from_cur_loc_through_end_of_function()
        } else if false_feasible {
            debug!("only the false branch is feasible");
            bvcond.not().assert()?; // unnecessary, but may help Boolector more than it hurts?
            self.state
                .cur_loc
                .move_to_start_of_bb_by_name(&condbr.false_dest);
            self.symex_from_cur_loc_through_end_of_function()
        } else {
            debug!("neither branch is feasible");
            self.backtrack_and_continue()
        }
    }

    /// Continues to the target(s) of the `Switch` (saving backtracking points if
    /// necessary) and eventually returns the new `ReturnValue` representing the
    /// return value of the function (when it reaches the end of the function), or
    /// `Ok(None)` if no possible paths were found.
    fn symex_switch(
        &mut self,
        switch: &'p terminator::Switch,
    ) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing switch {:?}", switch);
        let switchval = self.state.operand_to_bv(&switch.operand)?;
        let dests = switch
            .dests
            .iter()
            .map(|(c, n)| self.state.const_to_bv(c).map(|c| (c, n)))
            .collect::<Result<Vec<(B::BV, &Name)>>>()?;
        let feasible_dests: Vec<_> = dests
            .iter()
            .map(|(c, n)| {
                self.state
                    .bvs_can_be_equal(&c, &switchval)
                    .map(|b| (c, *n, b))
            })
            .collect::<Result<Vec<(&B::BV, &Name, bool)>>>()?
            .into_iter()
            .filter(|(_, _, b)| *b)
            .map(|(c, n, _)| (c, n))
            .collect::<Vec<(&B::BV, &Name)>>();
        if feasible_dests.is_empty() {
            // none of the dests are feasible, we will always end up in the default dest
            self.state
                .cur_loc
                .move_to_start_of_bb_by_name(&switch.default_dest);
            self.symex_from_cur_loc_through_end_of_function()
        } else {
            // make backtracking points for all but the first destination
            for (val, name) in feasible_dests.iter().skip(1) {
                self.state
                    .save_backtracking_point(name, val._eq(&switchval));
            }
            // if the default dest is feasible, make a backtracking point for it
            let default_dest_constraint = dests
                .iter()
                .map(|(c, _)| c._eq(&switchval).not())
                .reduce(|a, b| a.and(&b))
                .unwrap_or_else(|| self.state.bv_from_bool(true)); // if `dests` was empty, that's weird, but the default dest is definitely feasible
            if self
                .state
                .sat_with_extra_constraints(std::iter::once(&default_dest_constraint))?
            {
                self.state
                    .save_backtracking_point(&switch.default_dest, default_dest_constraint);
            }
            // follow the first destination
            let (val, name) = &feasible_dests[0];
            val._eq(&switchval).assert()?; // unnecessary, but may help Boolector more than it hurts?
            self.state.cur_loc.move_to_start_of_bb_by_name(name);
            self.symex_from_cur_loc_through_end_of_function()
        }
    }

    /// Continues to the target of the `Invoke` and eventually returns the new
    /// `ReturnValue` representing the return value of the function (when it
    /// reaches the end of the function), or `Ok(None)` if no possible paths were
    /// found.
    fn symex_invoke(
        &mut self,
        invoke: &'p terminator::Invoke,
    ) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing invoke {:?}", invoke);
        match self.resolve_function(&invoke.function)? {
            ResolvedFunction::HookActive { hook, hooked_thing } => {
                let pretty_hookedthing = hooked_thing.to_string();
                let quiet = if let HookedThing::Intrinsic(_) = hooked_thing {
                    true // executing the built-in hook of an intrinsic is relatively unimportant from a logging standpoint
                } else {
                    false // executing a hook for an actual function call is relatively important from a logging standpoint
                };
                match self.symex_hook(invoke, &hook, &pretty_hookedthing, quiet)? {
                    // Assume that `symex_hook()` has taken care of validating the hook return value as necessary
                    ReturnValue::Return(retval) => {
                        self.state
                            .assign_bv_to_name(invoke.result.clone(), retval)?;
                    },
                    ReturnValue::ReturnVoid => {},
                    ReturnValue::Throw(bvptr) => {
                        info!("Hook for {} threw an exception, which we are catching at bb {} in function {:?}{}",
                            pretty_hookedthing, invoke.exception_label, self.state.cur_loc.func.name,
                            if self.state.config.print_module_name {
                                format!(", module {:?}", self.state.cur_loc.module.name)
                            } else {
                                String::new()
                            }
                        );
                        return self.catch_at_exception_label(&bvptr, &invoke.exception_label);
                    },
                    ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                };
                let old_bb_name = &self.state.cur_loc.bb.name;
                // We had a normal return, so continue at the `return_label`
                self.state
                    .cur_loc
                    .move_to_start_of_bb_by_name(&invoke.return_label);
                let log_level = if quiet {
                    log::Level::Debug
                } else {
                    log::Level::Info
                };
                log::log!(log_level, "Done processing hook for {}; continuing in function {:?}{} (hook was for the invoke in bb {}, now in bb {})",
                    pretty_hookedthing,
                    self.state.cur_loc.func.name,
                    if self.state.config.print_module_name {
                        format!(" in module {:?}", self.state.cur_loc.module.name)
                    } else {
                        String::new()
                    },
                    old_bb_name,
                    self.state.cur_loc.bb.name,
                );
                self.symex_from_cur_loc_through_end_of_function()
            },
            ResolvedFunction::NoHookActive { called_funcname } => {
                let at_max_callstack_depth = match self.state.config.max_callstack_depth {
                    Some(max_depth) => self.state.current_callstack_depth() >= max_depth,
                    None => false,
                };
                if at_max_callstack_depth {
                    info!("Ignoring a call to function {:?} due to max_callstack_len setting (current callstack depth is {}, max is {})", called_funcname, self.state.current_callstack_depth(), self.state.config.max_callstack_depth.unwrap());
                    match self.state.type_of(invoke).as_ref() {
                        Type::VoidType => {},
                        ty => {
                            let width = self.state.size_in_bits(&ty).ok_or_else(|| {
                                Error::MalformedInstruction(
                                    "Invoke return type is an opaque struct type".into(),
                                )
                            })?;
                            assert_ne!(width, 0, "Invoke return type has size 0 bits but isn't void type"); // void type was handled above
                            let bv = self.state.new_bv_with_name(
                                Name::from(format!("{}_retval", called_funcname)),
                                width,
                            )?;
                            self.state.assign_bv_to_name(invoke.result.clone(), bv)?;
                        },
                    }
                    self.state
                        .cur_loc
                        .move_to_start_of_bb_by_name(&invoke.return_label);
                    self.symex_from_cur_loc_through_end_of_function()
                } else if let Some((callee, callee_mod)) =
                    self.state.get_func_by_name(called_funcname)
                {
                    if invoke.arguments.len() != callee.parameters.len() {
                        if callee.is_var_arg {
                            return Err(Error::UnsupportedInstruction(format!(
                                "Call of a function named {:?} which is variadic",
                                callee.name
                            )));
                        } else {
                            return Err(Error::MalformedInstruction(format!("Call of a function named {:?} which has {} parameters, but {} arguments were given", callee.name, callee.parameters.len(), invoke.arguments.len())));
                        }
                    }
                    let bvargs: Vec<B::BV> = invoke
                        .arguments
                        .iter()
                        .map(|arg| self.state.operand_to_bv(&arg.0)) // have to do this before changing state.cur_loc, so that the lookups happen in the caller function
                        .collect::<Result<Vec<B::BV>>>()?;
                    let saved_loc = self.state.cur_loc.clone();
                    self.state.push_invokesite(invoke);
                    self.state.cur_loc = Location {
                        module: callee_mod,
                        func: callee,
                        bb: callee
                            .basic_blocks
                            .get(0)
                            .expect("Failed to get entry basic block"),
                        instr: BBInstrIndex::Instr(0),
                        source_loc: None, // this will be updated once we get there and begin symex of the instruction
                    };
                    for (bvarg, param) in bvargs.into_iter().zip_eq(callee.parameters.iter()) {
                        self.state.assign_bv_to_name(param.name.clone(), bvarg)?;
                        // have to do the assign_bv_to_name calls after changing state.cur_loc, so that the variables are created in the callee function
                    }
                    info!(
                        "Entering function {:?} in module {:?}",
                        called_funcname, &callee_mod.name
                    );
                    let returned_bv = self
                        .symex_from_cur_loc_through_end_of_function()?
                        .ok_or(Error::Unsat)?; // if symex_from_cur_loc_through_end_of_function() returns `None`, this path is unsat
                    match self.state.pop_callsite() {
                        None => Ok(Some(returned_bv)), // if there was no callsite to pop, then we finished elsewhere. See notes on `symex_call()`
                        Some(ref callsite)
                            if callsite.loc == saved_loc && callsite.instr.is_right() =>
                        {
                            let old_bb_name = &self.state.cur_loc.bb.name;
                            self.state.cur_loc = saved_loc;
                            match returned_bv {
                                ReturnValue::Return(retval) => {
                                    self.state
                                        .assign_bv_to_name(invoke.result.clone(), retval)?;
                                },
                                ReturnValue::ReturnVoid => {},
                                ReturnValue::Throw(bvptr) => {
                                    info!("Caller {:?} catching an exception thrown by callee {:?}: execution continuing at bb {} in caller {:?}{}",
                                        self.state.cur_loc.func.name, called_funcname, self.state.cur_loc.bb.name, self.state.cur_loc.func.name,
                                        if self.state.config.print_module_name {
                                            format!(", module {:?}", self.state.cur_loc.module.name)
                                        } else {
                                            String::new()
                                        },
                                    );
                                    return self
                                        .catch_at_exception_label(&bvptr, &invoke.exception_label);
                                },
                                ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                            }
                            // Returned normally, so continue at the `return_label`
                            self.state
                                .cur_loc
                                .move_to_start_of_bb_by_name(&invoke.return_label);
                            debug!("Completed ordinary return from invoke");
                            info!("Leaving function {:?}, continuing in caller {:?}{} (finished the invoke in bb {}, now in bb {})",
                                called_funcname,
                                self.state.cur_loc.func.name,
                                if self.state.config.print_module_name {
                                    format!(" in module {:?}", self.state.cur_loc.module.name)
                                } else {
                                    String::new()
                                },
                                old_bb_name,
                                self.state.cur_loc.bb.name,
                            );
                            self.symex_from_cur_loc_through_end_of_function()
                        }
                        Some(callsite) => panic!("Received unexpected callsite {:?}", callsite),
                    }
                } else {
                    match self.state.config.function_hooks.get_default_hook() {
                        None => Err(Error::FunctionNotFound(
                            self.state.demangle(called_funcname),
                        )),
                        Some(hook) => {
                            let hook = hook.clone(); // end the implicit borrow of `self` that arose from `get_default_hook()`. The `clone` is just an `Rc` and a `usize`, as of this writing
                            let pretty_funcname = self.state.demangle(called_funcname);
                            info!(
                                "Using default hook for a function named {:?}",
                                pretty_funcname
                            );
                            match self.symex_hook(invoke, &hook.clone(), &pretty_funcname, true)? {
                                // Assume that `symex_hook()` has taken care of validating the hook return value as necessary
                                ReturnValue::Return(retval) => {
                                    self.state
                                        .assign_bv_to_name(invoke.result.clone(), retval)?;
                                },
                                ReturnValue::ReturnVoid => {},
                                ReturnValue::Throw(bvptr) => {
                                    info!("Hook for {} threw an exception, which we are catching at bb {} in function {:?}{}",
                                        pretty_funcname, invoke.exception_label, self.state.cur_loc.func.name,
                                        if self.state.config.print_module_name {
                                            format!(", module {:?}", self.state.cur_loc.module.name)
                                        } else {
                                            String::new()
                                        }
                                    );
                                    return self
                                        .catch_at_exception_label(&bvptr, &invoke.exception_label);
                                },
                                ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                            }
                            Ok(None)
                        },
                    }
                }
            },
        }
    }

    fn symex_resume(
        &mut self,
        resume: &'p terminator::Resume,
    ) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing resume {:?}", resume);

        // (At least for C++ exceptions) the operand of the resume operand is the struct {exception_ptr, type_index}
        // (see notes on `catch_with_type_index()`). For now we don't handle the type_index, so we just strip out the
        // exception_ptr and throw that
        let operand = self.state.operand_to_bv(&resume.operand)?;
        let exception_ptr = operand.slice(self.project.pointer_size_bits() - 1, 0); // strip out the first element, assumed to be a pointer
        Ok(Some(ReturnValue::Throw(exception_ptr)))
    }

    /// Catches an exception, then continues execution in the function and
    /// eventually returns the `ReturnValue` representing the return value of the
    /// function (when it reaches the end of the function), or `Ok(None)` if no
    /// possible paths were found.
    ///
    /// `thrown_ptr`: pointer to the value or object that was thrown
    ///
    /// `bbname`: `Name` of the `landingpad` block which should catch the exception if appropriate
    fn catch_at_exception_label(
        &mut self,
        thrown_ptr: &B::BV,
        bbname: &Name,
    ) -> Result<Option<ReturnValue<B::BV>>> {
        // For now we just add an unconstrained type index
        let type_index = self
            .state
            .new_bv_with_name(Name::from("unconstrained_type_index_for_thrown_value"), 32)?;
        self.catch_with_type_index(thrown_ptr, &type_index, bbname)
    }

    /// Catches an exception, then continues execution in the function and
    /// eventually returns the `ReturnValue` representing the return value of the
    /// function (when it reaches the end of the function), or `Ok(None)` if no
    /// possible paths were found.
    ///
    /// `thrown_ptr`: pointer to the value or object that was thrown
    ///
    /// `type_index`: should be an `i32` indicating the type of value which was thrown.
    /// [LLVM's exception handling docs](https://releases.llvm.org/9.0.0/docs/ExceptionHandling.html#overview) call this a type info index.
    ///
    /// `bbname`: `Name` of the `landingpad` block which should catch the exception if appropriate
    fn catch_with_type_index(
        &mut self,
        thrown_ptr: &B::BV,
        type_index: &B::BV,
        bbname: &Name,
    ) -> Result<Option<ReturnValue<B::BV>>> {
        debug!(
            "Catching exception {{{:?}, {:?}}} at bb {}",
            thrown_ptr, type_index, bbname
        );
        self.state.cur_loc.move_to_start_of_bb_by_name(bbname);
        let mut found_landingpad = false;
        let mut first_iter = true; // is it the first iteration of the for loop
        for (instnum, inst) in self.state.cur_loc.bb.instrs.iter().enumerate() {
            self.state.cur_loc.instr = BBInstrIndex::Instr(instnum);
            self.state.cur_loc.source_loc = inst.get_debug_loc().as_ref();
            if first_iter {
                first_iter = false;
                self.state.record_path_entry(); // do this only on the first iteration
            }
            let result = match inst {
                Instruction::Phi(phi) => self.symex_phi(phi),  // phi instructions are allowed before the landingpad
                Instruction::LandingPad(lp) => { found_landingpad = true; self.symex_landing_pad(lp, thrown_ptr, type_index) },
                _ => Err(Error::MalformedInstruction(format!("Expected exception-catching block ({}) to have a `LandingPad` as its first non-phi instruction, but found {:?}", bbname, inst))),
            };
            match result {
                Ok(()) => {
                    if found_landingpad {
                        // continue executing the block normally
                        self.state.cur_loc.inc();
                        return self.symex_from_cur_loc_through_end_of_function();
                    } else {
                        // move on to the next instruction in our for loop
                        continue;
                    }
                },
                Err(Error::Unsat) | Err(Error::LoopBoundExceeded(_)) => {
                    // we can't continue down this path anymore
                    info!("Path is either unsat or exceeds the loop bound");
                    return self.backtrack_and_continue();
                },
                Err(e) => return Err(e), // propagate any other errors
            }
        }
        if found_landingpad {
            panic!("shouldn't reach this point if we found a landingpad")
        } else {
            Err(Error::MalformedInstruction(format!("Expected exception-catching block ({}) to have a `LandingPad`, but it seems not to", bbname)))
        }
    }

    /// `thrown_ptr` and `type_index` arguments: see descriptions on `self.throw()`
    fn symex_landing_pad(
        &mut self,
        lp: &'p instruction::LandingPad,
        thrown_ptr: &B::BV,
        type_index: &B::BV,
    ) -> Result<()> {
        debug!("Symexing landingpad {:?}", lp);
        let result_ty = self.state.type_of(lp);
        match result_ty.as_ref() {
            Type::StructType { element_types, .. } => {
                if element_types.len() != 2 {
                    return Err(Error::MalformedInstruction(format!("Expected landingpad result type to be a struct of 2 elements, got a struct of {} elements: {:?}", element_types.len(), element_types)));
                }
                match element_types[0].as_ref() {
                    ty@Type::PointerType { .. } => {
                        assert_eq!(
                            thrown_ptr.get_width(),
                            self.state.size_in_bits(ty).expect("ty is a pointer type, can't be a named struct type"),
                            "Expected thrown_ptr to be a pointer, got a value of width {:?}",
                            thrown_ptr.get_width()
                        );
                    },
                    ty => return Err(Error::MalformedInstruction(format!("Expected landingpad result type to be a struct with first element a pointer, got first element {:?}", ty))),
                }
                match element_types[1].as_ref() {
                    Type::IntegerType { bits: 32 } => {},
                    ty => return Err(Error::MalformedInstruction(format!("Expected landingpad result type to be a struct with second element an i32, got second element {:?}", ty))),
                }
            },
            _ => {
                return Err(Error::MalformedInstruction(format!(
                    "Expected landingpad result type to be a struct, got {:?}",
                    result_ty
                )))
            },
        }
        // Partly due to current restrictions in `llvm-ir` (not enough info
        // available on landingpad clauses - see `llvm-ir` docs), for now we
        // assume that the landingpad always catches
        self.state
            .record_bv_result(lp, type_index.concat(thrown_ptr))
    }

    fn symex_phi(&mut self, phi: &'p instruction::Phi) -> Result<()> {
        debug!("Symexing phi {:?}", phi);
        let path = self.state.get_path();
        let prev_bb = match path.len() {
            0|1 => panic!("not yet implemented: starting in a block with Phi instructions. or error: didn't expect a Phi in function entry block"),
            len => &path[len - 2].0.bb.name,  // the last entry is our current block, so we want the one before
        };
        let chosen_value = phi.incoming_values.iter()
            .find(|&(_, bbname)| bbname == prev_bb)
            .map(|(op, _)| op)
            .ok_or_else(|| Error::OtherError(format!("Failed to find a Phi member matching previous BasicBlock. Phi incoming_values are {:?} but we were looking for {:?}", phi.incoming_values, prev_bb)))?;
        self.state
            .record_bv_result(phi, self.state.operand_to_bv(&chosen_value)?)
    }

    fn symex_select(&mut self, select: &'p instruction::Select) -> Result<()> {
        debug!("Symexing select {:?}", select);
        let optype = {
            let truetype = self.state.type_of(&select.true_value);
            let falsetype = self.state.type_of(&select.false_value);
            if truetype != falsetype {
                return Err(Error::MalformedInstruction(format!("Expected Select operands to have identical type, but they have types {:?} and {:?}", truetype, falsetype)));
            }
            truetype
        };
        match self.state.type_of(&select.condition).as_ref() {
            Type::IntegerType { bits: 1 } => {
                let bvcond = self.state.operand_to_bv(&select.condition)?;
                let bvtrueval = self.state.operand_to_bv(&select.true_value)?;
                let bvfalseval = self.state.operand_to_bv(&select.false_value)?;
                let do_feasibility_checks = false;
                if do_feasibility_checks {
                    let true_feasible = self
                        .state
                        .sat_with_extra_constraints(std::iter::once(&bvcond))?;
                    let false_feasible = self
                        .state
                        .sat_with_extra_constraints(std::iter::once(&bvcond.not()))?;
                    if true_feasible && false_feasible {
                        self.state
                            .record_bv_result(select, bvcond.cond_bv(&bvtrueval, &bvfalseval))
                    } else if true_feasible {
                        bvcond.assert()?; // unnecessary, but may help Boolector more than it hurts?
                        self.state.record_bv_result(select, bvtrueval)
                    } else if false_feasible {
                        bvcond.not().assert()?; // unnecessary, but may help Boolector more than it hurts?
                        self.state.record_bv_result(select, bvfalseval)
                    } else {
                        // this path is unsat
                        Err(Error::Unsat)
                    }
                } else {
                    self.state
                        .record_bv_result(select, bvcond.cond_bv(&bvtrueval, &bvfalseval))
                }
            },
            #[cfg(feature = "llvm-11-or-greater")]
            Type::VectorType { scalable: true, .. } => {
                return Err(Error::UnsupportedInstruction("select on scalable vectors".into()));
            },
            Type::VectorType {
                element_type,
                num_elements,
                ..
            } => {
                match element_type.as_ref() {
                    Type::IntegerType { bits: 1 } => {},
                    ty => return Err(Error::MalformedInstruction(format!("Expected Select vector condition to be vector of i1, but got vector of {:?}", ty))),
                };
                let el_size = match optype.as_ref() {
                    #[cfg(feature = "llvm-11-or-greater")]
                    Type::VectorType { scalable: true, .. } => {
                        return Err(Error::MalformedInstruction("Select operands are scalable vectors but condition is not".into()));
                    },
                    Type::VectorType { element_type: op_el_type, num_elements: op_num_els, .. } => {
                        if num_elements != op_num_els {
                            return Err(Error::MalformedInstruction(format!("Select condition is a vector of {} elements but operands are vectors with {} elements", num_elements, op_num_els)));
                        }
                        self.state.size_in_bits(&op_el_type).ok_or_else(|| Error::MalformedInstruction("Select on a vector whose elements have opaque struct type".into()))?
                    },
                    _ => return Err(Error::MalformedInstruction(format!("Expected Select with vector condition to have vector operands, but operands are of type {:?}", optype))),
                };
                let condvec = self.state.operand_to_bv(&select.condition)?;
                let truevec = self.state.operand_to_bv(&select.true_value)?;
                let falsevec = self.state.operand_to_bv(&select.false_value)?;
                let final_bv = (0 .. *num_elements as u32)
                    .map(|idx| {
                        let bit = condvec.slice(idx, idx);
                        bit.cond_bv(
                            &truevec.slice((idx + 1) * el_size - 1, idx * el_size),
                            &falsevec.slice((idx + 1) * el_size - 1, idx * el_size),
                        )
                    })
                    .reduce(|a, b| b.concat(&a))
                    .ok_or_else(|| {
                        Error::MalformedInstruction("Select with vectors of 0 elements".to_owned())
                    })?;
                self.state.record_bv_result(select, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!(
                "Expected select condition to be i1 or vector of i1, but got {:?}",
                ty
            ))),
        }
    }

    fn symex_cmpxchg(&mut self, cmpxchg: &'p instruction::CmpXchg) -> Result<()> {
        debug!("Symexing cmpxchg {:?}", cmpxchg);
        let main_ty = {
            let expected_ty = self.state.type_of(&cmpxchg.expected);
            let replacement_ty = self.state.type_of(&cmpxchg.replacement);
            if expected_ty != replacement_ty {
                return Err(Error::MalformedInstruction(format!("Expected cmpxchg 'expected' and 'replacement' to be the same type, but their types are {:?} and {:?}", expected_ty, replacement_ty)));
            }
            expected_ty
        };
        let result_ty = self.state.type_of(cmpxchg);
        match result_ty.as_ref() {
            Type::StructType { element_types, .. } => {
                if element_types.len() != 2 {
                    return Err(Error::MalformedInstruction(format!("Expected cmpxchg result type to be a struct of 2 elements, got a struct of {} elements: {:?}", element_types.len(), element_types)));
                }
                if element_types[0] != main_ty {
                    return Err(Error::MalformedInstruction(format!("Expected cmpxchg result type to be a struct with first element equal to the expected/replacement type. Instead, first element of return type is {:?} while expected/replacement type is {:?}", element_types[0], main_ty)));
                }
                if element_types[1].as_ref() != &(Type::IntegerType { bits: 1 }) {
                    return Err(Error::MalformedInstruction(format!("Expected cmpxchg result type to be a struct with second element an i1; got second element {:?}", element_types[1])));
                }
            },
            _ => {
                return Err(Error::MalformedInstruction(format!(
                    "Expected cmpxchg result type to be a struct, got {:?}",
                    result_ty
                )))
            },
        }

        let addr = self.state.operand_to_bv(&cmpxchg.address)?;
        let expected = self.state.operand_to_bv(&cmpxchg.expected)?;
        let replacement = self.state.operand_to_bv(&cmpxchg.replacement)?;

        let read_value = self.state.read(&addr, expected.get_width())?;
        let match_flag = read_value._eq(&expected);
        self.state
            .write(&addr, match_flag.cond_bv(&replacement, &read_value))?;

        self.state
            .record_bv_result(cmpxchg, match_flag.concat(&read_value))
    }

    #[cfg(feature = "llvm-10-or-greater")]
    fn symex_atomicrmw(&mut self, armw: &'p instruction::AtomicRMW) -> Result<()> {
        debug!("Symexing atomicrmw {:?}", armw);
        use llvm_ir::instruction::RMWBinOp;
        let op_size = self
            .state
            .size_in_bits(&self.state.type_of(armw))
            .ok_or_else(|| {
                Error::MalformedInstruction("AtomicRMW result is an opaque struct type".into())
            })?;
        let addr = self.state.operand_to_bv(&armw.address)?;
        let val = self.state.operand_to_bv(&armw.value)?;
        let read_val = self.state.read(&addr, op_size)?;
        let modified_val = match armw.operation {
            RMWBinOp::Xchg => val,
            RMWBinOp::Add => read_val.add(&val),
            RMWBinOp::Sub => read_val.sub(&val),
            RMWBinOp::And => read_val.and(&val),
            RMWBinOp::Nand => read_val.and(&val).not(),
            RMWBinOp::Or => read_val.or(&val),
            RMWBinOp::Xor => read_val.xor(&val),
            RMWBinOp::Max => read_val.sgt(&val).cond_bv(&read_val, &val),
            RMWBinOp::Min => read_val.slt(&val).cond_bv(&read_val, &val),
            RMWBinOp::UMax => read_val.ugt(&val).cond_bv(&read_val, &val),
            RMWBinOp::UMin => read_val.ult(&val).cond_bv(&read_val, &val),
            RMWBinOp::FAdd | RMWBinOp::FSub => {
                return Err(Error::UnsupportedInstruction(
                    "Floating-point operation in an AtomicRMW".into(),
                ))
            },
        };
        self.state.write(&addr, modified_val)?;
        self.state.record_bv_result(armw, read_val)
    }
}

// Is the given `Constant` a `GlobalReference`
fn is_global_reference(c: &Constant) -> bool {
    match c {
        Constant::GlobalReference { .. } => true,
        _ => false,
    }
}

// Apply the given unary scalar operation to a vector
pub(crate) fn unary_on_vector<F: FnMut(&V) -> Result<V>, V: BV>(
    in_vector: &V,
    num_elements: u32,
    mut op: F,
) -> Result<V> {
    let in_vector_size = in_vector.get_width();
    assert_eq!(in_vector_size % num_elements, 0);
    let in_el_size = in_vector_size / num_elements;
    let in_scalars =
        (0 .. num_elements).map(|i| in_vector.slice((i + 1) * in_el_size - 1, i * in_el_size));
    let out_scalars = in_scalars.map(|s| op(&s)).collect::<Result<Vec<_>>>()?;
    out_scalars
        .into_iter()
        .reduce(|a, b| b.concat(&a))
        .ok_or_else(|| Error::MalformedInstruction("Vector operation with 0 elements".to_owned()))
    // LLVM disallows vectors of size 0: https://releases.llvm.org/9.0.0/docs/LangRef.html#vector-type
}

// Apply the given binary scalar operation to a vector
pub(crate) fn binary_on_vector<F, V: BV>(
    in_vector_0: &V,
    in_vector_1: &V,
    num_elements: u32,
    mut op: F,
) -> Result<V>
where
    F: for<'a> FnMut(&'a V, &'a V) -> V,
{
    let in_vector_0_size = in_vector_0.get_width();
    let in_vector_1_size = in_vector_1.get_width();
    if in_vector_0_size != in_vector_1_size {
        return Err(Error::MalformedInstruction(format!(
            "Binary operation's vector operands are different total sizes: {} vs. {}",
            in_vector_0_size, in_vector_1_size
        )));
    }
    let in_vector_size = in_vector_0_size;
    assert_eq!(in_vector_size % num_elements, 0);
    let in_el_size = in_vector_size / num_elements;
    let in_scalars_0 =
        (0 .. num_elements).map(|i| in_vector_0.slice((i + 1) * in_el_size - 1, i * in_el_size));
    let in_scalars_1 =
        (0 .. num_elements).map(|i| in_vector_1.slice((i + 1) * in_el_size - 1, i * in_el_size));
    let out_scalars = in_scalars_0
        .zip_eq(in_scalars_1)
        .map(|(s0, s1)| op(&s0, &s1));
    out_scalars.reduce(|a, b| b.concat(&a)).ok_or_else(|| {
        Error::MalformedInstruction("Binary operation on vectors with 0 elements".to_owned())
    }) // LLVM disallows vectors of size 0: https://releases.llvm.org/9.0.0/docs/LangRef.html#vector-type
}

#[derive(PartialEq, Eq, Clone)]
enum ResolvedFunction<'p, B: Backend> {
    HookActive {
        hook: FunctionHook<'p, B>,
        hooked_thing: HookedThing<'p>,
    },
    NoHookActive {
        called_funcname: &'p str,
    },
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum HookedThing<'p> {
    /// We are hooking the call of a function with this name
    Function(&'p str),
    /// We are hooking the call of an LLVM intrinsic with this name.
    ///
    /// Note: for this purpose,
    ///     (1) `memcpy`, `memset`, and `memmove` are considered functions rather than intrinsics; and
    ///     (2) if the `Config` overrides the default intrinsic hook for any intrinsic, that will result
    ///         in a `HookedThing::Function` as well.  That is, `HookedThing::Intrinsic` specifically
    ///         implies we're using the built-in intrinsic hook as well.
    Intrinsic(&'p str),
    /// We are hooking the call of a function pointer
    FunctionPtr,
    /// We are hooking a call to inline assembly
    InlineAsm,
}

impl<'p> fmt::Display for HookedThing<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HookedThing::Function(funcname) => write!(f, "function {:?}", funcname),
            HookedThing::Intrinsic(funcname) => write!(f, "intrinsic {:?}", funcname),
            HookedThing::FunctionPtr => write!(f, "a function pointer"),
            HookedThing::InlineAsm => write!(f, "inline assembly"),
        }
    }
}

#[cfg(test)]
mod tests {
    //! These tests check that the correct set of _paths_ are generated for various
    //! functions. In contrast, the integration tests in the tests/ folder test for
    //! specific solutions for function parameters and return values.

    use super::*;
    use std::fmt;

    type Result<T> = std::result::Result<T, String>;

    fn init_logging() {
        // capture log messages with test harness
        let _ = env_logger::builder().is_test(true).try_init();
    }

    /// a path consisting of `LocationDescription`s describing the start of each
    /// path entry, rather than rich `Location`s
    #[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
    struct Path<'p>(Vec<LocationDescription<'p>>);

    impl<'p> Path<'p> {
        /// shouldn't be necessary, but to satisfy the borrow checker, this
        /// function converts a `Path` over a particular lifetime to a `Path`
        /// over any arbitrary desired lifetime by stripping out the
        /// `source_loc`s
        fn strip_source_locs<'a>(self) -> Path<'a> {
            Path(
                self.0
                    .into_iter()
                    .map(|locdescr| LocationDescription {
                        modname: locdescr.modname,
                        funcname: locdescr.funcname,
                        bbname: locdescr.bbname,
                        instr: locdescr.instr,
                        source_loc: None,
                    })
                    .collect(),
            )
        }
    }

    impl<'p> fmt::Debug for Path<'p> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.0.is_empty() {
                write!(f, "<empty path>")?;
            } else {
                writeln!(f, "[")?;
                for pathentry in &self.0 {
                    write!(f, "  ")?;
                    pathentry.fmt(f)?;
                    writeln!(f)?;
                }
                write!(f, "]")?;
            }
            Ok(())
        }
    }

    /// Build a path from bbnames, that stays in a single function in the given module
    fn path_from_bbnames<'p>(
        modname: &str,
        funcname: &str,
        bbnames: impl IntoIterator<Item = Name>,
    ) -> Path<'p> {
        let mut vec = vec![];
        for bbname in bbnames {
            vec.push(LocationDescription {
                modname: modname.to_owned(),
                funcname: funcname.to_owned(),
                bbname,
                instr: BBInstrIndex::Instr(0),
                source_loc: None,
            });
        }
        Path(vec)
    }

    /// Like `path_from_bbnames`, but allows you to specify bbs by number rather than `Name`
    fn path_from_bbnums<'p>(
        modname: &str,
        funcname: &str,
        bbnums: impl IntoIterator<Item = usize>,
    ) -> Path<'p> {
        path_from_bbnames(modname, funcname, bbnums.into_iter().map(Name::from))
    }

    /// Build a path from (bbnum, instr) pairs, that stays in a single function in the given module
    fn path_from_bbnum_instr_pairs<'p>(
        modname: &str,
        funcname: &str,
        pairs: impl IntoIterator<Item = (usize, BBInstrIndex)>,
    ) -> Path<'p> {
        let mut vec = vec![];
        for (bbnum, instr) in pairs {
            vec.push(LocationDescription {
                modname: modname.to_owned(),
                funcname: funcname.to_owned(),
                bbname: Name::from(bbnum),
                instr,
                source_loc: None,
            });
        }
        Path(vec)
    }

    /// Build a path from (funcname, bbname, instr) tuples, that stays in the module with the given modname
    fn path_from_tuples_with_bbnames<'a, 'p>(
        modname: &str,
        tuples: impl IntoIterator<Item = (&'a str, Name, BBInstrIndex)>,
    ) -> Path<'p> {
        let mut vec = vec![];
        for (funcname, bbname, instr) in tuples {
            vec.push(LocationDescription {
                modname: modname.to_owned(),
                funcname: funcname.to_owned(),
                bbname,
                instr,
                source_loc: None,
            });
        }
        Path(vec)
    }

    /// Build a path from (funcname, bbnum, instr) tuples, that stays in the module with the given modname
    fn path_from_tuples_with_bbnums<'a, 'p>(
        modname: &str,
        tuples: impl IntoIterator<Item = (&'a str, usize, BBInstrIndex)>,
    ) -> Path<'p> {
        path_from_tuples_with_bbnames(
            modname,
            tuples
                .into_iter()
                .map(|(f, bbnum, instr)| (f, Name::from(bbnum), instr)),
        )
    }

    /// Build a path from (modname, funcname, bbnum, instr) tuples
    fn path_from_tuples_varying_modules<'a, 'p>(
        tuples: impl IntoIterator<Item = (&'a str, &'a str, usize, BBInstrIndex)>,
    ) -> Path<'p> {
        let mut vec = vec![];
        for (modname, funcname, bbnum, instr) in tuples {
            vec.push(LocationDescription {
                modname: modname.to_owned(),
                funcname: funcname.to_owned(),
                bbname: Name::from(bbnum),
                instr,
                source_loc: None,
            });
        }
        Path(vec)
    }

    /// Iterator over the paths through a function
    struct PathIterator<'p, B: Backend> {
        em: ExecutionManager<'p, B>,
    }

    impl<'p, B: Backend> PathIterator<'p, B> {
        /// For argument descriptions, see notes on `symex_function`
        pub fn new(
            funcname: &str,
            project: &'p Project,
            config: Config<'p, B>,
            params: Option<Vec<ParameterVal>>,
        ) -> Self {
            Self {
                em: symex_function(funcname, project, config, params).unwrap(),
            }
        }
    }

    impl<'p, B: Backend> Iterator for PathIterator<'p, B>
    where
        B: 'p,
    {
        type Item = Result<Path<'p>>;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.em.next() {
                    Some(Err(Error::LoopBoundExceeded(_))) => {
                        // for the purposes of the PathIterator for these tests,
                        // we silently ignore paths which exceeded the loop bound
                        continue;
                    },
                    res => {
                        return res.map(|res| match res {
                            Err(e) => {
                                // format the error nicely and propagate it
                                Err(self.em.state().full_error_message_with_context(e))
                            },
                            Ok(_) => Ok(Path(
                                self.em
                                    .state()
                                    .get_path()
                                    .iter()
                                    .map(|pathentry| LocationDescription::from(pathentry.0.clone()))
                                    .collect(),
                            )
                            .strip_source_locs()),
                        });
                    },
                }
            }
        }
    }

    use BBInstrIndex::{Instr, Terminator};

    #[test]
    #[rustfmt::skip]
    fn one_block() -> Result<()> {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "one_arg";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1]));
        assert_eq!(paths.len(), 1); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn two_paths() -> Result<()> {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "conditional_true";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![2, 4, 12]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![2, 8, 12]));
        assert_eq!(paths.len(), 2); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn four_paths() -> Result<()> {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "conditional_nozero";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![2, 4, 6, 14]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![2, 4, 8, 10, 14]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![2, 4, 8, 12, 14]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![2, 14]));
        assert_eq!(paths.len(), 4); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn switch() -> Result<()> {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "has_switch";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (2, Instr(0)),
            (4, Terminator),
            (14, Instr(0)),
        ]));
        assert_eq!(paths[1], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (2, Instr(0)),
            (5, Instr(0)),
            (14, Instr(0)),
        ]));
        assert_eq!(paths[2], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (2, Instr(0)),
            (7, Instr(0)),
            (14, Instr(0)),
        ]));
        assert_eq!(paths[3], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (2, Instr(0)),
            (10, Terminator),
            (14, Instr(0)),
        ]));
        assert_eq!(paths[4], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (2, Instr(0)),
            (11, Terminator),
            (14, Instr(0)),
        ]));
        assert_eq!(paths[5], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (2, Instr(0)),
            (12, Instr(0)),
            (14, Instr(0)),
        ]));
        assert_eq!(paths[6], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (2, Instr(0)),
            (14, Instr(0)),
        ]));
        assert_eq!(paths.len(), 7); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn while_loop() -> Result<()> {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "while_loop";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 6, 6, 6, 6, 6, 12]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 6, 6, 6, 6, 12]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 6, 6, 6, 12]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 6, 6, 12]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 6, 12]));
        assert_eq!(paths.len(), 5); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn for_loop() -> Result<()> {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "for_loop";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 6]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 9, 6]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 9, 9, 6]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 9, 9, 9, 6]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 9, 9, 9, 9, 6]));
        assert_eq!(paths[5], path_from_bbnums(modname, funcname, vec![1, 9, 9, 9, 9, 9, 6]));
        assert_eq!(paths.len(), 6); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn loop_more_blocks() -> Result<()> {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "loop_zero_iterations";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 5, 8, 18]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![1, 5, 11, 8, 18]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![1, 5, 11, 11, 8, 18]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![1, 5, 11, 11, 11, 8, 18]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![1, 5, 11, 11, 11, 11, 8, 18]));
        assert_eq!(paths[5], path_from_bbnums(modname, funcname, vec![1, 5, 11, 11, 11, 11, 11, 8, 18]));
        assert_eq!(paths[6], path_from_bbnums(modname, funcname, vec![1, 18]));
        assert_eq!(paths.len(), 7); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn loop_more_blocks_in_body() -> Result<()> {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "loop_with_cond";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
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
        assert_eq!(paths.len(), 5); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn two_loops() -> Result<()> {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "sum_of_array";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 30,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![1, 4,  4,  4,  4,  4,  4,  4,  4,  4,  4,
                                                                         11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 9]));
        assert_eq!(paths.len(), 1); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn nested_loop() -> Result<()> {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "nested_loop";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 30,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
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
        assert_eq!(paths.len(), 4); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn simple_call() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "simple_caller";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(&modname, vec![
            ("simple_caller", 1, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("simple_caller", 1, Terminator),
        ]));
        assert_eq!(paths.len(), 1); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn simple_call_maxdepth0() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "simple_caller";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            max_callstack_depth: Some(0),
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(&modname, vec![
            ("simple_caller", 1, Instr(0)),
            // shouldn't enter the call, due to `max_callstack_depth` setting
        ]));
        assert_eq!(paths.len(), 1); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn cross_module_simple_call() -> Result<()> {
        let callee_modname = "tests/bcfiles/call.bc";
        let caller_modname = "tests/bcfiles/crossmod.bc";
        let funcname = "cross_module_simple_caller";
        init_logging();
        let proj = Project::from_bc_paths(&[callee_modname, caller_modname])
            .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_varying_modules(vec![
            (caller_modname, "cross_module_simple_caller", 1, Instr(0)),
            (callee_modname, "simple_callee", 2, Instr(0)),
            (caller_modname, "cross_module_simple_caller", 1, Terminator),
        ]));
        assert_eq!(paths.len(), 1); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn conditional_call() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "conditional_caller";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("conditional_caller", 2, Instr(0)),
            ("conditional_caller", 4, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("conditional_caller", 4, Terminator),
            ("conditional_caller", 8, Instr(0)),
        ]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![2, 6, 8]));
        assert_eq!(paths.len(), 2); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn call_twice() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "twice_caller";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("twice_caller", 1, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("twice_caller", 1, Instr(1)),
            ("simple_callee", 2, Instr(0)),
            ("twice_caller", 1, Instr(2)),
        ]));
        assert_eq!(paths.len(), 1); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn cross_module_call_twice() -> Result<()> {
        let callee_modname = "tests/bcfiles/call.bc";
        let caller_modname = "tests/bcfiles/crossmod.bc";
        let funcname = "cross_module_twice_caller";
        init_logging();
        let proj = Project::from_bc_paths(&[callee_modname, caller_modname])
            .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_varying_modules(vec![
            (caller_modname, "cross_module_twice_caller", 1, Instr(0)),
            (callee_modname, "simple_callee", 2, Instr(0)),
            (caller_modname, "cross_module_twice_caller", 1, Instr(1)),
            (callee_modname, "simple_callee", 2, Instr(0)),
            (caller_modname, "cross_module_twice_caller", 1, Instr(2)),
        ]));
        assert_eq!(paths.len(), 1); // enusre there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn nested_call() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "nested_caller";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("nested_caller", 2, Instr(0)),
            ("simple_caller", 1, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("simple_caller", 1, Terminator),
            ("nested_caller", 2, Terminator),
        ]));
        assert_eq!(paths.len(), 1); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn nested_call_maxdepth1() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "nested_caller";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            max_callstack_depth: Some(1),
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("nested_caller", 2, Instr(0)),
            ("simple_caller", 1, Instr(0)),
            ("nested_caller", 2, Terminator), // shouldn't enter `simple_callee` due to the `max_callstack_depth` setting
        ]));
        assert_eq!(paths.len(), 1); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn cross_module_nested_near_call() -> Result<()> {
        let callee_modname = "tests/bcfiles/call.bc";
        let caller_modname = "tests/bcfiles/crossmod.bc";
        let funcname = "cross_module_nested_near_caller";
        init_logging();
        let proj = Project::from_bc_paths(&[callee_modname, caller_modname])
            .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_varying_modules(vec![
            (caller_modname, "cross_module_nested_near_caller", 2, Instr(0)),
            (caller_modname, "cross_module_simple_caller", 1, Instr(0)),
            (callee_modname, "simple_callee", 2, Instr(0)),
            (caller_modname, "cross_module_simple_caller", 1, Terminator),
            (caller_modname, "cross_module_nested_near_caller", 2, Terminator),
        ]));
        assert_eq!(paths.len(), 1); // enusre there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn cross_module_nested_far_call() -> Result<()> {
        let callee_modname = "tests/bcfiles/call.bc";
        let caller_modname = "tests/bcfiles/crossmod.bc";
        let funcname = "cross_module_nested_far_caller";
        init_logging();
        let proj = Project::from_bc_paths(&[callee_modname, caller_modname])
            .unwrap_or_else(|e| panic!("Failed to parse modules: {}", e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_varying_modules(vec![
            (caller_modname, "cross_module_nested_far_caller", 2, Instr(0)),
            (callee_modname, "simple_caller", 1, Instr(0)),
            (callee_modname, "simple_callee", 2, Instr(0)),
            (callee_modname, "simple_caller", 1, Terminator),
            (caller_modname, "cross_module_nested_far_caller", 2, Terminator),
        ]));
        assert_eq!(paths.len(), 1); // enusre there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn call_of_loop() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "caller_of_loop";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, Instr(0)),
            ("callee_with_loop", 2, Instr(0)),
            ("callee_with_loop", 9, Instr(0)),
            ("caller_of_loop", 1, Terminator),
        ]));
        assert_eq!(paths[1], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, Instr(0)),
            ("callee_with_loop", 2, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 9, Instr(0)),
            ("caller_of_loop", 1, Terminator),
        ]));
        assert_eq!(paths[2], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, Instr(0)),
            ("callee_with_loop", 2, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 9, Instr(0)),
            ("caller_of_loop", 1, Terminator),
        ]));
        assert_eq!(paths[3], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, Instr(0)),
            ("callee_with_loop", 2, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 9, Instr(0)),
            ("caller_of_loop", 1, Terminator),
        ]));
        assert_eq!(paths[4], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, Instr(0)),
            ("callee_with_loop", 2, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 9, Instr(0)),
            ("caller_of_loop", 1, Terminator),
        ]));
        assert_eq!(paths[5], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, Instr(0)),
            ("callee_with_loop", 2, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 13, Instr(0)),
            ("callee_with_loop", 9, Instr(0)),
            ("caller_of_loop", 1, Terminator),
        ]));
        assert_eq!(paths.len(), 6); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn call_in_loop() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "caller_with_loop";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 3,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("caller_with_loop", 1, Instr(0)),
            ("caller_with_loop", 8, Instr(0)),
        ]));
        assert_eq!(paths[1], path_from_tuples_with_bbnums(modname, vec![
            ("caller_with_loop", 1, Instr(0)),
            ("caller_with_loop", 10, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("caller_with_loop", 10, Instr(3)),
            ("caller_with_loop", 6, Instr(0)),
            ("caller_with_loop", 8, Instr(0)),
        ]));
        assert_eq!(paths[2], path_from_tuples_with_bbnums(modname, vec![
            ("caller_with_loop", 1, Instr(0)),
            ("caller_with_loop", 10, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("caller_with_loop", 10, Instr(3)),
            ("caller_with_loop", 10, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("caller_with_loop", 10, Instr(3)),
            ("caller_with_loop", 6, Instr(0)),
            ("caller_with_loop", 8, Instr(0)),
        ]));
        assert_eq!(paths[3], path_from_tuples_with_bbnums(modname, vec![
            ("caller_with_loop", 1, Instr(0)),
            ("caller_with_loop", 10, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("caller_with_loop", 10, Instr(3)),
            ("caller_with_loop", 10, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("caller_with_loop", 10, Instr(3)),
            ("caller_with_loop", 10, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("caller_with_loop", 10, Instr(3)),
            ("caller_with_loop", 6, Instr(0)),
            ("caller_with_loop", 8, Instr(0)),
        ]));
        assert_eq!(paths.len(), 4); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn recursive_simple() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_simple";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 5,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (9, Instr(0)),
            (6, Instr(1)),
            (6, Instr(1)),
            (6, Instr(1)),
            (6, Instr(1)),
        ]));
        assert_eq!(paths[1], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (9, Instr(0)),
            (6, Instr(1)),
            (6, Instr(1)),
            (6, Instr(1)),
            (6, Instr(1)),
        ]));
        assert_eq!(paths[2], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (9, Instr(0)),
            (6, Instr(1)),
            (6, Instr(1)),
            (6, Instr(1)),
        ]));
        assert_eq!(paths[3], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (9, Instr(0)),
            (6, Instr(1)),
            (6, Instr(1)),
            (6, Instr(1)),
        ]));
        assert_eq!(paths[4], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (9, Instr(0)),
            (6, Instr(1)),
            (6, Instr(1)),
        ]));
        assert_eq!(paths[5], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (9, Instr(0)),
            (6, Instr(1)),
            (6, Instr(1)),
        ]));
        assert_eq!(paths[6], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (9, Instr(0)),
            (6, Instr(1)),
        ]));
        assert_eq!(paths[7], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (1, Instr(0)),
            (9, Instr(0)),
            (6, Instr(1)),
        ]));
        assert_eq!(paths[8], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (9, Instr(0)),
        ]));
        assert_eq!(paths[9], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (9, Instr(0)),
        ]));
        assert_eq!(paths.len(), 10); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn recursive_double() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_double";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 4,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (20, Instr(0)),
            (8, Instr(2)),
            (8, Instr(2)),
            (8, Instr(2)),
        ]));
        assert_eq!(paths[1], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (20, Instr(0)),
            (8, Instr(2)),
            (8, Instr(2)),
        ]));
        assert_eq!(paths[2], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (20, Instr(0)),
            (8, Instr(2)),
        ]));
        assert_eq!(paths[3], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (12, Instr(0)),
            (14, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (20, Instr(0)),
            (8, Instr(2)),
            (8, Instr(2)),
            (14, Instr(2)),
        ]));
        assert_eq!(paths[4], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (12, Instr(0)),
            (14, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (8, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (20, Instr(0)),
            (8, Instr(2)),
            (14, Instr(2)),
        ]));
        assert_eq!(paths[5], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (12, Instr(0)),
            (14, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (12, Instr(0)),
            (18, Instr(0)),
            (20, Instr(0)),
            (14, Instr(2)),
        ]));
        assert_eq!(paths[6], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (12, Instr(0)),
            (14, Instr(0)),
            (1, Instr(0)),
            (4, Instr(0)),
            (20, Instr(0)),
            (14, Instr(2)),
        ]));
        assert_eq!(paths[7], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (6, Instr(0)),
            (12, Instr(0)),
            (18, Instr(0)),
            (20, Instr(0)),
        ]));
        assert_eq!(paths[8], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (4, Instr(0)),
            (20, Instr(0)),
        ]));
        assert_eq!(paths[9], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (20, Instr(0)),
        ]));
        assert_eq!(paths.len(), 10); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn recursive_not_tail() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_not_tail";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 3,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (3, Instr(0)),
            (15, Instr(0)),
        ]));
        assert_eq!(paths[1], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (3, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (10, Instr(0)),
            (15, Instr(0)),
        ]));
        assert_eq!(paths[2], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (3, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (12, Instr(0)),
            (15, Instr(0)),
        ]));
        assert_eq!(paths[3], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (3, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (10, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (10, Instr(0)),
            (15, Instr(0)),
        ]));
        assert_eq!(paths[4], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (3, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (10, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (12, Instr(0)),
            (15, Instr(0)),
        ]));
        assert_eq!(paths[5], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (3, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (12, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (10, Instr(0)),
            (15, Instr(0)),
        ]));
        assert_eq!(paths[6], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (5, Instr(0)),
            (1, Instr(0)),
            (3, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (12, Instr(0)),
            (15, Instr(0)),
            (5, Instr(2)),
            (12, Instr(0)),
            (15, Instr(0)),
        ]));
        assert_eq!(paths.len(), 7); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn recursive_and_normal_call() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_and_normal_caller";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 3,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(2)),
            ("recursive_and_normal_caller", 7, Instr(0)),
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(2)),
            ("recursive_and_normal_caller", 7, Instr(0)),
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(2)),
            ("recursive_and_normal_caller", 10, Instr(0)),
            ("recursive_and_normal_caller", 7, Instr(1)),
            ("recursive_and_normal_caller", 7, Instr(1)),
        ]));
        assert_eq!(paths[1], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(2)),
            ("recursive_and_normal_caller", 7, Instr(0)),
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(2)),
            ("recursive_and_normal_caller", 10, Instr(0)),
            ("recursive_and_normal_caller", 7, Instr(1)),
        ]));
        assert_eq!(paths[2], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(2)),
            ("recursive_and_normal_caller", 7, Instr(0)),
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 10, Instr(0)),
            ("recursive_and_normal_caller", 7, Instr(1)),
        ]));
        assert_eq!(paths[3], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(0)),
            ("simple_callee", 2, Instr(0)),
            ("recursive_and_normal_caller", 3, Instr(2)),
            ("recursive_and_normal_caller", 10, Instr(0)),
        ]));
        assert_eq!(paths[4], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, Instr(0)),
            ("recursive_and_normal_caller", 10, Instr(0)),
        ]));
        assert_eq!(paths.len(), 5); // ensure there are no more paths

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn mutually_recursive_functions() -> Result<()> {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "mutually_recursive_a";
        init_logging();
        let proj = Project::from_bc_path(modname)
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config {
            loop_bound: 3,
            ..Config::default()
        };
        let mut paths: Vec<Path> = PathIterator::<DefaultBackend>::new(funcname, &proj, config, None)
            .collect::<Result<Vec<Path>>>()
            .unwrap_or_else(|r| panic!("{}", r));
        paths.sort();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 3, Instr(0)),
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 3, Instr(0)),
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
            ("mutually_recursive_b", 3, Instr(2)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
            ("mutually_recursive_b", 3, Instr(2)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
        ]));
        assert_eq!(paths[1], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 3, Instr(0)),
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 3, Instr(0)),
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 7, Instr(0)),
            ("mutually_recursive_b", 3, Instr(2)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
            ("mutually_recursive_b", 3, Instr(2)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
        ]));
        assert_eq!(paths[2], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 3, Instr(0)),
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
            ("mutually_recursive_b", 3, Instr(2)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
        ]));
        assert_eq!(paths[3], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 3, Instr(0)),
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 7, Instr(0)),
            ("mutually_recursive_b", 3, Instr(2)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
        ]));
        assert_eq!(paths[4], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 3, Instr(0)),
            ("mutually_recursive_b", 1, Instr(0)),
            ("mutually_recursive_b", 7, Instr(0)),
            ("mutually_recursive_a", 3, Instr(2)),
            ("mutually_recursive_a", 7, Instr(0)),
        ]));
        assert_eq!(paths[5], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, Instr(0)),
            ("mutually_recursive_a", 7, Instr(0)),
        ]));
        assert_eq!(paths.len(), 6); // ensure there are no more paths

        Ok(())
    }
}
