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
/// tense: past : 食べました
/// example: 犬が走る。
/// ```
/// Strip a heading marker (e.g. `"##"` or `"###"`) from the start of a line.
///
/// Tolerates any whitespace between the marker and the heading text —
/// including the full-width ideographic space (U+3000) that IME input
/// sometimes inserts instead of an ASCII space. Returns `None` if `line`
/// does not start with `marker`, if it is actually a deeper heading (e.g.
/// `marker` is `"##"` but the line starts with `"###"`), or if no heading
/// text follows.
fn strip_heading(line: &str, marker: &str) -> Option<String> {
    let rest = line.strip_prefix(marker)?;
    if rest.starts_with('#') {
        return None;
    }
    let text = rest.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

/// Split a `key: value` field line into its key and value.
///
/// Tolerates a space before the colon (`"type : noun"`) and a full-width
/// colon (`"kanji：庭"`) — both of which appear in hand-edited markdown
/// alongside the standard `"key: value"` form.
fn split_field(line: &str) -> Option<(&str, String)> {
    let idx = line.find([':', '：'])?;
    let key = line[..idx].trim();
    let colon_len = line[idx..].chars().next()?.len_utf8();
    let value = line[idx + colon_len..].trim().to_string();
    Some((key, value))
}

/// Split a tense field's value (`"<label> : <conjugation>"`) on its own
/// first colon, tolerating the same conventions as [`split_field`]: a space
/// before the colon and the full-width colon (`：`).
///
/// Returns `None` if there is no second colon, or if the conjugation half
/// is empty after trimming — matching the documented rule that a `tense`
/// line without a spelling does not produce a new word.
fn split_tense_value(value: &str) -> Option<(String, String)> {
    let idx = value.find([':', '：'])?;
    let name = value[..idx].trim();
    let colon_len = value[idx..].chars().next()?.len_utf8();
    let conjugation = value[idx + colon_len..].trim();
    if conjugation.is_empty() {
        None
    } else {
        Some((name.to_string(), conjugation.to_string()))
    }
}

pub(crate) fn parse_vocabulary(source: &str) -> Vec<LessonData> {
    let mut lessons: Vec<LessonData> = Vec::new();
    let mut current_word: Option<WordData> = None;

    for raw_line in source.lines() {
        let line = raw_line.trim();
        if let Some(spelling) = strip_heading(line, "###") {
            // Flush any pending word before starting a new word entry.
            if let Some(word) = current_word.take() {
                if let Some(lesson) = lessons.last_mut() {
                    lesson.words.push(word);
                }
            }
            current_word = Some(WordData {
                spelling,
                ..Default::default()
            });
        } else if let Some(lesson_name) = strip_heading(line, "##") {
            // Flush any pending word before starting a new lesson.
            if let Some(word) = current_word.take() {
                if let Some(lesson) = lessons.last_mut() {
                    lesson.words.push(word);
                }
            }
            lessons.push(LessonData {
                name: lesson_name,
                words: vec![],
            });
        } else if let Some(word) = current_word.as_mut() {
            if let Some((key, value)) = split_field(line) {
                match key {
                    "kanji" => word.kanji = value,
                    "meaning" => word.meaning = value,
                    "type" => word.word_type = value,
                    "tense" => {
                        if let Some((name, conjugation)) = split_tense_value(&value) {
                            word.tenses.push(TenseData { name, conjugation });
                        }
                    }
                    "example" if !value.is_empty() => {
                        word.examples.push(value);
                    }
                    _ => {}
                }
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
                out.push_str(&format!("tense: {} : {}\n", tense.name, tense.conjugation));
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
        let src = "## Verbs\n\n### たべる\nmeaning: to eat\ntense: past : たべました\ntense: negative : たべません\nexample: 犬が走る。\nexample: その犬は大きい。\n";
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

    /// Reproduces the literal bundled-data pattern from `extended-vocab.md`,
    /// which separates the tense label from its conjugation with a second
    /// colon (e.g. `tense: present-formal : たべます`), not a pipe.
    #[test]
    fn parse_tense_with_bundled_data_colon_format() {
        let src = "## Verbs\n\n### たべる\nmeaning: to eat\ntense: present-formal : たべます\n";
        let lessons = parse_vocabulary(src);
        let word = &lessons[0].words[0];
        assert_eq!(word.tenses.len(), 1);
        assert_eq!(word.tenses[0].name, "present-formal");
        assert_eq!(word.tenses[0].conjugation, "たべます");
    }

    /// Per extended-vocab.md: "any `tense` that doesn't have the spelling
    /// will not provide a new word." A tense line whose conjugation half is
    /// empty (e.g. `tense: negative-present-formal:` — no space before the
    /// trailing colon, nothing after it) must not produce a `TenseData`.
    #[test]
    fn parse_tense_with_empty_conjugation_is_skipped() {
        let src = "## Verbs\n\n### たべる\nmeaning: to eat\ntense: negative-present-formal:\n";
        let lessons = parse_vocabulary(src);
        let word = &lessons[0].words[0];
        assert!(word.tenses.is_empty());
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

    /// Reproduces the せんせい/きょうじゅ bug: a heading using a full-width
    /// space (U+3000) after `###` must still start a new word, not be
    /// swallowed into the previous word's fields.
    #[test]
    fn parse_heading_with_fullwidth_space() {
        let src = "## People\n\n### せんせい\nkanji: 先生\nmeaning: teacher\ntype: noun\n\n###　きょうじゅ\nkanji:　教授\nmeaning: professor\ntype: noun\n";
        let lessons = parse_vocabulary(src);
        assert_eq!(lessons[0].words.len(), 2);
        assert_eq!(lessons[0].words[0].spelling, "せんせい");
        assert_eq!(lessons[0].words[0].meaning, "teacher");
        assert_eq!(lessons[0].words[1].spelling, "きょうじゅ");
        assert_eq!(lessons[0].words[1].kanji, "教授");
        assert_eq!(lessons[0].words[1].meaning, "professor");
    }

    #[test]
    fn parse_field_with_space_before_colon() {
        let src = "## Time\n\n### あさ\nkanji :　朝\nmeaning :  morning\ntype : noun\n";
        let lessons = parse_vocabulary(src);
        let word = &lessons[0].words[0];
        assert_eq!(word.kanji, "朝");
        assert_eq!(word.meaning, "morning");
        assert_eq!(word.word_type, "noun");
    }

    #[test]
    fn parse_field_with_fullwidth_colon() {
        let src = "## Places\n\n### にわ\nkanji：庭\nmeaning: yard\n";
        let lessons = parse_vocabulary(src);
        assert_eq!(lessons[0].words[0].kanji, "庭");
    }
}
