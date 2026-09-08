# Team Rocket's Watchtower reads `abilities_disabled` at every site

**Status:** Accepted — 2026-09-08

## Context

`Team Rocket's Watchtower` turns off every Ability in play, both
players', for as long as it stands. The engine already had one
Ability-disabling Stadium precedent, `Nighttime Mine`-style standing
effects read directly at their own site rather than dispatched
through a choice (ADR 0079). But no earlier card touched *every*
Ability at once: each prior passive effect (`Damp`, `Watchful Eye`,
`Skyliner`) named one specific Ability effect, read at its own one or
two call sites. Watchtower's own text has no such scope — it needs a
gate checked everywhere a standing Ability effect is read at all:
the Ability-offering loop in `legal_actions`, `effective_retreat_cost`,
`damage_counter_movement_blocked`, `self_knockout_abilities_disabled`,
the ex-immunity check in damage calculation, and the four
played-from-hand trigger functions.

A second question came up while testing: what happens when
`Battle Cage` is in play and `Munkidori`'s `Adrena-Brain` tries to
*move* damage counters onto a Benched Pokémon Battle Cage protects?
The genuine ruling (`compendium.pokegym.net`) says the counters still
leave the source — the move already started — but vanish rather than
landing, since Battle Cage blocks only the placement half. The
engine's first cut filtered a blocked Bench target out of
`legal_actions` entirely, which silently changed the ruling: it left
the counters sitting on the source instead of removing them.

## Decision

`GameState::abilities_disabled()` reads
`TrainerEffect::AbilitiesDisabled` off the current Stadium, the same
shape `tools_disabled()` and `abilities_disabled()`'s Battle-Cage
sibling `bench_damage_counters_blocked()` already use. Every site
above now checks it before offering, triggering, or reading a
standing Ability effect — there is no single choke point, because
Abilities are read from that many independent places already.

For the `Battle Cage`/`Adrena-Brain` interaction, `legal_actions`
keeps offering a blocked Bench target: the move is still legal to
attempt. `apply`'s own handler for
`Action::MoveDamageCountersFromOwnToOpponent` always removes the
counters from `source`, then checks
`bench_damage_counters_blocked(target)` before adding them to
`target` — blocked, they vanish instead of landing. `Battle Cage`'s
other gated effect, `Dragapult ex`'s `DamageCountersToOpponentBenchAnyWay`,
keeps its `legal_actions`-side filter and its any-legal-target guard
at the phase-opening site: that effect places fresh counters rather
than moving existing ones, so a fully blocked Bench has nothing to
place and nothing to vanish either.

## Consequences

A future Stadium or Ability that disables only *some* Abilities (by
name, by type, by owner) cannot reuse `abilities_disabled()` as a
single gate — it would need its own predicate threaded through the
same set of sites. Watchtower's own scope (everyone, everything) is
what let one flag cover every site; a narrower effect earns its own
audit of the same list.

## Errata

**2026-09-08.** Two facts this record stated were wrong, both found
by reading the printed rulings compendium after this record was
first written.

Watchtower's own printed text reads "**`{C}` Pokémon** in play (both
yours and your opponent's) have no Abilities" — not every Pokémon,
as the Context and Decision sections above said. The engine's first
cut disabled Abilities stadium-wide, with no read of the carrying
Pokémon's own type at all. Fixed: `abilities_disabled()` became
`abilities_disabled_for(id: PokemonId)`, reading `id`'s own printed
type; every call site now checks the specific Pokémon an Ability
would come from, not a single flag. A Pokémon of any type but
Colorless keeps its Ability under Watchtower.

Separately, `Battle Cage`'s own `bench_damage_counters_blocked`
blocked a placement onto *either* side's Bench, with no read of who
was placing it. The real ruling (Gardevoir ex's `Psychic Embrace`,
placing its own damage counters on its own Benched Psychic Pokémon)
confirms Battle Cage blocks only a *cross-side* placement — an
effect still places normally onto its own side's Bench. Fixed:
`bench_damage_counters_blocked` takes the acting player (`by`) and
reads `false` outright when `by` owns `target`. No admitted card
this session ever placed a counter on its own side's Bench, so this
one never changed behavior for a built card — only the general
helper's own correctness.
