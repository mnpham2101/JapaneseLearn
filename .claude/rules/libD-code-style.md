---
paths:
  - lib/exercise_generator/**/*.rs
---

# libD Pattern — Generic Data Transformation Service

A **libD** library is a pure Rust data transformation service. It defines a generic interface (`Transformer<S, T>`) that concrete classes implement, and a service dispatcher (`ExerciseGeneratorService`) that routes a runtime request to the correct transformer. The source database type and target database type are **fully abstract** — any source can be transformed into any target, and new transformations are added by implementing the trait in a new concrete struct.

## Purpose

This library implements the **lesson-database → exercise-database transformation pipeline**. Its single responsibility is converting raw lesson data (`VocabularyLesson`, and future grammar/reading lesson types) into structured exercise datasets (`FlashcardStackData`, and future exercise formats). It has no UI, no file I/O, and no platform calls.

## When to Use libD

Use the libD pattern when:
- The feature transforms a **source database** (e.g. vocabulary lessons) into a **target database** (e.g. exercise sets) on demand.
- Source and target databases must remain **decoupled** — conversion is triggered explicitly, not on every data change.
- The transformation is **pure computation** — no file I/O, no OS calls, no Slint types.
- Multiple **output formats** from the same source are likely (Open/Closed Principle: new format = new file, no existing code changed).

Do NOT use libD when:
- The library needs a Slint UI component → use **libA**.
- The library makes OS calls (file dialogs, TTS, file system) → use **libB**.
- The library only exports design tokens → use **libC**.
- The feature does not fit any catalogue entry → **consult the Planned Library Catalogue in `architecture.md`** before creating a new crate.

## SOLID Design Principles

| Principle | How libD satisfies it |
|---|---|
| **S** — Single Responsibility | Each concrete transformer struct does exactly one (S → T) conversion |
| **O** — Open/Closed | New output formats add a new struct + variant; existing transformers are never modified |
| **L** — Liskov Substitution | Any `impl Transformer<S, T>` can replace another with the same (S, T) |
| **I** — Interface Segregation | `Transformer<S, T>` exposes one method — no fat interface |
| **D** — Dependency Inversion | Callers (`lib/vocabulary`) depend on the `Transformer` trait, not on concrete structs |

## Folder Structure

```
lib/exercise_generator/        # libD: exercise transformation service
  src/
    lib.rs                     # re-exports: trait + service + all output types + all models
    models.rs                  # domain model structs — Slint-free; serde if needed
    transformer.rs             # Transformer<S, T> trait, ExerciseRequest enum, ExerciseOutput enum
    service.rs                 # ExerciseGeneratorService dispatcher
    flashcard_transformer.rs   # FlashcardExerciseTransformer: VocabularyLesson → FlashcardStackData
    # matching_transformer.rs  # (future) MatchingExerciseTransformer: VocabularyLesson → MatchingSet
  Cargo.toml                   # serde if needed; NO slint, NO slint-build, NO build.rs
```

**Prohibited in libD:**
- No `build.rs`
- No `ui/` directory
- No `slint` or `slint-build` in `Cargo.toml`
- No `init()` function
- No `std::fs` or platform `#[cfg]` guards — pure computation

## Cargo.toml

```toml
[package]
name = "exercise_generator"
version = "0.1.0"
edition = "2021"

[dependencies]
# serde only if domain models need (de)serialization
serde = { workspace = true, features = ["derive"] }
```

---

## Core Design — Four Components

### 1. Domain Models (`src/models.rs`)

Plain Rust structs with no Slint dependency. The calling libA handles all Slint ↔ Rust type conversion.

```rust
// src/models.rs

// ─── Source types ────────────────────────────────────────────────────────────

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
    pub back: String,     // formatted explanation
    pub known: bool,      // always false on generation
}

// future target type (not yet implemented):
// pub struct MatchingSet { pub pairs: Vec<(String, String)> }
```

---

### 2. Transformer Trait + Request/Output Abstraction (`src/transformer.rs`)

The **interface** that every concrete transformer must implement. `S` and `T` are fully generic — they can be any types.

`ExerciseRequest` selects which transformer to invoke at runtime. `ExerciseOutput` wraps all possible target types in one enum so the dispatcher can return a single type.

```rust
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
    // Matching,    // add when MatchingExerciseTransformer is implemented
    // FillBlank,   // add when FillBlankExerciseTransformer is implemented
}

// ─── Output wrapper ──────────────────────────────────────────────────────────

/// Wraps all possible exercise output types in a single return value.
/// The calling libA pattern-matches on this to extract the concrete type it needs.
pub enum ExerciseOutput {
    Flashcard(Vec<crate::models::FlashcardStackData>),
    // Matching(Vec<crate::models::MatchingSet>),   // add alongside the request variant
}
```

