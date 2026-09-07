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

- **Chien-Pao** — its `Strafe`/`Rising Blade` print reuses
  `SwitchOwnActive` and `BonusDamageIfDefenderIsEx` outright. Its
  other prints pair `Icicle Loop` (`MoveOwnAttachedEnergyToHand`, puts
  one of the attacker's own attached Energy into hand) with
  `Snow Sink` (`WhenBenchedFromHandMayDiscardStadium`, a play-triggered
  Ability sharing `trigger_last_ditch_catch`'s trigger site).
  Completes all 3 prints.

- **Iron Leaves ex**'s `Rapid Vernier`
  (`WhenBenchedFromHandMaySwitchThenMoveAnyEnergy`) — a play-triggered
  switch (the newly-benched Pokémon may switch in for the Active),
  then moving any amount of Energy from the player's other Pokémon to
  the switched-in one, one card at a time until the player stops. Its
  own attack reuses `AttackerCannotAttackNextTurn`. Completes all 6
  prints.

- **Toxtricity**'s `Sinister Surge`
  (`OncePerTurnMaySearchBasicEnergyOfTypeAttachToBenchedThenDamage`) —
  search for a Basic Energy of a type, attach it to a Benched Pokémon
  of the same type (the player's only real choice — which Energy card
  is found is not, the same reasoning ADR 0068 gave for a fixed-count
  cost), then damage it. Completes all 3 prints.

- **Fan Rotom**'s `Fan Call`
  (`OnceDuringFirstTurnMaySearchPokemonOfTypeWithHpAtMost`, gated on
  `turn_number <= 1` — each player's own first turn, not only the
  game's very first) and its own attack, `Assault Landing`
  (`FizzlesWithNoStadiumInPlay`, a full short-circuit at the top of
  `attack` mirroring `CoinFlipSelfInvulnerableNextTurn`'s own shape).
  Completes all 4 prints.

- **Pecharunt ex**'s `Subjugating Chains`
  (`OncePerTurnMaySwitchBenchedOfTypeExcludingNamedThenPoison`) — a
  standing switch (no positional requirement, unlike `Run Errand`) of
  a Benched Pokémon of a type, excluding a name, then Poisoning the
  newly Active one. Its own attack needed `Count::OpponentPrizesTakenCount`.
  Completes all 5 prints.

Coverage: 604 -> 636 across these nine PRs.

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
