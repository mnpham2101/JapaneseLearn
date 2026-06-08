// # Test Goals
// Page: VocabularyPage
//
// # [Task 6.7.1 + 6.9.2]:
// - Confirming lesson name creates a new VocabularyLessonModel appended to lesson-list
// - lesson-list reflects all created lessons in insertion order
// - lesson-delete-confirmed removes the lesson at selected-lesson-index; resets index to -1
//
// # [Task 6.8.1 + 6.9.2]:
// - Adding a word to a lesson appends VocabularyWordModel to lessons[idx].words
// - word-field-changed replaces word fields in-place
// - word-delete-confirmed removes the word at the given index
//
// # [Task 6.9.2]:
// - Data survives a simulated restart (load_vocabulary reads what save_vocabulary wrote)

use std::cell::Cell;

use slint::{ComponentHandle, Model};
use vocabulary::vocabulary::{VocabularyAppLogic, VocabularyTestWindow};

thread_local! {
    static BACKEND_INITED: Cell<bool> = const { Cell::new(false) };
}

fn init_backend() {
    BACKEND_INITED.with(|inited| {
        if !inited.get() {
            i_slint_backend_testing::init_no_event_loop();
            inited.set(true);
        }
    });
}

fn setup() -> VocabularyTestWindow {
    init_backend();
    let window = VocabularyTestWindow::new().unwrap();
    vocabulary::init(&window);
    // Clear after init() so load_vocabulary() output does not pollute tests.
    let logic = window.global::<VocabularyAppLogic>();
    logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(vec![])));
    window
}

fn seed_lesson(logic: &VocabularyAppLogic, name: &str) {
    logic.invoke_lesson_create_confirmed(name.into());
}

fn seed_word(logic: &VocabularyAppLogic, lesson_idx: i32, spelling: &str, meaning: &str) {
    logic.invoke_word_add_confirmed(
        lesson_idx,
        spelling.into(),
        "".into(),
        meaning.into(),
        "".into(),
    );
}

/// Covers: Task 6.7.1 + 6.9.2 — lesson creation appends to lesson-list
#[test]
fn vocabulary_page_create_lesson() {
    let window = setup();
    let logic = window.global::<VocabularyAppLogic>();

    logic.invoke_lesson_create_confirmed("Animals".into());

    let list = logic.get_lesson_list();
    assert_eq!(list.row_count(), 1);
    assert_eq!(list.row_data(0).unwrap().name, "Animals");
}

/// Covers: Task 6.7.1 + 6.9.2 — lesson-list reflects all created lessons in insertion order
#[test]
fn vocabulary_page_read_lessons() {
    let window = setup();
    let logic = window.global::<VocabularyAppLogic>();
    seed_lesson(&logic, "Animals");
    seed_lesson(&logic, "Verbs");

    let list = logic.get_lesson_list();
    assert_eq!(list.row_count(), 2);
    assert_eq!(list.row_data(0).unwrap().name, "Animals");
    assert_eq!(list.row_data(1).unwrap().name, "Verbs");
}

/// Covers: Task 6.8.1 + 6.9.2 — word-add-confirmed appends VocabularyWordModel to lesson words
#[test]
fn vocabulary_page_create_word() {
    let window = setup();
    let logic = window.global::<VocabularyAppLogic>();
    seed_lesson(&logic, "Animals");
    logic.set_selected_lesson_index(0);

    logic.invoke_word_add_confirmed(0, "いぬ".into(), "".into(), "dog".into(), "".into());

    let words = logic.get_lesson_list().row_data(0).unwrap().words;
    assert_eq!(words.row_count(), 1);
    let word = words.row_data(0).unwrap();
    assert_eq!(word.spelling, "いぬ");
    assert_eq!(word.meaning, "dog");
}

/// Covers: Task 6.8.1 + 6.9.2 — word-field-changed replaces word fields in-place
#[test]
fn vocabulary_page_update_word() {
    let window = setup();
    let logic = window.global::<VocabularyAppLogic>();
    seed_lesson(&logic, "Animals");
    logic.set_selected_lesson_index(0);
    seed_word(&logic, 0, "いぬ", "dog");

    logic.invoke_word_field_changed(0, 0, "ねこ".into(), "".into(), "cat".into(), "".into());

    let word = logic
        .get_lesson_list()
        .row_data(0)
        .unwrap()
        .words
        .row_data(0)
        .unwrap();
    assert_eq!(word.spelling, "ねこ");
    assert_eq!(word.meaning, "cat");
}

/// Covers: Task 6.8.1 + 6.9.2 — word-delete-confirmed removes the word at the given index
#[test]
fn vocabulary_page_delete_word() {
    let window = setup();
    let logic = window.global::<VocabularyAppLogic>();
    seed_lesson(&logic, "Animals");
    logic.set_selected_lesson_index(0);
    seed_word(&logic, 0, "いぬ", "dog");
    seed_word(&logic, 0, "ねこ", "cat");

    logic.invoke_word_delete_confirmed(0, 0);

    let words = logic.get_lesson_list().row_data(0).unwrap().words;
    assert_eq!(words.row_count(), 1);
    assert_eq!(words.row_data(0).unwrap().spelling, "ねこ");
}

/// Covers: Task 6.7.1 + 6.9.2 — lesson-delete-confirmed removes lesson at selected index; resets index to -1
#[test]
fn vocabulary_page_delete_lesson() {
    let window = setup();
    let logic = window.global::<VocabularyAppLogic>();
    seed_lesson(&logic, "Animals");
    seed_lesson(&logic, "Verbs");
    logic.set_selected_lesson_index(0);

    logic.invoke_lesson_delete_confirmed();

    assert_eq!(logic.get_lesson_list().row_count(), 1);
    assert_eq!(logic.get_selected_lesson_index(), -1);
    assert_eq!(logic.get_lesson_list().row_data(0).unwrap().name, "Verbs");
}

/// Covers: Task 6.9.2 — data survives a simulated restart (load_vocabulary reads what save_vocabulary wrote)
#[test]
fn vocabulary_page_persistence_round_trip() {
    // First session: create lesson and word; save happens automatically in callbacks.
    {
        let window = setup();
        let logic = window.global::<VocabularyAppLogic>();
        seed_lesson(&logic, "Saved Lesson");
        seed_word(&logic, 0, "いぬ", "dog");
    }

    // Second session: verify load restores state.
    {
        init_backend();
        let window = VocabularyTestWindow::new().unwrap();
        vocabulary::init(&window);
        let logic = window.global::<VocabularyAppLogic>();
        let list = logic.get_lesson_list();
        assert_eq!(list.row_count(), 1);
        assert_eq!(list.row_data(0).unwrap().name, "Saved Lesson");
        let words = list.row_data(0).unwrap().words;
        assert_eq!(words.row_count(), 1);
        assert_eq!(words.row_data(0).unwrap().spelling, "いぬ");
    }

    // Clean up test artifact.
    let _ = std::fs::remove_file("data/vocabulary.json");
}
