use std::fs;
use std::os;
use std::path;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR env var not set");
    let output_module_directory = path::Path::new(&out_dir).join("schema/generated_protos");
    fs::create_dir_all(&output_module_directory).expect("couldn't create output directory");

    let symlink_path = path::Path::new("src/generated");
    if symlink_path.exists() {
        fs::remove_file(&symlink_path).expect("");
    }
    os::unix::fs::symlink(&output_module_directory, symlink_path).expect(&format!(
        "failed to symlink output directory into '{}'",
        symlink_path.display()
    ));

    let schema_files = ["proto/example.proto"];
    let include_files: [&str; 0] = [];
    prost_build::Config::new()
        .include_file("mod.rs")
        .out_dir(output_module_directory)
        .type_attribute(".", "use serde::{Serialize, Deserialize};")
        .type_attribute(".", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .compile_protos(&schema_files, &include_files)
        .expect("protocol buffer compilation failed");

    for f in schema_files {
        println!("cargo::rerun-if-changed={}", f);
    }
    for f in include_files {
        println!("cargo::rerun-if-changed={}", f);
    }
}
