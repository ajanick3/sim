# Map: the Trainers still standing

## Destination

The seven cards the spec names play, and the primitives they need exist: a
search with no filter at all, a search bounded to the top of the deck, a
search returned to the top in a chosen order rather than shuffled, an
attach whose target is filtered rather than any Pokémon in play, a
requirement read from last turn, the player's own Active switched by
choice, and healing.

## Notes

Ordered so each ticket costs the least given what the last one built.

| Card                          | Slots | Needs                                       |
| ------------------------------ | ----- | --------------------------------------------- |
| Team Rocket's Petrel           | 33    | `CardFilter::AnyTrainer` — nothing else is new |
| N's PP Up / Wondrous Patch     | 46    | A filter on the Pokémon `Destination::Attach` may target |
| Pokégear 3.0 / Bug Catching Set| 36    | A search that reads only the first few cards of a zone |
| Ciphermaniac's Codebreaking    | 24    | `CardFilter::AnyCard`, and a destination that does not shuffle |
| Unfair Stamp                   | 25    | A requirement read from history, and an asymmetric shuffle-and-draw |
| Switch                         | 11    | `Phase::Promoting` opened by the player's own choice |
| Jumbo Ice Cream                | 26    | Healing, and a requirement read from a Pokémon's own attachments |

`Team Rocket's Petrel` is the cheapest real card: one new filter variant,
every other primitive already built. The last three tickets do not depend
on one another and may run in any order once the first four land.

## Decisions so far

- **`TargetFilter` is a value of its own, checked by
  `GameState::matches_target`, not a second case folded into `CardFilter`.**
  A `CardFilter` reads a card sitting in a zone; a `TargetFilter` reads a
  Pokémon already in play. `Wondrous Patch` needed both — a Psychic Energy
  onto a Psychic Pokémon — as two filters checked against two different
  values, not one combined expression. See
  [ADR 0021](../../docs/adr/0021-an-attach-can-filter-its-target.md).
- **The top-of-deck peek is `Slot::peek: Option<u32>`, not a new
  `Zone`.** A peek limit is a fact about the search, not about where a
  card sits — the same 7 cards are there whichever Trainer asks — and the
  cards not taken need no new shuffle rule: a plain whole-Library shuffle
  already lands on the same distribution as "shuffle only the cards seen
  back in." See
  [ADR 0022](../../docs/adr/0022-a-search-can-peek-a-bounded-prefix.md).

## Fog

- Whether Unfair Stamp's requirement — "a Pokémon of mine was Knocked Out
  during the opponent's last turn" — is a new field on `GameState`, cleared
  at `begin_turn`, or derivable from `history: Vec<Action>` without a new
  field. Milestone 7 asked the identical question for `Fezandipiti ex`'s
  Ability and left it open; whichever answer this ticket settles on
  applies there too, so record the reasoning, not only the result.
- Whether `Switch`'s new call site for `Phase::Promoting` needs the phase
  to change shape at all, or only needs a new way to enter the one that
  exists: `of == chooser`, opened by the player's own choice rather than by
  a knockout. If the shape already fits, say so and change nothing.
