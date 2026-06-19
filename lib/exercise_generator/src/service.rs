// src/service.rs

use crate::flashcard_transformer::FlashcardExerciseTransformer;
use crate::models::VocabularyLesson;
use crate::sentence_transformer::SentenceExerciseTransformer;
use crate::transformer::{ExerciseOutput, ExerciseRequest, Transformer};

/// Dispatcher trait — implemented once per source type S.
/// To support a new source database type, add a new `impl` block.
pub trait ExerciseGeneratorFor<S> {
    /// Select the concrete transformer for `request`, run it over `source`,
    /// and return the wrapped output. Returns `None` if the request type
    /// is not yet supported for this source.
    fn generate(&self, request: ExerciseRequest, source: &[S]) -> Option<ExerciseOutput>;
}

/// Stateless service struct. All logic lives in the `impl` blocks below.
pub struct ExerciseGeneratorService;

// ─── VocabularyLesson → * ────────────────────────────────────────────────────

impl ExerciseGeneratorFor<VocabularyLesson> for ExerciseGeneratorService {
    fn generate(
        &self,
        request: ExerciseRequest,
        source: &[VocabularyLesson],
    ) -> Option<ExerciseOutput> {
        match request {
            ExerciseRequest::Flashcard => Some(ExerciseOutput::Flashcard(
                FlashcardExerciseTransformer.transform(source),
            )),
            ExerciseRequest::Sentence => Some(ExerciseOutput::Sentence(
                SentenceExerciseTransformer.transform(source),
            )),
            // ExerciseRequest::Matching => Some(ExerciseOutput::Matching(
            //     MatchingExerciseTransformer.transform(source),
            // )),
        }
    }
}

// ─── Future: GrammarLesson → * ───────────────────────────────────────────────
// impl ExerciseGeneratorFor<GrammarLesson> for ExerciseGeneratorService {
//     fn generate(&self, request: ExerciseRequest, source: &[GrammarLesson]) -> Option<ExerciseOutput> {
//         match request {
//             ExerciseRequest::Matching => Some(ExerciseOutput::Matching(
//                 GrammarMatchingTransformer.transform(source),
//             )),
//             _ => None,
//         }
//     }
// }
