# One shape for the limits

Type: task
Status: resolved

Give the once-per-turn and once-per-game limits one shape.

`evolved_this_turn` sits on a Pokémon, `supporter_played_this_turn` on a
player, an Ability used this turn is not recorded anywhere, and neither is a
once-per-game marker. Four facts of one kind, in four shapes or none.

- [x] One way to ask whether a limit has been spent, and to spend it
- [x] The limits already built read through it, and behave as they did
- [ ] ~~An Ability used this turn has somewhere to be recorded~~ — not
      built, see the Answer

## Answer

Resolved 2026-09-06 on branch `feat/one-shape-for-the-limits`. Written test
first, `tests/limits.rs`, five tests.

`Limit` names what may be done once in a turn, and names its own owner:
`EnergyAttached`, `Retreated`, `SupporterPlayed` and `StadiumPlayed` take a
`PlayerId`; `Evolved` takes a `PokemonId`, because rule 20 belongs to a
Pokémon rather than to a player. `GameState.spent` is the one list, with
`is_spent` and `spend` either side of it. Five booleans across two structs
became one list and one type.

`begin_turn` clears every limit, not only the current player's. That is the
same game: every gate asks about whoever is acting, so the opponent's were
unreadable during this turn anyway.

**Two things the ticket asked for that were not built**, both because the
pool was checked first — the lesson from the Lost Zone, one ticket earlier:

- **A once-per-game scope.** `VSTAR` and `GX` are zero cards in marks H, I
  and J. Exactly one card in the pool reads "once per game", `Legacy Energy`,
  and it is a special Energy the engine refuses. A game-scoped list is the
  same shape with a list that is never cleared; it is one line when a card
  needs it, and nothing does.
- **An `AbilityUsed` limit.** 589 cards carry an ability and the engine runs
  none of them. The variant is one line when Abilities arrive. Adding it now
  would be a marker nothing sets and nothing reads.

The replay fingerprint now covers `spent`. It never covered the five
booleans, so a replay that diverged on a limit would have passed — the same
gap closed for the Stadium in ticket 03.
