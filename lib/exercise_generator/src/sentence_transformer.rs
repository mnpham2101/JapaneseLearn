// src/sentence_transformer.rs

use crate::models::{FlashcardCardData, FlashcardStackData, VocabularyLesson, VocabularyWord};
use crate::transformer::Transformer;

/// Concrete transformer: VocabularyLesson → FlashcardStackData, one card per
/// example sentence. Reuses the same FlashcardStackData/FlashcardCardData
/// shape as FlashcardExerciseTransformer; front = sentence, back = meaning.
pub struct SentenceExerciseTransformer;

impl Transformer<VocabularyLesson, FlashcardStackData> for SentenceExerciseTransformer {
    fn transform(&self, lessons: &[VocabularyLesson]) -> Vec<FlashcardStackData> {
        lessons
            .iter()
            .map(|lesson| FlashcardStackData {
                name: lesson.name.clone(),
                cards: lesson.words.iter().flat_map(make_sentence_cards).collect(),
            })
            .collect()
    }
}

fn make_sentence_cards(word: &VocabularyWord) -> Vec<FlashcardCardData> {
    word.examples
        .iter()
        .filter(|example| !example.sentence.trim().is_empty())
        .map(|example| FlashcardCardData {
            front: example.sentence.clone(),
            back: example.meaning.clone(),
            known: false,
            is_kanji: false,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ExampleEntry, VocabularyLesson, VocabularyWord};

    fn word_with_examples(spelling: &str, examples: Vec<(&str, &str)>) -> VocabularyWord {
        VocabularyWord {
            spelling: spelling.into(),
            kanji: None,
            meaning: "meaning".into(),
            word_type: Some("noun".into()),
            tenses: vec![],
            examples: examples
                .into_iter()
                .map(|(sentence, meaning)| ExampleEntry {
                    sentence: sentence.into(),
                    meaning: meaning.into(),
                })
                .collect(),
        }
    }

    #[test]
    fn word_with_two_examples_produces_two_cards() {
        let lessons = vec![VocabularyLesson {
            name: "L1".into(),
            words: vec![word_with_examples(
                "いぬ",
                vec![
                    ("犬が好きです", "I like dogs"),
                    ("犬が走ります", "The dog runs"),
                ],
            )],
        }];
        let stacks = SentenceExerciseTransformer.transform(&lessons);
        assert_eq!(stacks[0].cards.len(), 2);
        assert_eq!(stacks[0].cards[0].front, "犬が好きです");
        assert_eq!(stacks[0].cards[0].back, "I like dogs");
        assert_eq!(stacks[0].cards[1].front, "犬が走ります");
        assert_eq!(stacks[0].cards[1].back, "The dog runs");
    }

    #[test]
    fn word_with_no_examples_produces_no_cards() {
        let lessons = vec![VocabularyLesson {
            name: "L1".into(),
            words: vec![word_with_examples("いぬ", vec![])],
        }];
        let stacks = SentenceExerciseTransformer.transform(&lessons);
        assert!(stacks[0].cards.is_empty());
    }

    #[test]
    fn word_with_blank_sentence_example_produces_no_card_for_that_entry() {
        let lessons = vec![VocabularyLesson {
            name: "L1".into(),
            words: vec![word_with_examples(
                "いぬ",
                vec![("   ", "blank sentence"), ("犬が好きです", "I like dogs")],
            )],
        }];
        let stacks = SentenceExerciseTransformer.transform(&lessons);
        assert_eq!(stacks[0].cards.len(), 1);
        assert_eq!(stacks[0].cards[0].front, "犬が好きです");
        assert_eq!(stacks[0].cards[0].back, "I like dogs");
    }

    #[test]
    fn lesson_with_multiple_words_aggregates_all_example_cards_into_one_stack() {
        let lessons = vec![VocabularyLesson {
            name: "L1".into(),
            words: vec![
                word_with_examples("いぬ", vec![("犬が好きです", "I like dogs")]),
                word_with_examples("ねこ", vec![("猫が好きです", "I like cats")]),
            ],
        }];
        let stacks = SentenceExerciseTransformer.transform(&lessons);
        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].cards.len(), 2);
        assert_eq!(stacks[0].cards[0].front, "犬が好きです");
        assert_eq!(stacks[0].cards[1].front, "猫が好きです");
    }

    #[test]
    fn empty_lesson_list_produces_no_stacks() {
        let stacks = SentenceExerciseTransformer.transform(&[]);
        assert!(stacks.is_empty());
    }
}
