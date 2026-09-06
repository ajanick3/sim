# The first trainer effects

Type: task
Status: resolved
Blocked by: 02

Build the primitives ticket 02 named, and the Trainers that need only them.

- [x] An effect is a value the engine executes, never text read at run time
- [x] A mid-effect choice is a phase, with its own legal actions
- [x] The Trainers built are playable in a game, once per turn for a Supporter
- [x] The coverage count and both committed decks are measured after

## Answer

Resolved 2026-09-06 on branch `feat/the-first-trainer-effects`, on top of
the resolution machinery from `feat/resolution-phases` (the operator's own
design decisions: `Phase::Deciding` generalized from six primitives, and
`Promoting` split into `of`/`chooser`).

`CardDef::Trainer(Trainer { print_id, name, kind, effect })` carries a
`TrainerEffect` value — never text read at run time (ADR 0009).
`Action::PlayTrainer` discards the card, sets the once-per-turn flag for a
Supporter or a Stadium, and dispatches on the effect. Six of the eight open
`Phase::Deciding`; `Boss's Orders` opens the generalized `Promoting`; `Judge`
and `Lillie's Determination` need no phase at all.

One phase this ticket needed that neither prior design anticipated:
`Phase::DiscardingOpponentEnergy`, for `Crushing Hammer`'s heads case. A
Pokémon's attachments are not a `Zone` — they are indexed by `PokemonId`, not
a player's zone list — so `Deciding` could not express "discard one Energy
attached to any of the opponent's Pokémon in play." Recorded as its own
minimal phase rather than stretched into `Deciding`.

All eight cards are wired to the real artifact, by name, in
`known_trainer_effect`: each has exactly one distinct effect across every
printing in the pool today, which was checked before relying on it, not
assumed.

**Coverage moved from 346/3051 (11.3%) to 368/3051 (12.1%)** — 22 real
prints across the eight names. Both committed decks moved further, since
Trainers are the largest blocker in every real deck:

| Deck | Before this ticket | After |
| --- | --- | --- |
| `decks/2026-worlds/03-brent-tonisson.txt` | 4/60 (7%) | 23/60 (38%) |
| `decks/2026-worlds/07-mateusz-laszkiewicz.txt` | 0/60 (0%) | 12/60 (20%) |

Written test first throughout: `tests/trainers.rs` (11 tests, engine-level,
against a synthetic set) and one test in `tests/import.rs` confirming all
eight are admitted from the real artifact.

Three tests written before this ticket assumed every admitted card is a
Pokémon, or hardcoded a coverage count that this ticket correctly moved:
`tests/prizes.rs` (two `.as_pokemon().unwrap()` calls, now `filter_map`),
`tests/card_types.rs`, and `tests/card_records.rs`. Each was updated to
reflect the new true count, with a comment saying why it moved.
