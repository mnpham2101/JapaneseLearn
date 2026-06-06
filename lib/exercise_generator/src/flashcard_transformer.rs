// src/flashcard_transformer.rs — stub (full implementation in task 6.5)

use crate::models::{FlashcardStackData, VocabularyLesson};
use crate::transformer::Transformer;

/// Concrete transformer: VocabularyLesson → FlashcardStackData.
/// This is a stub implementation — the full logic is added in task 6.5.
pub struct FlashcardExerciseTransformer;

impl Transformer<VocabularyLesson, FlashcardStackData> for FlashcardExerciseTransformer {
    fn transform(&self, _source: &[VocabularyLesson]) -> Vec<FlashcardStackData> {
        vec![]
    }
}
