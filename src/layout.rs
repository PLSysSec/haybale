//! Functions related to the in-memory layout of data types.

use crate::backend::*;
use crate::error::*;
use llvm_ir::types::{Type, FPType};
use std::sync::{Arc, RwLock};

/// Get the size of the `Type`, in bits
pub fn size(ty: &Type) -> usize {
    match ty {
        Type::IntegerType { bits } => *bits as usize,
        Type::PointerType { .. } => 64,  // our convention is that pointers are 64 bits
        Type::ArrayType { element_type, num_elements } => num_elements * size(element_type),
        Type::VectorType { element_type, num_elements } => num_elements * size(element_type),
        Type::StructType { element_types, .. } => element_types.iter().map(size).sum(),
        Type::NamedStructType { ty, .. } => size(&ty.as_ref()
            .expect("Can't get size of an opaque struct type")
            .upgrade()
            .expect("Failed to upgrade weak reference")
            .read()
            .unwrap()
        ),
        Type::FPType(fpt) => fp_size(*fpt),
        ty => panic!("Not sure how to get the size of {:?}", ty),
    }
}

/// Get the size of the `FPType`, in bits
pub fn fp_size(fpt: FPType) -> usize {
    match fpt {
        FPType::Half => 16,
        FPType::Single => 32,
        FPType::Double => 64,
        FPType::FP128 => 128,
        FPType::X86_FP80 => 80,
        FPType::PPC_FP128 => 128,
    }
}

/// Get the offset (in _bytes_) of the element at the given index, as well as the
/// `Type` of the element at that index.
//
// TODO: how to return `&Type` here (like get_offset_bv_index below) despite the
// weak reference in the `NamedStructType` case
pub fn get_offset_constant_index(base_type: &Type, index: usize) -> Result<(usize, Type)> {
    match base_type {
        Type::PointerType { pointee_type: element_type, .. }
        | Type::ArrayType { element_type, .. }
        | Type::VectorType { element_type, .. }
        => {
            let el_size_bits = size(element_type);
            if el_size_bits % 8 != 0 {
                Err(Error::UnsupportedInstruction(format!("Encountered a type with size {} bits", el_size_bits)))
            } else {
                let el_size_bytes = el_size_bits / 8;
                Ok((index * el_size_bytes, (**element_type).clone()))
            }
        },
        Type::StructType { element_types, .. } => {
            let mut offset_bits = 0;
            for ty in element_types.iter().take(index) {
                offset_bits += size(ty);
            }
            if offset_bits % 8 != 0 {
                Err(Error::UnsupportedInstruction(format!("Struct offset of {} bits", offset_bits)))
            } else {
                Ok((offset_bits / 8, element_types[index].clone()))
            }
        },
        Type::NamedStructType { ty, .. } => {
            let arc: Arc<RwLock<Type>> = ty.as_ref()
                .ok_or_else(|| Error::MalformedInstruction("get_offset on an opaque struct type".to_owned()))?
                .upgrade()
                .expect("Failed to upgrade weak reference");
            let actual_ty: &Type = &arc.read().unwrap();
            if let Type::StructType { ref element_types, .. } = actual_ty {
                // this code copied from the StructType case, unfortunately
                let mut offset_bits = 0;
                for ty in element_types.iter().take(index) {
                    offset_bits += size(ty);
                }
                if offset_bits % 8 != 0 {
                    Err(Error::UnsupportedInstruction(format!("Struct offset of {} bits", offset_bits)))
                } else {
                    Ok((offset_bits / 8, element_types[index].clone()))
                }
            } else {
                Err(Error::MalformedInstruction(format!("Expected NamedStructType inner type to be a StructType, but got {:?}", actual_ty)))
            }
        },
        _ => panic!("get_offset_constant_index with base type {:?}", base_type),
    }
}

/// Get the offset (in _bytes_) of the element at the given index, as well as a
/// reference to the `Type` of the element at that index.
///
/// This function differs from `get_offset_constant_index` in that it takes an
/// arbitrary `BV` as index instead of a `usize`, and likewise returns its offset
/// as a `BV`.
///
/// The result `BV` will have the same width as the input `index`.
pub fn get_offset_bv_index<'t, V: BV>(base_type: &'t Type, index: &V, solver: V::SolverRef) -> Result<(V, &'t Type)> {
    match base_type {
        Type::PointerType { pointee_type: element_type, .. }
        | Type::ArrayType { element_type, .. }
        | Type::VectorType { element_type, .. }
        => {
            let el_size_bits = size(element_type);
            if el_size_bits % 8 != 0 {
                Err(Error::UnsupportedInstruction(format!("Encountered a type with size {} bits", el_size_bits)))
            } else {
                let el_size_bytes = el_size_bits / 8;
                Ok((index.mul(&V::from_u64(solver, el_size_bytes as u64, index.get_width())), &element_type))
            }
        },
        Type::StructType { .. } | Type::NamedStructType { .. } => {
            Err(Error::MalformedInstruction("Index into struct type must be constant; consider using `get_offset_constant_index` instead of `get_offset_bv_index`".to_owned()))
        },
        _ => panic!("get_offset_bv_index with base type {:?}", base_type),
    }
}
