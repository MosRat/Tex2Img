// extern crate cbindgen;

// use std::env;
// use std::path::PathBuf;

// fn main() {
//     let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
//     let output_file = PathBuf::from(&crate_dir)
//         .join("target")
//         .join("include")
//         .join("tex2img.h");

//     std::fs::create_dir_all(output_file.parent().unwrap()).unwrap();

//     cbindgen::Builder::new()
//         .with_crate(&crate_dir)
//         .with_language(cbindgen::Language::C)
//         .generate()
//         .unwrap()
//         .write_to_file(&output_file);
// }

fn main() {
    println!("cargo:rustc-link-arg=-fPIC");
    println!("cargo:rustc-link-arg=-static"); // Enforce static linking
}