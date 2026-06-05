use slint::Model;

pub mod flashcard {
    slint::include_modules!();
}

use flashcard::FlashcardAppLogic;

// ── Persistence ──────────────────────────────────────────────────────────────

const STACKS_FILE: &str = "stacks.json";

#[derive(serde::Serialize, serde::Deserialize)]
struct CardData {
    jap_obj: String,
    explanation: String,
    known: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct StackData {
    stackname: String,
    flashcards: Vec<CardData>,
}

fn to_stack_data(stacks: &[flashcard::FlashcardStackModel]) -> Vec<StackData> {
    stacks
        .iter()
        .map(|s| StackData {
            stackname: s.stackname.to_string(),
            flashcards: s
                .flashcards
                .iter()
                .map(|c| CardData {
                    jap_obj: c.jap_obj.to_string(),
                    explanation: c.explanation.to_string(),
                    known: c.known,
                })
                .collect(),
        })
        .collect()
}

fn from_stack_data(data: Vec<StackData>) -> Vec<flashcard::FlashcardStackModel> {
    data.into_iter()
        .map(|s| flashcard::FlashcardStackModel {
            stackname: s.stackname.into(),
            flashcards: slint::ModelRc::new(slint::VecModel::from(
                s.flashcards
                    .into_iter()
                    .map(|c| flashcard::FlashcardModel {
                        jap_obj: c.jap_obj.into(),
                        explanation: c.explanation.into(),
                        known: c.known,
                    })
                    .collect::<Vec<_>>(),
            )),
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn save_stacks(stacks: &[flashcard::FlashcardStackModel]) {
    if let Ok(json) = serde_json::to_string_pretty(&to_stack_data(stacks)) {
        let _ = std::fs::write(STACKS_FILE, json);
    }
}

#[cfg(target_arch = "wasm32")]
fn save_stacks(_stacks: &[flashcard::FlashcardStackModel]) {}

#[cfg(not(target_arch = "wasm32"))]
fn load_stacks() -> Option<Vec<flashcard::FlashcardStackModel>> {
    let json = std::fs::read_to_string(STACKS_FILE).ok()?;
    let data: Vec<StackData> = serde_json::from_str(&json).ok()?;
    Some(from_stack_data(data))
}

#[cfg(target_arch = "wasm32")]
fn load_stacks() -> Option<Vec<flashcard::FlashcardStackModel>> {
    None
}

fn update_progress<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> FlashcardAppLogic<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<FlashcardAppLogic>();
    let stack_idx = logic.get_selected_stack_index();
    if stack_idx < 0 {
        logic.set_known_count(0);
        logic.set_total_count(0);
        return;
    }
    let stacks: Vec<flashcard::FlashcardStackModel> = logic.get_flashcard_list().iter().collect();
    if let Some(stack) = stacks.get(stack_idx as usize) {
        let cards: Vec<flashcard::FlashcardModel> = stack.flashcards.iter().collect();
        logic.set_total_count(cards.len() as i32);
        logic.set_known_count(cards.iter().filter(|c| c.known).count() as i32);
    } else {
        logic.set_known_count(0);
        logic.set_total_count(0);
    }
}

pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> FlashcardAppLogic<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<FlashcardAppLogic>();

    // Restore persisted stacks on startup (no-op on WASM).
    if let Some(stacks) = load_stacks() {
        logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks)));
    }

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
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks.clone())));
            save_stacks(&stacks);
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
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks.clone())));
            save_stacks(&stacks);
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
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks.clone())));
            save_stacks(&stacks);
        });
    }

    {
        let ui_weak = ui.as_weak();
        logic.on_flashcard_delete_confirmed(move |card_index| {
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
                if card_idx < cards.len() {
                    cards.remove(card_idx);
                }
                stack.flashcards = slint::ModelRc::new(slint::VecModel::from(cards));
            }
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks.clone())));
            save_stacks(&stacks);
        });
    }

    {
        let ui_weak = ui.as_weak();
        logic.on_stack_delete_confirmed(move || {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<FlashcardAppLogic>();
            let stack_idx = logic.get_selected_stack_index();
            if stack_idx < 0 {
                return;
            }
            let idx = stack_idx as usize;
            let mut stacks: Vec<flashcard::FlashcardStackModel> =
                logic.get_flashcard_list().iter().collect();
            if idx < stacks.len() {
                stacks.remove(idx);
            }
            logic.set_selected_stack_index(-1);
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks.clone())));
            save_stacks(&stacks);
        });
    }

    {
        let ui_weak = ui.as_weak();
        logic.on_flashcard_reordered(move |from_index, to_index| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<FlashcardAppLogic>();
            let stack_idx = logic.get_selected_stack_index();
            if stack_idx < 0 || from_index < 0 || to_index < 0 || from_index == to_index {
                return;
            }
            let stack_idx = stack_idx as usize;
            let from = from_index as usize;
            let to = to_index as usize;
            let mut stacks: Vec<flashcard::FlashcardStackModel> =
                logic.get_flashcard_list().iter().collect();
            if let Some(stack) = stacks.get_mut(stack_idx) {
                let mut cards: Vec<flashcard::FlashcardModel> = stack.flashcards.iter().collect();
                if from < cards.len() && to < cards.len() {
                    let card = cards.remove(from);
                    cards.insert(to, card);
                }
                stack.flashcards = slint::ModelRc::new(slint::VecModel::from(cards));
            }
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks.clone())));
            save_stacks(&stacks);
        });
    }

    {
        let ui_weak = ui.as_weak();
        logic.on_known_changed(move |stack_index, card_index, known| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<FlashcardAppLogic>();
            if stack_index < 0 || card_index < 0 {
                return;
            }
            let stack_idx = stack_index as usize;
            let card_idx = card_index as usize;
            let mut stacks: Vec<flashcard::FlashcardStackModel> =
                logic.get_flashcard_list().iter().collect();
            if let Some(stack) = stacks.get_mut(stack_idx) {
                let mut cards: Vec<flashcard::FlashcardModel> = stack.flashcards.iter().collect();
                if let Some(card) = cards.get_mut(card_idx) {
                    card.known = known;
                }
                stack.flashcards = slint::ModelRc::new(slint::VecModel::from(cards));
            }
            logic.set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(stacks.clone())));
            save_stacks(&stacks);
            update_progress(&ui);
        });
    }
}
