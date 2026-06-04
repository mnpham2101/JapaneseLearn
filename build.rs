use std::{collections::HashMap, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let library_paths = HashMap::from([(
        "flashcard".to_string(),
        manifest_dir.join("lib/flashcard"),
    )]);

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(library_paths);

    slint_build::compile_with_config("ui/main_window.slint", config).expect("Slint build failed");
}
