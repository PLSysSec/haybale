use llvm_ir::*;
use llvm_ir::instruction::{BinaryOp, InlineAssembly};
use log::{debug, info};
use either::Either;
use reduce::Reduce;
use std::convert::TryInto;
use std::fmt;
use std::sync::{Arc, RwLock};

pub use crate::state::{State, Location, PathEntry, pretty_bb_name, pretty_var_name};
use crate::backend::*;
use crate::config::*;
use crate::error::*;
use crate::extend::*;
use crate::function_hooks::*;
use crate::layout::*;
use crate::solver_utils::PossibleSolutions;
use crate::project::Project;
use crate::return_value::*;

/// Begin symbolic execution of the function named `funcname`, obtaining an
/// `ExecutionManager`. The function's parameters will start completely
/// unconstrained.
///
/// `project`: The `Project` (set of LLVM modules) in which symbolic execution
/// should take place. In the absence of function hooks (see
/// [`Config`](struct.Config.html)), we will try to enter calls to any functions
/// defined in the `Project`.
pub fn symex_function<'p, B: Backend>(
    funcname: &str,
    project: &'p Project,
    config: Config<'p, B>,
) -> ExecutionManager<'p, B> {
    debug!("Symexing function {}", funcname);
    let (func, module) = project.get_func_by_name(funcname).unwrap_or_else(|| panic!("Failed to find function named {:?}", funcname));
    let bb = func.basic_blocks.get(0).expect("Failed to get entry basic block");
    let start_loc = Location {
        module,
        func,
        bbname: bb.name.clone(),
        instr: 0,
    };
    let mut state = State::new(project, start_loc, config);
    let bvparams: Vec<_> = func.parameters.iter().map(|param| {
        state.new_bv_with_name(param.name.clone(), size_opaque_aware(&param.ty, project) as u32).unwrap()
    }).collect();
    ExecutionManager::new(state, project, bvparams, &bb)
}

/// An `ExecutionManager` allows you to symbolically explore executions of a
/// function. Conceptually, it is an `Iterator` over possible paths through the
/// function. Calling `next()` on an `ExecutionManager` explores another possible
/// path, returning a [`ReturnValue`](enum.ReturnValue.html) representing the
/// function's symbolic return value at the end of that path.
///
/// Importantly, after any call to `next()`, you can access the `State` resulting
/// from the end of that path using the `state()` or `mut_state()` methods.
///
/// When `next()` returns `None`, there are no more possible paths through the
/// function.
pub struct ExecutionManager<'p, B: Backend> {
    state: State<'p, B>,
    project: &'p Project,
    bvparams: Vec<B::BV>,
    start_bb: &'p BasicBlock,
    /// Whether the `ExecutionManager` is "fresh". A "fresh" `ExecutionManager`
    /// has not yet produced its first path, i.e., `next()` has not been called
    /// on it yet.
    fresh: bool,
}

impl<'p, B: Backend> ExecutionManager<'p, B> {
    fn new(state: State<'p, B>, project: &'p Project, bvparams: Vec<B::BV>, start_bb: &'p BasicBlock) -> Self {
        Self {
            state,
            project,
            bvparams,
            start_bb,
            fresh: true,
        }
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

impl<'p, B: Backend> Iterator for ExecutionManager<'p, B> where B: 'p {
    type Item = std::result::Result<ReturnValue<B::BV>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let retval = if self.fresh {
            self.fresh = false;
            info!("Beginning symex in function {:?}", self.state.cur_loc.func.name);
            self.symex_from_bb_through_end_of_function(self.start_bb)
        } else {
            debug!("ExecutionManager: requesting next path");
            self.backtrack_and_continue()
        };
        retval.transpose().map(|r| r.map_err(|e| {
            let mut err_msg = format!("Received the following error:\n  {}\n", e);
            err_msg.push_str(&format!("LLVM backtrace:\n{}", self.state.pretty_llvm_backtrace()));
            if std::env::var("HAYBALE_DUMP_PATH") == Ok("1".to_owned()) {
                err_msg.push_str("Path to error:\n");
                for path_entry in self.state.get_path() {
                    err_msg.push_str(&format!("  {:?}\n", path_entry));
                }
            } else {
                err_msg.push_str("note: For a dump of the path that led to this error, rerun with `HAYBALE_DUMP_PATH=1` environment variable.\n");
            }
            if std::env::var("HAYBALE_DUMP_VARS") == Ok("1".to_owned()) {
                err_msg.push_str("\nLatest values of variables at time of error, in current function:\n");
                err_msg.push_str("(Ignore any values from past the point of error, they may be from other paths)\n\n");
                for (varname, value) in self.state.all_vars_in_cur_fn() {
                    err_msg.push_str(&format!("  {}: {:?}\n", pretty_var_name(varname), value));
                }
            } else {
                err_msg.push_str("note: For a dump of variable values at time of error, rerun with `HAYBALE_DUMP_VARS=1` environment variable.\n");
                err_msg.push_str("note: to enable (much) more detailed logs, rerun with `RUST_LOG=haybale`.\n");
                err_msg.push_str("  (For how to enable more granular logging options, see docs for the env_logger crate).\n");
            }
            err_msg.push_str("\n");
            err_msg
        }))
    }
}

