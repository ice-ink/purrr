//! The main menu.
//!
//! The menu is a tiny state machine of its own: it keeps asking until the
//! player picks one of three valid choices, then reports that choice to its
//! caller (`main`). Separating the menu from `main` keeps `main` small and
//! gives the menu a single, easily testable job.

use crate::engine;

/// The three things the player can do from the main menu.
///
/// An `enum` is a type that is exactly one of its listed variants. Using an
/// enum here (rather than, say, returning a number) makes the meaning of each
/// choice explicit and lets the compiler check that every caller handles all
/// the possibilities.
pub enum MenuChoice {
    NewGame,
    LoadGame,
    Exit,
}

/// Show the menu and block until the player makes a valid choice.
pub fn show() -> MenuChoice {
    loop {
        // `engine::prompt` returns `Option<String>`: `Some` when a line was read,
        // `None` when input ended (Ctrl-D / piped input exhausted).
        match engine::prompt("purrr — a cozy text adventure\n\n  1. New game\n  2. Load save\n  3. Exit\n\n> ") {
            // No more input: leave the program cleanly rather than looping.
            None => return MenuChoice::Exit,
            Some(answer) => match answer.as_str() {
                "1" | "new" | "n" => return MenuChoice::NewGame,
                "2" | "load" | "l" => return MenuChoice::LoadGame,
                "3" | "exit" | "quit" | "q" => return MenuChoice::Exit,
                // Anything else loops around and asks again.
                _ => println!("Please choose 1, 2, or 3."),
            },
        }
    }
}
