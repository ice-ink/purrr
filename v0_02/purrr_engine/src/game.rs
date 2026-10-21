//! The game: all the in-game rules and state.
//!
//! `Game` owns the three entities (player, house, NPC) plus the turn counter,
//! and translates the player's typed commands into changes to that state. The
//! engine never sees these rules — it just calls `render`, `handle`, and
//! `is_over`.

use crate::character::Character;
use crate::house::House;
use crate::npc::Npc;
use crate::save;

/// The text shown by the `help` command. The leading backslash-line-continuation
/// keeps the string readable while giving us a single newline at the start.
const HELP: &str = "\
Commands:
  help    show this list
  look    look around the house
  me      view your character
  npc     check on the house's resident
  expand  add a room to the house (costs gold)
  save    save your progress
  quit    return to the main menu";

/// The whole in-game state.
pub struct Game {
    turn: u32,
    /// True once the player has asked to return to the main menu.
    over: bool,
    player: Character,
    house: House,
    npc: Npc,
}

impl Game {
    /// Start a fresh game from a newly created player and a default world.
    pub fn new(player: Character, house: House, npc: Npc) -> Self {
        Self {
            turn: 0,
            over: false,
            player,
            house,
            npc,
        }
    }

    /// Rebuild a game from a loaded save. The `state` value is moved in and its
    /// fields are taken out and placed into the new `Game`.
    pub fn from_save(state: save::LoadedState) -> Self {
        Self {
            turn: state.turn,
            over: false,
            player: state.player,
            house: state.house,
            npc: state.npc,
        }
    }

    /// The prompt shown before each turn.
    pub fn render(&self) -> String {
        format!(
            "\n--- {} (level {}) ---\nTurn {}\nWhat would you like to do? (type 'help' for options)\n> ",
            self.house.name, self.house.level, self.turn
        )
    }

    /// React to one command. Returns `Some(message)` to print, or `None` to
    /// stay silent. Every command advances the clock by one turn.
    pub fn handle(&mut self, input: &str) -> Option<String> {
        self.turn += 1;

        // The NPC lives on its own schedule: it advances every turn regardless
        // of what the player typed.
        self.npc.update();

        match input {
            "" => Some("You pause for a moment.".to_string()),
            "help" => Some(HELP.to_string()),
            "look" => Some(self.house.describe()),
            "me" | "status" => Some(self.player.describe()),
            "npc" => Some(self.npc.describe()),
            // `expand` mutably borrows both the house and the player. These are
            // different fields of `self`, so the borrow checker allows it.
            "expand" => Some(self.house.expand(&mut self.player)),
            "save" => match save::save(self) {
                Ok(()) => Some("Game saved.".to_string()),
                Err(message) => Some(format!("Could not save: {}", message)),
            },
            "quit" | "menu" => {
                self.over = true;
                Some("Returning to the main menu.".to_string())
            }
            other => Some(format!("You don't know how to '{}'. Try 'help'.", other)),
        }
    }

    /// Whether the game loop should stop (the player asked to return to menu).
    pub fn is_over(&self) -> bool {
        self.over
    }

    // Read-only accessors. These let other modules (notably `save`) read the
    // state without reaching into `Game`'s private fields. Data flows out
    // through methods rather than by poking fields directly — a good habit that
    // keeps the representation easy to change later.
    pub fn player(&self) -> &Character {
        &self.player
    }

    pub fn house(&self) -> &House {
        &self.house
    }

    pub fn npc(&self) -> &Npc {
        &self.npc
    }

    pub fn turn(&self) -> u32 {
        self.turn
    }
}
