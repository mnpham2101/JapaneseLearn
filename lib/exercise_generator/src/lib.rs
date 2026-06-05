// Exercise generator service — pure Rust data transformation (libD).

pub mod models;
pub mod transformer;

pub use models::{
    FlashcardCardData, FlashcardStackData, TenseEntry, VocabularyLesson, VocabularyWord,
};
pub use transformer::{ExerciseOutput, ExerciseRequest, Transformer};
