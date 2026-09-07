# Rosa's Encouragement

Type: task
Status: ready-for-agent

*"You can use this card only if you have more Prize cards remaining than
your opponent. Attach up to 2 Basic Energy cards from your discard pile
to 1 of your Stage 2 Pokémon."* 9 slots.

The requirement compares the player's own Prizes to the opponent's;
`Requirement::OpponentPrizesAtMost` reads only the opponent's count today,
so this is a new comparison, not a new kind of fact. The attach reuses
`Destination::Attach`'s `TargetFilter`, needing a case for "a Stage 2 the
player controls" — check the pool for whether a stage-scoped target
filter already covers this or needs its own variant.

- [ ] A requirement can compare the player's own Prizes to the
      opponent's, not only read the opponent's alone
- [ ] `Destination::Attach` can target a Stage 2 Pokémon specifically
- [ ] `Rosa's Encouragement` plays, and cannot be played holding fewer or
      equal Prizes
