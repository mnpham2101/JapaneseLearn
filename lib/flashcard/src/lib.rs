pub mod flashcard {
    slint::include_modules!();
}

use flashcard::FlashcardAppLogic;

pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> FlashcardAppLogic<'a>: slint::Global<'a, T>,
{
    let _ = ui.global::<FlashcardAppLogic>(); // Global bound verified; Phase 2 adds handlers here
}
