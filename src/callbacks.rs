//! Functions and structures for defining and activating instruction callbacks

use crate::backend::Backend;
use crate::error::Result;
use crate::state::State;
use std::rc::Rc;

#[derive(Clone)]
pub struct Callbacks<'p, B: Backend> {
    /// `haybale` will call each of these functions before processing each
    /// LLVM non-terminator instruction.
    ///
    /// If the callback returns an `Err`, `haybale` will propagate it accordingly.
    #[allow(clippy::type_complexity)]
    pub(crate) instruction_callbacks:
        Vec<Rc<dyn Fn(&'p llvm_ir::Instruction, &State<B>) -> Result<()> + 'p>>,

    /// `haybale` will call each of these functions before processing each
    /// LLVM terminator instruction.
    ///
    /// If the callback returns an `Err`, `haybale` will propagate it accordingly.
    #[allow(clippy::type_complexity)]
    pub(crate) terminator_callbacks:
        Vec<Rc<dyn Fn(&'p llvm_ir::Terminator, &State<B>) -> Result<()> + 'p>>,
}

impl<'p, B: Backend> Callbacks<'p, B> {
    /// Add an instruction callback. `haybale` will call the provided function
    /// before processing each LLVM non-terminator instruction.
    ///
    /// If multiple instruction callbacks are added (by calling this function
    /// multiple times), `haybale` will call each of them before processing each
    /// instruction.
    ///
    /// If any callback returns an `Err`, `haybale` will propagate it accordingly.
    pub fn add_instruction_callback(
        &mut self,
        cb: impl Fn(&'p llvm_ir::Instruction, &State<B>) -> Result<()> + 'p,
    ) {
        self.instruction_callbacks.push(Rc::new(cb))
    }

    /// Add a terminator callback. `haybale` will call the provided function
    /// before processing each LLVM terminator instruction.
    ///
    /// If multiple terminator callbacks are added (by calling this function
    /// multiple times), `haybale` will call each of them before processing each
    /// terminator.
    ///
    /// If any callback returns an `Err`, `haybale` will propagate it accordingly.
    pub fn add_terminator_callback(
        &mut self,
        cb: impl Fn(&'p llvm_ir::Terminator, &State<B>) -> Result<()> + 'p,
    ) {
        self.terminator_callbacks.push(Rc::new(cb))
    }
}

impl<'p, B: Backend> Default for Callbacks<'p, B> {
    fn default() -> Self {
        Self {
            instruction_callbacks: Vec::new(),
            terminator_callbacks: Vec::new(),
        }
    }
}
