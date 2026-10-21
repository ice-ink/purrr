# purrr — v0_02

The second version: v0_01's loop grows into a **small, complete game**.

This version adds a main menu, character creation, a house hub you can expand, an
independent NPC, and a save/load system — all still with zero external crates.

## Running it

```sh
cd v0_02/purrr_engine
cargo run          # build and run
cargo test         # run the unit tests
```

## Playing it

### Main menu

```
purrr — a cozy text adventure

  1. New game
  2. Load save
  3. Exit
```

- `1` / `new` — create a character and start a fresh game.
- `2` / `load` — load the saved game (`save.purrr` in the directory you run from).
- `3` / `exit` — leave the program.

### Character creation

Starting a new game asks a series of questions:

| Question | Kind | Values |
| --- | --- | --- |
| Name | text | any string |
| Cat person? | binary | `yes` / `no` |
| Early riser? | binary | `yes` / `no` |
| Strength | numerical | 0–10 |
| Agility | numerical | 0–10 |
| Charm | numerical | 0–10 |

You start with **50 gold**, used to expand your house. (This demonstrates the two
input shapes the agenda asked for — binary and numerical.)

### In-game commands

| Command | Effect |
| --- | --- |
| `help` | List the commands. |
| `look` | Describe the house and its rooms. |
| `me` | Show your character's stats and gold. |
| `npc` | Check on the house's resident and what it's doing. |
| `expand` | Add a room to the house (costs 50 gold). |
| `save` | Save the game to `save.purrr`. |
| `quit` | Return to the main menu (does **not** auto-save). |

Every command advances the clock by one **turn**. The NPC advances on its own
schedule every turn, independent of what you type — its mood, energy, and
current activity all change over time.

### Saving and loading

`save` writes the whole game — player, house, NPC, and turn counter — to a plain
text file named `save.purrr`. Loading (menu option `2`) reads it back. The format
is designed to be human-readable:

```text
# purrr save file — version 1

[player]
name=Alice
cat_person=true
early_riser=false
strength=7
agility=5
charm=6
gold=0

[house]
name=Cozy Cottage
level=2
room=living room
room=kitchen
room=bedroom

[npc]
name=Whiskers
personality=curious but aloof
mood=0
energy=86
step=6

[game]
turn=6
```

- `[section]` headers group related fields.
- `key=value` lines hold one value each.
- Rooms use **repeated** `room=` lines (one per room).
- `#` starts a comment; blank lines are ignored.

If the file is missing or malformed, loading reports a clear error (including the
line number) instead of crashing.

## How the code works (a learner's walkthrough)

The program now has eight modules. The control flow from startup is:

```
main.rs ──▶ menu.rs  ──▶ character.rs (new game only)
                    ──▶ save.rs      (load game only)
        ──▶ engine.rs ──▶ game.rs ──▶ character.rs / house.rs / npc.rs / save.rs
```

### `main.rs` — the top-level loop

`main` runs the menu in a loop and starts or loads a game based on the choice:

```rust
loop {
    match menu::show() {
        MenuChoice::NewGame => { /* create + run */ }
        MenuChoice::LoadGame => { /* load + run, or report the error */ }
        MenuChoice::Exit => break,
    }
}
```

The `match` on an **enum** is a key pattern: the compiler *checks that every
variant is handled*, so you can't forget a case. Compare that to a C-style
`switch` on a number, where nothing forces you to cover all values.

### `menu.rs` — an enum and a tiny state machine

```rust
pub enum MenuChoice { NewGame, LoadGame, Exit }
```

An **enum** is a type that is exactly one of its listed variants. `show()` loops,
prompting until it gets a valid answer, then returns the matching variant.

### `engine.rs` — the reused loop, plus a prompt helper

`engine::run` is the same generic REPL as v0_01 — it only calls `render`,
`handle`, and `is_over`. Two changes from v0_01:

- The "quit/exit" special case is gone; ending is now the game's job (via
  `is_over`), keeping the engine purely generic.
