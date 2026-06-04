# Japanese Learn

A Japanese language learning flashcard application built with Rust and Slint.

## Overview

- Rust core logic for flashcard management and study mode
- Slint UI for cross-platform desktop and WebAssembly support
- Local JSON/Markdown persistence planned for flashcard data

## Prerequisites

- **Rust 1.96.0 or later** — Install from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **Slint** — Integrated as a Cargo dependency; automatically downloaded during build
- **Python 3** — Required for the web version; install and add `python3` to your PATH
- **wasm-opt** — Install with `cargo install wasm-opt` before using the web build

For detailed setup and troubleshooting, see the official [Slint Rust documentation](https://slint.rs/docs/rust/).

## Getting Started

1. Install Rust using the official installer from https://www.rust-lang.org/tools/install
2. Clone this repository
3. Run `cargo build` to build the Window desktop application
4. Run `cargo run` to launch the Window desktop application

## Running

### Desktop

- `cargo run --bin japanese_learn`

This builds and runs the native desktop application using Cargo.

### Web

Before running the web version, install the web prerequisites:

- `cargo install wasm-opt`
- Install Python 3 and ensure `python3` is on your PATH

Then run:

- ` wasm-pack build --release --target web`
- `python3 -m http.server`

This performs `wasm-pack build`, starts a local web server on port `8000`, and you can open your browser at `http://localhost:8000/`.

> Note: the Slint website suggestion to install only `wasm-pack` is not enough for this project; `cargo install wasm-opt` is required for the web build.


## Project Structure

- `Cargo.toml` — Rust package manifest
- `src/main.rs` — application entry point
- `ui/main_window.slint` — Slint UI definition
