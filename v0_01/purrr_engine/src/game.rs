//! The game: all the in-game rules and state.
//!
//! `Game` owns the state of a play session (currently just a turn counter and a
//! "is it over?" flag) and turns the player's typed commands into changes to
//! that state. The engine never sees these rules — it only calls the three
//! methods `render`, `handle`, and `is_over`.

/// The whole state of one game session.
pub struct Game {
    /// How many turns have been played so far.
    turn: u32,
    /// True once the game should stop.
    over: bool,
}

impl Game {
    /// Build a brand-new game. `Self` is shorthand for "the type this `impl`
    /// block is for" (here, `Game`).
    pub fn new() -> Self {
        Self { turn: 0, over: false }
    }

    /// Produce the text shown before each turn (the prompt).
    pub fn render(&self) -> String {
        format!("\n=== Turn {} ===\n> ", self.turn)
    }

    /// React to one command. Returns `Some(message)` to print, or `None` to
    /// stay silent. `&mut self` means this method may change the game's state.
    pub fn handle(&mut self, input: &str) -> Option<String> {
        // Every command advances the turn counter by one.
        self.turn += 1;

        // `match` on the input string chooses what to do. Each arm returns a
        // `String` wrapped in `Some`, because we always have something to say.
        match input {
            "" => Some("You say nothing.".to_string()),
            "help" => Some("Try typing anything, or 'quit'.".to_string()),
            "win" => {
                // Typing "win" both ends the game and prints a message.
                self.over = true;
                Some("You win!".to_string())
            }
            // The `other` arm catches every string we didn't list above.
            other => Some(format!("You said: {}", other)),
        }
    }

    /// Whether the game loop should stop.
    pub fn is_over(&self) -> bool {
        self.over
    }
}

// Tests live in the same file as the code they test. `#[cfg(test)]` means "only
// compile this when running `cargo test`, never in a normal build", and the
// `mod tests` sits inside `game.rs` so it can reach `Game`'s private fields.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_advances() {
        let mut g = Game::new();
        g.handle("hello");
        assert_eq!(g.turn, 1);
    }

    #[test]
    fn win_ends_game() {
        let mut g = Game::new();
        g.handle("win");
        assert!(g.is_over());
    }
}
