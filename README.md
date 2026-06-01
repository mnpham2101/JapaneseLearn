# Japanese Learn

A Japanese language learning flashcard application built with Rust and Slint.

## Overview

- Rust core logic for flashcard management and study mode
- Slint UI for cross-platform desktop and WebAssembly support
- Local JSON/Markdown persistence planned for flashcard data

## Prerequisites

- **Rust 1.96.0 or later** — Install from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **Slint** — Integrated as a Cargo dependency; automatically downloaded during build

For detailed setup and troubleshooting, see the official [Slint Rust documentation](https://slint.rs/docs/rust/).

## Getting Started

1. Install Rust using the official installer from https://www.rust-lang.org/tools/install
2. Clone this repository
3. Run `cargo build` to build the application
4. Run `cargo run` to launch the application

## Project Structure

- `Cargo.toml` — Rust package manifest
- `src/main.rs` — application entry point
- `ui/main_window.slint` — Slint UI definition
