# Flashcard Stack Markdown Format

This document defines the markdown file format used by the JapaneseLearn application to import and export flashcard stacks.

## Format Specification

A markdown file may contain **one or more flashcard stacks**. Each stack is delimited by a level-2 heading (`## Stack Name`). Under each heading, cards are listed as rows in a GitHub-Flavored Markdown (GFM) pipe table with two columns: **Japanese** and **Meaning**.

### Rules

- Each `##` heading starts a new stack. The heading text becomes the stack name.
- The table header row must be `| Japanese | Meaning |` (case-insensitive column names are accepted on import).
- The separator row (`|---|---|`) is required and must follow the header row immediately.
- Each data row represents one flashcard: the first cell is the Japanese word/phrase, the second is the meaning.
- Empty rows and rows outside a table are ignored on import.
- A stack with no table rows is valid (creates an empty stack).
- Text before the first `##` heading is ignored.

### File Extension

Files must use the `.md` extension. The application's file dialog filters for `*.md` files.

---

## Worked Example

```markdown
## Hiragana Basics

| Japanese | Meaning |
|---|---|
| あ | a |
| い | i |
| う | u |
| え | e |
| お | o |

## Common Greetings

| Japanese | Meaning |
|---|---|
| こんにちは | Hello |
| おはようございます | Good morning |
| ありがとうございます | Thank you |
| すみません | Excuse me / Sorry |

## Numbers

| Japanese | Meaning |
|---|---|
| 一 (いち) | 1 (one) |
| 二 (に) | 2 (two) |
| 三 (さん) | 3 (three) |
```

The example above produces three stacks:
- **Hiragana Basics** — 5 cards
- **Common Greetings** — 4 cards
- **Numbers** — 3 cards

---

## Round-Trip Guarantee

`serialize_stacks` produces output that `parse_stacks` can read back without data loss. The serialized form always uses the canonical header `| Japanese | Meaning |` and a `|---|---|` separator, regardless of the source file's original formatting.
