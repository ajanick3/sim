# Spec: the record

## Problem

The engine computes the rules well and records almost nothing.

[ADR 0002, superseded by 0007](../../docs/adr/0007-card-data-comes-from-tcgdex-into-this-repository.md)
says the engine is a pure function of its state, its seed, and its actions,
"which makes a replay and a test the same thing". Nothing keeps the actions.
A game cannot be replayed, undone, saved, or handed to another process. The
purity is real; the benefit it was supposed to buy has never been collected.

Three smaller gaps have the same shape — the engine knows a fact while it
runs and keeps no record of it:

- A game holds no Lost Zone and no Stadium in play. `stadium_played_this_turn`
  exists with nowhere for the Stadium to go.
- Per-turn and per-game limits are ad hoc: `evolved_this_turn` on a Pokémon,
  `supporter_played_this_turn` on a player, and nothing at all for an Ability
  used this turn or a once-per-game marker.
- The decklist parser reads one dialect. Three of the 64 committed decks do
  not parse, and the reasons are known.

## Solution

Give the engine a record: an action log a game can be replayed from, the two
missing zones, one shape for the limits, and a parser that reads the dialects
real lists are written in.

## User stories

- An operator replays a finished game from its seed and its actions, and gets
  the same game.
- An operator undoes the last action.
- An operator imports a decklist written in any dialect a tournament site
  exports, or is told exactly which line failed.
- A card that goes to the Lost Zone has somewhere to go.

## Implementation decisions

The log records what was applied, never what was intended. It stores
`Action` values, which are already `Copy` and comparable, so a replay is
`GameState::new(seed) + actions.fold(apply)`.

The engine holds no I/O (ADR 0007), so writing a log to a file, or asking a
service to resolve a decklist line, belongs to a caller or a tool — never to
the engine.

## Testing decisions

A replay test is the strongest kind available here: play a seeded game to its
end, replay the log, and assert the two states agree. Property-shaped, and it
covers every rule the engine has without naming any of them.

## Out of scope

Networked multiplayer. A user interface for any of this. Abilities, which
`Team Rocket's Watchtower` and the ability-used counter both wait on.

## What this milestone learned from elsewhere

`ptcg-sim` (github.com/xxmichaellong/ptcg-sim), read on 2026-09-06, is a
tabletop simulator: it enforces no rules, computes no damage, and refuses no
move — its `attack` posts a chat message. What it does have is the record
this milestone is named for. Every state change goes through one
`acceptAction(user, action, parameters)` funnel keyed by action name, and
multiplayer sync, replay, state import, and undo all fall out of that. Its
decklist import reads six shapes before falling back to a service. Both are
worth taking; neither is a rule.
