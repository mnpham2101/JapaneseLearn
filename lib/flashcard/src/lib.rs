use slint::Model;

pub mod flashcard {
    slint::include_modules!();
}

use flashcard::FlashcardAppLogic;

pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> FlashcardAppLogic<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<FlashcardAppLogic>();

    {
        let ui_weak = ui.as_weak();
        logic.on_stack_create_confirmed(move |name| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<FlashcardAppLogic>();
            let mut stacks: Vec<flashcard::FlashcardStackModel> =
                logic.get_flashcard_list().iter().collect();
            stacks.push(flashcard::FlashcardStackModel {
                stackname: name,
                flashcards: slint::ModelRc::new(slint::VecModel::from(vec![])),
            });
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks)));
        });
    }

    {
        let ui_weak = ui.as_weak();
        logic.on_flashcard_add_confirmed(move |jap, meaning| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<FlashcardAppLogic>();
            let idx = logic.get_selected_stack_index();
            if idx < 0 {
                return;
            }
            let idx = idx as usize;
            let mut stacks: Vec<flashcard::FlashcardStackModel> =
                logic.get_flashcard_list().iter().collect();
            if let Some(stack) = stacks.get_mut(idx) {
                let mut cards: Vec<flashcard::FlashcardModel> = stack.flashcards.iter().collect();
                cards.push(flashcard::FlashcardModel {
                    jap_obj: jap,
                    explanation: meaning,
                    known: false,
                });
                stack.flashcards = slint::ModelRc::new(slint::VecModel::from(cards));
            }
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks)));
        });
    }

    {
        let ui_weak = ui.as_weak();
        logic.on_flashcard_field_changed(move |card_index, jap, meaning| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<FlashcardAppLogic>();
            let stack_idx = logic.get_selected_stack_index();
            if stack_idx < 0 || card_index < 0 {
                return;
            }
            let stack_idx = stack_idx as usize;
            let card_idx = card_index as usize;
            let mut stacks: Vec<flashcard::FlashcardStackModel> =
                logic.get_flashcard_list().iter().collect();
            if let Some(stack) = stacks.get_mut(stack_idx) {
                let mut cards: Vec<flashcard::FlashcardModel> = stack.flashcards.iter().collect();
                if let Some(card) = cards.get_mut(card_idx) {
                    card.jap_obj = jap;
                    card.explanation = meaning;
                }
                stack.flashcards = slint::ModelRc::new(slint::VecModel::from(cards));
            }
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks)));
        });
    }
}
