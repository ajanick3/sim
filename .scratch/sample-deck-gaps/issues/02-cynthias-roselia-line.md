# Cynthia's Roselia → Roserade

Type: task
Status: needs-triage

A sample deck named this line. Neither stage builds.

Card text (`data/cards.json`, id `sv10-007`, `sv10-008`):

- Cynthia's Roselia — Spike Sting, 20 damage, no effect.
- Cynthia's Roserade — Cheer On to Glory ability: attacks by your
  Cynthia's Pokémon do 30 more damage to the opponent's Active, before
  Weakness/Resistance. Leaf Step, 80 damage, no effect.

## Acceptance criteria

- [ ] Both build a Pokémon.
- [ ] Cheer On to Glory reads as a name-prefix damage boost, mirroring
      the pattern noted for `Cynthia's Power Weight` in the
      standard-trainers-beyond-the-field map (name-prefix filters).
- [ ] A test attacks with a non-boosted Pokémon while Roserade is in
      play and confirms the +30 lands only from a Cynthia's attacker.
- [ ] README progress table updated.
