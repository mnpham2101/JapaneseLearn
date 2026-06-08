# Description:

The `Flashcard's back should only show the meaning of the word without the remaining data. Need to generate callflow diagram to illustrate the following use case after bug fix.
1. the Exercise Generation request
2. App start up and default data is load
3. New Flashcard is added

# Suggested fix:
The following structs have been defined to match the `Vocabulary.rs`. `FlashcardCardData` should simply use only meaning field from `VocabularyWord` 
```Rust
#[derive(Debug, Clone)]
pub struct VocabularyLesson {
    pub name: String,
    pub words: Vec<VocabularyWord>,
}

#[derive(Debug, Clone)]
pub struct VocabularyWord {
    pub spelling: String,          // hiragana / katakana / romaji
    pub kanji: Option<String>,     // kanji form, if provided
    pub meaning: String,
    pub word_type: Option<String>, // noun, verb, adjective, …
    pub tenses: Vec<TenseEntry>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TenseEntry {
    pub name: String,          // e.g. "past", "negative"
    pub conjugation: String,   // e.g. "食べました"
}

// ─── Target types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FlashcardStackData {
    pub name: String,
    pub cards: Vec<FlashcardCardData>,
}

#[derive(Debug, Clone)]
pub struct FlashcardCardData {
    pub front: String,    // kanji OR spelling — one card per form
    pub back: String,     // formatted explanation that includes only meaning!!!                 
    pub known: bool,      // always false on generation
}
```

# Confirmed findings

## Symptom
Flashcards generated from a vocabulary lesson show, on the back, the meaning
concatenated with word type, tense conjugations, and example sentences (e.g.
`"dog [noun] | past: 食べました | e.g. 犬が好きです"`) instead of just the meaning
(`"dog"`).

## Root cause
`format_explanation()` in
[lib/exercise_generator/src/flashcard_transformer.rs:42-54](../../lib/exercise_generator/src/flashcard_transformer.rs#L42-L54)
builds the `back` string by joining `meaning`, `[word_type]`, every `tenses`
entry, and every `examples` entry with `" | "`:

```rust
fn format_explanation(word: &VocabularyWord) -> String {
    let mut parts = vec![word.meaning.clone()];
    if let Some(word_type) = &word.word_type { parts.push(format!("[{}]", word_type)); }
    for tense in &word.tenses { parts.push(format!("{}: {}", tense.name, tense.conjugation)); }
    for example in &word.examples { parts.push(format!("e.g. {}", example)); }
    parts.join(" | ")
}
```

This is the only place in the codebase that derives `FlashcardCardData::back`
from a `VocabularyWord` — confirmed via `grep` for `FlashcardCardData` /
`format_explanation` / `FlashcardStackData` across `lib/` and `src/`.

## Scope confirmation (the three flows named in the description)
1. **Exercise Generation request** — `lib/vocabulary/src/lib.rs`
   `on_generate_exercises_clicked` → `ExerciseGeneratorService::generate` →
   `FlashcardExerciseTransformer::transform` → `make_cards` /
   `format_explanation`. **This is the buggy path.**
2. **App start-up / default data load** — `flashcard::init()` →
   `load_stacks()` ([lib/flashcard/src/lib.rs:91](../../lib/flashcard/src/lib.rs#L91))
   reads persisted `stacks.json`. The `back` values it loads were themselves
   produced by the same transformer and persisted by `save_stacks`, so fixing
   the transformer fixes all future generations (existing `stacks.json`
   entries keep their old verbose `back` until regenerated — out of scope for
   a code fix).
3. **New Flashcard is added** — `on_flashcard_add_confirmed`
   ([lib/flashcard/src/lib.rs:155](../../lib/flashcard/src/lib.rs#L155)) stores
   the user-typed `meaning` directly as `explanation` — not affected.

## Confirmed by user
User confirmed root cause and scope on 2026-06-08.

