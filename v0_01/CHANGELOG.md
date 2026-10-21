# Changelog — v0_01

The first playable version of the purrr engine.

## What this version introduces

- **Project scaffold** — a `Cargo.toml` (Rust 2024 edition, no dependencies) and
  a `src/` tree, so the project can be built with `cargo build` and run with
  `cargo run`.
- **Module layout** — three files with one job each:
  - `main.rs` — entry point.
  - `engine.rs` — the generic read-eval-print loop.
  - `game.rs` — the game state (`Game`) and its rules.
- **A playable loop** — the program prompts with a turn counter, reads a line,
  and responds.
- **Commands** — `help`, `win` (ends the game), and `quit`/`exit` (end the game).
  Any other input is echoed back.
- **Turn counter** — every command advances `Game::turn`, which is shown in the
  prompt.
- **Unit tests** — `turn_advances` and `win_ends_game`, runnable with
  `cargo test`.

## Changes from the initial commit

The initial code built but the two tests were dead code: they lived in a file
named `src/test` (no `.rs` extension), which Cargo never compiled, so
`cargo test` reported "0 tests". This version fixes that by moving the tests
into a proper `#[cfg(test)] mod tests` block inside `game.rs`, and removes an
unused import in `main.rs`. Both issues were found during the documentation
review.
