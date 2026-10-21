//! The engine: the shared "read → act → repeat" loop.
//!
//! This module is deliberately generic. It knows nothing about the rules of the
//! game — it only knows how to talk to a `Game`, which is the object that
//! actually understands the player's commands.
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

/// Run the main game loop until the game reports it is over.
pub fn run(game: &mut Game) {
    loop {
        // 1. Render. `render` returns a `String`, which we print directly.
        print!("{}", game.render());

        // stdout is *buffered*: text written with `print!` may not appear until
        // a newline or an explicit flush. We flush so the prompt shows up before
        // we block waiting for input.
        io::stdout().flush().expect("failed to flush stdout");

        // 2. Read. `read_line` appends the next line to `input`.
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            // `Ok(0)` means zero bytes were read: end-of-input (Ctrl-D, or a
            // piped input stream ran out). There is nothing left to do, so stop.
            Ok(0) => {
                println!("\nGoodbye.");
                break;
            }
            Ok(_) => {
                // Trim the trailing newline (and any surrounding whitespace).
                let input = input.trim();

                // The two built-in ways to leave the loop. In a later version
                // this could move into `Game` so the engine stays purely generic.
                if input == "quit" || input == "exit" {
                    println!("Goodbye.");
                    break;
                }

                // 3. Evaluate. `handle` returns `Some(text)` only when the game
                //    has something to say; `None` means "stay silent".
                if let Some(response) = game.handle(input) {
                    println!("{}", response);
                }

                // 4. Loop — unless the game asked to stop.
                if game.is_over() {
                    break;
                }
            }
            // `Err` means reading itself failed (rare). Report it and stop.
            Err(e) => {
                eprintln!("Input error: {}", e);
                break;
            }
        }
    }
}
