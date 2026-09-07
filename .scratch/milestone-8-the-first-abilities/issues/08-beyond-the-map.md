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

- **Genesect ex**'s `Metallic Signal`
  (`OncePerTurnMaySearchEvolutionPokemonOfType`) — a standing search
  for Evolution Pokémon of a type, `CardFilter::EvolutionPokemonOfType`
  reused from `EvolutionPokemon` narrowed by type, the same way
  `PokemonOfTypeOrBasicEnergyOfType` narrows `PokemonOrBasicEnergy`.
  Its own attack, `Protect Charge`, needed
  `AttackEffect::SelfDamageReductionNextTurn` — reduced damage taken
  through the opponent's next turn, after Weakness and Resistance.
  Building this uncovered a real bug in `opponent_next_turn_restriction`'s
  own clearing logic, fixed and recorded as an erratum on ADR 0077:
  a self-targeted restriction (the granting player's own Pokémon
  named as the target, as `CoinFlipSelfInvulnerableNextTurn` and this
  new effect both do) was cleared one turn too early, inferred from
  the wrong owner. Completes all 3 Genesect ex prints.

- **Blaziken ex**'s `Seething Spirit`
  (`OncePerTurnMayAttachBasicEnergyFromDiscardToChosen`) — attaching
  Energy from the discard pile to a chosen own Pokémon, the mirror of
  `Teal Dance`'s hand-to-self attach but from a different zone to a
  free choice of target. Its own attack reuses
  `AttackerCannotAttackNextTurn`. Admits its one print.

Coverage: 604 -> 615 across these four PRs.

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
