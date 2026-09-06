# Map: the second batch of Trainers

## Destination

The nine most-played unbuilt Trainers play, and the primitives they need
exist: a search into play, a filter on stage and HP, a requirement paid on
play, a move between attachments, and a two-link evolution chain.

## Notes

What each of the nine needs that the first eight did not:

| Card                 | Slots | Needs                                     |
| -------------------- | ----- | ----------------------------------------- |
| Buddy-Buddy Poffin   | 169   | search into play; a stage and HP filter   |
| Ultra Ball           | 160   | a requirement paid to play                |
| Crispin              | 67    | two cards to two places, with a constraint |
| Energy Switch        | 45    | a move between two Pokémon's attachments  |
| Dawn                 | 44    | a stage filter, one of each               |
| Hilda                | 41    | a stage filter                            |
| Rare Candy           | 40    | an evolution chain two links long         |
| Cyrano               | 37    | a filter for a Pokémon with a Rule Box    |
| Special Red Card     | 36    | a requirement read from the opponent      |

`Cyrano` is the cheapest of the nine: "up to 3 Pokémon ex" is
`prizes > 1`, the inverse of a filter that already exists.

## Decisions so far

- **A filter is a flat, named variant of `CardFilter`, not a combinator.**
  `PokemonEx` and `BasicPokemonWithHpAtMost(u32)` each name a condition a
  card actually prints. A general "and" of conditions would be more
  expressive than any card in this pool needs.
- **A search for one of each of several things is one primitive, not a
  filter.** `Hilda`, `Dawn`, and `Crispin` each search for two or three
  *different* cards, one of each. `Decide` carries one filter and one limit,
  so no filter can express them. Ticket 05 covers all three; ticket 02 gave
  up its `Hilda` line to it.

## Fog

- `Rare Candy` names a Stage 2 that evolves from a Basic, and `evolve_from`
  holds one link only: a Stage 2 names the Stage 1 it comes from, never the
  Basic under it. Whether the chain is walked at load or at play is the
  ticket's question.
- `Crispin` searches for two Basic Energy *of different types*. A constraint
  between two choices is a shape `Phase::Deciding` has never had to hold.
- Checking that a name is safe to match on compares the printed text of every
  print. That check reports a false difference on whitespace alone:
  `Ultra Ball` looks like two cards and is one. Normalise before comparing.
- `Area Zero Underdepths` is the tenth most-played and out of scope: it
  changes the Bench size while a player has a Tera Pokémon in play, which
  needs continuous rules and a Tera concept.
- Rule 3, one ACE SPEC per deck, can be checked after all. `decklist.rs`
  calls it uncheckable and pokemontcg.io answers it — see
  [card data findings](../../docs/architecture/card-data.md). It is a data
  question rather than a Trainer one, so it belongs to its own effort and not
  to this milestone's spec.
