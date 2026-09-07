# An identity swap keeps the same `PokemonInPlay`, only the card changes

**Status:** Accepted — 2026-09-07

## Context

`Transformation Tome`'s effect reads: *"Choose a Basic Pokémon in your
discard pile and switch it with 1 of your Basic Pokémon in play. Any
attached cards, damage counters, Special Conditions, turns in play, and
any other effects remain on the new Pokémon."* Every switch built so
far (`Boss's Orders`, `Switch`, `Promote`) moves *which Pokémon* sits in
a board slot, carrying that Pokémon's own damage and attachments with
it as a whole. This card is the opposite: the board slot — and
everything attached to it — stays put, and only which card names what
occupies it changes.

## Decision

`Action::SwapIdentityWithDiscarded` replaces `PokemonInPlay.cards` in
place — `std::mem::replace(&mut pokemon.cards, vec![new_card])` — and
sends the old cards to discard. `damage`, `attached`, `conditions`, and
`played_on_turn` are untouched fields on the same `PokemonInPlay`, so
"remain on the new Pokémon" costs nothing to implement: there is
nothing else to move.

The new card only ever replaces a Basic's single-card stack
(`Phase::SwappingIdentity` only offers a Basic in play as the target,
per the card's own text), so `cards` is always replaced with exactly
one card, never appended.

## Consequences

A card that changes what a Pokémon *is* without displacing it swaps
`PokemonInPlay.cards` directly, distinct from every switch, which moves
`PokemonId`s between `active` and `bench`. A future card doing the same
to an evolved stack (not only a Basic) would need to decide which
layers of the stack move and which stay — this card never asks that
question, since Basic-to-Basic has only one card to replace.
