//! The player's character.
//!
//! A `Character` is plain data plus a couple of tiny behaviours. Keeping it in
//! its own module — separate from `game` (the rules) and `save` (persistence)
//! — is an example of *separation of concerns*: each module has one job, which
//! makes the whole program easier to read, test, and grow.

use crate::engine;

/// Gold the player starts with — enough for their first home improvement.
const STARTING_GOLD: i32 = 50;

/// Everything we track about the player's character.
pub struct Character {
    pub name: String,
    /// Binary choice made at creation: cat person (`true`) or dog person (`false`).
    pub cat_person: bool,
    /// Binary choice: early riser (`true`) or night owl (`false`).
    pub early_riser: bool,
    /// Numerical stats, each on a 0–10 scale.
    pub strength: u8,
    pub agility: u8,
    pub charm: u8,
    /// Money used to expand the house.
    pub gold: i32,
}

impl Character {
    /// Build a character from explicit values. This is also used when loading a
    /// save, so creation and loading share the same construction path.
    pub fn new(
        name: String,
        cat_person: bool,
        early_riser: bool,
        strength: u8,
        agility: u8,
        charm: u8,
        gold: i32,
    ) -> Self {
        // When a field has the same name as a variable, Rust lets us write just
        // the name instead of `name: name`.
        Self {
            name,
            cat_person,
            early_riser,
            strength,
            agility,
            charm,
            gold,
        }
    }

    /// A human-readable summary, shown by the `me` command.
    pub fn describe(&self) -> String {
        format!(
            "{} — {} person, {}\n  STR {:>2}  AGI {:>2}  CHA {:>2}\n  Gold: {}",
            self.name,
            if self.cat_person { "cat" } else { "dog" },
            if self.early_riser { "early riser" } else { "night owl" },
            self.strength,
            self.agility,
            self.charm,
            self.gold,
        )
    }
}

/// Interactively create a character by asking the player a series of questions.
///
/// This shows the two kinds of input we need to handle:
///   * binary choices (`ask_bool`), and
///   * numerical values with a range check (`ask_number`).
pub fn create() -> Character {
    let name = engine::prompt("What is your name? ").unwrap_or_else(|| "Traveler".to_string());

    let cat_person = ask_bool("Are you a cat person? (yes/no) ");
    let early_riser = ask_bool("Are you an early riser? (yes/no) ");

    let strength = ask_number("Strength (0-10): ");
    let agility = ask_number("Agility (0-10): ");
    let charm = ask_number("Charm (0-10): ");

    Character::new(name, cat_person, early_riser, strength, agility, charm, STARTING_GOLD)
}

/// Keep asking until the player answers yes or no; return the boolean.
fn ask_bool(question: &str) -> bool {
    loop {
        match engine::prompt(question) {
            // Input ended mid-creation: use a default and carry on.
            None => return false,
            Some(answer) => match answer.to_lowercase().as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                _ => println!("Please answer yes or no."),
            },
        }
    }
}

/// Keep asking until the player types a whole number in the allowed range.
fn ask_number(question: &str) -> u8 {
    loop {
        match engine::prompt(question) {
            // Input ended mid-creation: use a middle-of-the-road default.
            None => return 5,
            Some(answer) => {
                // `parse::<u8>()` returns `Result`: `Ok(n)` on success, `Err(_)`
                // if the text wasn't a valid number (or was negative / too big).
                match answer.parse::<u8>() {
                    // A guard (`if n <= 10`) narrows the match to acceptable values.
                    Ok(n) if n <= 10 => return n,
                    Ok(_) => println!("Please enter a number between 0 and 10."),
                    Err(_) => println!("That isn't a number. Try again."),
                }
            }
        }
    }
}
