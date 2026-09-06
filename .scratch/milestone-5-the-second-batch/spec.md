# Spec: the second batch of Trainers

## Problem

A Trainer the engine has not built is still the largest thing standing between
it and a real deck. Across the 61 committed decks, counted by each card's
first blocker out of 3660 slots:

| Blocker              | Slots | Share |
| -------------------- | ----- | ----- |
| Trainer, not yet built | 1230 | 33.6% |
| Ability              | 703   | 19.2% |
| Trainer, built       | 634   | 17.3% |
| Basic Energy, plays  | 480   | 13.1% |
| Attack text          | 279   | 7.6%  |

Nine cards account for 639 of those slots — 17.5% of every deck in the field,
and more than the whole rest of the unbuilt Trainer list. Two of them,
`Buddy-Buddy Poffin` and `Ultra Ball`, are worth more than the other seven
together.

Milestone 3 built the eight Trainers that needed only what was already there.
These nine each need something that is not.

## Solution

Build the nine, and the primitives they need: a search that puts a Pokémon
into play rather than moving a card between zones, a filter that reads stage
and HP, a requirement paid to play a card at all, a move between two
Pokémon's attachments, and an evolution chain two links long.

## User stories

- A player searches their deck for two small Basics and benches both.
- A player discards two cards to play `Ultra Ball`, and cannot play it
  holding nothing else.
- A player moves an Energy from one of their Pokémon to another.
- A player plays `Rare Candy` on a Basic and puts a Stage 2 straight on it.

## Implementation decisions

Every primitive is a value the engine executes, as
[ADR 0009](../../docs/adr/0009-an-effect-is-a-value-the-engine-executes.md)
decided, and a mid-effect choice is a phase, as
[ADR 0012](../../docs/adr/0012-trainer-resolution-shares-two-phases.md)
decided. A card is admitted only when the engine can run all of it
([ADR 0008](../../docs/adr/0008-the-engine-refuses-a-card-it-cannot-run.md)).

Each ticket checks the pool before it builds. Two mechanics were taken from
elsewhere in milestone 4 and had to be removed or narrowed because no card in
this format used them; the check costs one query.

## Testing decisions

Test first. The coverage count and the committed decks are measured after
each ticket, since the point of the milestone is a number that moves.

## Out of scope

Abilities — 703 slots, and the milestone after this one.
`Area Zero Underdepths`, which changes the Bench size for a player with a Tera
Pokémon in play: it needs continuous rules and a Tera concept, and neither
exists.
