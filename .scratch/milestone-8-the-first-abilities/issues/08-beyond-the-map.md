# Beyond the map

Type: task
Status: open

Ticket 07 closed the map's own planned order (01-07). This session
then kept sweeping `cargo run --bin blockers`'s `Ability` bucket for
names reachable from an already-built or closely-mirrored shape, the
same way Milestone 11 continued past its own spec:

- **Abra**'s `Teleporter` (`OncePerTurnWhileActiveMayShuffleSelfIntoDeck`)
  — `Run Away Draw` minus the draw. Admits sv06-080.
- **Dusclops**'s and **Dusknoir**'s `Cursed Blast`
  (`OncePerTurnMayDamageOpponentThenKnockOutSelf`) — the first Ability
  to Knock out its own carrier outright, modeled by raising its damage
  to effective HP rather than a separate forced-knockout primitive
  (ADR 0076). Completes both species (6 prints).

Coverage: 604 -> 611 across these two PRs.

## What is left

Per `blockers`, still real and not yet attempted: `Pecharunt ex`
(switch-and-Poison), `Chien-Pao`/`Iron Leaves ex` (a play-triggered
switch, not yet built from an Ability), `Genesect ex` (a standing
search for Evolution Metal Pokémon), `Blaziken ex` (attach Energy
*from discard*, not hand), `Tatsugiri` (a bounded top-of-deck peek —
the same shape ticket 08 of the milestone-8 spec's own "Out of scope"
already named for `Drakloak`), `Toxtricity` (search-and-attach with a
damage side effect), `Kyurem` (an attack unlocked by a board fact),
`Psyduck`/`Patrat` (each a static, continuous effect — the milestone's
own "Out of scope" bucket). None built here; read fresh against the
pool before assuming refusal, the same discipline every earlier
ticket held to.
