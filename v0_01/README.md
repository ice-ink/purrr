# purrr — v0_01

The first version: a **minimal text game loop** in Rust.

There is no world, no character, and no save system yet. What exists is the
foundation every later version builds on — a *read-eval-print loop* (REPL) that
prompts the player, reads a line of text, hands it to the game, prints the
game's response, and repeats.

## Running it

```sh
cd v0_01/purrr_engine
cargo run          # build and run
cargo test         # run the unit tests
```

## What you can do

| Command | Effect |
| --- | --- |
| *(anything)* | Echoes back what you typed, advancing the turn counter. |
| `help` | Prints a one-line hint. |
| `win` | Prints "You win!" and ends the game. |
| `quit` / `exit` | Ends the game. |
| *(Ctrl-D / empty piped input)* | Ends the game cleanly. |

## How the code works (a learner's walkthrough)

The project has three files, each with one job. Follow the arrows to see the
flow of control when you type a command.

```
main.rs        engine.rs          game.rs
─────────      ─────────          ───────
create Game ──▶ run(&mut game) ──▶ render()   ── print the prompt
                                   handle()   ── react to a command
                                   is_over()  ── should we stop?
```

### `main.rs` — the entry point

```rust
mod engine;
mod game;

fn main() {
    let mut game = game::Game::new();
    engine::run(&mut game);
}
```

- `mod engine; mod game;` declares the other two modules. A module is just
  another source file whose items are reachable as `engine::...` / `game::...`.
- `Game::new()` is an *associated function* — a function attached to the type
  rather than to a particular value. It returns a fresh `Game`.
- `let mut game = ...` makes the binding mutable, because `run` needs to change
  the game as the player types.
- `engine::run(&mut game)` passes a **mutable reference**. The `&mut` means "the
  callee may change `game`, and no one else is allowed to touch it meanwhile."
  This is the borrow checker in action: it prevents two parts of the program from
  mutating the same data at once.

### `engine.rs` — the loop

`engine::run` is deliberately generic. It knows nothing about the game's rules;
it only knows how to talk to a `Game` through three methods: `render`, `handle`,
and `is_over`. This separation means the loop can be reused unchanged in later
versions while the game itself grows.

The loop is a classic REPL:

1. **Render** — `game.render()` returns a `String` (the prompt), which is printed.
2. **Read** — `io::stdin().read_line(...)` waits for a line. A read of **0 bytes**
   (`Ok(0)`) means end-of-input (Ctrl-D, or piped input ran out), so we stop.
3. **Evaluate** — `game.handle(input)` returns an `Option<String>`:
   `Some(text)` when the game wants to say something, `None` when it stays silent.
4. **Print** — any `Some(text)` is printed.
5. **Loop** — unless `game.is_over()` says to stop.

Two things worth noticing:

- **`io::stdout().flush()`** — stdout is *buffered*; text printed without a
  newline won't appear until it is flushed. Without the flush, the prompt would
  stay hidden until after you'd already typed.
- **`Option`** — instead of using an empty string or a special "no message"
  value, `handle` returns `Option<String>`. `if let Some(response) = ...` is the
  idiomatic way to say "do this only if there is a message."

### `game.rs` — the rules

```rust
pub struct Game {
    turn: u32,
    over: bool,
}
```

A `struct` is a bundle of named fields. `Game` holds the state of one session:
a turn counter and an "is it over?" flag. Both are **private** (no `pub`), which
means only code inside `game.rs` can touch them directly — everyone else must go
through the methods.

```rust
impl Game {
    pub fn new() -> Self { Self { turn: 0, over: false } }
    pub fn render(&self) -> String { format!("\n=== Turn {} ===\n> ", self.turn) }
    pub fn handle(&mut self, input: &str) -> Option<String> { /* ... */ }
    pub fn is_over(&self) -> bool { self.over }
}
```

- `impl Game { ... }` is an **inherent impl block**: it attaches methods to the
  `Game` type. `Self` inside it is shorthand for `Game`.
- `&self` means "borrow me, don't change me" (for `render` / `is_over`); `&mut
  self` means "borrow me and allow changes" (for `handle`).
- `handle` uses a **`match`** on the input string. `match` forces you to handle
  every possibility; the `other =>` arm is a catch-all for anything not listed.

### The tests

Unit tests live in the same file as the code they test, inside a
`#[cfg(test)] mod tests` block:

- `#[cfg(test)]` means "compile this only for `cargo test`, never for a normal
  build", so test code is never shipped.
- A test is just a function marked `#[test]` that uses `assert!` / `assert_eq!`
  to check a claim. A panic (from a failed assertion) fails the test.

Because the tests sit *inside* `game.rs`, they can reach `Game`'s private fields
like `turn`. Run them with `cargo test`.

## What to notice as a learner

- **Modules** keep one idea per file: entry point, loop, rules.
- **Ownership & borrowing** (`&` vs `&mut`) govern who may read vs. change data.
- **`Option` and `match`** replace null/flag conventions with explicit handling.
- **Structs + impl blocks** bundle state with the operations on that state.

Continue to [v0_02](../v0_02/README.md) to see this loop turned into a real game
with a menu, a character, a house, an NPC, and saving/loading.
