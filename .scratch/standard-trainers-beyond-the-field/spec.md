# Spec: every Standard Trainer and Special Energy, beyond the field

Milestones 7–10 built every Trainer the competitive field plays, and 12
started the Special Energy work. `blockers` now reports the 61
`decks/2026-worlds/` decks at 100%.

This effort takes the engine the rest of the way: every remaining
Standard-legal Trainer and Special Energy, whether or not a deck plays it.
`coverage` at the open of this effort:

| Kind           | Refused prints |
| -------------- | -------------- |
| Supporter      | 125            |
| Item           | 68             |
| Tool           | 35             |
| Stadium        | 24             |
| Special Energy | 12             |

## Order

Supporters, then Items, then Tools, then Stadiums, then Special Energy —
the milestone order. Within a kind, the cards that reuse an effect the
engine already runs come first (a fast pass over the count), then the
cards that need a new effect variant, `Phase`, or decision.

## Rules

- Build every card, reskins included. A card that only reuses an existing
  `TrainerEffect` is one `known_trainer` line.
- A card that needs a new decision the player makes gets an ADR when that
  decision is first made.
- Every card admitted gets a test, written before the code, and an
  `..._is_admitted_from_the_artifact` check.
- The README progress table moves with each PR that changes what plays.
- Abilities (448) and attack-text Pokémon (1536) are out of scope here;
  they come after.
