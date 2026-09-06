# A search that puts a pokemon into play

Type: task
Status: resolved

`Buddy-Buddy Poffin` searches the deck for up to 2 Basic Pokémon with 70 HP
or less and puts them **onto the Bench**. 169 slots, the most-played unbuilt
Trainer in the field.

`Phase::Deciding` moves a card between zones. The Bench is not a zone: it
holds Pokémon in play, and putting a card there makes a `PokemonInPlay`
rather than moving a `CardId`. That is the whole of this ticket.

- [x] A search can put a Pokémon into play, not only into a zone
- [x] The Bench limit of 5 is respected, and a full Bench ends the choice
- [x] `Buddy-Buddy Poffin` plays, and the decks are measured after

## Resolution

`Decide` and `Phase::Deciding` carry a `Destination` — either a `Zone` or the
Bench — rather than a `Zone`.
[ADR 0016](../../../docs/adr/0016-a-search-names-a-destination-not-a-zone.md)
records why the Bench did not become a fourth `Zone`.

A card sent to the Bench takes the path a Basic played from hand takes,
`GameState::put_into_play`, so it is stamped with the turn it arrived on and
is not a second kind of Pokémon. `legal_actions` offers nothing to take once
the Bench holds five, so a Bench that fills part-way through a two-card
search ends the choice rather than refusing the second card.

Coverage went 370 → 375 (five prints), and the field went 786 → 955 playable
slots of 3660 — 26.1%, up from 21.5%.
