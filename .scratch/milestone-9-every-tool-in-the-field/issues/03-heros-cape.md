# Hero's Cape

Type: task
Status: resolved

*"The Pokémon this card is attached to gets +100 HP."*

Follows [ADR 0042](../../../docs/adr/0042-a-static-effect-is-read-not-dispatched.md)'s
shape exactly: no new ADR needed. New `GameState::effective_hp`, and
`remaining_hp` now derives from it instead of the printed value
directly. `view.rs` and the terminal display (`main.rs`) both switched
from `pokemon_def(id).hp` to `effective_hp(id)`, since both are in-play
reads.

- [x] Adds 100 to effective HP
- [x] `remaining_hp` grows with it — a Pokémon can survive damage past
      its printed HP

## Resolution

One new `TrainerEffect`, one new `GameState` method, three read sites
switched. `CardFilter::BasicPokemonWithHpAtMost` (Buddy-Buddy Poffin)
is untouched — it reads a card sitting in a zone, which has no Tool
attached to raise it, so it stays the printed value, per ADR 0042.

Coverage: `admitted` 486 -> 487 (1 print); `trainers` (refused)
305 -> 304.
