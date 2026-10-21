//! Entry point for the purrr engine (v0_02).
//!
//! `main` owns the top-level control flow of the whole program: it shows the
//! main menu in a loop and starts — or loads — a game depending on the player's
//! choice. The actual game loop lives in `engine`, and all the in-game rules
//! live in `game`.

// Each `mod` declaration tells the compiler "there is another source file
// called <name>.rs in this crate; compile it and expose its items as
// <name>::something".
mod character;
mod engine;
mod game;
mod house;
mod menu;
mod npc;
mod save;

fn main() {
    // The menu runs until the player chooses to exit the program.
    loop {
        match menu::show() {
            menu::MenuChoice::NewGame => {
                // 1. Ask the player to create a character (interactive prompts).
                let player = character::create();

                // 2. Assemble a brand-new world around that character: a default
                //    house and a default NPC.
                let mut game = game::Game::new(player, house::House::new(), npc::Npc::new());

                // 3. Hand control to the game loop. It returns when the player
                //    chooses to go back to the menu.
                engine::run(&mut game);
            }
            menu::MenuChoice::LoadGame => {
                // Loading can fail (e.g. no save file yet). `match` lets us
                // handle the success and failure cases separately instead of
                // crashing.
                match save::load() {
                    Ok(state) => {
                        let mut game = game::Game::from_save(state);
                        engine::run(&mut game);
                    }
                    Err(message) => {
                        // Show why the load failed, then fall back to the menu.
                        println!("Could not load save: {}", message);
                    }
                }
            }
            menu::MenuChoice::Exit => break,
        }
    }

    println!("Goodbye.");
}
