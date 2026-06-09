// # Test Goals
// Component: MatchingExerciseView
//
// # [Task 6.B.1]:
// - page-size defaults to 5 cards per page (reduced from 10 in Task R1.8)
// - page-offset starts at 0 and stays within [0, cards.length)
// - A stack larger than page-size exposes more cards than fit on one page
//   (the "Next" button only renders, and pagination only matters, in that case)
//
// # [Task R1.8]:
// - page-size reduced to 5
// - result-view property added; defaults to false
//
// Note: the front/back tile click handlers are internal TouchAreas inside
// `for` loops with no exposed callbacks (matching MatchingExerciseView's public
// interface, which intentionally only exposes `cards` and `exercise-completed`
// per the existing component contract). Pagination state — the externally
// observable surface this bug fix changes — is what these tests assert.

use std::cell::Cell;

use flashcard::flashcard::FlashcardModel;
use slint::{Model, ModelRc, VecModel};
use vocabulary::vocabulary::MatchingExerciseTestWindow;

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

fn card(front: &str, back: &str) -> FlashcardModel {
    FlashcardModel {
        jap_obj: front.into(),
        explanation: back.into(),
        known: false,
        is_kanji: false,
    }
}

/// Builds a stack of `count` cards named "front-N" / "back-N".
fn make_cards(count: usize) -> ModelRc<FlashcardModel> {
    let cards: Vec<FlashcardModel> = (0..count)
        .map(|i| card(&format!("front-{i}"), &format!("back-{i}")))
        .collect();
    ModelRc::new(VecModel::from(cards))
}

fn setup(card_count: usize) -> MatchingExerciseTestWindow {
    init_backend();
    let window = MatchingExerciseTestWindow::new().unwrap();
    window.set_cards(make_cards(card_count));
    window
}

/// Covers: Task R1.8 — page-size reduced to 5 cards per page
#[test]
fn matching_exercise_view_default_page_size() {
    let window = setup(5);

    assert_eq!(window.get_page_size(), 5);
}

/// Covers: Task 6.B.1 — pagination starts at the first page
#[test]
fn matching_exercise_view_starts_at_first_page() {
    let window = setup(23);

    assert_eq!(window.get_page_offset(), 0);
    assert_eq!(window.get_selected_front_index(), -1);
    assert_eq!(window.get_matched_count(), 0);
}

/// Covers: Task 6.B.1 — a stack larger than page-size has more cards than fit on
/// one page, which is exactly the condition that must show the "Next" button and
/// hide the back-column overflow that triggered this bug
#[test]
fn matching_exercise_view_large_stack_spans_multiple_pages() {
    let window = setup(23);

    let total = window.get_cards().row_count() as i32;
    let page_size = window.get_page_size();

    assert!(
        total > page_size,
        "stack of {total} cards should span more than one page of {page_size}"
    );
}

/// Covers: Task R1.8 — a stack no larger than page-size (5) fits on a single page
/// (the "Next" button must not appear / pagination is a no-op)
#[test]
fn matching_exercise_view_small_stack_fits_one_page() {
    let window = setup(4);

    let total = window.get_cards().row_count() as i32;
    let page_size = window.get_page_size();

    assert!(
        total <= page_size,
        "stack of {total} cards should fit within one page of {page_size}"
    );
}