---

### 3. Service Dispatcher (`src/service.rs`)

The **service** that routes a runtime request to the right concrete transformer. It is implemented as a trait so any source type `S` can be supported by adding a new `impl` block.

```rust
// src/service.rs

use crate::models::{FlashcardStackData, VocabularyLesson};
use crate::transformer::{ExerciseOutput, ExerciseRequest, Transformer};
use crate::flashcard_transformer::FlashcardExerciseTransformer;

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
    fn generate(&self, request: ExerciseRequest, source: &[VocabularyLesson]) -> Option<ExerciseOutput> {
        match request {
            ExerciseRequest::Flashcard => Some(ExerciseOutput::Flashcard(
                FlashcardExerciseTransformer.transform(source),
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
```

---

### 4. Concrete Transformer (`src/flashcard_transformer.rs`)

A **concrete class** that implements the `Transformer<S, T>` interface for one specific (S, T) pair. The kanji duplication rule lives here.

```rust
// src/flashcard_transformer.rs

use crate::models::{FlashcardCardData, FlashcardStackData, VocabularyLesson, VocabularyWord};
use crate::transformer::Transformer;

/// Concrete transformer: VocabularyLesson → FlashcardStackData.
/// Kanji duplication rule: if a word has a kanji field, two cards are created —
/// one with spelling as front and one with kanji as front; both share the same back.
pub struct FlashcardExerciseTransformer;

impl Transformer<VocabularyLesson, FlashcardStackData> for FlashcardExerciseTransformer {
    fn transform(&self, lessons: &[VocabularyLesson]) -> Vec<FlashcardStackData> {
        lessons.iter().map(|lesson| FlashcardStackData {
            name: lesson.name.clone(),
            cards: lesson.words.iter()
                .flat_map(|word| make_cards(word))
                .collect(),
        }).collect()
    }
}

fn make_cards(word: &VocabularyWord) -> Vec<FlashcardCardData> {
    let back = format_explanation(word);
    let mut cards = vec![FlashcardCardData {
        front: word.spelling.clone(),
        back: back.clone(),
        known: false,
    }];
    if let Some(kanji) = &word.kanji {
        cards.push(FlashcardCardData {
            front: kanji.clone(),
            back,
            known: false,
        });
    }
    cards
}

fn format_explanation(word: &VocabularyWord) -> String {
    let mut parts = vec![word.meaning.clone()];
    if let Some(word_type) = &word.word_type {
        parts.push(format!("[{}]", word_type));
    }
    for tense in &word.tenses {
        parts.push(format!("{}: {}", tense.name, tense.conjugation));
    }
    for example in &word.examples {
        parts.push(format!("e.g. {}", example));
    }
    parts.join(" | ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{VocabularyLesson, VocabularyWord};

    fn word(spelling: &str, kanji: Option<&str>, meaning: &str) -> VocabularyWord {
        VocabularyWord {
            spelling: spelling.into(),
            kanji: kanji.map(Into::into),
            meaning: meaning.into(),
            word_type: None,
            tenses: vec![],
            examples: vec![],
        }
    }

    #[test]
    fn spelling_only_produces_one_card() {
        let lessons = vec![VocabularyLesson {
            name: "L1".into(),
            words: vec![word("inu", None, "dog")],
        }];
        let stacks = FlashcardExerciseTransformer.transform(&lessons);
        assert_eq!(stacks[0].cards.len(), 1);
        assert_eq!(stacks[0].cards[0].front, "inu");
    }

    #[test]
    fn kanji_word_produces_two_cards_sharing_same_back() {
        let lessons = vec![VocabularyLesson {
            name: "L1".into(),
            words: vec![word("いぬ", Some("犬"), "dog")],
        }];
        let stacks = FlashcardExerciseTransformer.transform(&lessons);
        assert_eq!(stacks[0].cards.len(), 2);
        assert_eq!(stacks[0].cards[0].front, "いぬ");
        assert_eq!(stacks[0].cards[1].front, "犬");
        assert_eq!(stacks[0].cards[0].back, stacks[0].cards[1].back);
    }

    #[test]
    fn each_lesson_becomes_one_stack() {
        let lessons = vec![
            VocabularyLesson { name: "L1".into(), words: vec![word("a", None, "a")] },
            VocabularyLesson { name: "L2".into(), words: vec![word("b", None, "b")] },
        ];
        let stacks = FlashcardExerciseTransformer.transform(&lessons);
        assert_eq!(stacks.len(), 2);
        assert_eq!(stacks[0].name, "L1");
        assert_eq!(stacks[1].name, "L2");
    }

    #[test]
    fn empty_lesson_list_produces_no_stacks() {
        let stacks = FlashcardExerciseTransformer.transform(&[]);
        assert!(stacks.is_empty());
    }
}
```

