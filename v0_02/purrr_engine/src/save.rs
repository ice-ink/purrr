//! Saving and loading: turning the game state into a text file and back.
//!
//! v0_02 has no external dependencies, so instead of a serialization library
//! (like `serde`) we hand-write the format. The file is a simple, human-readable
//! text format with `[sections]` and `key=value` lines. Writing the format by
//! hand is also a good way to see what serialization actually *is*: turning a
//! live program's data into a durable representation, and back again.
//!
//! Example save file:
//!
//! ```text
//! # purrr save file — version 1
//!
//! [player]
//! name=Ada
//! cat_person=true
//! early_riser=false
//! strength=7
//! ...
//! ```
//!
//! See `v0_02/README.md` for the full format.

use std::fs;
use std::path::Path;

use crate::character::Character;
use crate::game::Game;
use crate::house::House;
use crate::npc::Npc;

/// Where the single save slot lives (relative to where you run the program).
const SAVE_PATH: &str = "save.purrr";

/// The three entities plus the turn counter, rebuilt by `load`.
///
/// This plain-data struct carries the parts of the game back out of the file;
/// `Game::from_save` reassembles them into a running `Game`.
pub struct LoadedState {
    pub player: Character,
    pub house: House,
    pub npc: Npc,
    pub turn: u32,
}

/// Serialize the game to text and write it to disk.
///
/// `Result<(), String>` means "nothing useful on success, or a human-readable
/// error message on failure". Using `String` as the error type keeps this
/// version simple; a later version could introduce a proper error enum.
pub fn save(game: &Game) -> Result<(), String> {
    let text = serialize(game);
    fs::write(SAVE_PATH, text).map_err(|e| format!("{}", e))
}

/// Read the save file from disk and parse it back into a `LoadedState`.
pub fn load() -> Result<LoadedState, String> {
    if !Path::new(SAVE_PATH).exists() {
        return Err("no save file found (start a new game first)".to_string());
    }
    let text = fs::read_to_string(SAVE_PATH).map_err(|e| format!("{}", e))?;
    parse(&text)
}

/// Turn the whole game into a string in our save-file format.
fn serialize(game: &Game) -> String {
    let p = game.player();
    let h = game.house();
    let n = game.npc();

    // Build the file line by line. `out.push_str` appends to the string; we
    // reuse `format!` to build each line.
    let mut out = String::new();
    out.push_str("# purrr save file — version 1\n\n");

    out.push_str("[player]\n");
    out.push_str(&format!("name={}\n", p.name));
    out.push_str(&format!("cat_person={}\n", p.cat_person));
    out.push_str(&format!("early_riser={}\n", p.early_riser));
    out.push_str(&format!("strength={}\n", p.strength));
    out.push_str(&format!("agility={}\n", p.agility));
    out.push_str(&format!("charm={}\n", p.charm));
    out.push_str(&format!("gold={}\n", p.gold));

    out.push_str("\n[house]\n");
    out.push_str(&format!("name={}\n", h.name));
    out.push_str(&format!("level={}\n", h.level));
    // Rooms are stored as repeated `room=` lines, one per room.
    for room in &h.rooms {
        out.push_str(&format!("room={}\n", room));
    }

    out.push_str("\n[npc]\n");
    out.push_str(&format!("name={}\n", n.name));
    out.push_str(&format!("personality={}\n", n.personality));
    out.push_str(&format!("mood={}\n", n.mood));
    out.push_str(&format!("energy={}\n", n.energy));
    out.push_str(&format!("step={}\n", n.step));

    out.push_str("\n[game]\n");
    out.push_str(&format!("turn={}\n", game.turn()));

    out
}

