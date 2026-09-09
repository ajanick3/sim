# `N's Zoroark ex` is refused for its own attack, not its Ability

**Status:** Superseded by [0089](0089-night-joker-runs-attack_with-on-a-second-cards-attack.md) — 2026-09-09

Ticket 02 named `N's Zoroark ex`'s Ability, `Trade` (a cost paid to
use an Ability — discard a card, then may draw 2), as the milestone's
second target. Its only printed attack, `Night Joker` — "Choose 1 of
your Benched N's Pokémon's attacks and use it as this attack" — turns
out to need the exact mechanism `Slowking`'s `Seek Inspiration` was
already refused for (ADR 0066): dispatching, at resolution time, to a
second card's own `AttackEffect`, which `AttackEffect`'s fixed-value
shape (ADR 0009) cannot express without restructuring how an attack's
effect is represented at all. `Night Joker` differs from `Seek
Inspiration` only in how the other card is found (a player's own
choice among the Bench, not a random discard) — the structural problem
is identical. `N's Zoroark ex` is **refused outright**, under
`Refusal::AttackHasText`, regardless of whatever its Ability builds.

No card in the sample decks' `HasAnAbility` field pairs a cost-gated
Ability with a buildable attack: `Iono's Kilowattrel`, `Quaquaval`,
`Meowstic`, `Team Rocket's Porygon-Z`, and `Volcarona` each print the
same "must discard/put back... may [do X]" shape, but none appears in
`decks/`. Ticket 02's own primitive (`Phase::Paying`-shaped or its own,
per the ticket's own fog) is **deferred**, not built speculatively —
the project's standing discipline (`prioritize-cards-in-sample-decks`)
scopes work by what the sample decks actually need, and nothing there
needs this shape yet. The ticket order continues at ticket 03.
