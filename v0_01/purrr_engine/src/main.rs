//! Entry point for the purrr engine (v0_01).
//!
//! `main` is the first function the operating system calls. Its only job here
//! is to create a fresh game and hand control to the engine loop. All the
//! interesting behaviour lives in the `engine` and `game` modules.

// A `mod` declaration tells the compiler "there is another source file in this
// crate, compile it too, and make its items available as `engine::...` and
// `game::...`". The file names are implied: `mod engine` looks for `engine.rs`
// next to this file.
mod engine;
mod game;

fn main() {
    // Create a new, empty game.
    let mut game = game::Game::new();

    // `engine::run` borrows the game mutably (`&mut`) because the loop needs to
    // change the game's state as the player types. The borrow ends when `run`
    // returns.
    engine::run(&mut game);
}
