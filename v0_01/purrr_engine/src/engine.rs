use crate::game::Game;
use std::io::{self, Write};

pub fn run(game: &mut Game) {
    loop {
        // 1. Render the current state
        let output = game.render();
        print!("{}", output);
        io::stdout().flush().expect("failed to flush stdout");

        // 2. Read a line of input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (e.g. piped input ended, or Ctrl-D)
                println!("\nGoodbye.");
                break;
            }
            Ok(_) => {
                let input = input.trim();
                if input == "quit" || input == "exit" {
                    println!("Goodbye.");
                    break;
                }
                // 3. Hand input to the game
                if let Some(response) = game.handle(input) {
                    println!("{}", response);
                }
                if game.is_over() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Input error: {}", e);
                break;
            }
        }
    }
}