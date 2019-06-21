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

pub struct ParamsIterator {
    func: FunctionValue,  // the function we're getting the params for
    cur_param_num: u32,  // the number of the parameter that will be returned by the next call to next()
    num_params: u32,  // cache this here to avoid calling into LLVM every time
}

impl ParamsIterator {
    pub fn new(func: FunctionValue) -> Self {
        ParamsIterator {
            func,
            cur_param_num: 0,
            num_params: func.count_params(),
        }
    }
}

impl Iterator for ParamsIterator {
    type Item = BasicValueEnum;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_param_num >= self.num_params {
            None
        } else {
            let rval = self.func.get_nth_param(self.cur_param_num);
            self.cur_param_num += 1;
            rval
        }
    }
}

pub struct PhiIterator {
    phi: PhiValue,  // the phi we're iterating over the members of
    cur_member_num: u32,  // the index of the member that will be returned by the next call to next()
    num_members: u32,  // cache this here to avoid calling into LLVM every time
}

impl PhiIterator {
    pub fn new(phi: PhiValue) -> Self {
        PhiIterator {
            phi,
            cur_member_num: 0,
            num_members: phi.count_incoming(),
        }
    }
}

impl Iterator for PhiIterator {
    type Item = (BasicValueEnum, BasicBlock);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_member_num >= self.num_members {
            None
        } else {
            let rval = self.phi.get_incoming(self.cur_member_num);
            self.cur_member_num += 1;
            rval
        }
    }
}
