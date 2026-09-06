# An effect is a value the engine executes

**Status:** Accepted — 2026-09-06

Half of every real deck is Trainers, and every Trainer carries rules text. Three ways to run that text were live. The engine could interpret the printed English at run time, which was rejected outright: the text is written for people, and a parser for it would be the whole project. Each card could carry a closure or a trait object that mutates the state, which was rejected because a closure cannot be inspected, compared, or printed, and a bot that wants to know what a card does could only run it. So a card's effect is a value: a sequence of primitives from a small enum, read from a card definition and executed by the engine.

[The effect vocabulary](../architecture/effects.md) names the nine primitives, taken by counting what the committed decks play. One of them — move cards, from a zone to a zone by a filter — covers fourteen of the nineteen Trainers those decks hold.

A primitive that asks the player something resolves as a phase with its own legal actions, which [ADR 0003](0003-legal-actions-is-the-engine-interface.md) already decided for every other mid-action choice. A requirement such as "only if you discard 2 other cards" is not part of the effect: it is checked before the card is legal to play, so it lives in `legal_actions`.

## Consequences

An effect can be read as well as run. A card's text can be printed from its value, a bot can look at what a card does before choosing it, and a test can assert on the effect rather than on the state it produced.

The enum grows one variant at a time, and every card is written by hand. That is the cost, and it is the point: [ADR 0008](0008-the-engine-refuses-a-card-it-cannot-run.md) refuses a card the engine cannot run in full, so a card is written only when its primitives exist.

Two filters need data the engine does not hold: a card's stage, which evolution brings, and whether a card has a Rule Box, which is the same fact as what a knockout of it is worth. Neither Trainer effects nor those two filters can be finished before their tickets are.
