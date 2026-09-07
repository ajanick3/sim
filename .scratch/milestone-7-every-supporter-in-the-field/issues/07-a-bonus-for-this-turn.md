# Black Belt's Training and Gladion's Final Battle

Type: task
Status: ready-for-agent

`Black Belt's Training`: *"During this turn, attacks used by your
Pokémon do 40 more damage to your opponent's Active Pokémon ex."*
`Gladion's Final Battle`: *"You can use this card only when it is the
last card in your hand. During this turn, attacks used by your Pokémon
that don't have a Rule Box do 80 more damage to your opponent's Active
Pokémon."* 10 and 3 slots.

Nothing built lasts longer than the moment it resolves, and nothing built
reads a fact at attack-damage time beyond the attack's own numbers. Both
cards need a bonus that outlives the card itself for exactly the turn it
was played, read wherever `damage_dealt` runs, then gone. Distinct from a
static effect a Tool or Stadium carries as long as it stays in play
(still out of scope) — this expires at the turn's own end regardless of
anything staying in play.

- [ ] A bonus can apply to this turn's attacks only, cleared the same way
      a once-a-turn `Limit` is
- [ ] The bonus can be restricted to a target with no Rule Box, or to a
      Pokémon ex specifically, since the two cards name different targets
- [ ] `Black Belt's Training` plays
- [ ] `Gladion's Final Battle` plays, and cannot be played holding any
      other card
