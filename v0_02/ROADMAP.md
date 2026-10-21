# Roadmap — v0_03

This plan was written at the end of v0_02 and describes the next version.

## Goals for v0_03

1. **Put the character stats to work.** Strength, agility, and charm currently
   exist but do nothing. v0_03 should make them affect gameplay — e.g. a
   "venture out" action whose outcome depends on a stat check.
2. **Leave the house.** The house is a hub, but so far there is nowhere to go.
   Add at least one explorable location reachable from the hub, and a command to
   travel between the hub and it.
3. **Player–NPC interaction.** The NPC is currently observation-only. Add a
   simple interaction (e.g. `pet`, `talk`) that changes a relationship value
   while still letting the NPC live independently between turns.
4. **A win condition.** Give the game a goal — for example, fully expand the
   house (build every listed room) — with a clear "you win" state.
5. **Multiple save slots.** Replace the single `save.purrr` with a small set of
   named slots, and let the player choose which to load.

## Technical improvements

- **A real error type.** Replace the `Result<T, String>` convention with a
  `SaveError` enum, to practise defining and matching custom error types.
- **More tests.** Add tests for the menu parsing, character-creation validation,
  and the new stat/win logic.
- **(Optional) switch to `serde`/`serde_json`** for saving once the hand-rolled
  format's limits become annoying — but only if introducing a dependency is
  worth the teaching trade-off.

## Docs carried forward

Every version keeps the same conventions: a `README.md` (usage + learner
walkthrough), a `CHANGELOG.md` (what changed), this `ROADMAP.md` (the next
plan), and detailed inline comments in the source.
