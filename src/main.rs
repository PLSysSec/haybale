use llvm_sys::bit_reader::LLVMGetBitcodeModule2;
use llvm_sys::core::LLVMCreateMemoryBufferWithContentsOfFile;
use llvm_sys::core::LLVMGetFirstFunction;
use llvm_sys::prelude::*;
use std::ptr;
use libc::c_char;
use std::ffi::{CString, CStr};

fn main() {
    let filepath_cstring = CString::new("c_examples/basic/basic.bc").unwrap();
    unsafe {
        let filepath: *const c_char = filepath_cstring.as_ptr();
        let mut out_message: *mut c_char = ptr::null_mut() as *mut c_char;
        let mut mem_buf: LLVMMemoryBufferRef = ptr::null_mut();
        let status = LLVMCreateMemoryBufferWithContentsOfFile(filepath, &mut mem_buf, &mut out_message);
        if status != 0 {
            if let Ok(message) = CStr::from_ptr(out_message).to_str() {
                panic!("LLVMCreateMemoryBufferWithContentsOfFile failed with code {}: {}", status, message);
            } else {
                panic!("LLVMCreateMemoryBufferWithContentsOfFile failed with code {}, failed to recover error message", status);
            }
        }
        let mut llvm_mod: LLVMModuleRef = ptr::null_mut();
        let status = LLVMGetBitcodeModule2(mem_buf, &mut llvm_mod);
        if status != 0 {
            panic!("LLVMGetBitcodeModule2 failed");
        }
        let func = LLVMGetFirstFunction(llvm_mod);
        println!("First function in the LLVM file is {:?}", func)
    }
}
