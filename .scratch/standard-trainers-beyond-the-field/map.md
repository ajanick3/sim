# Map: every Standard Trainer and Special Energy, beyond the field

## Destination

`coverage` shows 0 refused Trainers and 0 refused Special Energy.

## Notes

`known_trainer` in `src/import.rs` matches a card by name to a
`(Requirement, TrainerEffect)`. `known_trainer_by_print` overrides by
print id where two prints of one name differ. `resolve_trainer` in
`src/engine.rs` runs the effect; `legal_actions` gates the card. Adding a
plain card is a `known_trainer` line plus, where the effect is new, a
`TrainerEffect` variant and its `resolve_trainer` arm.

The refused Supporters split roughly into: plain draw, shuffle-and-draw
variants, deck searches, discard-pile retrieval, heals, switches, and
opponent-hand disruption — most map onto an effect the engine already
has, in a new shape.

## Tickets

- 01 — plain-draw Supporters (`Cheren`, `Friends in Paldea`, `Urbain`):
  a `TrainerEffect::Draw(u32)`.

## Decisions so far

Ticket 01 resolved 2026-09-10: `TrainerEffect::Draw(u32)` admits the plain-draw
Supporters (`Cheren`, `Friends in Paldea`, `Urbain`), and `progress_table` now
prints a Standard-coverage summary; details under [the ticket's Answer](issues/01-plain-draw-supporters.md).

Supporter clusters merged (each a PR, TDD, guards + README moved):
- #248 plain draw — `Draw(u32)`.
- #249 heals — `HealActive` reskin, `HealEachYours { amount, of_type }`.
- #250 draw variants — `CoinFlipDraw`, `DrawPerOpponentMegaEx`, `DrawUpToHandSize`.
- #251 hand refresh — `DiscardHandThenDraw`, `BothShuffleHandThenDraw` reskin, `Decide` last-card.
- #252 deck search — `Decide` reskin, new `CardFilter::PokemonOfType`.
- #253 discard retrieval — `Decide` reskin, new `TargetFilter::OfType`.
- #254 conditional draw — `DrawThenBonusIf*`, `DrawPerPokemonInOpponentHand`.

Standard Supporters at #254: 25 of 78 names.
Still deferred: next-turn restrictions (Roxie's, Jasmine's Gaze, Acerola's),
name-prefix "X's Pokémon" searches (Ethan's Adventure, Team Rocket's *),
peek-and-discard-rest (Explorer's Guidance, Drayton), Tyme's HP-guess minigame,
Salvatore (evolve from deck), the "first turn allowed" flag (Carmine's rider,
Team Rocket's Proton), Amarys' end-of-turn discard.
