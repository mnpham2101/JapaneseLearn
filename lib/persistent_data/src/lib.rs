// Persistent data service: markdown I/O and file dialogs.

pub mod file_io;
pub mod markdown_io;

use flashcard::flashcard::{FlashcardAppLogic, FlashcardModel, FlashcardStackModel};
use markdown_io::{CardData, StackData};
use slint::{Model, ModelRc, SharedString, VecModel};

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
