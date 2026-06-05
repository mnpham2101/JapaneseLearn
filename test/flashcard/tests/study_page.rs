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
    let _ = std::fs::remove_file("stacks.json");
}
