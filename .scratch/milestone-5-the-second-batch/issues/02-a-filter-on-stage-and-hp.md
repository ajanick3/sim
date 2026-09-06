# A filter on stage and hp

Type: task
Status: resolved

Three of the nine read a Pokémon's stage or its HP: `Buddy-Buddy Poffin`
wants a Basic with 70 HP or less, `Hilda` an Evolution, `Dawn` a Basic and a
Stage 1 and a Stage 2. `Cyrano` wants a Pokémon ex, which is `prizes > 1`.

Evolution gave the engine `evolve_from`, so a stage is knowable: `None` is a
Basic. Nothing yet reads it as a filter.

- [x] `CardFilter` can name a stage, and an HP at most
- [x] `Cyrano` is built, being the cheapest of the nine
- [ ] ~~`Hilda` is built~~ — moved to ticket 05, see below

## Hilda does not belong here

`Hilda` searches for "an Evolution Pokémon **and** an Energy card": two
cards, two different filters, one of each. No filter expresses that, because
`Decide` carries one filter and one limit. `Dawn` (a Basic, a Stage 1, and a
Stage 2) and `Crispin` (two Energy of different types, to two places) have
the same shape. The three are one primitive — a search with a sequence of
slots — so ticket 05 now covers all three, and this ticket built only the
filter vocabulary they will each name.

## Resolution

`CardFilter` gained two variants:

- `PokemonEx` — a Pokémon worth more than 1 Prize. ADR 0010 reads the prize
  value from the name, so in this pool a card worth more than 1 is exactly a
  card printed `ex`.
- `BasicPokemonWithHpAtMost(u32)` — both halves printed on the card that
  reads them. Ticket 01 gives it a destination and `Buddy-Buddy Poffin`
  plays.

`Cyrano` plays. Coverage went 368 → 370 (both prints), and the field went
749 → 786 playable slots of 3660.
