fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let styles_path = manifest_dir.join("../../lib/styles/styles.slint");
    let flashcard_path = manifest_dir.join("../../lib/flashcard/ui/flashcard_lib.slint");
    let library_paths = std::collections::HashMap::from([
        ("styles".to_string(), styles_path),
        ("flashcard".to_string(), flashcard_path),
    ]);
    let config = slint_build::CompilerConfiguration::new()
        .as_library("vocabulary")
        .rust_module("vocabulary")
        .with_library_paths(library_paths);
    slint_build::compile_with_config("ui/vocabulary_lib.slint", config).unwrap();
}
