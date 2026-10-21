//! The player's home: the hub they start in and can expand over time.
//!
//! The house is a central place the game always returns to. Expanding it costs
//! gold and adds rooms, giving the player a simple long-term goal even before
//! the wider game exists.

use crate::character::Character;

/// How much gold one expansion costs.
const EXPANSION_COST: i32 = 50;

/// Rooms are added in this order as the house grows. The first expansion adds
/// the first entry, the second expansion the second, and so on.
const NEW_ROOMS: [&str; 6] = [
    "bedroom",
    "study",
    "garden",
    "attic",
    "greenhouse",
    "observatory",
];

/// The house is the central hub of the game.
pub struct House {
    pub name: String,
    /// How many times the house has been expanded (starts at 1).
    pub level: u8,
    /// The names of the rooms the house currently has.
    pub rooms: Vec<String>,
}

impl House {
    /// A brand-new starting house.
    pub fn new() -> Self {
        Self {
            name: "Cozy Cottage".to_string(),
            level: 1,
            // `vec!` is a convenience macro that builds a `Vec` from a list.
            rooms: vec!["living room".to_string(), "kitchen".to_string()],
        }
    }

    /// Build from explicit values (used when loading a save).
    pub fn from_parts(name: String, level: u8, rooms: Vec<String>) -> Self {
        Self { name, level, rooms }
    }

    /// Try to add a room. Succeeds only if the player has enough gold.
    ///
    /// Note the two parameters: `&mut self` (we change the house) and
    /// `&mut Character` (we also deduct gold from the player). A function may
    /// mutably borrow two different values at once — it only fails when it
    /// tries to borrow the *same* value mutably more than once. The house and
    /// the player are distinct, so this is fine.
    pub fn expand(&mut self, player: &mut Character) -> String {
        if player.gold < EXPANSION_COST {
            // `return` exits early; the rest of the function only runs when we
            // can afford the expansion.
            return format!(
                "You can't afford to expand. It costs {} gold, but you have {}.",
                EXPANSION_COST, player.gold
            );
        }

        player.gold -= EXPANSION_COST;
        self.level += 1;

        // Pick the next room from the fixed list. `get` returns an `Option`,
        // which is `None` once we've built every listed room; `unwrap_or`
        // substitutes a generic name in that case.
        let room = NEW_ROOMS
            .get((self.level - 2) as usize)
            .copied()
            .unwrap_or("spare room")
            .to_string();
        self.rooms.push(room.clone());

        format!(
            "You add a {} to the house! It is now level {}.",
            room, self.level
        )
    }

    /// A human-readable description, shown by the `look` command.
    pub fn describe(&self) -> String {
        format!(
            "{} (level {}) — rooms: {}",
            self.name,
            self.level,
            self.rooms.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::Character;

    #[test]
    fn expand_adds_a_room_and_charges_gold() {
        let mut house = House::new();
        let mut player = Character::new("T".to_string(), true, true, 5, 5, 5, 50);

        let rooms_before = house.rooms.len();
        let message = house.expand(&mut player);

        assert_eq!(house.rooms.len(), rooms_before + 1);
        assert_eq!(house.level, 2);
        assert_eq!(player.gold, 0);
        assert!(message.contains("bedroom"));
    }

    #[test]
    fn expand_fails_without_enough_gold() {
        let mut house = House::new();
        let mut player = Character::new("T".to_string(), true, true, 5, 5, 5, 10);

        let message = house.expand(&mut player);

        assert_eq!(house.level, 1);
        assert_eq!(player.gold, 10);
        assert!(message.contains("can't afford"));
    }
}
