# Revealing a hand lands in the log alone, with no state to change

**Status:** Accepted — 2026-09-07

`Hoothoot`'s `Silent Wing` reads "Your opponent reveals their hand" —
the milestone's first attack that reads from the opponent's hand
rather than moving cards in or out of it. The engine already tracks
every zone in full for every player; `view::PlayerView` hides a hand's
contents from the opponent only at render time, and nothing in
`legal_actions` ever gates a choice on hidden opponent-hand knowledge
the way a real multiplayer client would need to. A "reveal" that
persisted — flipping a flag `PlayerView` reads, cleared at the next
`begin_turn` — was the other shape considered, matching how a real
table moment lasts until the next look. It was rejected: nothing reads
`PlayerView` mid-resolution to make an engine choice, so a persisted
flag would carry state nothing in the engine, and no test, could ever
observe — the same "nothing to build against" reasoning that keeps
`AttackEffect::IgnoresDefendersEffects` a flag read once rather than a
lingering one. `AttackEffect::RevealOpponentsHand` logs the reveal and
changes nothing else. If a future `PlayerView` consumer needs the
reveal to persist, this record is the place to note it stopped
holding, per the ADR template's supersession rule.
