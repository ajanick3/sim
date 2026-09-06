# A cost paid to use an Ability

Type: task
Status: ready-for-agent

`N's Zoroark ex`'s Ability, "Trade": *"You must discard a card from your
hand in order to use this Ability. Once during your turn, you may draw 2
cards."* 36 slots.

ADR 0017 drew the line for a Trainer: a requirement gates whether the card
may be played at all, and a cost paid to meet it is a phase, not the first
step of the effect. An Ability's cost is the same shape, but the thing it
gates is `legal_actions` offering the Ability, not `PlayTrainer` — and the
phase it opens, once paid, must resolve an Ability's effect rather than a
Trainer's, which lives on a `PokemonInPlay`, not a `CardId` in a zone.

- [ ] An Ability that demands a cost is not offered where the cost cannot be
      paid
- [ ] The cost is paid as a phase, the way `Phase::Paying` already does for
      a Trainer's requirement — reused, or given its own shape if an
      Ability's effect will not fit what `Phase::Paying` reads back
- [ ] `N's Zoroark ex` plays, and cannot be used while the hand holds
      nothing to discard
