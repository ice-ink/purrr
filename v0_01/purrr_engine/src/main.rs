use std::io::{self, Write};

mod engine;
mod game;

fn main() {
    let mut game = game::Game::new();

    engine::run(&mut game);
}