# Spec: evolution and the first effects

## Problem

The engine plays 274 of 3051 Standard cards, and 12 of the 60 in one committed
deck and 1 of 60 in the other. The gap is not spread evenly: half of every real
deck is Trainers, and every Trainer carries rules text the engine cannot
execute.

Two smaller gaps matter as much. The engine has no evolution, so the Stage 1
and Stage 2 lines both decks are built on cannot be played at all. And a
knockout always takes 1 Prize, though 538 cards in Standard are worth 2 or 3.

## Solution

Add evolution, which needs no effect system. Then build an effect vocabulary
chosen by counting what the committed decks actually play, rather than by
reading the card pool front to back. Fix the prize count while the cards that
need it are arriving.

## User stories

- A player evolves a Basic into a Stage 1 and keeps its damage and attachments.
- A player plays the Trainers their deck is built on.
- Knocking out a Pokémon ex takes 2 Prizes.
- A reader can tell which printed card an implementation came from.

## Implementation decisions

An effect is a value the engine executes, never text it interprets at run time.
A mid-effect choice is a phase of the state, which
[ADR 0003](../../docs/adr/0003-legal-actions-is-the-engine-interface.md)
decided.

A card is admitted only when the engine can run all of it, which
[ADR 0008](../../docs/adr/0008-the-engine-refuses-a-card-it-cannot-run.md)
decided. The coverage count reports the result of this effort, and the two
committed decks are the measure that matters.

## Testing decisions

Test first. Each effect primitive earns a test that fails without it. The
coverage number and the two decks are checked after each ticket.

## Out of scope

Abilities. Stadiums. Special Energy. A general parser for English card text:
the effects are written as values, one card at a time.
