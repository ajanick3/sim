# A raised Bench limit shrinks back through `settle`'s own sweep, not a dedicated hook

**Status:** Accepted — 2026-09-09

`Area Zero Underdepths` raises a player's own Bench limit from 5 to 8 while they have a Tera Pokémon in play, but the card's own text does not stop there: the moment a player's *last* Tera Pokémon leaves play, they must discard Bench Pokémon back down to 5 — and when the Stadium itself leaves play, *both* players do, its own owner first. The first half reads directly off the board, the same way `Nighttime Mine`'s attack surcharge already does (`GameState::bench_limit`). The second half is reactive: a Tera Pokémon can leave play by a Knockout, a switch effect, or several other routes this engine already models separately, and hooking every one of them individually would mean re-deriving the same check at each site.

## Decision

Read the reactive half from `settle`, the same general sweep `knock_out_the_dead` already is: every pass through `settle`'s own loop, before promotion is considered, check whether either player's Bench still exceeds `BENCH_LIMIT` while they no longer have a Tera Pokémon in play, and open `Phase::DiscardingBenchDownTo` the moment that is true. No call site needs to know it might have caused this — `settle` runs after every action that could have.

The Stadium-leaving-play half is narrower and is hooked directly, at the two sites a Stadium can leave play (replaced by a new one, or discarded by `Snow Sink`): a shared helper, `open_discard_bench_down_to_if_stadium_left`, checks whether the departing card was this one and opens the same phase, chained through `then` so the departing card's own owner discards first and the opponent follows.

`discard_benched_pokemon` is a new, separate function from `knock_out` — sharing the "move everything to discard" mechanics but not the Prize, `knocked_out` flag, or `knocked_out_last_turn` side effects a real Knockout carries. A Bench shrinking to fit a limit is not a Knockout.

## Consequences

`open_discard_bench_down_to` returns whether it opened a phase, rather than deciding for its caller whether to fall back to `Phase::Main` and `settle` — the two call sites need different answers (`Action::DiscardBenchedPokemon`'s own handler is ending an action-apply cycle and must settle; the Stadium-departure sites are mid-way through a larger `Action::PlayTrainer`/`Action::AcceptSnowSink` handler and must not call `settle` early). Any future card needing the same "shrink back once a standing modifier goes away" shape can reuse this split rather than re-deriving it.
