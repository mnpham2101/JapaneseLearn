slint::include_modules!();

pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
{
    // Phase 2 callback registrations will be added here incrementally.
    let _ = ui; // suppress unused warning until Phase 2 wires callbacks
}
