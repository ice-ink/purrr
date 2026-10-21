//! The NPC: a resident of the house who is entirely independent of the player.
//!
//! "Independent" means the NPC has its own parameters and changes on its own
//! schedule. Every turn, `update` advances its internal clock, mood, and energy
//! without any input from the player. The player can only *observe* it (via the
//! `npc` command), not control it.

/// The NPC's daily routine, cycled through automatically. This is a fixed list
/// for now; a later version can give the NPC real goals and choices.
const ROUTINE: [&str; 5] = [
    "napping in a warm sunbeam",
    "watching birds through the window",
    "grooming its fur",
    "pacing lazily across the floor",
    "dozing by the fire",
];

/// A resident of the house with its own parameters, attributes, and schedule.
pub struct Npc {
    pub name: String,
    pub personality: String,
    /// Mood from -10 (grumpy) to +10 (content). Drifts over time.
    pub mood: i32,
    /// Energy from 0 (exhausted) to 100 (full). Refills while resting.
    pub energy: i32,
    /// An internal clock: it counts every turn. The NPC's activity is derived
    /// from this, so it appears to live on its own without any player input.
    pub step: u32,
}

impl Npc {
    /// A brand-new NPC: a cat named Whiskers.
    pub fn new() -> Self {
        Self {
            name: "Whiskers".to_string(),
            personality: "curious but aloof".to_string(),
            mood: 0,
            energy: 80,
            step: 0,
        }
    }

    /// Build from explicit values (used when loading a save).
    pub fn from_parts(
        name: String,
        personality: String,
        mood: i32,
        energy: i32,
        step: u32,
    ) -> Self {
        Self {
            name,
            personality,
            mood,
            energy,
            step,
        }
    }

    /// Advance the NPC one turn. Called automatically by the game each turn.
    ///
    /// This is what makes the NPC "independent": it changes on its own clock,
    /// regardless of what the player typed.
    pub fn update(&mut self) {
        self.step += 1;

        // Resting turns refill energy; active turns drain a little. Because the
        // routine changes every two steps, energy rises and falls in a cycle.
        if self.step % 2 == 0 {
            self.energy = (self.energy + 5).min(100);
        } else {
            self.energy = (self.energy - 3).max(0);
        }

        // Each activity nudges mood in its own way, so mood drifts over time.
        // `clamp` forces the result back into the allowed -10..=10 range.
        let mood_change: i32 = match self.routine_index() {
            0 | 4 => 1, // napping / dozing: restful
            3 => -1,    // pacing: restless
            _ => 0,
        };
        self.mood = (self.mood + mood_change).clamp(-10, 10);
    }

    /// Which entry of the routine the NPC is currently on.
    fn routine_index(&self) -> usize {
        // Divide by 2 so each activity lasts two turns; `% ROUTINE.len()` wraps
        // back to the start after the last one.
        ((self.step / 2) as usize) % ROUTINE.len()
    }

    /// What the NPC is doing right now.
    pub fn current_activity(&self) -> &str {
        ROUTINE[self.routine_index()]
    }

    /// A human-readable description, shown by the `npc` command.
    pub fn describe(&self) -> String {
        format!(
            "{} — {}\n  Mood: {}  Energy: {}\n  Currently: {}",
            self.name,
            self.personality,
            self.mood,
            self.energy,
            self.current_activity(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_advances_clock_and_keeps_values_in_range() {
        let mut npc = Npc::new();
        for _ in 0..100 {
            npc.update();
        }
        assert_eq!(npc.step, 100);
        assert!((0..=100).contains(&npc.energy));
        assert!((-10..=10).contains(&npc.mood));
    }

    #[test]
    fn activity_cycles_through_the_routine() {
        let mut npc = Npc::new();
        let mut seen = std::collections::HashSet::new();
        for _ in 0..20 {
            // Store an owned copy so the borrow from `npc` ends before `update`
            // needs a mutable borrow on the next line.
            seen.insert(npc.current_activity().to_string());
            npc.update();
        }
        // Over 20 turns the NPC should have moved through several activities.
        assert!(seen.len() > 1);
    }
}