---

### 5. Public API (`src/lib.rs`)

```rust
// src/lib.rs

pub mod flashcard_transformer;
pub mod models;
pub mod service;
pub mod transformer;

pub use models::{
    FlashcardCardData, FlashcardStackData, TenseEntry, VocabularyLesson, VocabularyWord,
};
pub use service::{ExerciseGeneratorFor, ExerciseGeneratorService};
pub use transformer::{ExerciseOutput, ExerciseRequest, Transformer};
```

---

## How to Extend — Adding a New Exercise Type

**Scenario**: add a `Matching` exercise type that pairs vocabulary word fronts with backs.

**Steps** (touch only new files; existing code is not modified):

1. **Add target model** to `src/models.rs`:
   ```rust
   pub struct MatchingSet {
       pub stack_name: String,
       pub pairs: Vec<(String, String)>,  // (front, back)
   }
   ```

2. **Add concrete transformer** in new file `src/matching_transformer.rs`:
   ```rust
   pub struct MatchingExerciseTransformer;
   impl Transformer<VocabularyLesson, MatchingSet> for MatchingExerciseTransformer { ... }
   ```

3. **Add request variant** to `ExerciseRequest` in `src/transformer.rs`:
   ```rust
   pub enum ExerciseRequest {
       Flashcard,
       Matching,   // ← new
   }
   ```

4. **Add output variant** to `ExerciseOutput` in `src/transformer.rs`:
   ```rust
   pub enum ExerciseOutput {
       Flashcard(Vec<FlashcardStackData>),
       Matching(Vec<MatchingSet>),          // ← new
   }
   ```

5. **Add match arm** in `src/service.rs` `impl ExerciseGeneratorFor<VocabularyLesson>`:
   ```rust
   ExerciseRequest::Matching => Some(ExerciseOutput::Matching(
       MatchingExerciseTransformer.transform(source),
   )),
   ```

> `FlashcardExerciseTransformer` and its tests are never touched.

---

## Integration Pattern — How libA Calls libD

libD is **called from a libA `init()` handler**. All Slint ↔ Rust type conversions happen in the calling libA (`lib/vocabulary/src/lib.rs`). libD never sees Slint types.

```rust
// lib/vocabulary/src/lib.rs

use exercise_generator::{
    ExerciseGeneratorFor, ExerciseGeneratorService,
    ExerciseOutput, ExerciseRequest, VocabularyLesson,
};

logic.on_generate_exercises_clicked({
    let exercise_type = ExerciseRequest::Flashcard;   // set by the UI callback argument
    move || {
        let ui = ui_weak.unwrap();
        let vocab_logic = ui.global::<VocabularyAppLogic>();
        let flashcard_logic = ui.global::<FlashcardAppLogic>();

        // 1. Convert Slint vocabulary models → libD plain Rust input structs
        let lessons: Vec<VocabularyLesson> = slint_to_vocabulary_lessons(&vocab_logic);

        // 2. Dispatch to the right transformer — pure computation
        let service = ExerciseGeneratorService;
        let output = service.generate(exercise_type, &lessons);

        // 3. Convert output → Slint types and update flashcard lib
        if let Some(ExerciseOutput::Flashcard(stacks)) = output {
            let slint_stacks = flashcard_stacks_to_slint(&stacks);
            flashcard_logic.set_flashcard_list(slint_stacks.into());
            save_stacks(&stacks);
        }
    }
});
```

**Call flow summary:**
```
UI callback (on_generate_exercises_clicked)
  → lib/vocabulary init() handler
    → slint_to_vocabulary_lessons()       [Slint → plain Rust]
    → ExerciseGeneratorService::generate()  [libD: pure computation]
    → flashcard_stacks_to_slint()          [plain Rust → Slint]
    → flashcard_logic.set_flashcard_list() [Slint model update]
    → save_stacks()                        [persistence]
```

---

## Task-Manager Assignment Rules

> **All libD tasks are assigned exclusively to `rust-developer`.** libD has no `.slint` files, no `build.rs`, and no `init()`. Do not assign libD tasks to `slint-developer` or `slint-tester`.
>
> The `slint-developer` is responsible for the calling libA's Slint components (vocabulary lesson form, callback declarations). The `rust-developer` is responsible for:
> - The entire `lib/exercise_generator` crate (all libD files)
> - The callback handler in `lib/vocabulary/src/lib.rs` that calls the service

## Running libD Tests

```powershell
cargo test -p exercise_generator
```

No `slint::testing::init()` needed. Tests are plain Rust `#[cfg(test)]` blocks.
