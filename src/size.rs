use llvm_ir::types::{Type, FPType};

pub fn size(ty: &Type) -> usize {
    match ty {
        Type::IntegerType { bits } => *bits as usize,
        Type::PointerType { .. } => 64,  // our convention is that pointers are 64 bits
        Type::ArrayType { element_type, num_elements } => num_elements * size(element_type),
        Type::VectorType { element_type, num_elements } => num_elements * size(element_type),
        Type::StructType { element_types, .. } => element_types.iter().map(size).sum(),
        Type::FPType(fpt) => fp_size(fpt),
        _ => panic!("Not sure how to get the size of {:?}", ty),
    }
}

pub fn fp_size(fpt: &FPType) -> usize {
    match fpt {
        FPType::Half => 16,
        FPType::Single => 32,
        FPType::Double => 64,
        FPType::FP128 => 128,
        FPType::X86_FP80 => 80,
        FPType::PPC_FP128 => 128,
    }
}
