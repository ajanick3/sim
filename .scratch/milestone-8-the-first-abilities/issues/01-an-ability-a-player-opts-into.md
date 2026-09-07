# An Ability a player opts into

Type: task
Status: resolved

`Mega Kangaskhan ex`'s Ability, "Run Errand": *"Once during your turn, if
this Pokémon is in the Active Spot, you may use this Ability. Draw 2 cards.
You can't use more than 1 Run Errand Ability each turn."* 63 slots, the
most-played Ability in the field, and the simplest shape one comes in: no
cost, no target, one trigger, one effect.

Nothing in the engine reads an Ability today. `Refusal::HasAnAbility`
refuses every card that carries one, the same way for all of them. This
ticket builds the first Ability primitive and admits the first card that
needs it.

- [x] A Pokémon can carry an Ability the engine runs, the way a Trainer
      carries an effect
- [x] A player may use it once during their own turn, and not the
      opponent's turn or a turn already spent
- [x] Whether the limit is keyed by the Ability's name or by the Pokémon is
      checked against the pool, not assumed, and the decision is recorded
- [x] `Mega Kangaskhan ex` plays, offered only while it is the Active
      Pokémon

Recorded in [ADR 0069](../../../docs/adr/0069-an-ability-is-its-own-standing-action-not-a-phase.md).

## Resolution

`Pokemon` gains `ability: Option<Ability>`. `Ability` carries a name
and an `AbilityEffect`, mirroring `Attack`'s own shape. `Action::UseAbility { pokemon }`
is offered directly from `Phase::Main`'s own action-building tail — no
new phase, since using this Ability needs no follow-up choice.
`Limit::AbilityUsed(PlayerId, &'static str)`, keyed by the player and
the Ability's own printed name (its own text names the restriction that
way; ADR 0069 records the choice).

`Mega Kangaskhan ex`'s only attack, `Rapid-Fire Combo` ("Flip a coin
until you get tails. This attack does 50 more damage for each heads"),
needed a new `AttackEffect::DamagePerCoinFlipUntilTails` alongside the
Ability — the card admits only once every printed piece of it reads
(ADR 0008).

A mechanical note: `Pokemon` gaining a new mandatory field touched
every `Pokemon { ... }` literal in the codebase (38 sites across 19
files) — the same blast radius `Attack`'s own `effect` field caused in
Milestone 11's first ticket.

Admits all 4 Mega Kangaskhan ex prints. Coverage: `admitted` 581 -> 585.

The README's Pokémon table now carries two columns, Attacks and
Ability, rather than one Status — Milestone 11 and Milestone 8 track
separate progress on the same species, and `playable` alone cannot
tell the two apart. `sim::import::attacks_read` and
`sim::import::ability_reads` answer each half independently for
`progress_table`'s own use.
