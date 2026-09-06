# A filter on stage and hp

Type: task
Status: ready-for-agent

Three of the nine read a Pokémon's stage or its HP: `Buddy-Buddy Poffin`
wants a Basic with 70 HP or less, `Hilda` an Evolution, `Dawn` a Basic and a
Stage 1 and a Stage 2. `Cyrano` wants a Pokémon ex, which is `prizes > 1`.

Evolution gave the engine `evolve_from`, so a stage is knowable: `None` is a
Basic. Nothing yet reads it as a filter.

- [ ] `CardFilter` can name a stage, and an HP at most
- [ ] `Cyrano` is built, being the cheapest of the nine
- [ ] `Hilda` is built
