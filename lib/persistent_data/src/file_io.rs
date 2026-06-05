// File dialog and I/O for markdown flashcard files.
// All desktop I/O is gated #[cfg(not(target_arch = "wasm32"))].

/// Opens a file dialog for the user to pick a markdown file and returns its content.
///
/// Returns `Some(content)` on success, or `None` if the user cancels or an I/O error occurs.
///
/// # Example
/// ```no_run
/// if let Some(content) = persistent_data::file_io::open_markdown_file() {
///     println!("File content: {}", content);
/// }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn open_markdown_file() -> Option<String> {
    use std::io::Read;
    let path = rfd::FileDialog::new()
        .add_filter("Markdown", &["md"])
        .pick_file()?;
    let mut content = String::new();
    std::fs::File::open(path)
        .ok()?
        .read_to_string(&mut content)
        .ok()?;
    Some(content)
}

/// WASM stub — file dialogs are not available on WebAssembly.
#[cfg(target_arch = "wasm32")]
pub fn open_markdown_file() -> Option<String> {
    None
}

/// Opens a save-file dialog for the user to choose a destination and writes `content` as markdown.
///
/// Returns `true` on success, or `false` if the user cancels or an I/O error occurs.
///
/// # Example
/// ```no_run
/// let saved = persistent_data::file_io::save_markdown_file("# My Stacks\n");
/// assert!(saved);
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn save_markdown_file(content: &str) -> bool {
    use std::io::Write;
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Markdown", &["md"])
        .set_file_name("stacks.md")
        .save_file()
    else {
        return false;
    };
    let Ok(mut file) = std::fs::File::create(path) else {
        return false;
    };
    file.write_all(content.as_bytes()).is_ok()
}

/// WASM stub — file dialogs are not available on WebAssembly.
#[cfg(target_arch = "wasm32")]
pub fn save_markdown_file(_content: &str) -> bool {
    false
}
