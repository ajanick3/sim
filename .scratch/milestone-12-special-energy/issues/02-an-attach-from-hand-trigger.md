# An attach-from-hand trigger

Type: task
Status: open

`Enriching Energy`: *"As long as this card is attached to a Pokémon,
it provides {C} Energy. When you attach this card from your hand to a
Pokémon, draw 4 cards."* 9 slots.

Nothing today fires the moment a card is manually attached from hand
— a play-triggered Ability hooks `Action::PlayBasic` or
`Action::Evolve` directly (ADR 0071), but a manual Energy attach has
no such hook yet. This ticket adds one, read only by an Energy's own
effect for now.

- [ ] The manual Energy-attach action offers this Special Energy the
      same as any other from hand
- [ ] Attaching it from hand draws 4 cards, once, the moment it lands
- [ ] Discovering it already attached (never manually played this
      turn — dealt onto the board directly, the way a test fixture
      does) does not re-fire the draw
- [ ] `Enriching Energy` plays

Blocked by: 01
