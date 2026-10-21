# Changelog — v0_02

The engine grows from a bare loop into a small, complete game. All features are
built with the standard library only — no external crates.

## Added

- **Main menu** (`menu.rs`) — three choices: *new game*, *load save*, *exit*.
  Modelled as a `MenuChoice` enum with an exhaustive `match` in `main.rs`.
- **Character creation** (`character.rs`) — the player sets their name, two
  *binary* traits (cat/dog person, early riser/night owl), and three *numerical*
  stats (strength, agility, charm, 0–10), with input validation and re-prompts.
- **House hub** (`house.rs`) — the player starts in a `Cozy Cottage` (level 1,
  living room + kitchen) and can `expand` it: 50 gold adds a room and raises the
  level. Rooms are added from a fixed progression, then a generic fallback.
- **Independent NPC** (`npc.rs`) — `Whiskers`, a cat with its own name,
  personality, mood, energy, and an internal clock. It updates **every turn** on
  its own (cycling a routine, drifting mood/energy) regardless of player input;
  the player can only `npc` to observe it.
- **Save/load system** (`save.rs`) — a hand-written, human-readable text format
  (`save.purrr`) with `[sections]` and `key=value` lines. It tracks the player,
  the house (including every room), the NPC, and the turn counter. Malformed or
  missing files produce clear, line-numbered errors.
- **Unit tests** — house expansion (success and "can't afford"), NPC clock/range
  invariants and routine cycling, and a save round-trip. `cargo test` runs 6
  tests.

## Changed

- `engine.rs` — the REPL loop is unchanged in spirit, but the `quit`/`exit`
  special case was removed so the engine stays purely generic; a new
  `prompt() -> Option<String>` helper handles end-of-input cleanly (used by the
  menu and character creation).
- `main.rs` — no longer starts the game directly; it drives the menu loop and
  assembles/loads games.
- `game.rs` — `Game` now owns a `Character`, a `House`, and an `Npc`, exposes
  read-only accessors for saving, and maps the full command set to behaviour.

## Fixed (carried from the v0_01 review)

- The two v0_01 tests that were dead code (`src/test` had no `.rs` extension) are
  now proper tests; the same discipline (inline `#[cfg(test)]` modules) is used
  throughout v0_02.
- Removed the unused import that produced a build warning in v0_01's `main.rs`.
