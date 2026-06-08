// src/flashcard_transformer.rs

use crate::models::{FlashcardCardData, FlashcardStackData, VocabularyLesson, VocabularyWord};
use crate::transformer::Transformer;

/// Concrete transformer: VocabularyLesson → FlashcardStackData.
/// Kanji duplication rule: if a word has a kanji field, two cards are created —
/// one with spelling as front and one with kanji as front; both share the same back.
pub struct FlashcardExerciseTransformer;

impl Transformer<VocabularyLesson, FlashcardStackData> for FlashcardExerciseTransformer {
    fn transform(&self, lessons: &[VocabularyLesson]) -> Vec<FlashcardStackData> {
        lessons
            .iter()
            .map(|lesson| FlashcardStackData {
                name: lesson.name.clone(),
                cards: lesson.words.iter().flat_map(make_cards).collect(),
            })
            .collect()
    }
}

fn make_cards(word: &VocabularyWord) -> Vec<FlashcardCardData> {
    let back = word.meaning.clone();
    let mut cards = vec![FlashcardCardData {
        front: word.spelling.clone(),
        back: back.clone(),
        known: false,
        is_kanji: false, // spelling front — not kanji
    }];
    if let Some(kanji) = &word.kanji {
        cards.push(FlashcardCardData {
            front: kanji.clone(),
            back,
            known: false,
            is_kanji: true, // kanji front — large brush font
        });
    }
    cards
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{TenseEntry, VocabularyLesson, VocabularyWord};

    fn word(spelling: &str, kanji: Option<&str>, meaning: &str) -> VocabularyWord {
        VocabularyWord {
            spelling: spelling.into(),
            kanji: kanji.map(Into::into),
            meaning: meaning.into(),
            word_type: Some("noun".into()),
            tenses: vec![TenseEntry {
                name: "past".into(),
                conjugation: "食べました".into(),
            }],
            examples: vec!["犬が好きです".into()],
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
        assert_eq!(stacks[0].cards[0].back, "dog");
        assert!(!stacks[0].cards[0].is_kanji);
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
        assert_eq!(stacks[0].cards[0].back, "dog");
        assert_eq!(stacks[0].cards[1].back, "dog");
        assert_eq!(stacks[0].cards[0].back, stacks[0].cards[1].back);
        assert!(!stacks[0].cards[0].back.contains('['));
        assert!(!stacks[0].cards[0].back.contains("e.g."));
        assert!(!stacks[0].cards[0].is_kanji); // spelling card
        assert!(stacks[0].cards[1].is_kanji); // kanji card
    }

    #[test]
    fn each_lesson_becomes_one_stack() {
        let lessons = vec![
            VocabularyLesson {
                name: "L1".into(),
                words: vec![word("a", None, "a")],
            },
            VocabularyLesson {
                name: "L2".into(),
                words: vec![word("b", None, "b")],
            },
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
