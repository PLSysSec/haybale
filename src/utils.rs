use inkwell::basic_block::BasicBlock;
use inkwell::types::*;
use inkwell::values::*;

// In most cases get_name() would seem appropriate instead of this function,
// but unfortunately, get_name() is empty for values like '%0'
pub fn get_value_name(v: impl AnyValue) -> String {
    v.print_to_string().to_string()
}

pub fn get_dest_name(inst: InstructionValue) -> String {
    // seems like there should be a more efficient way?
    let bve: BasicValueEnum = inst.get_first_use().unwrap().get_used_value().left().unwrap();
    get_value_name(bve)
}

pub fn get_dest_type(inst: InstructionValue) -> BasicTypeEnum {
    inst.get_first_use().unwrap().get_used_value().left().unwrap().get_type()
}

pub fn get_bb_name(bb: BasicBlock) -> String {
    let name = bb.get_name().to_str().expect("Failed to convert from CStr").to_owned();
    if name == "" {
        format!("<BB starting with {}>", get_value_name(bb.get_first_instruction().expect("Failed to get first instruction of BB")))
    } else {
        name
    }
}

pub fn get_func_name(func: FunctionValue) -> String {
    func.get_name().to_str().expect("Failed to convert from CStr").to_owned()
}
