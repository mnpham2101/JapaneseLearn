// # Test Goals
// Page: VocabularyPage (Generate Flashcards notification)
//
// # [Task 6.B.2]:
// - Generating flashcards names only newly created stacks (not pre-existing ones) in
//   generation-notification, and switches active-view to the Exercise tab (1)
// - Regenerating with no new lessons produces no notification and does not change active-view
//
// # [Task 8.V.13]:
// - Generating exercises from a word with a tense entry and an example sentence produces,
//   end-to-end: a spelling flashcard + a tense-derived flashcard in flashcard-list, and a
//   single sentence flashcard (front = sentence, back = meaning) in sentence-stack-list

use std::cell::Cell;

use slint::{ComponentHandle, Model};
use vocabulary::vocabulary::{VocabularyAppLogic, VocabularyExerciseTestWindow};

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

/// Removes any persisted artifacts written by save_current_list / save_vocabulary
/// during the test so runs stay isolated and repeatable.
fn cleanup_artifacts() {
    let _ = std::fs::remove_file("data/stacks.json");
    let _ = std::fs::remove_file("data/vocabulary.json");
    let _ = std::fs::remove_file("data/sentences.json");
}

fn setup() -> VocabularyExerciseTestWindow {
    init_backend();
    cleanup_artifacts();
    let window = VocabularyExerciseTestWindow::new().unwrap();
    ::vocabulary::init(&window);
    ::flashcard::init(&window);
    ::vocabulary::init_exercise_generator(&window);

    // Clear state seeded by init() (default vocabulary / persisted stacks) so each
    // test starts from a clean, deterministic slate.
    let vocab_logic = window.global::<VocabularyAppLogic>();
    vocab_logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(vec![])));
    let flashcard_logic = window.global::<flashcard::flashcard::FlashcardAppLogic>();
    flashcard_logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(vec![])));
    vocab_logic.set_generation_notification("".into());
    vocab_logic.set_active_view(0);

    window
}

fn seed_lesson(logic: &VocabularyAppLogic, name: &str) {
    logic.invoke_lesson_create_confirmed(name.into());
    let lesson_idx = (logic.get_lesson_list().row_count() as i32) - 1;
    logic.invoke_word_add_confirmed(
        lesson_idx,
        "spelling".into(),
        "".into(),
        "meaning".into(),
        "".into(),
    );
}

/// Covers: Task 6.B.2 — first generation names the new stack and switches to Exercise tab
#[test]
fn generate_exercises_notifies_new_stack_and_switches_tab() {
    let window = setup();
    let vocab_logic = window.global::<VocabularyAppLogic>();
    seed_lesson(&vocab_logic, "N5 Verbs");

    vocab_logic.invoke_generate_exercises_clicked();

    let notification = vocab_logic.get_generation_notification().to_string();
    assert!(
        notification.contains("N5 Verbs"),
        "notification should name the new stack 'N5 Verbs', got: {notification}"
    );
    assert_eq!(vocab_logic.get_active_view(), 1);

    cleanup_artifacts();
}

/// Covers: Task 6.B.2 — regenerating with the same lessons creates no new stacks,
/// so no notification appears and the tab does not switch
#[test]
fn generate_exercises_no_notification_when_no_new_stacks() {
    let window = setup();
    let vocab_logic = window.global::<VocabularyAppLogic>();
    seed_lesson(&vocab_logic, "N5 Adjectives");

    // First generation creates the stack and switches tabs — reset observable state
    // to isolate the second (no-op) generation.
    vocab_logic.invoke_generate_exercises_clicked();
    vocab_logic.set_generation_notification("".into());
    vocab_logic.set_active_view(0);

    // Second generation from the same lesson list creates no new stacks.
    vocab_logic.invoke_generate_exercises_clicked();

    assert_eq!(vocab_logic.get_generation_notification(), "");
    assert_eq!(vocab_logic.get_active_view(), 0);

    cleanup_artifacts();
}

