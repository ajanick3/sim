# Spec: keep the raw card record

## Problem

`load` reads all 3051 cards in the artifact and keeps playable data for the
274 the engine can run. Every other card survives as `{ id, name, because }`
and nothing else: its HP, its attacks, and its printed text are dropped at
load.

That is right for playing a game and wrong for everything around one. A deck
builder, a card browser, and a legality check all need the printed fields of a
card the engine cannot yet play. Each is cheaper to allow now than to retrofit
once callers depend on the current shape.

## Solution

Keep the card's record for every card read, admitted or refused, and let a
caller look one up. The engine's own play is unchanged: a refused card is still
refused, and still never reaches a game.

## User stories

- A tool lists every Standard card with its HP and its printed text, including
  the ones the engine cannot play.
- A tool checks a decklist against deck construction without the engine being
  able to play a single card in it.
- A reader of a refusal can see the text that caused it.

## Implementation decisions

The engine holds no I/O, so the record comes from the same JSON string `load`
already takes. Whether the kept record is the raw value or a typed struct is
this effort's question, and the ticket decides it.

## Testing decisions

One test that a refused card can be read back in full. One that the admitted
cards still play, unchanged.

## Out of scope

Playing a refused card. Anything that would weaken
[ADR 0008](../../docs/adr/0008-the-engine-refuses-a-card-it-cannot-run.md): a
card the engine cannot run in full still never enters a game.
