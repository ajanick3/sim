# `Night Joker` runs `attack_with` on a second card's own `Attack`

**Status:** Accepted — 2026-09-09

ADR 0070 refused `N's Zoroark ex` outright: `Night Joker` — "Choose 1
of your Benched N's Pokémon's attacks and use it as this attack" —
reads as needing `AttackEffect`'s fixed-value shape (ADR 0009) itself
restructured, the same problem `Seek Inspiration` was refused for
(ADR 0066). On a closer look this session, the two problems are not
the same. `Seek Inspiration` reads a card the engine has not yet
seen — the top of the library, discovered only at resolution time — so
running it in full would mean carrying every `AttackEffect`'s own
dispatch logic as data reachable from inside a single match arm.
`Night Joker` reads a card already in play, whose own `Attack` (cost,
`base_damage`, `inflicts`, `effect`) the engine already holds in full,
the moment the Bench does. Nothing about `AttackEffect` needs to
change shape — only where its value comes from.

`attack()` split into a thin wrapper (invulnerability, Confusion, and
the attacker's own printed `Attack` lookup) and `attack_with(state,
attacker, defender, attack)`, which carries everything downstream —
damage, `inflicts`, `resolve_attack_effect`. The wrapper calls
`attack_with` with the attacker's own printed `Attack`, ordinarily.
`Night Joker`'s own effect,
`AttackEffect::CopiesChosenBenchedPokemonAttackByNamePrefix`, is a
new variant, but a narrow one: it opens
`Phase::ChoosingBenchedPokemonAttackToCopy` (no candidate opens no
phase) rather than computing anything itself, and the action that
answers it — naming a Benched Pokémon and one of its own attack
indices — reads that Pokémon's own `Attack` and calls the exact same
`attack_with`. Every other `AttackEffect` variant's dispatch is
unchanged; `Night Joker` is the first attack whose own `Attack` value
comes from a different Pokémon's printed card, not a restructuring of
how any `Attack` is read once found.

`Trade`, deferred alongside `Night Joker` in ADR 0070 for lacking a
paired sample-deck card, is built now that one exists:
`AbilityEffect::OncePerTurnMayDiscardFromHandThenDrawCards(u32)` opens
`Phase::DiscardingHandCardThenDrawing`, spending the Ability's own
once-per-turn limit at the moment the discard is chosen — the
discard is the Ability's stated cost, not an optional follow-up the
way attaching Energy from discard already is for `Blaziken ex`'s
`Seething Spirit`.

## Errata

- 2026-09-09: [ADR 0070](0070-ns-zoroark-ex-is-refused-for-its-own-attack.md) is superseded by this record.
