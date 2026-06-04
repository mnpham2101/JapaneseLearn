fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .as_library("flashcard")
        .rust_module("flashcard");
    slint_build::compile_with_config("ui/flashcard_lib.slint", config).unwrap();
}
