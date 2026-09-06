# The missing zones

Type: task
Status: resolved

Add the Lost Zone and the Stadium in play.

`stadium_played_this_turn` already exists with nowhere to put the Stadium. A
Lost Zone card has nowhere to go at all, so no card that uses one can be
built.

- [x] `Zone` holds the Lost Zone, and a card can be moved there
- [x] A game holds the Stadium in play, and a new one replaces the old
- [x] A Stadium a player plays is not discarded on play, unlike an Item

## Answer

Resolved 2026-09-06 on branch `feat/the-missing-zones`. Written test first,
`tests/zones.rs`, four tests.

`Zone::LostZone` and `PlayerState.lost_zone` make the Lost Zone an ordinary
zone: `Phase::Deciding` moves a card there with no new machinery, since a
zone was all it ever needed.

`GameState.stadium: Option<(PlayerId, CardId)>` holds the Stadium in play.
The player is kept beside the card because rule 58 sends the one already
there to *its own owner's* discard, not to the discard of whoever replaced
it. Playing a Stadium no longer discards it, unlike an Item or a Supporter.

Rule 59 came with it and cost one condition in `legal_actions`: a Stadium
whose name is already in play is not offered.

Two things beyond the ticket, both because leaving them would have been
quietly wrong:

- The Lost Zone joined `PlayerView`. It is public, like the discard pile, and
  a bot that could not see it would be reading a false board.
- The replay fingerprint in `tests/replay.rs` now covers the Lost Zone and
  the Stadium. Without that, a replay bug touching either would have passed
  unnoticed — the fingerprint is the definition of "the same state", so a
  state it does not read is a state a replay does not check.

`TrainerEffect::Nothing` exists for a Stadium whose placement is its whole
effect, and `known_trainer_effect` never produces it: a real Stadium carries
a continuous rule the engine cannot run, and ADR 0008 refuses a card it
cannot run in full. The synthetic Stadiums in the test are test doubles.
