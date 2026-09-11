# The deferred-on-inspection cards

Type: task
Status: resolved

The three cards the spec set aside for their own build-or-refuse
decision, plus every print of the same three species this ticket could
admit along the way with shapes already built.

## Decisions

- **`Slowking`'s `Seek Inspiration`: refused outright.** Copies another
  card's attack, discovered at runtime — structurally outside
  `AttackEffect`'s fixed-value shape (ADR 0009). See ADR 0066.
- **`Dedenne`'s `Tail Generator`: deferred, not refused.** Every piece
  of it is a shape this pool has already built once; it lacks only a
  card to build it against. See ADR 0066.
- **`Dwebble`'s `Ascension`: built.** `AttackEffect::SearchDeckToEvolveSelf`
  — a search straight to evolution, no hand step, the same
  hand-skipping shape Rare Candy already runs but pulled from the
  deck.

## Also admitted along the way

None of these needed a card the spec called deferred — each reused an
already-built shape, or a small new one this ticket's remaining time
covered:

- **`Dwebble`'s `Flail`** — `AttackEffect::DamagePerCount(OwnDamageCounters, 10)`,
  a shape ticket 02 already built. Admits both `sv10.5b` prints.
- **`Slowking`'s `Wash the Slate Clean`** — `AttackEffect::MayReturnOpponentsActiveEnergyToHand(u32)`,
  new: an optional move of the opponent's own Active's Energy into
  their own hand, opening `Phase::MovingOpponentsActiveEnergyToHand`.
  Admits `sv08.5-019`.
- **`Dedenne`'s `Electromagnetic Sonar`** — `AttackEffect::TakeTrainerFromDiscard`,
  new: a Trainer taken from the player's own discard pile, opening
  `Phase::TakingTrainerFromDiscard`. Admits `sv08-087`.

## Resolution

Four new `AttackEffect` variants, three new `Phase` variants, four new
`Action` variants.

Coverage: `admitted` 546 -> 551 (5 prints: 2 Dwebble via `Flail`, 1
Slowking, 1 Dedenne, 1 Dwebble via `Ascension`).

This closes the spec's own 13-ticket order, but `cargo run --bin
blockers` still shows an `Attack text` bucket with real slots
(`Dunsparce`, `Budew`, `Duskull`, `Wellspring Mask Ogerpon ex`,
`Slowpoke`, `Moltres`, `Beldum`, and more) — names the spec's
"recurring shapes" survey did not catch. Milestone 11 does not close
yet; the next session should read that list fresh and decide, per
card, whether it fits an already-built shape or needs its own ticket.
