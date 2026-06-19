// # Test Goals
// Page: StudyPage
//
// # [Task 2.9]:
// - Confirming stack name creates a new FlashcardStackModel appended to flashcard-list
// - Confirming flashcard add appends a FlashcardModel to the selected stack
// - flashcard-list reflects all created stacks in insertion order
// - Editing field values are reflected in the model at the correct indices
// - Reordering moves a card from source to target index (remove-then-insert semantics)
// - Deleting a card removes the FlashcardModel at the given index
// - Deleting a stack removes it from flashcard-list and resets selected-stack-index to -1
// - Data survives a simulated restart (load_stacks reads what save_stacks wrote)
//
// # [Task 8.1]:
// - update_progress edge case: no stack ever selected (selected-stack-index stays at its
//   default -1) leaves known-count/total-count at 0 — on_known_changed early-returns before
//   reaching update_progress whenever stack_index < 0, so the no-selection path is exercised
//   by asserting the untouched default state rather than invoking that callback
// - update_progress edge case: an empty stack (zero cards) is selected, then on_known_changed
//   is invoked with a card index that does not exist in that stack — the handler's card
//   mutation is skipped via get_mut returning None, but update_progress still runs and
//   recomputes known-count/total-count as 0/0 from the empty stack
//
// # [Task 8.2]:
// - on_known_changed repeated-call guard: invoking known-changed with the same boolean value
//   the card already holds (e.g. a resync-triggered navigation re-firing the callback) does
//   not corrupt state — known stays correct and known-count/total-count remain accurate across
//   repeated identical invocations

use std::cell::Cell;

use flashcard::flashcard::{FlashcardAppLogic, FlashcardTestWindow};
use slint::{ComponentHandle, Model};

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

fn setup() -> FlashcardTestWindow {
    init_backend();
    let window = FlashcardTestWindow::new().unwrap();
    flashcard::init(&window);
    // Clear after init() so load_stacks() output does not pollute tests.
    let logic = window.global::<FlashcardAppLogic>();
    logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(vec![])));
    window
}

fn seed_stack(logic: &FlashcardAppLogic, name: &str) {
    logic.invoke_stack_create_confirmed(name.into());
}

fn seed_card(logic: &FlashcardAppLogic, jap: &str, meaning: &str) {
    logic.invoke_flashcard_add_confirmed(jap.into(), meaning.into());
}

/// Covers: Task 2.9 — stack creation appends to flashcard-list
#[test]
fn study_page_create_stack() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();

    logic.invoke_stack_create_confirmed("Hiragana".into());

    let list = logic.get_flashcard_list();
    assert_eq!(list.row_count(), 1);
    assert_eq!(list.row_data(0).unwrap().stackname, "Hiragana");
}

/// Covers: Task 2.9 — add-card form appends FlashcardModel to selected stack
#[test]
fn study_page_create_card() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();
    seed_stack(&logic, "Stack A");
    logic.set_selected_stack_index(0);

    logic.invoke_flashcard_add_confirmed("犬".into(), "dog".into());

    let cards = logic.get_flashcard_list().row_data(0).unwrap().flashcards;
    assert_eq!(cards.row_count(), 1);
    let card = cards.row_data(0).unwrap();
    assert_eq!(card.jap_obj, "犬");
    assert_eq!(card.explanation, "dog");
    assert!(!card.known);
}

/// Covers: Task 2.9 — list reflects all created stacks in insertion order
#[test]
fn study_page_read_list() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();
    seed_stack(&logic, "Stack A");
    seed_stack(&logic, "Stack B");

    let list = logic.get_flashcard_list();
    assert_eq!(list.row_count(), 2);
    assert_eq!(list.row_data(0).unwrap().stackname, "Stack A");
    assert_eq!(list.row_data(1).unwrap().stackname, "Stack B");
}

/// Covers: Task 2.9 — edited field values are reflected in the model
#[test]
fn study_page_update_card() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();
    seed_stack(&logic, "Stack A");
    logic.set_selected_stack_index(0);
    seed_card(&logic, "猫", "cat");

    logic.invoke_flashcard_field_changed(0, "犬".into(), "dog".into());

    let card = logic
        .get_flashcard_list()
        .row_data(0)
        .unwrap()
        .flashcards
        .row_data(0)
        .unwrap();
    assert_eq!(card.jap_obj, "犬");
    assert_eq!(card.explanation, "dog");
}

/// Covers: Task 2.9 — reorder moves card from source to target index
#[test]
fn study_page_reorder_cards() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();
    seed_stack(&logic, "Stack A");
    logic.set_selected_stack_index(0);
    seed_card(&logic, "猫", "cat");
    seed_card(&logic, "犬", "dog");
    seed_card(&logic, "鳥", "bird");

    logic.invoke_flashcard_reordered(0, 2);

    let cards = logic.get_flashcard_list().row_data(0).unwrap().flashcards;
    assert_eq!(cards.row_data(0).unwrap().jap_obj, "犬");
    assert_eq!(cards.row_data(1).unwrap().jap_obj, "鳥");
    assert_eq!(cards.row_data(2).unwrap().jap_obj, "猫");
}

