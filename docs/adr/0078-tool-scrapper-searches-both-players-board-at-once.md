# `Tool Scrapper` searches both players' boards in one phase

**Status:** Accepted — 2026-09-08

`Tool Scrapper` — "Choose up to 2 Pokémon Tools attached to Pokémon
(yours or your opponent's) and discard them" — is the first card to
choose from cards attached across *both* players' boards at once.
Every earlier "up to N, choose which" search (`Phase::SearchingLibraryForBasics`,
`Phase::SearchingLibraryForEvolutionPokemonOfType`, and the rest)
scoped itself to the player's own zones; `TargetFilter` and
`CardFilter` likewise never needed to look at the opponent's side to
answer whether a card qualifies.

`Phase::DiscardingToolsAnywhere` carries only `player` (who is
choosing) and `remaining` — no zone or side, since the search itself
spans every Pokémon either player controls. `legal_actions` walks
`[PlayerId::One, PlayerId::Two]` directly rather than reading from one
`PlayerState`, and `Action::DiscardToolAnywhere`'s own handler finds
which player currently owns the chosen card by searching both sides'
`in_play()`, discarding it to *that* Pokémon's own owner — not
necessarily the player using `Tool Scrapper`. No new `CardFilter` or
`TargetFilter` variant was needed: "is a Tool" is answered directly
from `TrainerKind::Tool`, the same check `PlayTool`'s own gate already
makes.
