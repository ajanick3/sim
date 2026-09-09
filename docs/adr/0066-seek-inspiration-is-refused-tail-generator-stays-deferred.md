# `Seek Inspiration` is refused outright; `Tail Generator` stays deferred

**Status:** Accepted — 2026-09-07

Ticket 13 closes the three cards Milestone 11's spec deferred on
inspection, one decision each.

`Slowking`'s `Seek Inspiration` — "Discard the top card of your deck,
and if that card is a Pokémon that doesn't have a Rule Box, choose 1
of its attacks and use it as this attack" — asks the engine to resolve
an attack it discovers only at runtime, chosen from a card it has not
yet read. `AttackEffect` is a fixed value baked in at import time (ADR
0009: effects as values, not runtime-interpreted text); every existing
effect, however conditional, is known in full before the game starts.
Running this card in full would mean carrying every other
`AttackEffect`'s dispatch logic as data reachable from inside a single
match arm, or reading a second card's own `Attack` at resolution time
and re-entering the whole dispatch — a structural change to how an
attack's effect is represented, not a new variant alongside the
others. No card built or planned does this. **Refused**, under
`Refusal::AttackHasText`, the same reason its printed text already
carries — recorded here as a deliberate decision rather than a name
missing from `known_attack` by omission.

`Dedenne`'s `Tail Generator` — "Choose Basic Energy cards from your
discard pile up to the amount of Energy attached to all of your
opponent's Pokémon, and attach them to your Energy Pokémon in any way
you like" — is not structurally impossible the way `Seek Inspiration`
is: every piece of it is a shape this pool already has ticket doc
evidence for (a computed limit, a filtered choice from the discard
pile, a distributed attach across the player's own Pokémon). What it
lacks is a single card to build it against and verify it end to end;
this ticket found two other prints (`Wash the Slate Clean`,
`Electromagnetic Sonar`) already reachable with a smaller shape each,
and admitted the rest of `Dedenne`'s own coverage (`sv08-087`) through
one of those instead. **Deferred**, not refused: a future ticket that
needs a computed-limit distribute-and-attach shape should build
`Tail Generator` against it rather than starting from nothing.

## Errata

- 2026-09-09: `Seek Inspiration`'s own refusal no longer holds — see
  [0089](0089-night-joker-runs-attack_with-on-a-second-cards-attack.md),
  which built `N's Zoroark ex`'s `Night Joker` on the same
  read-a-second-card's-`Attack`-at-resolution-time shape this record
  called structurally impossible without restructuring
  `AttackEffect`. It turned out narrower than assumed: `attack()`
  splitting into a thin wrapper and a shared `attack_with` was enough,
  no `AttackEffect` variant needed to change shape. `Seek Inspiration`
  itself is now built the same way. `Tail Generator`'s own deferral is
  unaffected.