impl<'p, B: Backend> ExecutionManager<'p, B> where B: 'p {
    /// Symex from the current `Location` through the rest of the function.
    /// Returns the `ReturnValue` representing the return value of the function,
    /// or `Ok(None)` if no possible paths were found.
    fn symex_from_cur_loc_through_end_of_function(&mut self) -> Result<Option<ReturnValue<B::BV>>> {
        let bb = self.state.cur_loc.func.get_bb_by_name(&self.state.cur_loc.bbname)
            .unwrap_or_else(|| panic!("Failed to find bb named {:?} in function {:?}", self.state.cur_loc.bbname, self.state.cur_loc.func.name));
        self.symex_from_inst_in_bb_through_end_of_function(bb, self.state.cur_loc.instr)
    }

    /// Symex the given bb, through the rest of the function.
    /// Returns the `ReturnValue` representing the return value of the function,
    /// or `Ok(None)` if no possible paths were found.
    fn symex_from_bb_through_end_of_function(&mut self, bb: &'p BasicBlock) -> Result<Option<ReturnValue<B::BV>>> {
        self.symex_from_inst_in_bb_through_end_of_function(bb, 0)
    }

    /// Symex starting from the given `inst` index in the given bb, through the rest of the function.
    /// Returns the `ReturnValue` representing the return value of the function,
    /// or `Ok(None)` if no possible paths were found.
    fn symex_from_inst_in_bb_through_end_of_function(&mut self, bb: &'p BasicBlock, inst: usize) -> Result<Option<ReturnValue<B::BV>>> {
        assert_eq!(bb.name, self.state.cur_loc.bbname);
        debug!("Symexing basic block {:?} in function {}", bb.name, self.state.cur_loc.func.name);
        self.state.cur_loc.instr = inst;
        self.state.record_path_entry();
        for (instnum, inst) in bb.instrs.iter().enumerate().skip(inst) {
            self.state.cur_loc.instr = instnum;
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
                    Instruction::Phi(phi) => self.symex_phi(phi),
                    Instruction::Select(select) => self.symex_select(select),
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
                Ok(_) => {},  // no error, we can continue
                Err(Error::Unsat) | Err(Error::LoopBoundExceeded) => {
                    // we can't continue down this path anymore
                    info!("Path is either unsat or exceeds the loop bound");
                    return self.backtrack_and_continue();
                }
                Err(e) => return Err(e),  // propagate any other errors
            };
        }
        match &bb.term {
            Terminator::Ret(ret) => self.symex_return(ret).map(Some),
            Terminator::Br(br) => self.symex_br(br),
            Terminator::CondBr(condbr) => self.symex_condbr(condbr),
            Terminator::Switch(switch) => self.symex_switch(switch),
            Terminator::Invoke(invoke) => self.symex_invoke(invoke),
            Terminator::Resume(resume) => self.symex_resume(resume),
            Terminator::Unreachable(_) => Err(Error::OtherError("Reached an LLVM 'Unreachable' instruction".to_owned())),
            term => Err(Error::UnsupportedInstruction(format!("terminator {:?}", term))),
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
            info!("Reverted to backtrack point; {} more backtrack points available", self.state.count_backtracking_points());
            info!("Continuing in bb {} in function {:?}, module {:?}", pretty_bb_name(&self.state.cur_loc.bbname), self.state.cur_loc.func.name, self.state.cur_loc.module.name);
            self.symex_from_cur_loc()
        } else {
            // No backtrack points (and therefore no paths) remain
            Ok(None)
        }
    }

    /// Symex starting from the current location, returning (using the saved
    /// callstack) all the way back to the end of the top-level function.
    ///
    /// Returns the `ReturnValue` representing the final return value, or
    /// `Ok(None)` if no possible paths were found.
    fn symex_from_cur_loc(&mut self) -> Result<Option<ReturnValue<B::BV>>> {
        let bb = self.state.cur_loc.func.get_bb_by_name(&self.state.cur_loc.bbname)
            .unwrap_or_else(|| panic!("Failed to find bb named {:?} in function {:?}", self.state.cur_loc.bbname, self.state.cur_loc.func.name));
        self.symex_from_inst_in_bb(&bb, self.state.cur_loc.instr)
    }

    /// Symex starting from the given `inst` index in the given bb, returning
    /// (using the saved callstack) all the way back to the end of the top-level
    /// function.
    ///
    /// Returns the `ReturnValue` representing the final return value, or
    /// `Ok(None)` if no possible paths were found.
    fn symex_from_inst_in_bb(&mut self, bb: &'p BasicBlock, inst: usize) -> Result<Option<ReturnValue<B::BV>>> {
        match self.symex_from_inst_in_bb_through_end_of_function(bb, inst)? {
            Some(ReturnValue::Throw(bvptr)) => {
                // pop callsites until we find an `invoke` instruction that can direct us to a catch block
                loop {
                    match self.state.pop_callsite() {
                        Some(callsite) => match callsite.instr {
                            Either::Left(_call) => {
                                // a normal callsite, not an `invoke` instruction
                                info!("Caller {:?} (bb {}) in module {:?} is not prepared to catch the exception, rethrowing",
                                    callsite.loc.func.name,
                                    pretty_bb_name(&callsite.loc.bbname),
                                    callsite.loc.module.name,
                                );
                                continue;
                            },
                            Either::Right(invoke) => {
                                // catch the thrown value
                                info!("Caller {:?} (bb {}) in module {:?} catching the thrown value at bb {}",
                                    callsite.loc.func.name,
                                    pretty_bb_name(&callsite.loc.bbname),
                                    callsite.loc.module.name,
                                    pretty_bb_name(&invoke.exception_label),
                                );
                                self.state.cur_loc = callsite.loc.clone();
                                return self.catch_at_exception_label(&bvptr, &invoke.exception_label);
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
                        info!("Leaving function {:?}, continuing in caller {:?} (bb {}) in module {:?}",
                            self.state.cur_loc.func.name,
                            callsite.loc.func.name,
                            pretty_bb_name(&callsite.loc.bbname),
                            callsite.loc.module.name,
                        );
                        self.state.cur_loc = callsite.loc.clone();
                        // Assign the returned value as the result of the caller's call instruction
                        match symexresult {
                            ReturnValue::Return(bv) => {
                                if self.state.assign_bv_to_name(call.dest.as_ref().unwrap().clone(), bv).is_err() {
                                    // This path is dead, try backtracking again
                                    return self.backtrack_and_continue();
                                };
                            },
                            ReturnValue::ReturnVoid => { },
                            ReturnValue::Throw(_) => panic!("This case should have been handled above"),
                            ReturnValue::Abort => panic!("This case should have been handled above"),
                        };
                        // Continue execution in caller, with the instruction after the call instruction
                        self.state.cur_loc.instr += 1;  // advance past the call instruction, to the next instruction
                        self.symex_from_cur_loc()
                    },
                    Either::Right(invoke) => {
                        // Normal return to an `Invoke` instruction
                        info!("Leaving function {:?}, continuing in caller {:?} (finished invoke in bb {}, now in bb {}) in module {:?}",
                            self.state.cur_loc.func.name,
                            callsite.loc.func.name,
                            pretty_bb_name(&callsite.loc.bbname),
                            pretty_bb_name(&invoke.return_label),
                            callsite.loc.module.name,
                        );
                        self.state.cur_loc = callsite.loc.clone();
                        // Assign the returned value as the result of the `Invoke` instruction
                        match symexresult {
                            ReturnValue::Return(bv) => {
                                if self.state.assign_bv_to_name(invoke.result.clone(), bv).is_err() {
                                    // This path is dead, try backtracking again
                                    return self.backtrack_and_continue();
                                };
                            },
                            ReturnValue::ReturnVoid => { },
                            ReturnValue::Throw(_) => panic!("This case should have been handled above"),
                            ReturnValue::Abort => panic!("This case should have been handled above"),
                        };
                        // Continue execution in caller, at the normal-return label of the `Invoke` instruction
                        self.state.cur_loc.bbname = invoke.return_label.clone();
                        self.state.cur_loc.instr = 0;
                        self.symex_from_cur_loc()
                    }
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

    fn binop_to_bvbinop<'a, V: BV + 'a>(bop: &instruction::groups::BinaryOp) -> Result<Box<dyn for<'b> Fn(&'b V, &'b V) -> V + 'a>> {
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

    // Apply the given unary scalar operation to a vector
    fn unary_on_vector<F>(in_vector: &B::BV, num_elements: u32, mut op: F) -> Result<B::BV>
        where F: FnMut(&B::BV) -> B::BV
    {
        let in_vector_size = in_vector.get_width();
        assert_eq!(in_vector_size % num_elements, 0);
        let in_el_size = in_vector_size / num_elements;
        let in_scalars = (0 .. num_elements).map(|i| in_vector.slice((i+1)*in_el_size - 1, i*in_el_size));
        let out_scalars = in_scalars.map(|s| op(&s));
        out_scalars.reduce(|a,b| b.concat(&a)).ok_or_else(|| Error::OtherError("Vector operation with 0 elements".to_owned()))
    }

    // Apply the given binary scalar operation to a vector
    fn binary_on_vector<F>(in_vector_0: &B::BV, in_vector_1: &B::BV, num_elements: u32, mut op: F) -> Result<B::BV>
        where F: for<'a> FnMut(&'a B::BV, &'a B::BV) -> B::BV
    {
        let in_vector_0_size = in_vector_0.get_width();
        let in_vector_1_size = in_vector_1.get_width();
        if in_vector_0_size != in_vector_1_size {
            return Err(Error::MalformedInstruction(format!("Binary operation's vector operands are different total sizes: {} vs. {}", in_vector_0_size, in_vector_1_size)));
        }
        let in_vector_size = in_vector_0_size;
        assert_eq!(in_vector_size % num_elements, 0);
        let in_el_size = in_vector_size / num_elements;
        let in_scalars_0 = (0 .. num_elements).map(|i| in_vector_0.slice((i+1)*in_el_size - 1, i*in_el_size));
        let in_scalars_1 = (0 .. num_elements).map(|i| in_vector_1.slice((i+1)*in_el_size - 1, i*in_el_size));
        let out_scalars = in_scalars_0.zip(in_scalars_1).map(|(s0,s1)| op(&s0, &s1));
        out_scalars.reduce(|a,b| b.concat(&a)).ok_or_else(|| Error::MalformedInstruction("Binary operation on vectors with 0 elements".to_owned()))
    }

    fn symex_binop(&mut self, bop: &instruction::groups::BinaryOp) -> Result<()> {
        debug!("Symexing binop {:?}", bop);
        // We expect these binops to only operate on integers or vectors of integers
        let op0 = &bop.get_operand0();
        let op1 = &bop.get_operand1();
        let op0_type = op0.get_type();
        let op1_type = op1.get_type();
        if op0_type != op1_type {
            return Err(Error::MalformedInstruction(format!("Expected binary op to have two operands of same type, but have types {:?} and {:?}", op0_type, op1_type)));
        }
        let op_type = op0_type;
        let bvop0 = self.state.operand_to_bv(op0)?;
        let bvop1 = self.state.operand_to_bv(op1)?;
        let bvoperation = Self::binop_to_bvbinop(bop)?;
        match op_type {
            Type::IntegerType { .. } => {
                self.state.record_bv_result(bop, bvoperation(&bvop0, &bvop1))
            },
            Type::VectorType { element_type, num_elements } => {
                match *element_type {
                    Type::IntegerType { .. } => {
                        self.state.record_bv_result(bop, Self::binary_on_vector(&bvop0, &bvop1, num_elements as u32, bvoperation)?)
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
        let op0_type = icmp.operand0.get_type();
        let op1_type = icmp.operand1.get_type();
        if op0_type != op1_type {
            return Err(Error::MalformedInstruction(format!("Expected icmp to compare two operands of same type, but have types {:?} and {:?}", op0_type, op1_type)));
        }
        match icmp.get_type() {
            Type::IntegerType { bits } if bits == 1 => match op0_type {
                Type::IntegerType { .. } | Type::VectorType { .. } | Type::PointerType { .. } => {
                    self.state.record_bv_result(icmp, bvpred(&bvfirstop, &bvsecondop))
                },
                ty => Err(Error::MalformedInstruction(format!("Expected ICmp to have operands of type integer, pointer, or vector of integers, but got type {:?}", ty))),
            },
            Type::VectorType { element_type, num_elements } => match *element_type {
                Type::IntegerType { bits } if bits == 1 => match op0_type {
                    Type::IntegerType { .. } | Type::VectorType { .. } | Type::PointerType { .. } => {
                        let zero = self.state.zero(1);
                        let one = self.state.one(1);
                        let final_bv = Self::binary_on_vector(&bvfirstop, &bvsecondop, num_elements as u32, |a,b| bvpred(a,b).cond_bv(&one, &zero))?;
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
        match zext.operand.get_type() {
            Type::IntegerType { bits } => {
                let bvop = self.state.operand_to_bv(&zext.operand)?;
                let source_size = bits;
                let dest_size = size(&zext.get_type()) as u32;
                self.state.record_bv_result(zext, bvop.zext(dest_size - source_size))
            },
            Type::VectorType { element_type, num_elements } => {
                let in_vector = self.state.operand_to_bv(&zext.operand)?;
                let in_el_size = size(&element_type) as u32;
                let out_el_size = match zext.get_type() {
                    Type::VectorType { element_type: out_el_type, num_elements: out_num_els } => {
                        if out_num_els != num_elements {
                            return Err(Error::MalformedInstruction(format!("ZExt operand is a vector of {} elements but output is a vector of {} elements", num_elements, out_num_els)));
                        }
                        size(&out_el_type) as u32
                    },
                    ty => return Err(Error::MalformedInstruction(format!("ZExt operand is a vector type, but output is not: it is {:?}", ty))),
                };
                let final_bv = Self::unary_on_vector(&in_vector, num_elements as u32, |el| {
                    el.zext(out_el_size - in_el_size)
                })?;
                self.state.record_bv_result(zext, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!("Expected ZExt operand type to be integer or vector of integers; got {:?}", ty))),
        }
    }

    fn symex_sext(&mut self, sext: &'p instruction::SExt) -> Result<()> {
        debug!("Symexing sext {:?}", sext);
        match sext.operand.get_type() {
            Type::IntegerType { bits } => {
                let bvop = self.state.operand_to_bv(&sext.operand)?;
                let source_size = bits;
                let dest_size = size(&sext.get_type()) as u32;
                self.state.record_bv_result(sext, bvop.sext(dest_size - source_size))
            },
            Type::VectorType { element_type, num_elements } => {
                let in_vector = self.state.operand_to_bv(&sext.operand)?;
                let in_el_size = size(&element_type) as u32;
                let out_el_size = match sext.get_type() {
                    Type::VectorType { element_type: out_el_type, num_elements: out_num_els } => {
                        if out_num_els != num_elements {
                            return Err(Error::MalformedInstruction(format!("SExt operand is a vector of {} elements but output is a vector of {} elements", num_elements, out_num_els)));
                        }
                        size(&out_el_type) as u32
                    },
                    ty => return Err(Error::MalformedInstruction(format!("SExt operand is a vector type, but output is not: it is {:?}", ty))),
                };
                let final_bv = Self::unary_on_vector(&in_vector, num_elements as u32, |el| {
                    el.sext(out_el_size - in_el_size)
                })?;
                self.state.record_bv_result(sext, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!("Expected SExt operand type to be integer or vector of integers; got {:?}", ty))),
        }
    }

    fn symex_trunc(&mut self, trunc: &'p instruction::Trunc) -> Result<()> {
        debug!("Symexing trunc {:?}", trunc);
        match trunc.operand.get_type() {
            Type::IntegerType { .. } => {
                let bvop = self.state.operand_to_bv(&trunc.operand)?;
                let dest_size = size(&trunc.get_type()) as u32;
                self.state.record_bv_result(trunc, bvop.slice(dest_size-1, 0))
            },
            Type::VectorType { num_elements, .. } => {
                let in_vector = self.state.operand_to_bv(&trunc.operand)?;
                let dest_el_size = match trunc.get_type() {
                    Type::VectorType { element_type: out_el_type, num_elements: out_num_els } => {
                        if out_num_els != num_elements {
                            return Err(Error::MalformedInstruction(format!("Trunc operand is a vector of {} elements but output is a vector of {} elements", num_elements, out_num_els)));
                        }
                        size(&out_el_type) as u32
                    },
                    ty => return Err(Error::MalformedInstruction(format!("Trunc operand is a vector type, but output is not: it is {:?}", ty))),
                };
                let final_bv = Self::unary_on_vector(&in_vector, num_elements as u32, |el| el.slice(dest_el_size-1, 0))?;
                self.state.record_bv_result(trunc, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!("Expected Trunc operand type to be integer or vector of integers; got {:?}", ty))),
        }
    }

    /// Use this for any unary operation that can be treated as a cast
    fn symex_cast_op(&mut self, cast: &'p impl instruction::UnaryOp) -> Result<()> {
        debug!("Symexing cast op {:?}", cast);
        let bvop = self.state.operand_to_bv(&cast.get_operand())?;
        self.state.record_bv_result(cast, bvop)  // from Boolector's perspective a cast is simply a no-op; the bit patterns are equal
    }

    fn symex_load(&mut self, load: &'p instruction::Load) -> Result<()> {
        debug!("Symexing load {:?}", load);
        let bvaddr = self.state.operand_to_bv(&load.address)?;
        let dest_size = size(&load.get_type());
        self.state.record_bv_result(load, self.state.read(&bvaddr, dest_size as u32)?)
    }

    fn symex_store(&mut self, store: &'p instruction::Store) -> Result<()> {
        debug!("Symexing store {:?}", store);
        let bvval = self.state.operand_to_bv(&store.value)?;
        let bvaddr = self.state.operand_to_bv(&store.address)?;
        self.state.write(&bvaddr, bvval)
    }

    fn symex_gep(&mut self, gep: &'p instruction::GetElementPtr) -> Result<()> {
        debug!("Symexing gep {:?}", gep);
        match gep.get_type() {
            Type::PointerType { .. } => {
                let bvbase = self.state.operand_to_bv(&gep.address)?;
                let offset = Self::get_offset_recursive(&self.state, gep.indices.iter(), &gep.address.get_type(), bvbase.get_width())?;
                self.state.record_bv_result(gep, bvbase.add(&offset))
            },
            Type::VectorType { .. } => Err(Error::UnsupportedInstruction("GEP calculating a vector of pointers".to_owned())),
            ty => Err(Error::MalformedInstruction(format!("Expected GEP result type to be pointer or vector of pointers; got {:?}", ty))),
        }
    }

    /// Get the offset of the element (in bytes, as a `BV` of `result_bits` bits)
    fn get_offset_recursive(state: &State<'p, B>, mut indices: impl Iterator<Item = &'p Operand>, base_type: &Type, result_bits: u32) -> Result<B::BV> {
        match indices.next() {
            None => Ok(state.zero(result_bits)),
            Some(index) => match base_type {
                Type::PointerType { .. } | Type::ArrayType { .. } | Type::VectorType { .. } => {
                    let index = zero_extend_to_bits(state.operand_to_bv(index)?, result_bits);
                    let (offset, nested_ty) = get_offset_bv_index(base_type, &index, state.solver.clone())?;
                    Self::get_offset_recursive(state, indices, nested_ty, result_bits)
                        .map(|bv| bv.add(&offset))
                },
                Type::StructType { .. } => match index {
                    Operand::ConstantOperand(Constant::Int { value: index, .. }) => {
                        let (offset, nested_ty) = get_offset_constant_index(base_type, *index as usize)?;
                        Self::get_offset_recursive(state, indices, &nested_ty, result_bits)
                            .map(|bv| bv.add(&state.bv_from_u32(offset as u32, result_bits)))
                    },
                    _ => Err(Error::MalformedInstruction(format!("Expected index into struct type to be constant, but got index {:?}", index))),
                },
                Type::NamedStructType { ty, .. } => {
                    let arc: Arc<RwLock<Type>> = ty.as_ref()
                        .ok_or_else(|| Error::MalformedInstruction("get_offset on an opaque struct type".to_owned()))?
                        .upgrade()
                        .expect("Failed to upgrade weak reference");
                    let actual_ty: &Type = &arc.read().unwrap();
                    if let Type::StructType { .. } = actual_ty {
                        // this code copied from the StructType case
                        match index {
                            Operand::ConstantOperand(Constant::Int { value: index, .. }) => {
                                let (offset, nested_ty) = get_offset_constant_index(actual_ty, *index as usize)?;
                                Self::get_offset_recursive(state, indices, &nested_ty, result_bits)
                                    .map(|bv| bv.add(&state.bv_from_u32(offset as u32, result_bits)))
                            },
                            _ => Err(Error::MalformedInstruction(format!("Expected index into struct type to be constant, but got index {:?}", index))),
                        }
                    } else {
                        Err(Error::MalformedInstruction(format!("Expected NamedStructType inner type to be a StructType, but got {:?}", actual_ty)))
                    }
                }
                _ => panic!("get_offset_recursive with base type {:?}", base_type),
            }
        }
    }

    fn symex_alloca(&mut self, alloca: &'p instruction::Alloca) -> Result<()> {
        debug!("Symexing alloca {:?}", alloca);
        match &alloca.num_elements {
            Operand::ConstantOperand(Constant::Int { value: num_elements, .. }) => {
                let allocation_size = size_opaque_aware(&alloca.allocated_type, self.project) as u64 * num_elements;
                let allocated = self.state.allocate(allocation_size);
                self.state.record_bv_result(alloca, allocated)
            },
            op => Err(Error::UnsupportedInstruction(format!("Alloca with num_elements not a constant int: {:?}", op))),
        }
    }

    fn symex_extractelement(&mut self, ee: &'p instruction::ExtractElement) -> Result<()> {
        debug!("Symexing extractelement {:?}", ee);
        let vector = self.state.operand_to_bv(&ee.vector)?;
        match &ee.index {
            Operand::ConstantOperand(Constant::Int { value: index, .. }) => {
                let index = *index as u32;
                match ee.vector.get_type() {
                    Type::VectorType { element_type, num_elements } => {
                        if index >= num_elements as u32 {
                            Err(Error::MalformedInstruction(format!("ExtractElement index out of range: index {} with {} elements", index, num_elements)))
                        } else {
                            let el_size = size(&element_type) as u32;
                            self.state.record_bv_result(ee, vector.slice((index+1)*el_size - 1, index*el_size))
                        }
                    },
                    ty => Err(Error::MalformedInstruction(format!("Expected ExtractElement vector to be a vector type, got {:?}", ty))),
                }
            },
            op => Err(Error::UnsupportedInstruction(format!("ExtractElement with index not a constant int: {:?}", op))),
        }
    }

    fn symex_insertelement(&mut self, ie: &'p instruction::InsertElement) -> Result<()> {
        debug!("Symexing insertelement {:?}", ie);
        let vector = self.state.operand_to_bv(&ie.vector)?;
        let element = self.state.operand_to_bv(&ie.element)?;
        match &ie.index {
            Operand::ConstantOperand(Constant::Int { value: index, .. }) => {
                let index = *index as u32;
                match ie.vector.get_type() {
                    Type::VectorType { element_type, num_elements } => {
                        if index >= num_elements as u32 {
                            Err(Error::MalformedInstruction(format!("InsertElement index out of range: index {} with {} elements", index, num_elements)))
                        } else {
                            let vec_size = vector.get_width();
                            let el_size = size(&element_type) as u32;
                            assert_eq!(vec_size, el_size * num_elements as u32);
                            let insertion_bitindex_low = index * el_size;  // lowest bit number in the vector which will be overwritten
                            let insertion_bitindex_high = (index+1) * el_size - 1;  // highest bit number in the vector which will be overwritten

                            let with_insertion = Self::overwrite_bv_segment(&mut self.state, &vector, element, insertion_bitindex_low, insertion_bitindex_high);

                            self.state.record_bv_result(ie, with_insertion)
                        }
                    },
                    ty => Err(Error::MalformedInstruction(format!("Expected InsertElement vector to be a vector type, got {:?}", ty))),
                }
            },
            op => Err(Error::UnsupportedInstruction(format!("InsertElement with index not a constant int: {:?}", op))),
        }
    }

    fn symex_shufflevector(&mut self, sv: &'p instruction::ShuffleVector) -> Result<()> {
        debug!("Symexing shufflevector {:?}", sv);
        let op0_type = sv.operand0.get_type();
        let op1_type = sv.operand1.get_type();
        if op0_type != op1_type {
            return Err(Error::MalformedInstruction(format!("Expected ShuffleVector operands to be exactly the same type, but they are {:?} and {:?}", op0_type, op1_type)));
        }
        let op_type = op0_type;
        match op_type {
            Type::VectorType { element_type, num_elements } => {
                let mask: Vec<u32> = match &sv.mask {
                    Constant::Vector(mask) => mask.iter()
                        .map(|c| match c {
                            Constant::Int { value: idx, .. } => Ok(*idx as u32),
                            Constant::Undef(_) => Ok(0),
                            _ => Err(Error::UnsupportedInstruction(format!("ShuffleVector with a mask entry which is not a Constant::Int or Constant::Undef but instead {:?}", c))),
                        })
                        .collect::<Result<Vec<u32>>>()?,
                    Constant::AggregateZero(ty) | Constant::Undef(ty) => match ty {
                        Type::VectorType { num_elements, .. } => itertools::repeat_n(0, *num_elements).collect(),
                        _ => return Err(Error::MalformedInstruction(format!("Expected ShuffleVector mask (which is an AggregateZero or Undef) to have vector type, but its type is {:?}", ty))),
                    },
                    c => return Err(Error::MalformedInstruction(format!("Expected ShuffleVector mask to be a Constant::Vector, Constant::AggregateZero, or Constant::Undef, but got {:?}", c))),
                };
                let op0 = self.state.operand_to_bv(&sv.operand0)?;
                let op1 = self.state.operand_to_bv(&sv.operand1)?;
                assert_eq!(op0.get_width(), op1.get_width());
                let el_size = size(&element_type) as u32;
                let num_elements = num_elements as u32;
                assert_eq!(op0.get_width(), el_size * num_elements);
                let final_bv = mask.into_iter()
                    .map(|idx| {
                        if idx < num_elements {
                            op0.slice((idx+1) * el_size - 1, idx * el_size)
                        } else {
                            let idx = idx - num_elements;
                            op1.slice((idx+1) * el_size - 1, idx * el_size)
                        }
                    })
                    .reduce(|a,b| b.concat(&a)).ok_or_else(|| Error::MalformedInstruction("ShuffleVector mask had 0 elements".to_owned()))?;
                self.state.record_bv_result(sv, final_bv)
            },
            ty => Err(Error::MalformedInstruction(format!("Expected ShuffleVector operands to be vectors, got {:?}", ty))),
        }
    }

    fn symex_extractvalue(&mut self, ev: &'p instruction::ExtractValue) -> Result<()> {
        debug!("Symexing extractvalue {:?}", ev);
        let aggregate = self.state.operand_to_bv(&ev.aggregate)?;
        let (offset_bytes, size_bits) = Self::get_offset_recursive_const_indices(ev.indices.iter().map(|i| *i as usize), &ev.aggregate.get_type())?;
        let low_offset_bits = offset_bytes * 8;  // inclusive
        let high_offset_bits = low_offset_bits + size_bits;  // exclusive
        assert!(aggregate.get_width() >= high_offset_bits as u32, "Trying to extractvalue from an aggregate with total size {} bits, extracting offset {} bits to {} bits (inclusive) is out of bounds", aggregate.get_width(), low_offset_bits, high_offset_bits - 1);
        self.state.record_bv_result(ev, aggregate.slice((high_offset_bits - 1) as u32, low_offset_bits as u32))
    }

    fn symex_insertvalue(&mut self, iv: &'p instruction::InsertValue) -> Result<()> {
        debug!("Symexing insertvalue {:?}", iv);
        let aggregate = self.state.operand_to_bv(&iv.aggregate)?;
        let element = self.state.operand_to_bv(&iv.element)?;
        let (offset_bytes, size_bits) = Self::get_offset_recursive_const_indices(iv.indices.iter().map(|i| *i as usize), &iv.aggregate.get_type())?;
        let low_offset_bits = offset_bytes * 8;  // inclusive
        let high_offset_bits = low_offset_bits + size_bits - 1;  // inclusive
        assert!(aggregate.get_width() >= high_offset_bits as u32, "Trying to insertvalue into an aggregate with total size {} bits, inserting offset {} bits to {} bits (inclusive) is out of bounds", aggregate.get_width(), low_offset_bits, high_offset_bits);

        let new_aggregate = Self::overwrite_bv_segment(&mut self.state, &aggregate, element, low_offset_bits as u32, high_offset_bits as u32);

        self.state.record_bv_result(iv, new_aggregate)
    }

    /// Like `get_offset_recursive()` above, but with constant indices rather than `Operand`s.
    ///
    /// Returns the start offset (in bytes) of the indicated element, and the size (in bits) of the indicated element.
    fn get_offset_recursive_const_indices(mut indices: impl Iterator<Item = usize>, base_type: &Type) -> Result<(usize, usize)> {
        match indices.next() {
            None => Ok((0, size(base_type))),
            Some(index) => match base_type {
                Type::PointerType { .. } | Type::ArrayType { .. } | Type::VectorType { .. } | Type::StructType { .. } => {
                    let (offset, nested_ty) = get_offset_constant_index(base_type, index)?;
                    Self::get_offset_recursive_const_indices(indices, &nested_ty).map(|(val, size)| (val + offset, size))
                },
                Type::NamedStructType { ty, .. } => {
                    let arc: Arc<RwLock<Type>> = ty.as_ref()
                        .ok_or_else(|| Error::MalformedInstruction("get_offset on an opaque struct type".to_owned()))?
                        .upgrade()
                        .expect("Failed to upgrade weak reference");
                    let actual_ty: &Type = &arc.read().unwrap();
                    if let Type::StructType { .. } = actual_ty {
                        let (offset, nested_ty) = get_offset_constant_index(actual_ty, index)?;
                        Self::get_offset_recursive_const_indices(indices, &nested_ty).map(|(val, size)| (val + offset, size))
                    } else {
                        Err(Error::MalformedInstruction(format!("Expected NamedStructType inner type to be a StructType, but got {:?}", actual_ty)))
                    }
                },
                _ => panic!("get_offset_recursive_const_indices with base type {:?}", base_type),
            }
        }
    }

    /// Helper function which overwrites a particular segment of a BV, returning the new BV
    ///
    /// Specifically, offsets `low_bitindex` to `high_bitindex` of
    /// `original_bv` (inclusive) will be overwritten with the data in
    /// `overwrite_data`, which must be exactly the correct length
    fn overwrite_bv_segment(state: &mut State<B>, original_bv: &B::BV, overwrite_data: B::BV, low_bitindex: u32, high_bitindex: u32) -> B::BV {
        let full_width = original_bv.get_width();
        let highest_bit_index = full_width - 1;
        assert!(high_bitindex <= highest_bit_index, "overwrite_bv_segment: high_bitindex {} is larger than highest valid bit index {} for an original_bv of width {}", high_bitindex, highest_bit_index, full_width);
        assert!(high_bitindex >= low_bitindex, "overwrite_bv_segment: high_bitindex {} is lower than low_bitindex {}", high_bitindex, low_bitindex);
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
            let top = state.ones(highest_bit_index - high_bitindex).concat(&zeroes);
            if low_bitindex == 0 {
                top
            } else {
                top.concat(&state.ones(low_bitindex))
            }
        };

        // mask_overwrite is the overwrite data in the appropriate bit positions, 0's elsewhere
        let top = zero_extend_to_bits(overwrite_data, full_width - low_bitindex);
        let mask_overwrite = if low_bitindex == 0 {
            top
        } else {
            top.concat(&state.zero(low_bitindex))
        };

        original_bv
            .and(&mask_clear)  // zero out the segment we'll be writing
            .or(&mask_overwrite)  // write the data into the appropriate position
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
                match self.symex_hook(call, &hook, &pretty_hookedthing)? {
                    // Assume that `symex_hook()` has taken care of validating the hook return value as necessary
                    ReturnValue::Return(retval) => {
                        // can't quite use `state.record_bv_result(call, retval)?` because Call is not HasResult
                        self.state.assign_bv_to_name(call.dest.as_ref().unwrap().clone(), retval)?;
                    },
                    ReturnValue::ReturnVoid => {},
                    ReturnValue::Throw(bvptr) => {
                        debug!("Hook threw an exception, but caller isn't inside a try block; rethrowing upwards");
                        return Ok(Some(ReturnValue::Throw(bvptr)));
                    },
                    ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                }
                info!("Done processing hook for {}; continuing in bb {} in function {:?}, module {:?}", pretty_hookedthing, pretty_bb_name(&self.state.cur_loc.bbname), self.state.cur_loc.func.name, self.state.cur_loc.module.name);
                Ok(None)
            },
            ResolvedFunction::NoHookActive { called_funcname } => {
                if let Some((callee, callee_mod)) = self.project.get_func_by_name(called_funcname) {
                    assert_eq!(call.arguments.len(), callee.parameters.len());
                    let bvargs: Vec<B::BV> = call.arguments.iter()
                        .map(|arg| self.state.operand_to_bv(&arg.0))  // have to do this before changing state.cur_loc, so that the lookups happen in the caller function
                        .collect::<Result<Vec<B::BV>>>()?;
                    let saved_loc = self.state.cur_loc.clone();
                    self.state.push_callsite(call);
                    let bb = callee.basic_blocks.get(0).expect("Failed to get entry basic block");
                    self.state.cur_loc = Location {
                        module: callee_mod,
                        func: callee,
                        bbname: bb.name.clone(),
                        instr: 0,
                    };
                    for (bvarg, param) in bvargs.into_iter().zip(callee.parameters.iter()) {
                        self.state.assign_bv_to_name(param.name.clone(), bvarg)?;  // have to do the assign_bv_to_name calls after changing state.cur_loc, so that the variables are created in the callee function
                    }
                    info!("Entering function {:?} in module {:?}", called_funcname, &callee_mod.name);
                    let returned_bv = self.symex_from_bb_through_end_of_function(&bb)?.ok_or(Error::Unsat)?;  // if symex_from_bb_through_end_of_function() returns `None`, this path is unsat
                    match self.state.pop_callsite() {
                        None => Ok(Some(returned_bv)),  // if there was no callsite to pop, then we finished elsewhere. See notes on `symex_call()`
                        Some(ref callsite) if callsite.loc == saved_loc && callsite.instr.is_left() => {
                            self.state.cur_loc = saved_loc;
                            self.state.cur_loc.instr += 1;  // advance past the call instruction itself before recording the path entry
                            self.state.record_path_entry();
                            match returned_bv {
                                ReturnValue::Return(bv) => {
                                    // can't quite use `state.record_bv_result(call, bv)?` because Call is not HasResult
                                    self.state.assign_bv_to_name(call.dest.as_ref().unwrap().clone(), bv)?;
                                },
                                ReturnValue::ReturnVoid => assert_eq!(call.dest, None),
                                ReturnValue::Throw(bvptr) => {
                                    debug!("Callee threw an exception, but caller isn't inside a try block; rethrowing upwards");
                                    return Ok(Some(ReturnValue::Throw(bvptr)));
                                },
                                ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                            };
                            debug!("Completed ordinary return to caller");
                            info!("Leaving function {:?}, continuing in caller {:?} (bb {}) in module {:?}", called_funcname, &self.state.cur_loc.func.name, pretty_bb_name(&self.state.cur_loc.bbname), &self.state.cur_loc.module.name);
                            Ok(None)
                        },
                        Some(callsite) => panic!("Received unexpected callsite {:?}", callsite),
                    }
                } else {
                    Err(Error::FunctionNotFound(called_funcname.to_owned()))
                }
            },
        }
    }

    #[allow(clippy::if_same_then_else)]  // in this case, having some identical `if` blocks actually improves readability, I think
    fn resolve_function(&mut self, function: &'p Either<InlineAssembly, Operand>) -> Result<ResolvedFunction<'p, B>> {
        use crate::global_allocations::Callable;
        let funcname_or_hook: Either<&str, FunctionHook<B>> = match function {
            // the first two cases are really just optimizations for the third case; things should still work without the first two lines
            Either::Right(Operand::ConstantOperand(Constant::GlobalReference { name: Name::Name(name), .. })) => Either::Left(name),
            Either::Right(Operand::ConstantOperand(Constant::GlobalReference { name, .. })) => panic!("Function with a numbered name: {:?}", name),
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
            Either::Left(funcname) => match self.state.config.function_hooks.get_hook_for(funcname) {
                Some(hook) => Ok(ResolvedFunction::HookActive { hook: hook.clone(), hooked_thing: HookedThing::Function(funcname) }),
                None => {
                    // No hook currently defined for this function, check if any intrinsic hooks apply
                    // (see notes on function resolution in function_hooks.rs)
                    if funcname.starts_with("llvm.memset")
                        || funcname.starts_with("__memset")
                    {
                        Ok(ResolvedFunction::HookActive {
                            hook: self.state.intrinsic_hooks.get_hook_for("intrinsic: llvm.memset").cloned().expect("Failed to find LLVM intrinsic memset hook"),
                            hooked_thing: HookedThing::Function(funcname),
                        })
                    } else if funcname.starts_with("llvm.memcpy")
                        || funcname.starts_with("llvm.memmove")
                        || funcname.starts_with("__memcpy")
                    {
                        // Our memcpy implementation also works for memmove
                        Ok(ResolvedFunction::HookActive {
                            hook: self.state.intrinsic_hooks.get_hook_for("intrinsic: llvm.memcpy/memmove").cloned().expect("Failed to find LLVM intrinsic memcpy/memmove hook"),
                            hooked_thing: HookedThing::Function(funcname),
                        })
                    } else if funcname.starts_with("llvm.objectsize") {
                        Ok(ResolvedFunction::HookActive {
                            hook: self.state.intrinsic_hooks.get_hook_for("intrinsic: llvm.objectsize").cloned().expect("Failed to find LLVM intrinsic objectsize hook"),
                            hooked_thing: HookedThing::Function(funcname),
                        })
                    } else if funcname.starts_with("llvm.read_register")
                        || funcname.starts_with("llvm.write_register")
                    {
                        // These can just ignore their arguments and return unconstrained data, as appropriate
                        Ok(ResolvedFunction::HookActive {
                            hook: self.state.intrinsic_hooks.get_hook_for("intrinsic: generic_stub_hook").cloned().expect("Failed to find intrinsic generic stub hook"),
                            hooked_thing: HookedThing::Function(funcname),
                        })
                    } else if funcname.starts_with("llvm.lifetime")
                        || funcname.starts_with("llvm.invariant")
                        || funcname.starts_with("llvm.launder.invariant")
                        || funcname.starts_with("llvm.strip.invariant")
                        || funcname.starts_with("llvm.dbg")
                    {
                        // these are all safe to ignore
                        Ok(ResolvedFunction::HookActive {
                            hook: self.state.intrinsic_hooks.get_hook_for("intrinsic: generic_stub_hook").cloned().expect("Failed to find intrinsic generic stub hook"),
                            hooked_thing: HookedThing::Function(funcname),
                        })
                    } else {
                        // No hook currently defined for this function, and none of our intrinsic hooks apply
                        Ok(ResolvedFunction::NoHookActive { called_funcname: funcname })
                    }
                },
            },
            Either::Right(hook) => Ok(ResolvedFunction::HookActive { hook, hooked_thing: HookedThing::FunctionPtr }),
        }
    }

    /// Execute the hook `hook` hooking the call `call`, returning the hook's `ReturnValue`.
    ///
    /// `hooked_funcname`: Name of the hooked function, used only for logging and error messages
    fn symex_hook(&mut self, call: &'p impl IsCall, hook: &FunctionHook<'p, B>, hooked_funcname: &str) -> Result<ReturnValue<B::BV>> {
        info!("Processing hook for {}", hooked_funcname);
        match hook.call_hook(&self.project, &mut self.state, call)? {
            ReturnValue::ReturnVoid => {
                if call.get_type() != Type::VoidType {
                    Err(Error::OtherError(format!("Hook for {:?} returned void but call needs a return value", hooked_funcname)))
                } else {
                    Ok(ReturnValue::ReturnVoid)
                }
            },
            ReturnValue::Return(retval) => {
                let ret_type = call.get_type();
                if ret_type == Type::VoidType {
                    Err(Error::OtherError(format!("Hook for {:?} returned a value but call is void-typed", hooked_funcname)))
                } else {
                    let retwidth = size(&ret_type);
                    if retval.get_width() != retwidth as u32 {
                        Err(Error::OtherError(format!("Hook for {:?} returned a {}-bit value but call's return type requires a {}-bit value", hooked_funcname, retval.get_width(), retwidth)))
                    } else {
                        Ok(ReturnValue::Return(retval))
                    }
                }
            },
            ReturnValue::Throw(bvptr) => Ok(ReturnValue::Throw(bvptr)),  // throwing is always OK and doesn't need to be checked against function type
            ReturnValue::Abort => Ok(ReturnValue::Abort),  // aborting is always OK and doesn't need to be checked against function type
        }
    }

    /// Returns the `ReturnValue` representing the return value
    fn symex_return(&self, ret: &'p terminator::Ret) -> Result<ReturnValue<B::BV>> {
        debug!("Symexing return {:?}", ret);
        Ok(ret.return_operand
            .as_ref()
            .map(|op| self.state.operand_to_bv(op))
            .transpose()?  // turns Option<Result<_>> into Result<Option<_>>, then ?'s away the Result
            .map(ReturnValue::Return)
            .unwrap_or(ReturnValue::ReturnVoid))
    }

    /// Continues to the target of the `Br` and eventually returns the new `ReturnValue`
    /// representing the return value of the function (when it reaches the end of the
    /// function), or `Ok(None)` if no possible paths were found.
    fn symex_br(&mut self, br: &'p terminator::Br) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing br {:?}", br);
        self.state.cur_loc.bbname = br.dest.clone();
        self.state.cur_loc.instr = 0;
        self.symex_from_cur_loc_through_end_of_function()
    }

    /// Continues to the target(s) of the `CondBr` (saving a backtracking point if
    /// necessary) and eventually returns the new `ReturnValue` representing the
    /// return value of the function (when it reaches the end of the function), or
    /// `Ok(None)` if no possible paths were found.
    fn symex_condbr(&mut self, condbr: &'p terminator::CondBr) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing condbr {:?}", condbr);
        let bvcond = self.state.operand_to_bv(&condbr.condition)?;
        let true_feasible = self.state.sat_with_extra_constraints(std::iter::once(&bvcond))?;
        let false_feasible = self.state.sat_with_extra_constraints(std::iter::once(&bvcond.not()))?;
        if true_feasible && false_feasible {
            debug!("both true and false branches are feasible");
            // for now we choose to explore true first, and backtrack to false if necessary
            self.state.save_backtracking_point(condbr.false_dest.clone(), bvcond.not());
            bvcond.assert()?;
            self.state.cur_loc.bbname = condbr.true_dest.clone();
            self.state.cur_loc.instr = 0;
            self.symex_from_cur_loc_through_end_of_function()
        } else if true_feasible {
            debug!("only the true branch is feasible");
            bvcond.assert()?;  // unnecessary, but may help Boolector more than it hurts?
            self.state.cur_loc.bbname = condbr.true_dest.clone();
            self.state.cur_loc.instr = 0;
            self.symex_from_cur_loc_through_end_of_function()
        } else if false_feasible {
            debug!("only the false branch is feasible");
            bvcond.not().assert()?;  // unnecessary, but may help Boolector more than it hurts?
            self.state.cur_loc.bbname = condbr.false_dest.clone();
            self.state.cur_loc.instr = 0;
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
    fn symex_switch(&mut self, switch: &'p terminator::Switch) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing switch {:?}", switch);
        let switchval = self.state.operand_to_bv(&switch.operand)?;
        let dests = switch.dests
            .iter()
            .map(|(c,n)| {
                self.state.const_to_bv(c)
                    .map(|c| (c,n))
            })
            .collect::<Result<Vec<(B::BV, &Name)>>>()?;
        let feasible_dests: Vec<_> = dests.iter()
            .map(|(c,n)| {
                self.state.bvs_can_be_equal(&c, &switchval).map(|b| (c,*n,b))
            })
            .collect::<Result<Vec<(&B::BV, &Name, bool)>>>()?
            .into_iter()
            .filter(|(_,_,b)| *b)
            .map(|(c,n,_)| (c,n))
            .collect::<Vec<(&B::BV, &Name)>>();
        if feasible_dests.is_empty() {
            // none of the dests are feasible, we will always end up in the default dest
            self.state.cur_loc.bbname = switch.default_dest.clone();
            self.state.cur_loc.instr = 0;
            self.symex_from_cur_loc_through_end_of_function()
        } else {
            // make backtracking points for all but the first destination
            for (val, name) in feasible_dests.iter().skip(1) {
                self.state.save_backtracking_point((*name).clone(), val._eq(&switchval));
            }
            // if the default dest is feasible, make a backtracking point for it
            let default_dest_constraint = dests.iter()
                .map(|(c,_)| c._eq(&switchval).not())
                .reduce(|a,b| a.and(&b))
                .unwrap_or_else(|| self.state.bv_from_bool(true));  // if `dests` was empty, that's weird, but the default dest is definitely feasible
            if self.state.sat_with_extra_constraints(std::iter::once(&default_dest_constraint))? {
                self.state.save_backtracking_point(switch.default_dest.clone(), default_dest_constraint);
            }
            // follow the first destination
            let (val, name) = &feasible_dests[0];
            val._eq(&switchval).assert()?;  // unnecessary, but may help Boolector more than it hurts?
            self.state.cur_loc.bbname = (*name).clone();
            self.state.cur_loc.instr = 0;
            self.symex_from_cur_loc_through_end_of_function()
        }
    }

    /// Continues to the target of the `Invoke` and eventually returns the new
    /// `ReturnValue` representing the return value of the function (when it
    /// reaches the end of the function), or `Ok(None)` if no possible paths were
    /// found.
    fn symex_invoke(&mut self, invoke: &'p terminator::Invoke) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing invoke {:?}", invoke);
        match self.resolve_function(&invoke.function)? {
            ResolvedFunction::HookActive { hook, hooked_thing } => {
                let pretty_hookedthing = hooked_thing.to_string();
                match self.symex_hook(invoke, &hook, &pretty_hookedthing)? {
                    // Assume that `symex_hook()` has taken care of validating the hook return value as necessary
                    ReturnValue::Return(retval) => {
                        self.state.assign_bv_to_name(invoke.result.clone(), retval)?;
                    },
                    ReturnValue::ReturnVoid => {},
                    ReturnValue::Throw(bvptr) => {
                        info!("Hook for {} threw an exception, which we are catching at bb {} in function {:?}, module {:?}", pretty_hookedthing, pretty_bb_name(&invoke.exception_label), self.state.cur_loc.func.name, self.state.cur_loc.module.name);
                        return self.catch_at_exception_label(&bvptr, &invoke.exception_label);
                    },
                    ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                };
                let old_bb_name = self.state.cur_loc.bbname.clone();
                // We had a normal return, so continue at the `return_label`
                self.state.cur_loc.bbname = invoke.return_label.clone();
                self.state.cur_loc.instr = 0;
                info!("Done processing hook for {}; continuing in function {:?} (hook was for the invoke in bb {}, now in bb {}) in module {:?}", pretty_hookedthing, self.state.cur_loc.func.name, pretty_bb_name(&old_bb_name), pretty_bb_name(&self.state.cur_loc.bbname), self.state.cur_loc.module.name);
                self.symex_from_cur_loc_through_end_of_function()
            },
            ResolvedFunction::NoHookActive { called_funcname } => {
                if let Some((callee, callee_mod)) = self.project.get_func_by_name(called_funcname) {
                    assert_eq!(invoke.arguments.len(), callee.parameters.len());
                    let bvargs: Vec<B::BV> = invoke.arguments.iter()
                        .map(|arg| self.state.operand_to_bv(&arg.0))  // have to do this before changing state.cur_loc, so that the lookups happen in the caller function
                        .collect::<Result<Vec<B::BV>>>()?;
                    let saved_loc = self.state.cur_loc.clone();
                    self.state.push_invokesite(invoke);
                    let bb = callee.basic_blocks.get(0).expect("Failed to get entry basic block");
                    self.state.cur_loc = Location {
                        module: callee_mod,
                        func: callee,
                        bbname: bb.name.clone(),
                        instr: 0,
                    };
                    for (bvarg, param) in bvargs.into_iter().zip(callee.parameters.iter()) {
                        self.state.assign_bv_to_name(param.name.clone(), bvarg)?;  // have to do the assign_bv_to_name calls after changing state.cur_loc, so that the variables are created in the callee function
                    }
                    info!("Entering function {:?} in module {:?}", called_funcname, &callee_mod.name);
                    let returned_bv = self.symex_from_bb_through_end_of_function(&bb)?.ok_or(Error::Unsat)?;  // if symex_from_bb_through_end_of_function() returns `None`, this path is unsat
                    match self.state.pop_callsite() {
                        None => Ok(Some(returned_bv)),  // if there was no callsite to pop, then we finished elsewhere. See notes on `symex_call()`
                        Some(ref callsite) if callsite.loc == saved_loc && callsite.instr.is_right() => {
                            let old_bb_name = self.state.cur_loc.bbname.clone();
                            self.state.cur_loc = saved_loc;
                            match returned_bv {
                                ReturnValue::Return(retval) => {
                                    self.state.assign_bv_to_name(invoke.result.clone(), retval)?;
                                },
                                ReturnValue::ReturnVoid => {},
                                ReturnValue::Throw(bvptr) => {
                                    info!("Caller {:?} catching an exception thrown by callee {:?}: execution continuing at bb {} in caller {:?}, module {:?}", self.state.cur_loc.func.name, called_funcname, pretty_bb_name(&self.state.cur_loc.bbname), self.state.cur_loc.func.name, self.state.cur_loc.module.name);
                                    return self.catch_at_exception_label(&bvptr, &invoke.exception_label);
                                },
                                ReturnValue::Abort => return Ok(Some(ReturnValue::Abort)),
                            }
                            // Returned normally, so continue at the `return_label`
                            self.state.cur_loc.bbname = invoke.return_label.clone();
                            self.state.cur_loc.instr = 0;
                            debug!("Completed ordinary return from invoke");
                            info!("Leaving function {:?}, continuing in caller {:?} (finished the invoke in bb {}, now in bb {}) in module {:?}", called_funcname, &self.state.cur_loc.func.name, pretty_bb_name(&old_bb_name), pretty_bb_name(&self.state.cur_loc.bbname), &self.state.cur_loc.module.name);
                            self.symex_from_cur_loc_through_end_of_function()
                        },
                        Some(callsite) => panic!("Received unexpected callsite {:?}", callsite),
                    }
                } else {
                    Err(Error::FunctionNotFound(called_funcname.to_owned()))
                }
            },
        }
    }

    fn symex_resume(&mut self, resume: &'p terminator::Resume) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Symexing resume {:?}", resume);

        // (At least for C++ exceptions) the operand of the resume operand is the struct {exception_ptr, type_index}
        // (see notes on `catch_with_type_index()`). For now we don't handle the type_index, so we just strip out the
        // exception_ptr and throw that
        let operand = self.state.operand_to_bv(&resume.operand)?;
        let exception_ptr = operand.slice(POINTER_SIZE_BITS as u32 - 1, 0);  // strip out the first element, assumed to be a pointer
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
    fn catch_at_exception_label(&mut self, thrown_ptr: &B::BV, bbname: &Name) -> Result<Option<ReturnValue<B::BV>>> {
        // For now we just add an unconstrained type index
        let type_index = self.state.new_bv_with_name(Name::from("unconstrained_type_index_for_thrown_value"), 32)?;
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
    fn catch_with_type_index(&mut self, thrown_ptr: &B::BV, type_index: &B::BV, bbname: &Name) -> Result<Option<ReturnValue<B::BV>>> {
        debug!("Catching exception {{{:?}, {:?}}} at bb {}", thrown_ptr, type_index, pretty_bb_name(bbname));
        self.state.cur_loc.bbname = bbname.clone();
        self.state.cur_loc.instr = 0;
        self.state.record_path_entry();
        let bb = self.state.cur_loc.func.get_bb_by_name(&bbname)
            .unwrap_or_else(|| panic!("Failed to find bb named {:?} in function {:?}", bbname, self.state.cur_loc.func.name));
        let mut found_landingpad = false;
        for (instnum, inst) in bb.instrs.iter().enumerate() {
            self.state.cur_loc.instr = instnum;
            let result = match inst {
                Instruction::Phi(phi) => self.symex_phi(phi),  // phi instructions are allowed before the landingpad
                Instruction::LandingPad(lp) => { found_landingpad = true; self.symex_landing_pad(lp, thrown_ptr, type_index) },
                _ => Err(Error::MalformedInstruction(format!("Expected exception-catching block ({}) to have a `LandingPad` as its first non-phi instruction, but found {:?}", pretty_bb_name(bbname), inst))),
            };
            match result {
                Ok(()) => {
                    if found_landingpad {
                        // continue executing the block normally
                        return self.symex_from_inst_in_bb_through_end_of_function(bb, instnum+1);
                    } else {
                        // move on to the next instruction in our for loop
                        continue;
                    }
                },
                Err(Error::Unsat) | Err(Error::LoopBoundExceeded) => {
                    // we can't continue down this path anymore
                    info!("Path is either unsat or exceeds the loop bound");
                    return self.backtrack_and_continue();
                },
                Err(e) => return Err(e),  // propagate any other errors
            }
        }
        if found_landingpad {
            panic!("shouldn't reach this point if we found a landingpad")
        } else {
            Err(Error::MalformedInstruction(format!("Expected exception-catching block ({}) to have a `LandingPad`, but it seems not to", pretty_bb_name(bbname))))
        }
    }

    /// `thrown_ptr` and `type_index` arguments: see descriptions on `self.throw()`
    fn symex_landing_pad(&mut self, lp: &'p instruction::LandingPad, thrown_ptr: &B::BV, type_index: &B::BV) -> Result<()> {
        debug!("Symexing landingpad {:?}", lp);
        let result_ty = lp.get_type();
        match result_ty {
            Type::StructType { element_types, .. } => {
                if element_types.len() != 2 {
                    return Err(Error::MalformedInstruction(format!("Expected landingpad result type to be a struct of 2 elements, got a struct of {} elements: {:?}", element_types.len(), element_types)));
                }
                match &element_types[0] {
                    ty@Type::PointerType { .. } => assert_eq!(thrown_ptr.get_width(), size(ty) as u32, "Expected thrown_ptr to be a pointer, got a value of width {:?}", thrown_ptr.get_width()),
                    ty => return Err(Error::MalformedInstruction(format!("Expected landingpad result type to be a struct with first element a pointer, got first element {:?}", ty))),
                }
                match &element_types[1] {
                    Type::IntegerType { bits: 32 } => {},
                    ty => return Err(Error::MalformedInstruction(format!("Expected landingpad result type to be a struct with second element an i32, got second element {:?}", ty))),
                }
            },
            _ => return Err(Error::MalformedInstruction(format!("Expected landingpad result type to be a struct, got {:?}", result_ty))),
        }
        // Partly due to current restrictions in `llvm-ir` (not enough info
        // available on landingpad clauses - see `llvm-ir` docs), for now we
        // assume that the landingpad always catches
        self.state.record_bv_result(lp, type_index.concat(thrown_ptr))
    }

    fn symex_phi(&mut self, phi: &'p instruction::Phi) -> Result<()> {
        debug!("Symexing phi {:?}", phi);
        let path = self.state.get_path();
        let prev_bb = match path.len() {
            0|1 => panic!("not yet implemented: starting in a block with Phi instructions. or error: didn't expect a Phi in function entry block"),
            len => &path[len - 2].bbname,  // the last entry is our current block, so we want the one before
        };
        let chosen_value = phi.incoming_values.iter()
            .find(|&(_, bbname)| bbname == prev_bb)
            .map(|(op, _)| op)
            .ok_or_else(|| Error::OtherError(format!("Failed to find a Phi member matching previous BasicBlock. Phi incoming_values are {:?} but we were looking for {:?}", phi.incoming_values, prev_bb)))?;
        self.state.record_bv_result(phi, self.state.operand_to_bv(&chosen_value)?)
    }

    fn symex_select(&mut self, select: &'p instruction::Select) -> Result<()> {
        debug!("Symexing select {:?}", select);
        let optype = {
            let truetype = select.true_value.get_type();
            let falsetype = select.false_value.get_type();
            if truetype != falsetype {
                return Err(Error::MalformedInstruction(format!("Expected Select operands to have identical type, but they have types {:?} and {:?}", truetype, falsetype)));
            }
            truetype
        };
        match select.condition.get_type() {
            Type::IntegerType { bits: 1 } => {
                let bvcond = self.state.operand_to_bv(&select.condition)?;
                let bvtrueval = self.state.operand_to_bv(&select.true_value)?;
                let bvfalseval = self.state.operand_to_bv(&select.false_value)?;
                let do_feasibility_checks = false;
                if do_feasibility_checks {
                    let true_feasible = self.state.sat_with_extra_constraints(std::iter::once(&bvcond))?;
                    let false_feasible = self.state.sat_with_extra_constraints(std::iter::once(&bvcond.not()))?;
                    if true_feasible && false_feasible {
                        self.state.record_bv_result(select, bvcond.cond_bv(&bvtrueval, &bvfalseval))
                    } else if true_feasible {
                        bvcond.assert()?;  // unnecessary, but may help Boolector more than it hurts?
                        self.state.record_bv_result(select, bvtrueval)
                    } else if false_feasible {
                        bvcond.not().assert()?;  // unnecessary, but may help Boolector more than it hurts?
                        self.state.record_bv_result(select, bvfalseval)
                    } else {
                        // this path is unsat
                        Err(Error::Unsat)
                    }
                } else {
                    self.state.record_bv_result(select, bvcond.cond_bv(&bvtrueval, &bvfalseval))
                }
            },
            Type::VectorType { element_type, num_elements } => {
                match *element_type {
                    Type::IntegerType { bits: 1 } => {},
                    ty => return Err(Error::MalformedInstruction(format!("Expected Select vector condition to be vector of i1, but got vector of {:?}", ty))),
                };
                let el_size = match optype {
                    Type::VectorType { element_type: op_el_type, num_elements: op_num_els } => {
                        if num_elements != op_num_els {
                            return Err(Error::MalformedInstruction(format!("Select condition is a vector of {} elements but operands are vectors with {} elements", num_elements, op_num_els)));
                        }
                        size(&op_el_type) as u32
                    },
                    _ => return Err(Error::MalformedInstruction(format!("Expected Select with vector condition to have vector operands, but operands are of type {:?}", optype))),
                };
                let condvec = self.state.operand_to_bv(&select.condition)?;
                let truevec = self.state.operand_to_bv(&select.true_value)?;
                let falsevec = self.state.operand_to_bv(&select.false_value)?;
                let final_bv = (0 .. num_elements as u32)
                    .map(|idx| {
                        let bit = condvec.slice(idx, idx);
                        bit.cond_bv(
                            &truevec.slice((idx+1) * el_size - 1, idx * el_size),
                            &falsevec.slice((idx+1) * el_size - 1, idx * el_size),
                        )
                    })
                    .reduce(|a,b| b.concat(&a)).ok_or_else(|| Error::MalformedInstruction("Select with vectors of 0 elements".to_owned()))?;
                self.state.record_bv_result(select, final_bv)
            }
            ty => Err(Error::MalformedInstruction(format!("Expected select condition to be i1 or vector of i1, but got {:?}", ty))),
        }
    }
}

enum ResolvedFunction<'p, B: Backend> {
    HookActive {
        hook: FunctionHook<'p, B>,
        hooked_thing: HookedThing<'p>,
    },
    NoHookActive {
        called_funcname: &'p str,
    },
}

enum HookedThing<'p> {
    /// We are hooking the call of a function with this name
    Function(&'p str),
    /// We are hooking the call of a function pointer
    FunctionPtr,
    /// We are hooking a call to inline assembly
    InlineAsm,
}

impl<'p> fmt::Display for HookedThing<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HookedThing::Function(funcname) => write!(f, "function {:?}", funcname),
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

    use llvm_ir::*;
    use super::*;
    use std::fmt;

    fn init_logging() {
        // capture log messages with test harness
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
    struct Path(Vec<PathEntry>);

    impl fmt::Debug for Path {
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
    fn path_from_bbnames(modname: &str, funcname: &str, bbnames: impl IntoIterator<Item = Name>) -> Path {
        let mut vec = vec![];
        for bbname in bbnames {
            vec.push(PathEntry { modname: modname.to_owned(), funcname: funcname.to_owned(), bbname, instr: 0 });
        }
        Path(vec)
    }

    /// Like `path_from_bbnames`, but allows you to specify bbs by number rather than `Name`
    fn path_from_bbnums(modname: &str, funcname: &str, bbnums: impl IntoIterator<Item = usize>) -> Path {
        path_from_bbnames(modname, funcname, bbnums.into_iter().map(Name::from))
    }

    /// Build a path from (bbnum, instr) pairs, that stays in a single function in the given module
    fn path_from_bbnum_instr_pairs(modname: &str, funcname: &str, pairs: impl IntoIterator<Item = (usize, usize)>) -> Path {
        let mut vec = vec![];
        for (bbnum, instr) in pairs {
            vec.push(PathEntry { modname: modname.to_owned(), funcname: funcname.to_owned(), bbname: Name::from(bbnum), instr });
        }
        Path(vec)
    }

    /// Build a path from (funcname, bbname, instr) tuples, that stays in the module with the given modname
    fn path_from_tuples_with_bbnames<'a>(modname: &str, tuples: impl IntoIterator<Item = (&'a str, Name, usize)>) -> Path {
        let mut vec = vec![];
        for (funcname, bbname, instr) in tuples {
            vec.push(PathEntry { modname: modname.to_owned(), funcname: funcname.to_owned(), bbname, instr });
        }
        Path(vec)
    }

    /// Build a path from (funcname, bbnum, instr) tuples, that stays in the module with the given modname
    fn path_from_tuples_with_bbnums<'a>(modname: &str, tuples: impl IntoIterator<Item = (&'a str, usize, usize)>) -> Path {
        path_from_tuples_with_bbnames(modname, tuples.into_iter().map(|(f, bbnum, instr)| (f, Name::from(bbnum), instr)))
    }

    /// Build a path from (modname, funcname, bbnum, instr) tuples
    fn path_from_tuples_varying_modules<'a>(tuples: impl IntoIterator<Item = (&'a str, &'a str, usize, usize)>) -> Path {
        let mut vec = vec![];
        for (modname, funcname, bbnum, instr) in tuples {
            vec.push(PathEntry { modname: modname.to_owned(), funcname: funcname.to_owned(), bbname: Name::from(bbnum), instr });
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
        ) -> Self {
            Self { em: symex_function(funcname, project, config) }
        }
    }

    impl<'p, B: Backend> Iterator for PathIterator<'p, B> where B: 'p {
        type Item = Path;

        fn next(&mut self) -> Option<Self::Item> {
            self.em.next().map(|_| Path(self.em.state().get_path().clone()))
        }
    }

    #[test]
    fn one_block() {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "one_arg";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = PathIterator::<BtorBackend>::new(funcname, &proj, config).collect();
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![2, 4, 6, 14]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![2, 4, 8, 10, 14]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![2, 4, 8, 12, 14]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![2, 14]));
        assert_eq!(paths.len(), 4);  // ensure there are no more paths
    }

    #[test]
    fn switch() {
        let modname = "tests/bcfiles/basic.bc";
        let funcname = "has_switch";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnums(modname, funcname, vec![2, 4, 14]));
        assert_eq!(paths[1], path_from_bbnums(modname, funcname, vec![2, 5, 14]));
        assert_eq!(paths[2], path_from_bbnums(modname, funcname, vec![2, 7, 14]));
        assert_eq!(paths[3], path_from_bbnums(modname, funcname, vec![2, 10, 14]));
        assert_eq!(paths[4], path_from_bbnums(modname, funcname, vec![2, 11, 14]));
        assert_eq!(paths[5], path_from_bbnums(modname, funcname, vec![2, 12, 14]));
        assert_eq!(paths[6], path_from_bbnums(modname, funcname, vec![2, 14]));
        assert_eq!(paths.len(), 7);  // ensure there are no more paths

    }

    #[test]
    fn while_loop() {
        let modname = "tests/bcfiles/loop.bc";
        let funcname = "while_loop";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
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
        let config = Config { loop_bound: 30, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
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
        let config = Config { loop_bound: 30, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(&modname, vec![
            ("simple_caller", 1, 0),
            ("simple_callee", 2, 0),
            ("simple_caller", 1, 1),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_varying_modules(vec![
            (caller_modname, "cross_module_simple_caller", 1, 0),
            (callee_modname, "simple_callee", 2, 0),
            (caller_modname, "cross_module_simple_caller", 1, 1),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("conditional_caller", 2, 0),
            ("conditional_caller", 4, 0),
            ("simple_callee", 2, 0),
            ("conditional_caller", 4, 1),
            ("conditional_caller", 8, 0),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("twice_caller", 1, 0),
            ("simple_callee", 2, 0),
            ("twice_caller", 1, 1),
            ("simple_callee", 2, 0),
            ("twice_caller", 1, 2),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_varying_modules(vec![
            (caller_modname, "cross_module_twice_caller", 1, 0),
            (callee_modname, "simple_callee", 2, 0),
            (caller_modname, "cross_module_twice_caller", 1, 1),
            (callee_modname, "simple_callee", 2, 0),
            (caller_modname, "cross_module_twice_caller", 1, 2),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("nested_caller", 2, 0),
            ("simple_caller", 1, 0),
            ("simple_callee", 2, 0),
            ("simple_caller", 1, 1),
            ("nested_caller", 2, 2),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_varying_modules(vec![
            (caller_modname, "cross_module_nested_near_caller", 2, 0),
            (caller_modname, "cross_module_simple_caller", 1, 0),
            (callee_modname, "simple_callee", 2, 0),
            (caller_modname, "cross_module_simple_caller", 1, 1),
            (caller_modname, "cross_module_nested_near_caller", 2, 2),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_varying_modules(vec![
            (caller_modname, "cross_module_nested_far_caller", 2, 0),
            (callee_modname, "simple_caller", 1, 0),
            (callee_modname, "simple_callee", 2, 0),
            (callee_modname, "simple_caller", 1, 1),
            (caller_modname, "cross_module_nested_far_caller", 2, 2),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, 0),
            ("callee_with_loop", 2, 0),
            ("callee_with_loop", 9, 0),
            ("caller_of_loop", 1, 1),
        ]));
        assert_eq!(paths[1], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, 0),
            ("callee_with_loop", 2, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 9, 0),
            ("caller_of_loop", 1, 1),
        ]));
        assert_eq!(paths[2], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, 0),
            ("callee_with_loop", 2, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 9, 0),
            ("caller_of_loop", 1, 1),
        ]));
        assert_eq!(paths[3], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, 0),
            ("callee_with_loop", 2, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 9, 0),
            ("caller_of_loop", 1, 1),
        ]));
        assert_eq!(paths[4], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, 0),
            ("callee_with_loop", 2, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 9, 0),
            ("caller_of_loop", 1, 1),
        ]));
        assert_eq!(paths[5], path_from_tuples_with_bbnums(modname, vec![
            ("caller_of_loop", 1, 0),
            ("callee_with_loop", 2, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 13, 0),
            ("callee_with_loop", 9, 0),
            ("caller_of_loop", 1, 1),
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
        let config = Config { loop_bound: 3, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("caller_with_loop", 1, 0),
            ("caller_with_loop", 8, 0),
        ]));
        assert_eq!(paths[1], path_from_tuples_with_bbnums(modname, vec![
            ("caller_with_loop", 1, 0),
            ("caller_with_loop", 10, 0),
            ("simple_callee", 2, 0),
            ("caller_with_loop", 10, 3),
            ("caller_with_loop", 6, 0),
            ("caller_with_loop", 8, 0),
        ]));
        assert_eq!(paths[2], path_from_tuples_with_bbnums(modname, vec![
            ("caller_with_loop", 1, 0),
            ("caller_with_loop", 10, 0),
            ("simple_callee", 2, 0),
            ("caller_with_loop", 10, 3),
            ("caller_with_loop", 10, 0),
            ("simple_callee", 2, 0),
            ("caller_with_loop", 10, 3),
            ("caller_with_loop", 6, 0),
            ("caller_with_loop", 8, 0),
        ]));
        assert_eq!(paths[3], path_from_tuples_with_bbnums(modname, vec![
            ("caller_with_loop", 1, 0),
            ("caller_with_loop", 10, 0),
            ("simple_callee", 2, 0),
            ("caller_with_loop", 10, 3),
            ("caller_with_loop", 10, 0),
            ("simple_callee", 2, 0),
            ("caller_with_loop", 10, 3),
            ("caller_with_loop", 10, 0),
            ("simple_callee", 2, 0),
            ("caller_with_loop", 10, 3),
            ("caller_with_loop", 6, 0),
            ("caller_with_loop", 8, 0),
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
        let config = Config { loop_bound: 5, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (9, 0),
            (6, 1),
            (6, 1),
            (6, 1),
            (6, 1),
        ]));
        assert_eq!(paths[1], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (9, 0),
            (6, 1),
            (6, 1),
            (6, 1),
            (6, 1),
        ]));
        assert_eq!(paths[2], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (9, 0),
            (6, 1),
            (6, 1),
            (6, 1),
        ]));
        assert_eq!(paths[3], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (9, 0),
            (6, 1),
            (6, 1),
            (6, 1),
        ]));
        assert_eq!(paths[4], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (9, 0),
            (6, 1),
            (6, 1),
        ]));
        assert_eq!(paths[5], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (9, 0),
            (6, 1),
            (6, 1),
        ]));
        assert_eq!(paths[6], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (4, 0),
            (9, 0),
            (6, 1),
        ]));
        assert_eq!(paths[7], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (1, 0),
            (9, 0),
            (6, 1),
        ]));
        assert_eq!(paths[8], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (9, 0),
        ]));
        assert_eq!(paths[9], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (9, 0),
        ]));
        assert_eq!(paths.len(), 10);  // ensure there are no more paths
    }

    #[test]
    fn recursive_double() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_double";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config { loop_bound: 4, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (20, 0),
            (8, 2),
            (8, 2),
            (8, 2),
        ]));
        assert_eq!(paths[1], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (20, 0),
            (8, 2),
            (8, 2),
        ]));
        assert_eq!(paths[2], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (20, 0),
            (8, 2),
        ]));
        assert_eq!(paths[3], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (12, 0),
            (14, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (20, 0),
            (8, 2),
            (8, 2),
            (14, 2),
        ]));
        assert_eq!(paths[4], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (12, 0),
            (14, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (8, 0),
            (1, 0),
            (4, 0),
            (20, 0),
            (8, 2),
            (14, 2),
        ]));
        assert_eq!(paths[5], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (12, 0),
            (14, 0),
            (1, 0),
            (4, 0),
            (6, 0),
            (12, 0),
            (18, 0),
            (20, 0),
            (14, 2),
        ]));
        assert_eq!(paths[6], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (12, 0),
            (14, 0),
            (1, 0),
            (4, 0),
            (20, 0),
            (14, 2),
        ]));
        assert_eq!(paths[7], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (6, 0),
            (12, 0),
            (18, 0),
            (20, 0),
        ]));
        assert_eq!(paths[8], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (4, 0),
            (20, 0),
        ]));
        assert_eq!(paths[9], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (20, 0),
        ]));
        assert_eq!(paths.len(), 10);  // ensure there are no more paths
    }

    #[test]
    fn recursive_not_tail() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_not_tail";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config { loop_bound: 3, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (3, 0),
            (15, 0),
        ]));
        assert_eq!(paths[1], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (5, 0),
            (1, 0),
            (3, 0),
            (15, 0),
            (5, 2),
            (10, 0),
            (15, 0),
        ]));
        assert_eq!(paths[2], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (5, 0),
            (1, 0),
            (3, 0),
            (15, 0),
            (5, 2),
            (12, 0),
            (15, 0),
        ]));
        assert_eq!(paths[3], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (5, 0),
            (1, 0),
            (5, 0),
            (1, 0),
            (3, 0),
            (15, 0),
            (5, 2),
            (10, 0),
            (15, 0),
            (5, 2),
            (10, 0),
            (15, 0),
        ]));
        assert_eq!(paths[4], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (5, 0),
            (1, 0),
            (5, 0),
            (1, 0),
            (3, 0),
            (15, 0),
            (5, 2),
            (10, 0),
            (15, 0),
            (5, 2),
            (12, 0),
            (15, 0),
        ]));
        assert_eq!(paths[5], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (5, 0),
            (1, 0),
            (5, 0),
            (1, 0),
            (3, 0),
            (15, 0),
            (5, 2),
            (12, 0),
            (15, 0),
            (5, 2),
            (10, 0),
            (15, 0),
        ]));
        assert_eq!(paths[6], path_from_bbnum_instr_pairs(modname, funcname, vec![
            (1, 0),
            (5, 0),
            (1, 0),
            (5, 0),
            (1, 0),
            (3, 0),
            (15, 0),
            (5, 2),
            (12, 0),
            (15, 0),
            (5, 2),
            (12, 0),
            (15, 0),
        ]));
        assert_eq!(paths.len(), 7);  // ensure there are no more paths
    }

    #[test]
    fn recursive_and_normal_call() {
        let modname = "tests/bcfiles/call.bc";
        let funcname = "recursive_and_normal_caller";
        init_logging();
        let proj = Project::from_bc_path(&std::path::Path::new(modname))
            .unwrap_or_else(|e| panic!("Failed to parse module {:?}: {}", modname, e));
        let config = Config { loop_bound: 3, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 3, 0),
            ("simple_callee", 2, 0),
            ("recursive_and_normal_caller", 3, 2),
            ("recursive_and_normal_caller", 7, 0),
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 3, 0),
            ("simple_callee", 2, 0),
            ("recursive_and_normal_caller", 3, 2),
            ("recursive_and_normal_caller", 7, 0),
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 3, 0),
            ("simple_callee", 2, 0),
            ("recursive_and_normal_caller", 3, 2),
            ("recursive_and_normal_caller", 10, 0),
            ("recursive_and_normal_caller", 7, 1),
            ("recursive_and_normal_caller", 7, 1),
        ]));
        assert_eq!(paths[1], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 3, 0),
            ("simple_callee", 2, 0),
            ("recursive_and_normal_caller", 3, 2),
            ("recursive_and_normal_caller", 7, 0),
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 3, 0),
            ("simple_callee", 2, 0),
            ("recursive_and_normal_caller", 3, 2),
            ("recursive_and_normal_caller", 10, 0),
            ("recursive_and_normal_caller", 7, 1),
        ]));
        assert_eq!(paths[2], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 3, 0),
            ("simple_callee", 2, 0),
            ("recursive_and_normal_caller", 3, 2),
            ("recursive_and_normal_caller", 7, 0),
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 10, 0),
            ("recursive_and_normal_caller", 7, 1),
        ]));
        assert_eq!(paths[3], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 3, 0),
            ("simple_callee", 2, 0),
            ("recursive_and_normal_caller", 3, 2),
            ("recursive_and_normal_caller", 10, 0),
        ]));
        assert_eq!(paths[4], path_from_tuples_with_bbnums(modname, vec![
            ("recursive_and_normal_caller", 1, 0),
            ("recursive_and_normal_caller", 10, 0),
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
        let config = Config { loop_bound: 3, ..Config::default() };
        let paths: Vec<Path> = itertools::sorted(PathIterator::<BtorBackend>::new(funcname, &proj, config)).collect();
        assert_eq!(paths[0], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 3, 0),
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 3, 0),
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
            ("mutually_recursive_b", 3, 2),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
            ("mutually_recursive_b", 3, 2),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
        ]));
        assert_eq!(paths[1], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 3, 0),
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 3, 0),
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 7, 0),
            ("mutually_recursive_b", 3, 2),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
            ("mutually_recursive_b", 3, 2),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
        ]));
        assert_eq!(paths[2], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 3, 0),
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
            ("mutually_recursive_b", 3, 2),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
        ]));
        assert_eq!(paths[3], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 3, 0),
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 7, 0),
            ("mutually_recursive_b", 3, 2),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
        ]));
        assert_eq!(paths[4], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 3, 0),
            ("mutually_recursive_b", 1, 0),
            ("mutually_recursive_b", 7, 0),
            ("mutually_recursive_a", 3, 2),
            ("mutually_recursive_a", 7, 0),
        ]));
        assert_eq!(paths[5], path_from_tuples_with_bbnums(modname, vec![
            ("mutually_recursive_a", 1, 0),
            ("mutually_recursive_a", 7, 0),
        ]));
        assert_eq!(paths.len(), 6);  // ensure there are no more paths
    }
}
