slint::include_modules!();

fn main() {
    let window = MainWindow::new().expect("Failed to create MainWindow");
    window.run().expect("Failed to run MainWindow");
}