- A new helper, `prompt(question) -> Option<String>`, is used by the menu and
  character creation. It returns **`None`** when input ends (Ctrl-D / piped input
  exhausted), so callers can react to end-of-input instead of looping forever.
  This is a small but important robustness improvement over v0_01.

### `game.rs` — owns the world and interprets commands

`Game` now holds three entities and a turn counter:

```rust
pub struct Game {
    turn: u32,
    over: bool,
    player: Character,
    house: House,
    npc: Npc,
}
```

`handle` is where the commands become behaviour. Two lines are especially worth
studying:

- `"expand" => Some(self.house.expand(&mut self.player))` — this passes **two
  mutable borrows at once** (`self.house` and `self.player`). That's allowed
  because they are *different* fields; the borrow checker forbids two mutable
  borrows of the *same* value, not of distinct values.
- `"npc" => Some(self.npc.describe())` — an **immutable** borrow, which can
  coexist with other immutable borrows.

`Game` also exposes read-only accessors (`player()`, `house()`, `npc()`,
`turn()`) so that `save.rs` can read the state without touching private fields.
Keeping fields private and going through methods is a core Rust idiom.

### `character.rs` — data plus interactive creation

`Character` is a plain data struct. `create()` asks the questions, with two
private helpers that show different input handling:

- `ask_bool` — loops until `yes`/`no`.
- `ask_number` — loops until a number in range, using `parse::<u8>()` and a match
  **guard** (`Ok(n) if n <= 10`). `parse` returns `Result`, which you must handle
  — this is how Rust replaces exceptions for expected, recoverable failures.

### `house.rs` — the expandable hub

`House` has a `level` and a list of `rooms`. `expand`:

1. refuses if the player can't afford it (early `return`),
2. deducts gold and raises the level,
3. picks the next room from a fixed list — `NEW_ROOMS.get(index)` returns an
   `Option`, and `unwrap_or("spare room")` supplies a fallback once the list is
   exhausted.

### `npc.rs` — an independent resident

The NPC (`Whiskers`, a cat) has its own name, personality, mood, energy, and an
internal `step` clock. `update()` runs **every turn** and is called from
`game.handle` regardless of what the player typed — that's what makes it
"independent". Its activity is derived from the clock:

```rust
fn routine_index(&self) -> usize {
    ((self.step / 2) as usize) % ROUTINE.len()
}
```

So the NPC cycles through a fixed routine (napping, watching birds, …) on its own,
with mood and energy drifting. `clamp(-10, 10)` keeps mood inside its range — a
nice example of "enforce invariants where you change the value".

### `save.rs` — serialization by hand

There are no crates here, so we write the format ourselves. Two directions:

- **`serialize(game) -> String`** — build the text from the live objects.
- **`parse(text) -> Result<LoadedState, String>`** — rebuild the objects from text.

`parse` is the interesting half. Every field starts as `None`; as lines are read,
a `match (section, key)` fills the matching variable via small typed parsers
(`parse_bool`, `parse_u8`, …). At the end, `ok_or_else(...)?` turns any missing
required field into an error. The `?` operator is shorthand for "if this is an
`Err`, return it early; otherwise give me the value."

Using `Result<T, String>` means "a `T` on success, or a readable message on
failure". It's the idiomatic way to model fallible operations (file I/O,
parsing) without exceptions.

## What to notice as a learner

- **Enums** (`MenuChoice`) model "one of a fixed set of choices" and make `match`
  exhaustive.
- **`Option`** ("maybe a value") and **`Result`** ("value or an error") replace
  nulls and exceptions.
- **Ownership & borrowing** show up everywhere: `&self` vs `&mut self`, multiple
  mutable borrows of *distinct* fields, and moving data out of a `LoadedState`.
- **Separation of concerns** — each module has one clear job, which is what makes
  the whole thing testable (see the unit tests in `house.rs`, `npc.rs`, and
  `save.rs`).

Continue to [v0_03](../v0_03/README.md) once it exists — see
[the roadmap](ROADMAP.md) for what's planned.
