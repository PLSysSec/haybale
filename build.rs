fn main() {
    let mut versions = vec![];
    if cfg!(feature = "llvm-9") {
        versions.push(9);
    }
    if cfg!(feature = "llvm-10") {
        versions.push(10);
    }
    if cfg!(feature = "llvm-11") {
        versions.push(11);
    }
    let selected_version = match versions.len() {
        0 => panic!("llvm-ir: Please select an LLVM version using a Cargo feature."),
        1 => versions[0],
        _ => panic!("llvm-ir: Multiple LLVM versions selected. Please activate only one LLVM version feature. (Got {:?})", versions),
    };

    // For convenience we set a number of configuration options to avoid
    // checking complex combinations of features all the time.
    if selected_version >= 10 {
        println!("cargo:rustc-cfg=LLVM_VERSION_10_OR_GREATER");
    }
    if selected_version >= 11 {
        println!("cargo:rustc-cfg=LLVM_VERSION_11_OR_GREATER");
    }
    if selected_version <= 10 {
        println!("cargo:rustc-cfg=LLVM_VERSION_10_OR_LOWER");
    }
    if selected_version <= 9 {
        println!("cargo:rustc-cfg=LLVM_VERSION_9_OR_LOWER");
    }
}
