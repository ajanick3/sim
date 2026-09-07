# The same Ability name resolves by species alone, no print-override table

**Status:** Accepted — 2026-09-07

`Kadabra` and `Alakazam` both print an Ability named `Psychic Draw` —
Kadabra draws 2, Alakazam draws 3. ADR 0020 gave Trainers a
print-id-override table (`known_trainer_by_print`) for exactly this
shape: two prints of the same name needing different effects. The
map's own fog asked whether Abilities need the same table, or their
own, since an Ability lives on a `Pokemon`, not a `Trainer`.

They do not need one. `known_ability` was already keyed by
`(pokemon_name, ability_name)` from ticket 01 onward, the same
`(pokemon_name, attack_name)` shape `known_attack` already took — and
`Kadabra` and `Alakazam` are different species names. The "same name"
collision Trainers can hit (one card's own name printed twice, across
sets, with different text) cannot happen here unless the same species
printed the same Ability name two different ways, which no card in
the pool does yet. `AbilityEffect::WhenEvolvedFromHandMayDrawCards(u32)`
carries the count as its own parameter, so `("Kadabra", "Psychic
Draw")` and `("Alakazam", "Psychic Draw")` are simply two ordinary
table entries, resolved the same way any other pair of species with
different attacks already is.

The trigger itself — "when you play this Pokémon from your hand to
evolve 1 of your Pokémon" — is `WhenBenchedFromHandMaySearchSupporter`'s
sibling (ADR 0071), hooked to `Action::Evolve` finishing rather than
`Action::PlayBasic`, and sharing `Limit::AbilityUsed` the same way.
