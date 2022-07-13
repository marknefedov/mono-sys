fn main() {
    println!("cargo:rustc-link-lib=mono-2.0");
    let mono_lib = pkg_config::Config::default().probe("mono-2").unwrap();
    println!("{:?}", mono_lib);
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(
            mono_lib
                .include_paths
                .iter()
                .map(|path| "-I".to_owned() + path.to_str().unwrap()),
        )
        .generate()
        .expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
