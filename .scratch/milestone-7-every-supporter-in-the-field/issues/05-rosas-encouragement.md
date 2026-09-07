# Rosa's Encouragement

Type: task
Status: resolved

*"You can use this card only if you have more Prize cards remaining than
your opponent. Attach up to 2 Basic Energy cards from your discard pile
to 1 of your Stage 2 Pokémon."* 9 slots.

The requirement compares the player's own Prizes to the opponent's;
`Requirement::OpponentPrizesAtMost` reads only the opponent's count today,
so this is a new comparison, not a new kind of fact. The attach reuses
`Destination::Attach`'s `TargetFilter`, needing a case for "a Stage 2 the
player controls" — check the pool for whether a stage-scoped target
filter already covers this or needs its own variant.

- [x] A requirement can compare the player's own Prizes to the
      opponent's, not only read the opponent's alone
- [x] `Destination::Attach` can target a Stage 2 Pokémon specifically
- [x] `Rosa's Encouragement` plays, and cannot be played holding fewer or
      equal Prizes

## Resolution

`Requirement::MorePrizesThanOpponent` compares both sides directly.
`TargetFilter::OfStage(Stage)` targets a Pokémon in play at a given stage,
Active or Benched alike — distinct from `BenchedNameStartsWith` and
`BenchedOfType`, which confine themselves to the Bench, since this card
does not.

Coverage went 422 → 425 (3 prints), and the field went 1604 → 1613
playable slots of 3660 — 44.1%. `Trainer, not yet built` (366) is now
smaller than `Attack text` (370) for the first time.
