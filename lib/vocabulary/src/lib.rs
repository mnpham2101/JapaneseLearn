use slint::Model;

pub mod vocabulary {
    slint::include_modules!();
}

pub mod vocabulary_markdown_io;

use exercise_generator::{
    ExerciseGeneratorFor, ExerciseGeneratorService, ExerciseOutput, ExerciseRequest, TenseEntry,
    VocabularyLesson, VocabularyWord,
};
use flashcard::flashcard::FlashcardAppLogic;
use vocabulary::VocabularyAppLogic;

// ── Shadow structs (serde-serialisable mirror of the Slint model) ─────────────

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct TenseData {
    pub(crate) name: String,
    pub(crate) conjugation: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct WordData {
    pub(crate) spelling: String,
    pub(crate) kanji: String,
    pub(crate) meaning: String,
    pub(crate) word_type: String,
    pub(crate) tenses: Vec<TenseData>,
    pub(crate) examples: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
pub(crate) struct LessonData {
    pub(crate) name: String,
    pub(crate) words: Vec<WordData>,
}

// ── Conversion helpers ────────────────────────────────────────────────────────

fn slint_to_lessons(logic: &VocabularyAppLogic) -> Vec<LessonData> {
    logic
        .get_lesson_list()
        .iter()
        .map(|lesson| LessonData {
            name: lesson.name.to_string(),
            words: lesson
                .words
                .iter()
                .map(|word| WordData {
                    spelling: word.spelling.to_string(),
                    kanji: word.kanji.to_string(),
                    meaning: word.meaning.to_string(),
                    word_type: word.word_type.to_string(),
                    tenses: word
                        .tenses
                        .iter()
                        .map(|t| TenseData {
                            name: t.tense_name.to_string(),
                            conjugation: t.conjugation.to_string(),
                        })
                        .collect(),
                    examples: word.examples.iter().map(|e| e.to_string()).collect(),
                })
                .collect(),
        })
        .collect()
}

fn lessons_to_slint(data: &[LessonData]) -> slint::ModelRc<vocabulary::VocabularyLessonModel> {
    slint::ModelRc::new(slint::VecModel::from(
        data.iter()
            .map(|lesson| vocabulary::VocabularyLessonModel {
                name: lesson.name.clone().into(),
                words: slint::ModelRc::new(slint::VecModel::from(
                    lesson
                        .words
                        .iter()
                        .map(|word| vocabulary::VocabularyWordModel {
                            spelling: word.spelling.clone().into(),
                            kanji: word.kanji.clone().into(),
                            meaning: word.meaning.clone().into(),
                            word_type: word.word_type.clone().into(),
                            tenses: slint::ModelRc::new(slint::VecModel::from(
                                word.tenses
                                    .iter()
                                    .map(|t| vocabulary::TenseEntryModel {
                                        tense_name: t.name.clone().into(),
                                        conjugation: t.conjugation.clone().into(),
                                    })
                                    .collect::<Vec<_>>(),
                            )),
                            examples: slint::ModelRc::new(slint::VecModel::from(
                                word.examples
                                    .iter()
                                    .map(|e| slint::SharedString::from(e.as_str()))
                                    .collect::<Vec<_>>(),
                            )),
                        })
                        .collect::<Vec<_>>(),
                )),
            })
            .collect::<Vec<_>>(),
    ))
}

// ── Persistence ───────────────────────────────────────────────────────────────

const VOCABULARY_FILE: &str = "data/vocabulary.json";

#[cfg(not(target_arch = "wasm32"))]
fn load_vocabulary() -> Vec<LessonData> {
    std::fs::read_to_string(VOCABULARY_FILE)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn load_vocabulary() -> Vec<LessonData> {
    vec![]
}

#[cfg(not(target_arch = "wasm32"))]
fn save_vocabulary(lessons: &[LessonData]) {
    if let Ok(json) = serde_json::to_string_pretty(lessons) {
        if let Some(parent) = std::path::Path::new(VOCABULARY_FILE).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(VOCABULARY_FILE, json);
    }
}

#[cfg(target_arch = "wasm32")]
fn save_vocabulary(_lessons: &[LessonData]) {}

// ── Default data ──────────────────────────────────────────────────────────────

/// Embed the three default JSON datasets at compile time and write them to
/// `vocabulary.json`.  Returns the combined lesson list.
/// Only compiled on non-WASM targets (uses `save_vocabulary` which gates on
/// the same cfg).
#[cfg(not(target_arch = "wasm32"))]
fn load_and_save_defaults() -> Vec<LessonData> {
    let verbs_json = include_str!("../ui/data/n5_verbs.json");
    let adjectives_json = include_str!("../ui/data/n5_adjectives.json");
    let vocabulary_json = include_str!("../ui/data/n5_vocabulary.json");

    let mut combined: Vec<LessonData> = Vec::new();
    for json in &[verbs_json, adjectives_json, vocabulary_json] {
        let lessons: Vec<LessonData> = serde_json::from_str(json).unwrap_or_default();
        combined.extend(lessons);
    }
    save_vocabulary(&combined);
    combined
}

// ── init ──────────────────────────────────────────────────────────────────────

/// Wire all vocabulary CRUD callbacks.  Call this from every host window,
/// including test windows that only expose `VocabularyAppLogic`.
pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> VocabularyAppLogic<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<VocabularyAppLogic>();

    // Restore persisted lessons on startup (no-op on WASM).
    // On first launch (vocabulary.json absent) embed the bundled N5 defaults.
    #[cfg(not(target_arch = "wasm32"))]
    {
        if !std::path::Path::new(VOCABULARY_FILE).exists() {
            let defaults = load_and_save_defaults();
            logic.set_lesson_list(lessons_to_slint(&defaults));
        } else {
            let saved = load_vocabulary();
            if !saved.is_empty() {
                logic.set_lesson_list(lessons_to_slint(&saved));
            }
        }
    }

    // ── lesson-create-confirmed ───────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_lesson_create_confirmed(move |name| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<VocabularyAppLogic>();
            let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                logic.get_lesson_list().iter().collect();
            lessons.push(vocabulary::VocabularyLessonModel {
                name,
                words: slint::ModelRc::new(slint::VecModel::from(vec![])),
            });
            logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
            save_vocabulary(&slint_to_lessons(&logic));
        });
    }

    // ── lesson-delete-confirmed ───────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_lesson_delete_confirmed(move || {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<VocabularyAppLogic>();
            let idx = logic.get_selected_lesson_index();
            if idx < 0 {
                return;
            }
            let idx = idx as usize;
            let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                logic.get_lesson_list().iter().collect();
            if idx < lessons.len() {
                lessons.remove(idx);
            }
            logic.set_selected_lesson_index(-1);
            logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
            save_vocabulary(&slint_to_lessons(&logic));
        });
    }

    // ── word-add-confirmed ────────────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_word_add_confirmed(move |lesson_idx, spelling, kanji, meaning, word_type| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<VocabularyAppLogic>();
            if lesson_idx < 0 {
                return;
            }
            let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                logic.get_lesson_list().iter().collect();
            if let Some(lesson) = lessons.get_mut(lesson_idx as usize) {
                let mut words: Vec<vocabulary::VocabularyWordModel> = lesson.words.iter().collect();
                words.push(vocabulary::VocabularyWordModel {
                    spelling,
                    kanji,
                    meaning,
                    word_type,
                    tenses: slint::ModelRc::new(slint::VecModel::from(vec![])),
                    examples: slint::ModelRc::new(slint::VecModel::from(vec![])),
                });
                lesson.words = slint::ModelRc::new(slint::VecModel::from(words));
            }
            logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
            save_vocabulary(&slint_to_lessons(&logic));
        });
    }

    // ── word-delete-confirmed ─────────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_word_delete_confirmed(move |lesson_idx, word_idx| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<VocabularyAppLogic>();
            if lesson_idx < 0 || word_idx < 0 {
                return;
            }
            let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                logic.get_lesson_list().iter().collect();
            if let Some(lesson) = lessons.get_mut(lesson_idx as usize) {
                let mut words: Vec<vocabulary::VocabularyWordModel> = lesson.words.iter().collect();
                if (word_idx as usize) < words.len() {
                    words.remove(word_idx as usize);
                }
                lesson.words = slint::ModelRc::new(slint::VecModel::from(words));
            }
            logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
            save_vocabulary(&slint_to_lessons(&logic));
        });
    }

    // ── word-field-changed ────────────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_word_field_changed(
            move |lesson_idx, word_idx, spelling, kanji, meaning, word_type| {
                let ui = ui_weak.unwrap();
                let logic = ui.global::<VocabularyAppLogic>();
                if lesson_idx < 0 || word_idx < 0 {
                    return;
                }
                let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                    logic.get_lesson_list().iter().collect();
                if let Some(lesson) = lessons.get_mut(lesson_idx as usize) {
                    let mut words: Vec<vocabulary::VocabularyWordModel> =
                        lesson.words.iter().collect();
                    if let Some(word) = words.get_mut(word_idx as usize) {
                        word.spelling = spelling;
                        word.kanji = kanji;
                        word.meaning = meaning;
                        word.word_type = word_type;
                    }
                    lesson.words = slint::ModelRc::new(slint::VecModel::from(words));
                }
                logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
                save_vocabulary(&slint_to_lessons(&logic));
            },
        );
    }

    // ── word-tense-add-confirmed ──────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_word_tense_add_confirmed(move |lesson_idx, word_idx, tense_name, conjugation| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<VocabularyAppLogic>();
            if lesson_idx < 0 || word_idx < 0 {
                return;
            }
            let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                logic.get_lesson_list().iter().collect();
            if let Some(lesson) = lessons.get_mut(lesson_idx as usize) {
                let mut words: Vec<vocabulary::VocabularyWordModel> = lesson.words.iter().collect();
                if let Some(word) = words.get_mut(word_idx as usize) {
                    let mut tenses: Vec<vocabulary::TenseEntryModel> = word.tenses.iter().collect();
                    tenses.push(vocabulary::TenseEntryModel {
                        tense_name,
                        conjugation,
                    });
                    word.tenses = slint::ModelRc::new(slint::VecModel::from(tenses));
                }
                lesson.words = slint::ModelRc::new(slint::VecModel::from(words));
            }
            logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
            save_vocabulary(&slint_to_lessons(&logic));
        });
    }

    // ── word-tense-delete-confirmed ───────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_word_tense_delete_confirmed(move |lesson_idx, word_idx, tense_idx| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<VocabularyAppLogic>();
            if lesson_idx < 0 || word_idx < 0 || tense_idx < 0 {
                return;
            }
            let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                logic.get_lesson_list().iter().collect();
            if let Some(lesson) = lessons.get_mut(lesson_idx as usize) {
                let mut words: Vec<vocabulary::VocabularyWordModel> = lesson.words.iter().collect();
                if let Some(word) = words.get_mut(word_idx as usize) {
                    let mut tenses: Vec<vocabulary::TenseEntryModel> = word.tenses.iter().collect();
                    if (tense_idx as usize) < tenses.len() {
                        tenses.remove(tense_idx as usize);
                    }
                    word.tenses = slint::ModelRc::new(slint::VecModel::from(tenses));
                }
                lesson.words = slint::ModelRc::new(slint::VecModel::from(words));
            }
            logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
            save_vocabulary(&slint_to_lessons(&logic));
        });
    }

    // ── word-example-add-confirmed ────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_word_example_add_confirmed(move |lesson_idx, word_idx, example| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<VocabularyAppLogic>();
            if lesson_idx < 0 || word_idx < 0 {
                return;
            }
            let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                logic.get_lesson_list().iter().collect();
            if let Some(lesson) = lessons.get_mut(lesson_idx as usize) {
                let mut words: Vec<vocabulary::VocabularyWordModel> = lesson.words.iter().collect();
                if let Some(word) = words.get_mut(word_idx as usize) {
                    let mut examples: Vec<slint::SharedString> = word.examples.iter().collect();
                    examples.push(example);
                    word.examples = slint::ModelRc::new(slint::VecModel::from(examples));
                }
                lesson.words = slint::ModelRc::new(slint::VecModel::from(words));
            }
            logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
            save_vocabulary(&slint_to_lessons(&logic));
        });
    }

    // ── word-example-delete-confirmed ─────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_word_example_delete_confirmed(move |lesson_idx, word_idx, example_idx| {
            let ui = ui_weak.unwrap();
            let logic = ui.global::<VocabularyAppLogic>();
            if lesson_idx < 0 || word_idx < 0 || example_idx < 0 {
                return;
            }
            let mut lessons: Vec<vocabulary::VocabularyLessonModel> =
                logic.get_lesson_list().iter().collect();
            if let Some(lesson) = lessons.get_mut(lesson_idx as usize) {
                let mut words: Vec<vocabulary::VocabularyWordModel> = lesson.words.iter().collect();
                if let Some(word) = words.get_mut(word_idx as usize) {
                    let mut examples: Vec<slint::SharedString> = word.examples.iter().collect();
                    if (example_idx as usize) < examples.len() {
                        examples.remove(example_idx as usize);
                    }
                    word.examples = slint::ModelRc::new(slint::VecModel::from(examples));
                }
                lesson.words = slint::ModelRc::new(slint::VecModel::from(words));
            }
            logic.set_lesson_list(slint::ModelRc::new(slint::VecModel::from(lessons.clone())));
            save_vocabulary(&slint_to_lessons(&logic));
        });
    }

    // ── import-vocabulary-clicked ─────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_import_vocabulary_clicked(move || {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Markdown", &["md"])
                    .pick_file()
                {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        let lessons = vocabulary_markdown_io::parse_vocabulary(&content);
                        let ui = ui_weak.unwrap();
                        let logic = ui.global::<VocabularyAppLogic>();
                        logic.set_lesson_list(lessons_to_slint(&lessons));
                        save_vocabulary(&lessons);
                    }
                }
            }
        });
    }

    // ── export-vocabulary-clicked ─────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_export_vocabulary_clicked(move || {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let ui = ui_weak.unwrap();
                let logic = ui.global::<VocabularyAppLogic>();
                let lessons = slint_to_lessons(&logic);
                let content = vocabulary_markdown_io::serialize_vocabulary(&lessons);
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Markdown", &["md"])
                    .save_file()
                {
                    let _ = std::fs::write(path, content);
                }
            }
        });
    }

    // ── restore-defaults-clicked ──────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_restore_defaults_clicked(move || {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let defaults = load_and_save_defaults();
                let ui = ui_weak.unwrap();
                let logic = ui.global::<VocabularyAppLogic>();
                logic.set_lesson_list(lessons_to_slint(&defaults));
            }
        });
    }
}

