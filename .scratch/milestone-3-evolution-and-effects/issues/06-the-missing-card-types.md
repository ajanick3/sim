# The missing card types

Type: task
Status: resolved

The model knows two kinds of card, a Pokémon and a basic Energy. Standard holds
four more: a Supporter, an Item, a Tool, a Stadium, and a special Energy. The
engine lumps all of them, and every evolution, into one refusal reason,
`NotABasicPokemon`, which tells a reader nothing about why a deck does not run.

Name them. A card type the engine cannot play is still a card type it should be
able to say out loud.

- [x] A refusal names the kind of card it refused
- [x] The coverage report breaks down by kind
- [x] A decklist report says what kind each card is
- [x] The once-per-turn limits rule 13 sets for a Supporter and a Stadium are
      recorded in the state, ready for the card that needs them

## Answer

Resolved 2026-09-06 on branch `feat/missing-card-types`.

`TrainerKind` names a Supporter, an Item, a Tool, and a Stadium. A refusal now
carries the kind it refused, so the coverage report reads:

```
   215  IsATrainer(Supporter)
   131  IsATrainer(Item)
    50  IsATrainer(Tool)
    49  IsATrainer(Stadium)
    21  IsASpecialEnergy
  1148  IsAnEvolution
   234  HasAnAbility
   929  AttackHasText
```

`NotABasicPokemon` survives for a Pokémon card that fits nothing above, and
refuses nothing in the current pool.

`PlayerState` records a Supporter and a Stadium played this turn, which rule 13
limits to one each. Nothing plays either yet; the flags clear at the start of a
turn, so the card that needs them will find its limit already kept.

No `CardDef::Trainer` variant yet. A Trainer needs an effect to be worth
building, and an effect is ticket 03. Adding the variant now would be a shape
nothing constructs.

Two older tests asserted the lumped reason for a Trainer and for an evolution.
They now assert the sharper ones, which is the tests following the behaviour
rather than the behaviour being loosened to suit them.
