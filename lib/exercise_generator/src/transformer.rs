// src/transformer.rs

// ─── Interface ───────────────────────────────────────────────────────────────

/// Generic transformation contract.
/// `S` = source element type (e.g. VocabularyLesson, GrammarLesson — any lesson database).
/// `T` = target element type (e.g. FlashcardStackData, MatchingSet — any exercise database).
///
/// Concrete transformers are stateless structs. One struct per (S, T) pair.
pub trait Transformer<S, T> {
    /// Transform a slice of source records into a collection of target records.
    /// Pure computation — no I/O, no side effects.
    fn transform(&self, source: &[S]) -> Vec<T>;
}

// ─── Request type ────────────────────────────────────────────────────────────

/// Identifies which exercise database to generate.
/// Extend by adding a new variant and a new concrete Transformer struct.
/// Existing transformers and match arms are never modified (Open/Closed Principle).
#[non_exhaustive]
pub enum ExerciseRequest {
    Flashcard,
    Sentence,
    // Matching,    // add when MatchingExerciseTransformer is implemented
    // FillBlank,   // add when FillBlankExerciseTransformer is implemented
}

// ─── Output wrapper ──────────────────────────────────────────────────────────

/// Wraps all possible exercise output types in a single return value.
/// The calling libA pattern-matches on this to extract the concrete type it needs.
pub enum ExerciseOutput {
    Flashcard(Vec<crate::models::FlashcardStackData>),
    Sentence(Vec<crate::models::FlashcardStackData>),
    // Matching(Vec<crate::models::MatchingSet>),   // add alongside the request variant
}
