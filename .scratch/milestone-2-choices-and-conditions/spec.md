# Spec: choices and conditions

## Problem

Milestone 1 plays a whole game, but it decides several things for the player
and leaves the between-turn step empty. Each shortcut hides a choice the real
game gives a player, and a bot trained against the shortcuts learns the wrong
game.

## Solution

Turn each Milestone 1 shortcut into an ordinary choice or an implemented rule.
Setup becomes a phase with its own legal actions. An attack cost reads Energy
types. Special Conditions arrive, and with them the Pokémon Checkup between
turns.

## User stories

- A person places their own Active and Bench at setup, and calls the opening
  coin flip.
- An attack that costs two Fire Energy refuses to fire on two Lightning.
- A Pokémon that falls asleep cannot attack, and flips to wake at the checkup.

## Implementation decisions

Every new choice is a phase of the state with its own legal actions, never a
suspended function. The rules in [the base rules](../../docs/architecture/rules.md)
keep their numbers, and the code cites them.

## Testing decisions

One integration test per rule that a player can observe. The seeded generator
and the scripted one already make a coin flip an assertion.

## Out of scope

Evolution, Trainers, Abilities, and Stadiums.

Real card data was out of scope when this spec was written. The operator
brought it in on 2026-09-05: ticket 06 imports it and ticket 07 reads it.
