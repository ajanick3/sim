# Map: the first Abilities

## Destination

The seven Abilities the spec names play, and the primitives they need
exist: an Ability a player opts into once a turn, a cost paid to use one, an
Ability that fires the instant its Pokémon enters play, the same Ability
name printed with two different effects, an Ability that reads last turn's
history, one that attaches Energy from hand, and one that returns its own
Pokémon to the deck.

## Notes

What each of the seven needs, in the order that makes each ticket build on
the last:

| Card                 | Slots | Needs                                       |
| --------------------- | ----- | -------------------------------------------- |
| Mega Kangaskhan ex     | 63    | The Ability primitive itself; opt-in, once a turn, Active only |
| N's Zoroark ex         | 36    | A cost paid to use an Ability                |
| Meowth ex              | 56    | A trigger tied to a moment, not "any time this turn" |
| Kadabra / Alakazam     | 70    | The same Ability name naming two effects     |
| Fezandipiti ex         | 54    | A fact read from the previous turn           |
| Teal Mask Ogerpon ex   | 38    | Attaching Energy from hand as the effect itself |
| Dudunsparce            | 27    | The Pokémon's own card leaving play as part of its effect |

`Mega Kangaskhan ex` is the cheapest real foothold: one Ability, one
trigger, one effect, no cost, no target. Every later ticket adds exactly
one new thing to it.

## Decisions so far

- Ticket 01: an Ability is offered as a standing `Action::UseAbility`
  straight out of `Phase::Main`, not a phase a card opens — nothing
  about using one needs a follow-up choice. `Limit::AbilityUsed` is
  keyed by the player and the Ability's own printed name, not by
  which Pokémon carries it, matching `Run Errand`'s own restriction
  text. See ADR 0069.

## Fog

- Whether an Ability's once-per-turn limit is keyed by the Ability's name or
  by the Pokémon carrying it. Two prints of the same Ability name
  (`Psychic Draw`) both restrict "no more than 1 Ability that has
  '\[name\]' in its name each turn" or the bare rule-49-adjacent "once during
  your turn" — ticket 01 checks the pool before deciding whether
  `Limit::AbilityUsed` is keyed by name or needs its own shape.
- Whether the cost `N's Zoroark ex` pays (discard a card) reuses
  `Phase::Paying`, built for a Trainer's requirement, or needs an
  Ability-scoped phase of its own. `Phase::Paying` today names the Trainer
  it is paying for and reads its effect back from the card; an Ability's
  effect lives on a `PokemonInPlay`, not a `CardId` in a zone, so the phase
  may need to name the Pokémon instead.
- Whether "the same Ability name printed with two different effects" is
  solved the same way ADR 0020 solved it for Trainers — a table keyed by
  print id, checked first — or whether Abilities need their own table
  because they live on `Pokemon`, not on a `Trainer`.
- Whether "a fact read from the previous turn" (`Fezandipiti ex`: was any of
  my Pokémon Knocked Out during the opponent's last turn) is new state on
  `GameState`, cleared at `begin_turn`, or is derivable from the action log
  the engine already keeps (`history: Vec<Action>`) without adding a field.
