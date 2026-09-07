# A requirement can read history, not only the board as it stands

**Status:** Accepted — 2026-09-07

Every `Requirement` before `Unfair Stamp` read the board at the moment a
card might be played — the hand's size, the opponent's Prizes. `Unfair
Stamp` reads *"if any of your Pokémon were Knocked Out during your
opponent's last turn"* — a fact about what happened, not about what is
true right now. The board a moment after a knockout looks the same whether
it happened last turn or five turns ago; only history tells them apart.

The alternative was to derive the fact from `history: Vec<Action>`, which
the engine already keeps in full and was built exactly to answer questions
like this without new state (ADR 0013). It was rejected here: no `Action`
records a knockout as its own event. `Attack` can cause one, `EndTurn` can
settle one a pending attack left queued, and a replay would have to
re-simulate every action since the last turn boundary to notice one,
every time the fact is asked — a search, not a read. `Requirement`'s other
two variants are both a comparison against a number already sitting in
`GameState`; this one earns the same shape, `knocked_out_last_turn: [bool;
2]`, rather than becoming the one exception that reconstructs an answer
from the log.

The field is set the moment a knockout happens — the same place a Prize
is taken — and cleared once, for the player it belongs to, when *their*
own turn ends. Clearing there rather than when their turn begins is what
leaves it true for the whole turn right after the knockout and false again
one full cycle later, which is the entire rule.

## Consequences

`GameState` grows a field only one card reads. `BothShuffleHandThenDraw`
grew `you` and `opponent` in the same change, since `Unfair Stamp` needed
the two counts to differ where `Judge` never did — a rename with one
caller, not a new variant beside it.
