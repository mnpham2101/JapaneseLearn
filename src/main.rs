#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_family = "wasm", wasm_bindgen(start))]
fn main() {
    let window = MainWindow::new().expect("Failed to create MainWindow");
    ::flashcard::init(&window);
    ::persistent_data::init(&window);
    window.run().expect("Failed to run MainWindow");
}
