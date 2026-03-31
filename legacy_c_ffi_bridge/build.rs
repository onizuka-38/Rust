fn main() {
    println!("cargo:rerun-if-changed=c_legacy/legacy_math.h");
    println!("cargo:rerun-if-changed=c_legacy/legacy_math.c");

    cc::Build::new()
        .file("c_legacy/legacy_math.c")
        .include("c_legacy")
        .flag_if_supported("-O3")
        .compile("legacy_math");

    let bindings = bindgen::Builder::default()
        .header("c_legacy/legacy_math.h")
        .allowlist_function("lm_.*")
        .allowlist_type("lm_.*")
        .allowlist_var("LM_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("bindgen failed to generate bindings from legacy_math.h");

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("failed to write bindings.rs");
}
