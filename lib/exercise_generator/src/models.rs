// ─── Source types ────────────────────────────────────────────────────────────

/// A vocabulary lesson containing a list of words to learn.
#[derive(Debug, Clone)]
pub struct VocabularyLesson {
    pub name: String,
    pub words: Vec<VocabularyWord>,
}

/// A single vocabulary word with optional kanji, conjugation tenses, and example sentences.
#[derive(Debug, Clone)]
pub struct VocabularyWord {
    pub spelling: String,      // hiragana / katakana / romaji
    pub kanji: Option<String>, // kanji form, if provided
    pub meaning: String,
    pub word_type: Option<String>, // noun, verb, adjective, …
    pub tenses: Vec<TenseEntry>,
    pub examples: Vec<ExampleEntry>,
}

/// A conjugation tense entry (e.g. name = "past", conjugation = "食べました").
#[derive(Debug, Clone)]
pub struct TenseEntry {
    pub name: String,
    pub conjugation: String,
}

/// An example sentence with its meaning (e.g. sentence = "犬が好きです",
/// meaning = "I like dogs").
#[derive(Debug, Clone)]
pub struct ExampleEntry {
    pub sentence: String,
    pub meaning: String,
}

// ─── Target types ────────────────────────────────────────────────────────────

/// A flashcard stack produced by transforming a `VocabularyLesson`.
#[derive(Debug, Clone)]
pub struct FlashcardStackData {
    pub name: String,
    pub cards: Vec<FlashcardCardData>,
}

/// A single flashcard with a front, a back, a known flag, and a kanji indicator.
///
/// `known` is always `false` on generation.
/// `front` is either the spelling or the kanji form — one card per form.
/// `is_kanji` is `true` when the front face uses the kanji form (triggers large calligraphic font).
#[derive(Debug, Clone)]
pub struct FlashcardCardData {
    pub front: String,  // kanji OR spelling — one card per form
    pub back: String,   // the word's meaning only
    pub known: bool,    // always false on generation
    pub is_kanji: bool, // true when front is kanji form — triggers calligraphic rendering
}

// future target type (not yet implemented):
// pub struct MatchingSet { pub pairs: Vec<(String, String)> }
