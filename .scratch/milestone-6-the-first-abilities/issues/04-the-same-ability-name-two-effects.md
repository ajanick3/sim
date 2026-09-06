# The same Ability name naming two effects

Type: task
Status: ready-for-agent

`Kadabra` and `Alakazam` both print an Ability named "Psychic Draw",
triggered *"when you play this Pokémon from your hand to evolve 1 of your
Pokémon."* Kadabra draws 2; Alakazam draws 3. 40 and 30 slots.

ADR 0020 solved exactly this shape for Trainers: match by name unless a
print overrides it, checked by
`tools/check_trainer_name_safety.py`. This is the first real Ability that
needs the same choice made on its own terms — an Ability lives on a
`Pokemon`, not a `Trainer`, so whatever answers this reads from that.

- [ ] `Action::Evolve` offers a triggered Ability the moment the evolution
      is played, the same shape ticket 03 gave `Action::PlayBasic`
- [ ] A name-safety check for Abilities exists, the way
      `check_trainer_name_safety.py` does for Trainers, and is run against
      the pool
- [ ] `Kadabra` draws 2 and `Alakazam` draws 3 — the same name, resolved to
      two different effects, by whatever the check above says is needed
