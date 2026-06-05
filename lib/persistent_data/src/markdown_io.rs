//! Markdown import/export for flashcard stacks.
//!
//! The format uses `## Stack Name` level-2 headings and GFM pipe tables:
//!
//! ```markdown
//! ## Hiragana Basics
//!
//! | Japanese | Meaning |
//! |---|---|
//! | あ | a |
//! | い | i |
//! ```

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// A single flashcard stack with a name and an ordered list of cards.
#[derive(Debug, Clone, PartialEq)]
pub struct StackData {
    pub name: String,
    pub cards: Vec<CardData>,
}

/// A single flashcard: one Japanese word/phrase and its meaning.
#[derive(Debug, Clone, PartialEq)]
pub struct CardData {
    pub japanese: String,
    pub meaning: String,
}

/// Parse one or more flashcard stacks from a markdown string.
///
/// - `## Heading` text becomes the stack name.
/// - GFM table rows (excluding the header row) beneath a heading become cards.
/// - Text before the first `##` heading is ignored.
/// - A heading with no table is valid and produces an empty stack.
pub fn parse_stacks(source: &str) -> Vec<StackData> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(source, options);

    let mut stacks: Vec<StackData> = Vec::new();

    // Parsing state
    let mut in_heading_2 = false;
    let mut pending_heading_text = String::new();

    // Table state — pulldown-cmark 0.12 has no TableBody tag;
    // body rows follow immediately after TableHead ends.
    let mut in_table_head = false;
    let mut past_table_head = false; // true once TableHead has ended
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();

    for event in parser {
        match event {
            // ---- headings ----
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                ..
            }) => {
                in_heading_2 = true;
                pending_heading_text.clear();
            }
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                in_heading_2 = false;
                let name = pending_heading_text.trim().to_owned();
                stacks.push(StackData {
                    name,
                    cards: Vec::new(),
                });
                pending_heading_text.clear();
            }

            // ---- table structure ----
            Event::Start(Tag::Table(_)) => {
                past_table_head = false;
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
                past_table_head = true;
                // Discard header row — we do not need column names at runtime.
                current_row.clear();
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                if past_table_head && !in_table_head {
                    if let Some(stack) = stacks.last_mut() {
                        if current_row.len() >= 2 {
                            stack.cards.push(CardData {
                                japanese: current_row[0].trim().to_owned(),
                                meaning: current_row[1].trim().to_owned(),
                            });
                        }
                    }
                }
                current_row.clear();
            }
            Event::Start(Tag::TableCell) => {
                current_cell.clear();
            }
            Event::End(TagEnd::TableCell) => {
                current_row.push(current_cell.clone());
                current_cell.clear();
            }

            // ---- text nodes ----
            Event::Text(text) | Event::Code(text) => {
                if in_heading_2 {
                    pending_heading_text.push_str(&text);
                } else if in_table_head || past_table_head {
                    current_cell.push_str(&text);
                }
            }

            _ => {}
        }
    }

    stacks
}

/// Serialize a slice of `StackData` into a markdown string.
///
/// Each stack is written as a `## Stack Name` heading followed by a GFM pipe
/// table. The output round-trips through [`parse_stacks`] without data loss.
pub fn serialize_stacks(stacks: &[StackData]) -> String {
    let mut out = String::new();

    for (i, stack) in stacks.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str("## ");
        out.push_str(&stack.name);
        out.push_str("\n\n");

        out.push_str("| Japanese | Meaning |\n");
        out.push_str("|---|---|\n");

        for card in &stack.cards {
            out.push_str("| ");
            out.push_str(&card.japanese);
            out.push_str(" | ");
            out.push_str(&card.meaning);
            out.push_str(" |\n");
        }
    }

    out
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// 1. Single stack with two cards.
    #[test]
    fn parse_single_stack_two_cards() {
        let md = "\
## Animals

| Japanese | Meaning |
|---|---|
| 犬 | dog |
| 猫 | cat |
";
        let stacks = parse_stacks(md);
        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].name, "Animals");
        assert_eq!(stacks[0].cards.len(), 2);
        assert_eq!(stacks[0].cards[0].japanese, "犬");
        assert_eq!(stacks[0].cards[0].meaning, "dog");
        assert_eq!(stacks[0].cards[1].japanese, "猫");
        assert_eq!(stacks[0].cards[1].meaning, "cat");
    }

    /// 2. Two stacks.
    #[test]
    fn parse_two_stacks() {
        let md = "\
## Greetings

| Japanese | Meaning |
|---|---|
| こんにちは | Hello |

## Numbers

| Japanese | Meaning |
|---|---|
| 一 | one |
| 二 | two |
";
        let stacks = parse_stacks(md);
        assert_eq!(stacks.len(), 2);

        assert_eq!(stacks[0].name, "Greetings");
        assert_eq!(stacks[0].cards.len(), 1);
        assert_eq!(stacks[0].cards[0].japanese, "こんにちは");

        assert_eq!(stacks[1].name, "Numbers");
        assert_eq!(stacks[1].cards.len(), 2);
        assert_eq!(stacks[1].cards[1].meaning, "two");
    }

    /// 3. Empty input produces an empty vec.
    #[test]
    fn parse_empty_input() {
        let stacks = parse_stacks("");
        assert!(stacks.is_empty());
    }

    /// 4. Stack with a heading but no table rows produces an empty card list.
    #[test]
    fn parse_stack_no_cards() {
        let md = "\
## Empty Stack
";
        let stacks = parse_stacks(md);
        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].name, "Empty Stack");
        assert!(stacks[0].cards.is_empty());
    }

    /// 5. Round-trip: `parse_stacks(serialize_stacks(original))` equals original.
    #[test]
    fn round_trip() {
        let original = vec![
            StackData {
                name: "Hiragana Basics".to_owned(),
                cards: vec![
                    CardData {
                        japanese: "あ".to_owned(),
                        meaning: "a".to_owned(),
                    },
                    CardData {
                        japanese: "い".to_owned(),
                        meaning: "i".to_owned(),
                    },
                ],
            },
            StackData {
                name: "Numbers".to_owned(),
                cards: vec![CardData {
                    japanese: "一 (いち)".to_owned(),
                    meaning: "1 (one)".to_owned(),
                }],
            },
        ];

        let serialized = serialize_stacks(&original);
        let reparsed = parse_stacks(&serialized);

        assert_eq!(reparsed, original);
    }
}
