fn main() {
    cc::Build::new()
        .file("beam/native/prism.f90")
        .compiler("gfortran")
        .flag("-O2")
        .flag("-fPIC")
        .compile("prism_fortran");

    // Find gfortran runtime library path from the compiler
    let output = std::process::Command::new("gfortran")
        .args(["-print-file-name=libgfortran.dylib"])
        .output()
        .expect("gfortran must be in PATH");
    let lib_path = String::from_utf8(output.stdout).unwrap();
    let lib_path = lib_path.trim();
    if let Some(dir) = std::path::Path::new(lib_path).parent() {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }
    println!("cargo:rustc-link-lib=dylib=gfortran");
}
