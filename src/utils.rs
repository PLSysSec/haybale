use inkwell::values::*;

// In most cases get_name() would seem appropriate instead of this function,
// but unfortunately, get_name() is empty for values like '%0'
pub fn get_value_name(v: impl AnyValue) -> String {
    match v.as_any_value_enum() {
        AnyValueEnum::ArrayValue(av) => {
            av.print_to_string().to_string()
        }
        AnyValueEnum::IntValue(iv) => {
            iv.print_to_string().to_string()
        },
        AnyValueEnum::FloatValue(fv) => {
            fv.print_to_string().to_string()
        },
        AnyValueEnum::PhiValue(pv) => {
            pv.print_to_string().to_string()
        },
        AnyValueEnum::FunctionValue(fv) => {
            let rval = fv.get_name().to_str().expect("Failed to convert from CStr").to_owned();
            assert_ne!(rval, "");
            rval
        },
        AnyValueEnum::PointerValue(pv) => {
            pv.print_to_string().to_string()
        },
        AnyValueEnum::StructValue(sv) => {
            sv.print_to_string().to_string()
        },
        AnyValueEnum::VectorValue(vv) => {
            vv.print_to_string().to_string()
        },
        AnyValueEnum::InstructionValue(_) => {
            unimplemented!("get_value_name() for InstructionValue");
        }
    }
}

pub fn get_dest_name(inst: InstructionValue) -> String {
    // seems like there should be a more efficient way?
    let bve: BasicValueEnum = inst.get_first_use().unwrap().get_used_value().left().unwrap();
    get_value_name(bve)
}
