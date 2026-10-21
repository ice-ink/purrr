# purrr

A text-based game engine written in **Rust**, built one small version at a time.

`purrr` is in **alpha**. Each iteration lives in its own directory — `v0_01`,
`v0_02`, and so on — so you can watch the code grow from a bare game loop into a
fuller game, version by version.

## Who these docs are for

The documentation is written for two audiences:

1. **People who want to play / use it** — how to build and run each version,
   what the commands are, and what the save file looks like.
2. **People who know programming fundamentals and are learning Rust** — each
   version has a walkthrough of *how the code works*, and the source files carry
   detailed inline comments explaining the Rust constructs as they appear.

You do not need to read the versions in order, but they build on one another, so
starting at `v0_01` is the gentlest path.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (the project uses edition
  2024, so a recent toolchain is required). There are **no external crates** —
  everything is built with the standard library.

## Building and running

Each version is a standalone Cargo project. From a version's directory:

```sh
cd v0_02/purrr_engine     # or v0_01/purrr_engine
cargo run                 # build and run
cargo test                # run that version's unit tests
```

To run the latest version from the repository root:

```sh
cd v0_02/purrr_engine && cargo run
```

## Versions

| Version | Summary | Docs |
| --- | --- | --- |
| `v0_01` | The first playable loop: a read-eval-print engine that echoes your commands, with a turn counter and a `quit`/`win` command. | [README](v0_01/README.md) · [CHANGELOG](v0_01/CHANGELOG.md) · [ROADMAP](v0_01/ROADMAP.md) |
| `v0_02` | A main menu, character creation, a house hub you can expand, an independent NPC, and a save/load system. | [README](v0_02/README.md) · [CHANGELOG](v0_02/CHANGELOG.md) · [ROADMAP](v0_02/ROADMAP.md) |

## How the documentation is organised

Each version directory contains:

- **`README.md`** — how to run this version, its commands, and a learner-oriented
  walkthrough of the code.
- **`CHANGELOG.md`** — what changed in this version compared to the previous one.
- **`ROADMAP.md`** — the plan for the *next* version.
- **inline comments** — the source files themselves are commented to explain
  each Rust construct as it is used.

## Agenda

The overall goal: build up a text-based game engine in Rust, documenting every
step so that someone learning the language can follow along.

## Notes

- Save files are written to `save.purrr` in the directory you run the game from,
  and are ignored by git (see `.gitignore`).
- Each version is independent; changes in one version do not rewrite earlier
  ones, so the history of the project stays readable.
