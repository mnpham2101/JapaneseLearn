use slint::Model;

pub mod vocabulary {
    slint::include_modules!();
}

use vocabulary::VocabularyAppLogic;

// ── Shadow structs (serde-serialisable mirror of the Slint model) ─────────────

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
struct TenseData {
    name: String,
    conjugation: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
struct WordData {
    spelling: String,
    kanji: String,
    meaning: String,
    word_type: String,
    tenses: Vec<TenseData>,
    examples: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default)]
struct LessonData {
    name: String,
    words: Vec<WordData>,
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

const VOCABULARY_FILE: &str = "vocabulary.json";

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
        let _ = std::fs::write(VOCABULARY_FILE, json);
    }
}

#[cfg(target_arch = "wasm32")]
fn save_vocabulary(_lessons: &[LessonData]) {}

// ── init ──────────────────────────────────────────────────────────────────────

pub fn init<T>(ui: &T)
where
    T: slint::ComponentHandle + 'static,
    for<'a> VocabularyAppLogic<'a>: slint::Global<'a, T>,
{
    let logic = ui.global::<VocabularyAppLogic>();

    // Restore persisted lessons on startup (no-op on WASM).
    let saved = load_vocabulary();
    if !saved.is_empty() {
        logic.set_lesson_list(lessons_to_slint(&saved));
    }

    // CRUD handlers will be wired in tasks 6.9.2–6.9.3
}