/// Covers: Task 2.9 — delete removes the card at the given index
#[test]
fn study_page_delete_card() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();
    seed_stack(&logic, "Stack A");
    logic.set_selected_stack_index(0);
    seed_card(&logic, "猫", "cat");
    seed_card(&logic, "犬", "dog");

    logic.invoke_flashcard_delete_confirmed(0);

    let cards = logic.get_flashcard_list().row_data(0).unwrap().flashcards;
    assert_eq!(cards.row_count(), 1);
    assert_eq!(cards.row_data(0).unwrap().jap_obj, "犬");
}

/// Covers: Task 2.9 — delete removes stack at selected index; resets index to -1
#[test]
fn study_page_delete_stack() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();
    seed_stack(&logic, "Stack A");
    seed_stack(&logic, "Stack B");
    logic.set_selected_stack_index(0);

    logic.invoke_stack_delete_confirmed();

    assert_eq!(logic.get_flashcard_list().row_count(), 1);
    assert_eq!(logic.get_selected_stack_index(), -1);
    assert_eq!(
        logic.get_flashcard_list().row_data(0).unwrap().stackname,
        "Stack B"
    );
}

/// Covers: Task 2.9 — data survives a simulated restart (load_stacks reads what save_stacks wrote)
#[test]
fn study_page_persistence_round_trip() {
    // First session: create and save
    {
        let window = setup();
        let logic = window.global::<FlashcardAppLogic>();
        seed_stack(&logic, "Saved Stack");
        logic.set_selected_stack_index(0);
        seed_card(&logic, "猫", "cat");
        // save_stacks is called automatically after each mutation via init() callbacks
    }

    // Second session: verify load restores state
    {
        init_backend();
        let window = FlashcardTestWindow::new().unwrap();
        flashcard::init(&window);
        let logic = window.global::<FlashcardAppLogic>();
        let list = logic.get_flashcard_list();
        assert_eq!(list.row_count(), 1);
        assert_eq!(list.row_data(0).unwrap().stackname, "Saved Stack");
        let cards = list.row_data(0).unwrap().flashcards;
        assert_eq!(cards.row_count(), 1);
        assert_eq!(cards.row_data(0).unwrap().jap_obj, "猫");
    }

    // Clean up test artifact
    let _ = std::fs::remove_file("data/stacks.json");
}

/// Covers: Task 8.1 — update_progress with no stack ever selected leaves counts at 0.
/// on_known_changed early-returns before reaching update_progress whenever stack_index < 0,
/// so there is no production call path that reaches update_progress with no selection; the
/// no-selection invariant is instead verified directly against the untouched default state.
#[test]
fn study_page_progress_no_selection() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();

    assert_eq!(logic.get_selected_stack_index(), -1);
    assert_eq!(logic.get_known_count(), 0);
    assert_eq!(logic.get_total_count(), 0);
}

/// Covers: Task 8.1 — update_progress with an empty selected stack (zero cards) recomputes
/// known-count/total-count as 0/0. on_known_changed's card mutation is skipped because
/// card_index 0 does not exist in the empty stack, but update_progress still runs
/// unconditionally afterward and reflects the empty stack's real card count.
#[test]
fn study_page_progress_empty_stack() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();
    seed_stack(&logic, "Empty Stack");
    logic.set_selected_stack_index(0);

    logic.invoke_known_changed(0, 0, false);

    assert_eq!(logic.get_known_count(), 0);
    assert_eq!(logic.get_total_count(), 0);
}

/// Covers: Task 8.2 — repeated on_known_changed calls with the same value are a no-op that
/// does not corrupt state. Simulates a resync-triggered navigation re-firing known-changed
/// with the value the card already holds (e.g. Prev/Next re-syncing `known` on every card
/// switch per Phase 8.B), confirming known stays true and known-count/total-count stay 1/1
/// across repeated identical invocations.
#[test]
fn study_page_known_changed_repeated_noop() {
    let window = setup();
    let logic = window.global::<FlashcardAppLogic>();
    seed_stack(&logic, "Stack A");
    logic.set_selected_stack_index(0);
    seed_card(&logic, "猫", "cat");

    logic.invoke_known_changed(0, 0, true);
    let card = logic
        .get_flashcard_list()
        .row_data(0)
        .unwrap()
        .flashcards
        .row_data(0)
        .unwrap();
    assert!(card.known);
    assert_eq!(logic.get_known_count(), 1);
    assert_eq!(logic.get_total_count(), 1);

    // Repeat the same call twice more — must remain a no-op.
    logic.invoke_known_changed(0, 0, true);
    logic.invoke_known_changed(0, 0, true);

    let card = logic
        .get_flashcard_list()
        .row_data(0)
        .unwrap()
        .flashcards
        .row_data(0)
        .unwrap();
    assert!(card.known);
    assert_eq!(logic.get_known_count(), 1);
    assert_eq!(logic.get_total_count(), 1);
}
