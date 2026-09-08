# An attach-from-hand trigger

Type: task
Status: resolved

`Enriching Energy`: *"As long as this card is attached to a Pokémon,
it provides {C} Energy. When you attach this card from your hand to a
Pokémon, draw 4 cards."* 9 slots.

Nothing today fires the moment a card is manually attached from hand
— a play-triggered Ability hooks `Action::PlayBasic` or
`Action::Evolve` directly (ADR 0071), but a manual Energy attach has
no such hook yet. This ticket adds one, read only by an Energy's own
effect for now.

- [x] The manual Energy-attach action offers this Special Energy the
      same as any other from hand
- [x] Attaching it from hand draws 4 cards, once, the moment it lands
- [x] Discovering it already attached (never manually played this
      turn — dealt onto the board directly, the way a test fixture
      does) does not re-fire the draw
- [x] `Enriching Energy` plays

Blocked by: 01

## Resolution

`EnergyEffect::DrawCardsOnAttachFromHand(u32)`, read directly inside
`Action::AttachEnergy`'s own apply handler — the only site a card
ever leaves hand to attach, so no new trigger plumbing was needed;
the existing handler simply reads the just-attached card's own
effect after pushing it into `attached`. A card dealt straight onto
the board (never through `Action::AttachEnergy`) never reaches this
code path at all, so the "any other way" acceptance criterion holds
for free — nothing extra had to check for it.

Admits `Enriching Energy`. Coverage moves from 677 to 678.
