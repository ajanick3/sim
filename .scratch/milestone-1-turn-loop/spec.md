# Spec: the Milestone 1 turn loop

Written after the fact on 2026-09-05. The effort ran before this repository
had a tracker, and this spec records what it decided.

## Problem

The repository held a design and no code. Nothing proved the design ran.

## Solution

Build the smallest engine that plays a whole game: two synthetic Basic Pokémon
that attack until someone wins. Cards are literals, so no card data is needed.

## User stories

- A person runs `cargo run`, reads the board, picks from a numbered list of
  legal actions, and finishes a game.
- A test drives a game to a winner from a seed and gets the same game back.

## Implementation decisions

- Objects live in arenas and move by typed index.
- `legal_actions(state)` is the interface; `apply` refuses anything outside it.
- The generator lives in the state, so a game replays from its seed.
- The damage order follows the rulebook's numbered steps.

## Testing decisions

Integration tests only, in `tests/milestone1.rs`. One test drives 19 seeded
games to a winner through a simple policy, which is the milestone's claim.

## Out of scope

Evolution, Trainers, Abilities, Stadiums, Special Conditions, matching Energy
types to an attack cost, the coin flip for who goes first, and choosing where
a Pokémon goes at setup.
