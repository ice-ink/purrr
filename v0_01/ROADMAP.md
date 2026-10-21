# Roadmap — v0_02

This is the plan written at the end of v0_01. It describes the next version,
v0_02. (See also `../v0_02/pre-dev.md`, the original agenda note.)

## Goals for v0_02

1. **Main menu** — three options: start a new game, load a save, or exit.
2. **Character creation** — when starting a new game, the player assigns
   parameters to their character. Some are *binary* choices (yes/no), others are
   *numerical* values.
3. **A central house** — the player starts in a house that acts as a hub and can
   be expanded over time.
4. **An independent NPC** — a separate resident with its own parameters,
   attributes, and characteristics, entirely independent of the player. (It will
   become much more advanced in later versions.)
5. **Save system** — save and load the game. The save must track:
   - the **player** data,
   - the **NPC** data,
   - the **house** data.

## Design notes carried forward

- Keep the engine loop generic (reuse `engine.rs`'s REPL structure).
- Keep the project dependency-free; hand-roll the save format for now.
- Document everything: usage, a learner walkthrough, inline comments, and this
  roadmap — every version follows the same convention.
