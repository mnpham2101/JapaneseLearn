// Exercise generator service — pure Rust data transformation (libD).

pub mod flashcard_transformer;
pub mod models;
pub mod service;
pub mod transformer;

pub use models::{
    FlashcardCardData, FlashcardStackData, TenseEntry, VocabularyLesson, VocabularyWord,
};
pub use service::{ExerciseGeneratorFor, ExerciseGeneratorService};
pub use transformer::{ExerciseOutput, ExerciseRequest, Transformer};