/// Covers: Task 6.B.2 — only the newly created stack is named when one lesson is new
/// and another already produced a stack in a prior generation
#[test]
fn generate_exercises_names_only_new_stack_among_existing() {
    let window = setup();
    let vocab_logic = window.global::<VocabularyAppLogic>();
    seed_lesson(&vocab_logic, "Existing Lesson");
    vocab_logic.invoke_generate_exercises_clicked();
    vocab_logic.set_generation_notification("".into());
    vocab_logic.set_active_view(0);

    // Add a second, genuinely new lesson alongside the existing one.
    seed_lesson(&vocab_logic, "New Lesson");
    vocab_logic.invoke_generate_exercises_clicked();

    let notification = vocab_logic.get_generation_notification().to_string();
    assert!(
        notification.contains("New Lesson"),
        "notification should name 'New Lesson', got: {notification}"
    );
    assert!(
        !notification.contains("Existing Lesson"),
        "notification should not re-name the pre-existing stack 'Existing Lesson', got: {notification}"
    );
    assert_eq!(vocab_logic.get_active_view(), 1);

    cleanup_artifacts();
}

/// Covers: Task 8.V.13 — generating exercises from a word with a tense entry and an
/// example sentence produces a tense-derived flashcard in flashcard-list and a sentence
/// flashcard in sentence-stack-list, end-to-end through invoke_generate_exercises_clicked()
#[test]
fn generate_exercises_creates_tense_and_sentence_cards() {
    let window = setup();
    let vocab_logic = window.global::<VocabularyAppLogic>();
    let flashcard_logic = window.global::<flashcard::flashcard::FlashcardAppLogic>();

    vocab_logic.invoke_lesson_create_confirmed("Test Lesson".into());
    vocab_logic.invoke_word_add_confirmed(
        0,
        "たべる".into(),
        "".into(),
        "to eat".into(),
        "verb".into(),
    );
    vocab_logic.invoke_word_tense_add_confirmed(0, 0, "past-formal".into(), "たべました".into());
    vocab_logic.invoke_word_example_add_confirmed(
        0,
        0,
        "おいしいです。".into(),
        "It is delicious.".into(),
    );

    vocab_logic.invoke_generate_exercises_clicked();

    // --- flashcard-list: spelling card + tense-derived card ---
    let flashcard_list = flashcard_logic.get_flashcard_list();
    let stack = (0..flashcard_list.row_count())
        .map(|i| flashcard_list.row_data(i).unwrap())
        .find(|s| s.stackname == "Test Lesson")
        .expect("expected a 'Test Lesson' stack in flashcard-list");

    let cards: Vec<_> = (0..stack.flashcards.row_count())
        .map(|i| stack.flashcards.row_data(i).unwrap())
        .collect();

    let spelling_card = cards
        .iter()
        .find(|c| c.jap_obj == "たべる")
        .expect("expected a spelling card with jap_obj == 'たべる'");
    assert_eq!(spelling_card.explanation, "to eat");

    let tense_card = cards
        .iter()
        .find(|c| c.jap_obj == "たべました")
        .expect("expected a tense-derived card with jap_obj == 'たべました'");
    assert_eq!(tense_card.explanation, "to eat");

    // --- sentence-stack-list: exactly one sentence card ---
    let sentence_stack_list = flashcard_logic.get_sentence_stack_list();
    let sentence_stack = (0..sentence_stack_list.row_count())
        .map(|i| sentence_stack_list.row_data(i).unwrap())
        .find(|s| s.stackname == "Test Lesson")
        .expect("expected a 'Test Lesson' stack in sentence-stack-list");

    assert_eq!(sentence_stack.flashcards.row_count(), 1);
    let sentence_card = sentence_stack.flashcards.row_data(0).unwrap();
    assert_eq!(sentence_card.jap_obj, "おいしいです。");
    assert_eq!(sentence_card.explanation, "It is delicious.");

    cleanup_artifacts();
}
