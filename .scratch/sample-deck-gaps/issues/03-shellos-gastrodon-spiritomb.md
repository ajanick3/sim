# Shellos → Gastrodon, and Cynthia's Spiritomb

Type: task
Status: needs-triage

A sample deck named these three. None build.

Card text (`data/cards.json`, id `sv08-046`, `sv08-107`, `sv10-129`):

- Shellos — Sprinkle Water, 30 damage, no effect.
- Gastrodon — Sticky Bind ability: while on the Bench, Benched Stage 2
  Pokémon (both players') have no Abilities. Mud Shot, 80 damage, no
  effect.
- Cynthia's Spiritomb — Raging Curse: 10 damage per damage counter on
  all Benched Cynthia's Pokémon (yours), ignores Weakness.

## Acceptance criteria

- [ ] All three build a Pokémon.
- [ ] Sticky Bind is a Bench-only Ability that blanks a Stage 2's
      Abilities on both sides — check whether an existing
      Ability-blank effect covers "Stage 2" as the filter, or add one.
- [ ] Raging Curse's per-counter damage across the Bench, filtered to
      a name prefix, has a test.
- [ ] README progress table updated.
