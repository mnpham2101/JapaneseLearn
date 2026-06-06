use crate::{LessonData, TenseData, WordData};

/// Parse a vocabulary markdown string into a list of [`LessonData`].
///
/// Format expected:
/// ```text
/// ## Lesson Name
///
/// ### spelling
/// kanji: 犬
/// meaning: dog
/// type: noun
/// tense: past|食べました
/// example: 犬が走る。
/// ```
pub(crate) fn parse_vocabulary(source: &str) -> Vec<LessonData> {
    let mut lessons: Vec<LessonData> = Vec::new();
    let mut current_word: Option<WordData> = None;

    for raw_line in source.lines() {
        let line = raw_line.trim();
        if let Some(lesson_name) = line.strip_prefix("## ") {
            // Flush any pending word before starting a new lesson.
            if let Some(word) = current_word.take() {
                if let Some(lesson) = lessons.last_mut() {
                    lesson.words.push(word);
                }
            }
            lessons.push(LessonData {
                name: lesson_name.trim().to_string(),
                words: vec![],
            });
        } else if let Some(spelling) = line.strip_prefix("### ") {
            // Flush any pending word before starting a new word entry.
            if let Some(word) = current_word.take() {
                if let Some(lesson) = lessons.last_mut() {
                    lesson.words.push(word);
                }
            }
            current_word = Some(WordData {
                spelling: spelling.trim().to_string(),
                ..Default::default()
            });
        } else if let Some(word) = current_word.as_mut() {
            if let Some(val) = line.strip_prefix("kanji: ") {
                word.kanji = val.to_string();
            } else if let Some(val) = line.strip_prefix("meaning: ") {
                word.meaning = val.to_string();
            } else if let Some(val) = line.strip_prefix("type: ") {
                word.word_type = val.to_string();
            } else if let Some(val) = line.strip_prefix("tense: ") {
                if let Some((name, conjugation)) = val.split_once('|') {
                    word.tenses.push(TenseData {
                        name: name.to_string(),
                        conjugation: conjugation.to_string(),
                    });
                }
            } else if let Some(val) = line.strip_prefix("example: ") {
                word.examples.push(val.to_string());
            }
        }
    }

    // Flush the final word.
    if let Some(word) = current_word {
        if let Some(lesson) = lessons.last_mut() {
            lesson.words.push(word);
        }
    }

    lessons
}

/// Serialize a list of [`LessonData`] to a vocabulary markdown string.
pub(crate) fn serialize_vocabulary(lessons: &[LessonData]) -> String {
    let mut out = String::new();
    for lesson in lessons {
        out.push_str(&format!("## {}\n\n", lesson.name));
        for word in &lesson.words {
            out.push_str(&format!("### {}\n", word.spelling));
            if !word.kanji.is_empty() {
                out.push_str(&format!("kanji: {}\n", word.kanji));
            }
            out.push_str(&format!("meaning: {}\n", word.meaning));
            if !word.word_type.is_empty() {
                out.push_str(&format!("type: {}\n", word.word_type));
            }
            for tense in &word.tenses {
                out.push_str(&format!("tense: {}|{}\n", tense.name, tense.conjugation));
            }
            for example in &word.examples {
                out.push_str(&format!("example: {}\n", example));
            }
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_lesson_one_word_no_kanji() {
        let src = "## Greetings\n\n### こんにちは\nmeaning: Hello\n";
        let lessons = parse_vocabulary(src);
        assert_eq!(lessons.len(), 1);
        assert_eq!(lessons[0].name, "Greetings");
        assert_eq!(lessons[0].words.len(), 1);
        let word = &lessons[0].words[0];
        assert_eq!(word.spelling, "こんにちは");
        assert_eq!(word.meaning, "Hello");
        assert!(word.kanji.is_empty());
    }

    #[test]
    fn parse_word_with_kanji() {
        let src = "## Animals\n\n### いぬ\nkanji: 犬\nmeaning: dog\ntype: noun\n";
        let lessons = parse_vocabulary(src);
        assert_eq!(lessons.len(), 1);
        let word = &lessons[0].words[0];
        assert_eq!(word.spelling, "いぬ");
        assert_eq!(word.kanji, "犬");
        assert_eq!(word.meaning, "dog");
        assert_eq!(word.word_type, "noun");
    }

    #[test]
    fn parse_word_with_tenses_and_examples() {
        let src = "## Verbs\n\n### たべる\nmeaning: to eat\ntense: past|たべました\ntense: negative|たべません\nexample: 犬が走る。\nexample: その犬は大きい。\n";
        let lessons = parse_vocabulary(src);
        assert_eq!(lessons.len(), 1);
        let word = &lessons[0].words[0];
        assert_eq!(word.tenses.len(), 2);
        assert_eq!(word.tenses[0].name, "past");
        assert_eq!(word.tenses[0].conjugation, "たべました");
        assert_eq!(word.tenses[1].name, "negative");
        assert_eq!(word.tenses[1].conjugation, "たべません");
        assert_eq!(word.examples.len(), 2);
        assert_eq!(word.examples[0], "犬が走る。");
        assert_eq!(word.examples[1], "その犬は大きい。");
    }

    #[test]
    fn parse_two_lessons() {
        let src = "## Lesson A\n\n### あ\nmeaning: A\n\n## Lesson B\n\n### い\nmeaning: I\n";
        let lessons = parse_vocabulary(src);
        assert_eq!(lessons.len(), 2);
        assert_eq!(lessons[0].name, "Lesson A");
        assert_eq!(lessons[0].words[0].spelling, "あ");
        assert_eq!(lessons[1].name, "Lesson B");
        assert_eq!(lessons[1].words[0].spelling, "い");
    }

    #[test]
    fn parse_empty_input_returns_empty_vec() {
        let lessons = parse_vocabulary("");
        assert!(lessons.is_empty());
    }
}
