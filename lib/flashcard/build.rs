fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let styles_path = manifest_dir.join("../../lib/styles/styles.slint");
    let library_paths = std::collections::HashMap::from([("styles".to_string(), styles_path)]);
    let config = slint_build::CompilerConfiguration::new()
        .as_library("flashcard")
        .rust_module("flashcard")
        .with_library_paths(library_paths);
    slint_build::compile_with_config("ui/flashcard_lib.slint", config).unwrap();
}