/// Parse the text of a save file back into a `LoadedState`.
fn parse(text: &str) -> Result<LoadedState, String> {
    // Start every field as `None`; we require the important ones before building
    // the structs, so a malformed file fails loudly instead of silently
    // producing a broken game. Rooms are collected into a `Vec` as we go.
    let mut name = None;
    let mut cat_person = None;
    let mut early_riser = None;
    let mut strength = None;
    let mut agility = None;
    let mut charm = None;
    let mut gold = None;

    let mut house_name = None;
    let mut level = None;
    let mut rooms: Vec<String> = Vec::new();

    let mut npc_name = None;
    let mut personality = None;
    let mut mood = None;
    let mut energy = None;
    let mut step = None;

    let mut turn = None;

    // The `[section]` we are currently inside.
    let mut section = String::new();

    // `enumerate` gives us `(line_number, line)`; we add 1 when reporting errors
    // so line numbers start at 1 for humans.
    for (line_number, raw) in text.lines().enumerate() {
        let line = raw.trim();

        // Blank lines and comments are just formatting.
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // A section header looks like `[player]`.
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].to_string();
            continue;
        }

        // Everything else must be `key=value`.
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| {
                format!(
                    "line {}: expected 'key=value', got '{}'",
                    line_number + 1,
                    line
                )
            })?;
        let key = key.trim();
        let value = value.trim();

        // Match on (section, key) and fill the matching variable. The typed
        // parsers (`parse_bool`, etc.) return errors that include the line
        // number, so a bad value points you at the exact problem line.
        match (section.as_str(), key) {
            ("player", "name") => name = Some(value.to_string()),
            ("player", "cat_person") => cat_person = Some(parse_bool(value, line_number)?),
            ("player", "early_riser") => early_riser = Some(parse_bool(value, line_number)?),
            ("player", "strength") => strength = Some(parse_u8(value, line_number)?),
            ("player", "agility") => agility = Some(parse_u8(value, line_number)?),
            ("player", "charm") => charm = Some(parse_u8(value, line_number)?),
            ("player", "gold") => gold = Some(parse_i32(value, line_number)?),
            ("house", "name") => house_name = Some(value.to_string()),
            ("house", "level") => level = Some(parse_u8(value, line_number)?),
            ("house", "room") => rooms.push(value.to_string()),
            ("npc", "name") => npc_name = Some(value.to_string()),
            ("npc", "personality") => personality = Some(value.to_string()),
            ("npc", "mood") => mood = Some(parse_i32(value, line_number)?),
            ("npc", "energy") => energy = Some(parse_i32(value, line_number)?),
            ("npc", "step") => step = Some(parse_u32(value, line_number)?),
            ("game", "turn") => turn = Some(parse_u32(value, line_number)?),
            _ => {
                return Err(format!(
                    "line {}: unknown key '[{}] {}'",
                    line_number + 1,
                    section,
                    key
                ));
            }
        }
    }

    // Now that every line is read, unwrap the required values. `ok_or_else`
    // turns a `None` (missing field) into a clear error; `?` propagates it.
    let player = Character::new(
        name.ok_or_else(|| "save is missing player.name".to_string())?,
        cat_person.ok_or_else(|| "save is missing player.cat_person".to_string())?,
        early_riser.ok_or_else(|| "save is missing player.early_riser".to_string())?,
        strength.ok_or_else(|| "save is missing player.strength".to_string())?,
        agility.ok_or_else(|| "save is missing player.agility".to_string())?,
        charm.ok_or_else(|| "save is missing player.charm".to_string())?,
        gold.ok_or_else(|| "save is missing player.gold".to_string())?,
    );

    let house = House::from_parts(
        house_name.ok_or_else(|| "save is missing house.name".to_string())?,
        level.ok_or_else(|| "save is missing house.level".to_string())?,
        rooms,
    );

    let npc = Npc::from_parts(
        npc_name.ok_or_else(|| "save is missing npc.name".to_string())?,
        personality.ok_or_else(|| "save is missing npc.personality".to_string())?,
        mood.ok_or_else(|| "save is missing npc.mood".to_string())?,
        energy.ok_or_else(|| "save is missing npc.energy".to_string())?,
        step.ok_or_else(|| "save is missing npc.step".to_string())?,
    );

    Ok(LoadedState {
        player,
        house,
        npc,
        // The turn counter is optional (older saves might not have it).
        turn: turn.unwrap_or(0),
    })
}

// Small typed parsers that turn "true"/"7" into real values. They take the line
// number (0-based from `enumerate`) and add 1 so error messages are 1-based.

fn parse_bool(value: &str, line: usize) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!(
            "line {}: expected true/false, got '{}'",
            line + 1,
            value
        )),
    }
}

fn parse_u8(value: &str, line: usize) -> Result<u8, String> {
    value
        .parse::<u8>()
        .map_err(|_| format!("line {}: expected a number, got '{}'", line + 1, value))
}

fn parse_u32(value: &str, line: usize) -> Result<u32, String> {
    value
        .parse::<u32>()
        .map_err(|_| format!("line {}: expected a number, got '{}'", line + 1, value))
}

fn parse_i32(value: &str, line: usize) -> Result<i32, String> {
    value
        .parse::<i32>()
        .map_err(|_| format!("line {}: expected a number, got '{}'", line + 1, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::Character;
    use crate::game::Game;
    use crate::house::House;
    use crate::npc::Npc;

    /// A save that is serialized and immediately parsed again must produce the
    /// same state. This is the property that makes "save then load" reliable.
    #[test]
    fn round_trip_preserves_state() {
        let mut house = House::new();
        house.expand(&mut Character::new(
            "T".to_string(),
            true,
            true,
            5,
            5,
            5,
            50,
        )); // use the player's gold to expand, so the house has 3 rooms

        let game = Game::new(
            Character::new("Ada".to_string(), true, false, 7, 5, 6, 120),
            house,
            Npc::from_parts("Whiskers".to_string(), "sleepy".to_string(), 4, 77, 9),
        );

        let text = serialize(&game);
        let loaded = parse(&text).expect("round-trip parse should succeed");

        assert_eq!(loaded.turn, game.turn());
        assert_eq!(loaded.player.name, game.player().name);
        assert_eq!(loaded.player.cat_person, game.player().cat_person);
        assert_eq!(loaded.player.gold, game.player().gold);
        assert_eq!(loaded.house.name, game.house().name);
        assert_eq!(loaded.house.level, game.house().level);
        assert_eq!(loaded.house.rooms, game.house().rooms);
        assert_eq!(loaded.npc.name, game.npc().name);
        assert_eq!(loaded.npc.mood, game.npc().mood);
        assert_eq!(loaded.npc.step, game.npc().step);
    }

    #[test]
    fn parse_rejects_malformed_lines() {
        let bad = "[player]\nstrength=not-a-number\n";
        assert!(parse(bad).is_err());
    }
}
