# A cost paid to use an Ability

Type: task
Status: resolved

`N's Zoroark ex`'s Ability, "Trade": *"You must discard a card from your
hand in order to use this Ability. Once during your turn, you may draw 2
cards."* 36 slots.

## Decision: refused, and deferred

`N's Zoroark ex`'s only printed attack, `Night Joker` ("Choose 1 of
your Benched N's Pokémon's attacks and use it as this attack"), needs
the same attack-copying mechanism `Slowking`'s `Seek Inspiration` was
already refused for — structurally outside `AttackEffect`'s fixed-value
shape, regardless of what this ticket builds for its Ability. **`N's
Zoroark ex` is refused outright.** See [ADR 0070](../../../docs/adr/0070-ns-zoroark-ex-is-refused-for-its-own-attack.md).

No other card in the sample decks pairs a cost-gated Ability with a
buildable attack (checked: `Iono's Kilowattrel`, `Quaquaval`,
`Meowstic`, `Team Rocket's Porygon-Z`, `Volcarona` — none is in
`decks/`). Building the cost-gated-Ability primitive now would be
speculative work with no current payoff, against the project's own
`prioritize-cards-in-sample-decks` discipline. **Deferred** until a
sample-deck card needs it.

## Resolution

No code built. Two decisions recorded (refuse, defer) rather than a
silent skip. Coverage unchanged. Ticket order continues at 03.
