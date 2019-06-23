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
