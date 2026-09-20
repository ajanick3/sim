# Cynthia's Gible → Gabite → Garchomp ex

Type: task
Status: needs-triage

A sample deck named this line. None of the three stages build.
`src/import.rs` has no match for `Cynthia's Gible`, `Cynthia's Gabite`,
or `Cynthia's Garchomp ex`.

Card text (from `data/cards.json`, id `sv10-102`, `sv10-103`,
`sv10-104`):

- Cynthia's Gible — Rock Hurl, 20 damage, ignores Resistance.
- Cynthia's Gabite — Champion's Call ability: once per turn, search the
  deck for a Cynthia's Pokémon to hand, then shuffle. Dragonslice, 40
  damage, no effect.
- Cynthia's Garchomp ex — Corkscrew Dive: 100 damage, draw to 6 cards
  in hand. Draconic Buster: 260 damage, discard all Energy from this
  Pokémon.

## Acceptance criteria

- [ ] All three build a Pokémon in `known_pokemon` (or its successor).
- [ ] Champion's Call searches for a Cynthia's Pokémon by name prefix,
      matching the `IncreasesHpForNamePrefix` pattern already used for
      `Cynthia's Power Weight`.
- [ ] Corkscrew Dive's draw-to-6 and Draconic Buster's self-discard each
      have a test.
- [ ] README progress table updated.
