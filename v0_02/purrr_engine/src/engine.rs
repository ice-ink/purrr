//! The engine: the shared "read → act → repeat" loop, plus a prompt helper.
//!
//! This module is deliberately generic. It knows nothing about menus,
//! characters, houses, or NPCs — it only knows how to talk to a `Game`, which
//! is the object that actually understands the rules. Keeping the loop generic
//! means we can reuse it unchanged in later versions while the game itself
//! grows.
//!
//! The loop it runs is a classic REPL (Read-Eval-Print Loop):
//!
//!   1. Render   — ask the game what to show the player.
//!   2. Read     — wait for a line of text from the player.
//!   3. Evaluate — hand that line to the game so it can react.
//!   4. Print    — show the game's reaction.
//!   5. Loop     — unless the game says it is over.

use std::io::{self, Write};

use crate::game::Game;

/// Print a prompt, wait for one line of input, and return it with surrounding
/// whitespace removed.
///
/// Returns `None` when input ends (Ctrl-D, or a piped input stream runs out) so
/// callers can react to "there is nothing more to read" instead of looping
/// forever. `Option<String>` is the idiomatic Rust way to say "maybe a string".
pub fn prompt(question: &str) -> Option<String> {
    print!("{}", question);

    // stdout is buffered: `print!` without a newline won't appear until we
    // flush. Flushing pushes the text out immediately, before we block on input.
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        // `Ok(0)` means zero bytes were read: end-of-input.
        Ok(0) => None,
        Ok(_) => Some(input.trim().to_string()),
        // Treat a read error the same way: there is nothing more to read.
        Err(_) => None,
    }
}

/// Run the main game loop until the game reports it is over.
pub fn run(game: &mut Game) {
    loop {
        // 1. Render.
        print!("{}", game.render());
        io::stdout().flush().expect("failed to flush stdout");

        // 2. Read.
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                // 3. Evaluate. `handle` returns `Some(text)` only when the game
                //    has something to say; `None` means "stay silent".
                if let Some(response) = game.handle(input.trim()) {
                    println!("{}", response);
                }

                // 4. Loop — unless the game asked to stop.
                if game.is_over() {
                    break;
                }
            }
            Err(error) => {
                eprintln!("Input error: {}", error);
                break;
            }
        }
    }
}