/// Wire the `generate-exercises-clicked` callback.  Requires the host window to
/// also expose `FlashcardAppLogic` as a global (e.g. `MainWindow`).  Not called
/// from test windows that only need vocabulary CRUD.
pub fn init_exercise_generator<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> VocabularyAppLogic<'a>: slint::Global<'a, T>,
    for<'a> FlashcardAppLogic<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<VocabularyAppLogic>();

    // ── generate-exercises-clicked ────────────────────────────────────────────
    {
        let ui_weak = ui.as_weak();
        logic.on_generate_exercises_clicked(move || {
            let ui = ui_weak.unwrap();
            let vocab_logic = ui.global::<VocabularyAppLogic>();
            let flashcard_logic = ui.global::<FlashcardAppLogic>();

            // Snapshot stack names that exist before generation overwrites the list,
            // so we can name only the newly created stacks in the notification below.
            let existing_stack_names: std::collections::HashSet<String> = flashcard_logic
                .get_flashcard_list()
                .iter()
                .map(|stack| stack.stackname.to_string())
                .collect();

            // Convert Slint vocabulary models to libD input types.
            let lessons: Vec<VocabularyLesson> = vocab_logic
                .get_lesson_list()
                .iter()
                .map(|lesson| VocabularyLesson {
                    name: lesson.name.to_string(),
                    words: lesson
                        .words
                        .iter()
                        .map(|word| VocabularyWord {
                            spelling: word.spelling.to_string(),
                            kanji: if word.kanji.is_empty() {
                                None
                            } else {
                                Some(word.kanji.to_string())
                            },
                            meaning: word.meaning.to_string(),
                            word_type: if word.word_type.is_empty() {
                                None
                            } else {
                                Some(word.word_type.to_string())
                            },
                            tenses: word
                                .tenses
                                .iter()
                                .map(|t| TenseEntry {
                                    name: t.tense_name.to_string(),
                                    conjugation: t.conjugation.to_string(),
                                })
                                .collect(),
                            examples: word.examples.iter().map(|e| e.to_string()).collect(),
                        })
                        .collect(),
                })
                .collect();

            // Dispatch through the libD service — pure computation.
            let service = ExerciseGeneratorService;
            let output = ExerciseGeneratorFor::<VocabularyLesson>::generate(
                &service,
                ExerciseRequest::Flashcard,
                &lessons,
            );

            // Convert output to Slint types and update the flashcard global.
            if let Some(ExerciseOutput::Flashcard(stacks)) = output {
                let slint_stacks: Vec<flashcard::flashcard::FlashcardStackModel> = stacks
                    .iter()
                    .map(|stack| flashcard::flashcard::FlashcardStackModel {
                        stackname: stack.name.clone().into(),
                        flashcards: slint::ModelRc::new(slint::VecModel::from(
                            stack
                                .cards
                                .iter()
                                .map(|card| flashcard::flashcard::FlashcardModel {
                                    jap_obj: card.front.clone().into(),
                                    explanation: card.back.clone().into(),
                                    known: card.known,
                                    is_kanji: card.is_kanji,
                                })
                                .collect::<Vec<_>>(),
                        )),
                    })
                    .collect();

                // Name only the stacks that did not exist before this generation.
                let new_stack_names: Vec<String> = slint_stacks
                    .iter()
                    .map(|stack| stack.stackname.to_string())
                    .filter(|name| !existing_stack_names.contains(name))
                    .collect();

                flashcard_logic
                    .set_flashcard_list(slint::ModelRc::new(slint::VecModel::from(slint_stacks)));
                ::flashcard::save_current_list(&ui);

                if !new_stack_names.is_empty() {
                    let quoted: Vec<String> = new_stack_names
                        .iter()
                        .map(|name| format!("'{name}'"))
                        .collect();
                    let message = if quoted.len() == 1 {
                        format!("Flashcard stack {} generated.", quoted[0])
                    } else {
                        format!("Flashcard stacks {} generated.", quoted.join(", "))
                    };
                    vocab_logic.set_generation_notification(message.into());
                    vocab_logic.set_active_view(2);
                }
            }
        });
    }
}
