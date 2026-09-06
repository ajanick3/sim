# The missing zones

Type: task
Status: resolved

Add the Lost Zone and the Stadium in play.

`stadium_played_this_turn` already exists with nowhere to put the Stadium. A
Lost Zone card has nowhere to go at all, so no card that uses one can be
built.

- [x] ~~`Zone` holds the Lost Zone~~ — withdrawn, see the Answer
- [x] A game holds the Stadium in play, and a new one replaces the old
- [x] A Stadium a player plays is not discarded on play, unlike an Item

## Answer

Resolved 2026-09-06 on branch `feat/the-missing-zones`. Written test first,
`tests/zones.rs`, four tests.

`GameState.stadium: Option<(PlayerId, CardId)>` holds the Stadium in play.
The player is kept beside the card because rule 58 sends the one already
there to *its own owner's* discard, not to the discard of whoever replaced
it. Playing a Stadium no longer discards it, unlike an Item or a Supporter.

Rule 59 came with it and cost one condition in `legal_actions`: a Stadium
whose name is already in play is not offered.

The replay fingerprint in `tests/replay.rs` now covers the Stadium. Without
that, a replay bug touching it would have passed unnoticed — the fingerprint
is the definition of "the same state", so a state it does not read is a state
a replay does not check.

**The Lost Zone was built and then removed the same day.** The operator said
it is not part of this format, and the data agrees: **no card of marks H, I,
or J mentions the Lost Zone at all.** It came from `ptcg-sim`'s zone list,
which serves older formats too, and it was taken on that survey's word
without being checked against this pool — the check this project applies to
everything else. `Zone`, `PlayerState`, `PlayerView`, the fingerprint, and
its test are back to what they were. What remains is the Stadium, which 49
cards in the pool need.

`TrainerEffect::Nothing` exists for a Stadium whose placement is its whole
effect, and `known_trainer_effect` never produces it: a real Stadium carries
a continuous rule the engine cannot run, and ADR 0008 refuses a card it
cannot run in full. The synthetic Stadiums in the test are test doubles.
