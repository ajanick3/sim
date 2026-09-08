# Map: Special Energy

## Destination

The seven Special Energy cards the spec names play, and the
primitives they need exist: `Energy` carries its own effect, an
attach-from-hand trigger, a flexible provided type, counter-damage
read from a hit taken, a passive effect-prevention on the carrier,
and a card that reattaches itself after its own discard.

## Notes

What each of the seven needs, in the order that makes each ticket
build on the last:

| Card                      | Slots | Needs                                        |
| -------------------------- | ----- | ----------------------------------------------- |
| Growing Grass Energy       | 12    | The primitive itself: `Energy` carries an effect, a passive numeric modifier read directly |
| Enriching Energy           | 9     | An attach-from-hand trigger — nothing reads "just attached" today |
| Telepathic Psychic Energy  | 55    | The same trigger, plus a search-to-Bench phase read from it |
| Spiky Energy                | 12    | Counter-damage to the attacker, read from a hit the carrier took |
| Mist Energy                | 21    | A passive effect-prevention on the carrier, broader than one numeric field |
| Boomerang Energy           | 7     | Reattaching itself after being discarded by an attack's own effect — needs tracking why a card left play |
| Prism Energy               | 12    | A flexible provided type read at cost-payment, not a fixed one — the most invasive to existing code |

`Growing Grass Energy` is the cheapest real foothold: one passive
numeric modifier, the same shape a Tool's `IncreasesHp` already reads
by, just from an Energy card instead. Every later ticket adds exactly
one new thing to it.

## Decisions so far

- Ticket 01: `Energy` gains `effect: Option<EnergyEffect>`; `None`
  for a Basic Energy every deck supplies for itself, `Some(_)` for a
  Special Energy matched by print name through a new `known_energy`
  in `src/import.rs`. `CardFilter::BasicEnergy` and its four siblings
  now read `effect.is_none()` rather than merely being an Energy card
  at all — until this ticket, every `Energy` the engine ever
  instantiated was Basic, so the distinction was invisible.
  `Growing Grass Energy`'s own `IncreasesCarrierHp` is read directly
  by `effective_hp`, unconditioned on `tools_disabled()` — a Stadium
  like `Jamming Tower` names only Tools in its own printed text.
  See [ADR 0080](../../docs/adr/0080-a-special-energy-carries-its-own-effect.md),
  which supersedes [ADR 0034](../../docs/adr/0034-enhanced-hammer-is-refused-for-a-card-kind-out-of-scope.md).
- Ticket 02: an attach-from-hand trigger needs no new plumbing —
  `Action::AttachEnergy`'s own apply handler is the only site a card
  ever leaves hand to attach, so `EnergyEffect::DrawCardsOnAttachFromHand`
  is read directly there, right after the card joins `attached`.
  Details under [the ticket's Answer](issues/02-an-attach-from-hand-trigger.md).
- Ticket 03: the search-to-Bench trigger is a narrower sibling phase
  of `Call for Family`'s own `Phase::SearchingLibraryForBasics`
  (`SearchingLibraryForBasicsOfType`, a new `CardFilter::BasicPokemonOfType`),
  not a type parameter added to the existing one — the same choice
  `ChoosingBenchedExDamageTarget` already made against
  `ChoosingAnyOpponentPokemonDamageTarget`. Details under [the
  ticket's Answer](issues/03-a-search-to-the-bench-from-an-attach.md).
- Ticket 04: no new "while Active" check was needed — `attack()`'s
  own `defender` parameter is, by construction, always the opponent's
  Active in this single-Active-format engine, so reading the counter-
  damage effect at the one site the base attack's damage lands
  already satisfies it for free. Details under [the ticket's
  Answer](issues/04-counter-damage-from-a-hit-taken.md).

## Fog

- Ticket 02 (Enriching Energy): where the attach-from-hand trigger
  hooks in — the manual `Action::AttachEnergy` handler most likely,
  mirroring how a play-triggered Ability hooks `Action::PlayBasic`
  directly (ADR 0071) rather than opening through a standing choice.
- Ticket 05 (Mist Energy): "prevent all effects... damage is not an
  effect" reads as broader than any one read site — closer in shape
  to the deferred `Flower Curtain`/`Spherical Shield` Bench-protection
  Abilities than to a single passive field. Whether it is cheap
  enough to build once `Rabsca`'s own Ability is tackled, or needs
  its own dedicated pass across every `AttackEffect` site, is open.
- Ticket 06 (Boomerang Energy): needs the engine to know *why* a card
  left play — nothing tracks that today. May need a new fact recorded
  alongside a discard, read only by this one card, rather than a
  general mechanism.
