use std::{fs, path::Path};
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "wgsl"))
    {
        let path = entry.path();
        validate_shader(path);
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

fn validate_shader(path: &Path) {
    let source =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read shader at {path:?}"));

    if let Err(err) = naga::front::wgsl::parse_str(&source) {
        eprintln!(
            "WGSL shader compile error in {:?}:\n{}",
            path,
            err.emit_to_string(&source)
        );
        panic!("Shader compilation failed.");
    }
}
