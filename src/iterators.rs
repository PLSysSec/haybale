use inkwell::basic_block::BasicBlock;
use inkwell::module::Module;
use inkwell::values::*;

pub struct FunctionIterator {
    cur_func: Option<FunctionValue>,  // the value that will be returned by the next call to next()
}

impl FunctionIterator {
    pub fn new(module: &Module) -> Self {
        FunctionIterator {
            cur_func: module.get_first_function(),
        }
    }
}

impl Iterator for FunctionIterator {
    type Item = FunctionValue;

    fn next(&mut self) -> Option<Self::Item> {
        let rval = self.cur_func;
        self.cur_func = self.cur_func.and_then(|f| f.get_next_function());
        rval
    }
}

pub struct InstructionIterator {
    cur_inst: Option<InstructionValue>,  // the value that will be returned by the next call to next()
}

impl InstructionIterator {
    pub fn new(bb: &BasicBlock) -> Self {
        InstructionIterator {
            cur_inst: bb.get_first_instruction(),
        }
    }
}

impl Iterator for InstructionIterator {
    type Item = InstructionValue;

    fn next(&mut self) -> Option<Self::Item> {
        let rval = self.cur_inst;
        self.cur_inst = self.cur_inst.and_then(|bb| bb.get_next_instruction());
        rval
    }
}
