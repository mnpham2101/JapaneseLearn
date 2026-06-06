// Persistent data service: markdown I/O and file dialogs.

pub mod file_io;
pub mod markdown_io;

use flashcard::flashcard::{FlashcardAppLogic, FlashcardModel, FlashcardStackModel};
use markdown_io::{CardData, StackData};
use slint::{Model, ModelRc, SharedString, VecModel};

/// Wire `import-stack-clicked` and `export-stack-clicked` callbacks on [`FlashcardAppLogic`].
///
/// - **Import**: opens a file dialog, parses the chosen markdown file into stacks, and replaces
///   the current `flashcard-list` on the global.
/// - **Export**: reads the current `flashcard-list` from the global, serialises it to markdown,
///   and opens a save-file dialog so the user can choose the destination.
pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> FlashcardAppLogic<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<FlashcardAppLogic>();

    {
        let ui_weak = ui.as_weak();
        logic.on_import_stack_clicked(move || {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let Some(content) = file_io::open_markdown_file() else {
                    return;
                };
                let parsed = markdown_io::parse_stacks(&content);
                let ui = ui_weak.unwrap();
                let logic = ui.global::<FlashcardAppLogic>();
                logic.set_flashcard_list(stacks_to_slint(&parsed));
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        logic.on_export_stack_clicked(move || {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let ui = ui_weak.unwrap();
                let logic = ui.global::<FlashcardAppLogic>();
                let list = logic.get_flashcard_list();
                let stacks = slint_to_stacks(&list);
                let content = markdown_io::serialize_stacks(&stacks);
                file_io::save_markdown_file(&content);
            }
        });
    }
}

/// Convert a slice of [`StackData`] into a [`ModelRc`] of [`FlashcardStackModel`].
fn stacks_to_slint(stacks: &[StackData]) -> ModelRc<FlashcardStackModel> {
    let vec: Vec<FlashcardStackModel> = stacks
        .iter()
        .map(|s| FlashcardStackModel {
            stackname: SharedString::from(s.name.as_str()),
            flashcards: cards_to_slint(&s.cards),
        })
        .collect();
    ModelRc::new(VecModel::from(vec))
}

/// Convert a slice of [`CardData`] into a [`ModelRc`] of [`FlashcardModel`].
fn cards_to_slint(cards: &[CardData]) -> ModelRc<FlashcardModel> {
    let vec: Vec<FlashcardModel> = cards
        .iter()
        .map(|c| FlashcardModel {
            jap_obj: SharedString::from(c.japanese.as_str()),
            explanation: SharedString::from(c.meaning.as_str()),
            known: false,
            is_kanji: false,
        })
        .collect();
    ModelRc::new(VecModel::from(vec))
}

/// Convert a [`ModelRc`] of [`FlashcardStackModel`] into a [`Vec`] of [`StackData`].
fn slint_to_stacks(list: &ModelRc<FlashcardStackModel>) -> Vec<StackData> {
    (0..list.row_count())
        .filter_map(|i| list.row_data(i))
        .map(|s| StackData {
            name: s.stackname.to_string(),
            cards: slint_to_cards(&s.flashcards),
        })
        .collect()
}

/// Convert a [`ModelRc`] of [`FlashcardModel`] into a [`Vec`] of [`CardData`].
fn slint_to_cards(cards: &ModelRc<FlashcardModel>) -> Vec<CardData> {
    (0..cards.row_count())
        .filter_map(|i| cards.row_data(i))
        .map(|c| CardData {
            japanese: c.jap_obj.to_string(),
            meaning: c.explanation.to_string(),
        })
        .collect()
}
