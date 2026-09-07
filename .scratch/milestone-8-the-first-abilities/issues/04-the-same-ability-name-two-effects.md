# The same Ability name naming two effects

Type: task
Status: resolved

`Kadabra` and `Alakazam` both print an Ability named "Psychic Draw",
triggered *"when you play this Pokémon from your hand to evolve 1 of your
Pokémon."* Kadabra draws 2; Alakazam draws 3. 40 and 30 slots.

- [x] `Action::Evolve` offers a triggered Ability the moment the evolution
      is played, the same shape ticket 03 gave `Action::PlayBasic`
- [x] A name-safety check for Abilities exists, the way
      `check_trainer_name_safety.py` does for Trainers, and is run against
      the pool
- [x] `Kadabra` draws 2 and `Alakazam` draws 3 — the same name, resolved to
      two different effects, by whatever the check above says is needed

Recorded in [ADR 0072](../../../docs/adr/0072-the-same-ability-name-resolves-by-species-alone.md).

## Resolution

No print-override table needed: `known_ability` was already keyed by
`(pokemon_name, ability_name)` from ticket 01, and Kadabra and Alakazam
are different species — two ordinary table entries, no collision.
`tools/check_ability_name_safety.py` checks the real collision case (the
*same* species printing the *same* Ability name two different ways) and
found two unrelated ones already in the pool (`Golduck`/`Psyduck`'s
`Damp`, genuinely reworded across prints; `Delphox`'s `Flaring Magic`,
a cosmetic symbol-vs-word rendering difference) — neither touches
`Kadabra` or `Alakazam`, and neither is built yet, so both are noted
for whichever future ticket reaches them rather than fixed here.

`AbilityEffect::WhenEvolvedFromHandMayDrawCards(u32)`, hooked to
`Action::Evolve` finishing — `trigger_last_ditch_catch`'s sibling,
sharing `Limit::AbilityUsed` unchanged.

Admits Kadabra's me01-055 print (its own attack, `Super Psy Bolt`, has
no printed text). `Alakazam`'s own prints still need `Powerful Hand`
(a damage-counter count read from hand size) or `Strange Hacking`/
`Psychic` (a damage bonus read from the defender's own attached
Energy) — deferred to their own tickets.

Coverage: `admitted` 588 -> 589.
