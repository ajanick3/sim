# An Ability a player opts into

Type: task
Status: ready-for-agent

`Mega Kangaskhan ex`'s Ability, "Run Errand": *"Once during your turn, if
this Pokémon is in the Active Spot, you may use this Ability. Draw 2 cards.
You can't use more than 1 Run Errand Ability each turn."* 63 slots, the
most-played Ability in the field, and the simplest shape one comes in: no
cost, no target, one trigger, one effect.

Nothing in the engine reads an Ability today. `Refusal::HasAnAbility`
refuses every card that carries one, the same way for all of them. This
ticket builds the first Ability primitive and admits the first card that
needs it.

- [ ] A Pokémon can carry an Ability the engine runs, the way a Trainer
      carries an effect
- [ ] A player may use it once during their own turn, and not the
      opponent's turn or a turn already spent
- [ ] Whether the limit is keyed by the Ability's name or by the Pokémon is
      checked against the pool, not assumed, and the decision is recorded
- [ ] `Mega Kangaskhan ex` plays, offered only while it is the Active
      Pokémon
