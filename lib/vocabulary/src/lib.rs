pub mod vocabulary {
    slint::include_modules!();
}

use vocabulary::VocabularyAppLogic;

pub fn init<T>(_ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> VocabularyAppLogic<'a>: slint::Global<'a, T>,
{
    // CRUD handlers will be wired in tasks 6.9.1–6.9.3
}
