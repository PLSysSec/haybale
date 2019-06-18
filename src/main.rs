use inkwell::module::Module;
use std::path::Path;

fn main() {
    let filepath = Path::new("/Users/craig/pitchfork-rs/c_examples/basic/basic.bc");
    if filepath.exists() {
        println!("Path definitely exists");
    } else {
        panic!("Specified path does not exist");
    }
    let llvm_mod = Module::parse_bitcode_from_path(&filepath).expect("Failed to parse module");
    if let Some(func) = llvm_mod.get_first_function() {
        println!("First function in the LLVM file is named {:?}", func.get_name());
    } else {
        println!("No functions found in the LLVM file");
    }
}
