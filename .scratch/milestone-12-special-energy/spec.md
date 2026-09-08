# Spec: Special Energy

## Problem

Special Energy is the only blocker the engine refuses by card kind
outright, not by what a given print's text says:
`Refusal::IsASpecialEnergy` fires for every Energy-category card in
the artifact, before any of its own text is even read. [ADR
0034](../../docs/adr/0034-enhanced-hammer-is-refused-for-a-card-kind-out-of-scope.md)
recorded why: `Energy` carried only a `print_id`, a `name`, and a
`kind: Type` — nowhere to put an effect, so admitting one with a
filter that could never match anything would not be a faithful build
of the card.

Counted by each card's first blocker, across the 61 committed decks,
3660 slots, Special Energy is 128 slots — 3.5% — behind seven distinct
names:

| Card                      | Slots | Needs                                          |
| -------------------------- | ----- | ----------------------------------------------- |
| Telepathic Psychic Energy  | 55    | An attach-from-hand trigger, and a search to the Bench read from it |
| Mist Energy                | 21    | A passive effect-prevention on the carrier itself |
| Prism Energy               | 12    | A flexible provided type, not a fixed one       |
| Spiky Energy                | 12    | Counter-damage to the attacker, read from a hit taken |
| Growing Grass Energy       | 12    | The primitive itself: `Energy` carries an effect |
| Enriching Energy           | 9     | The same attach-from-hand trigger `Telepathic Psychic Energy` needs |
| Boomerang Energy           | 7     | Reattaching itself after being discarded by an attack's own effect |

## Solution

Build the Special Energy primitive, and the seven cards that need it,
in the order that lets each ticket build on the last rather than by
raw slot count — the same choice Milestone 8 made for Abilities.
`Growing Grass Energy` is the cheapest real foothold: one passive
numeric modifier, read directly the same way a Tool's `IncreasesHp`
already is.

## User stories

- A player attaches a Special Energy to a Pokémon and it pays an
  attack's cost exactly like a Basic Energy of the same type would.
- A player's Pokémon carries a Special Energy that raises its HP,
  and effective HP reflects it for as long as the card stays attached.
- A player attaches a Special Energy from hand and an effect fires
  the instant it lands, not later in the turn.
- A player's Pokémon takes a hit while a Special Energy that reflects
  damage is attached, and the attacker takes counter-damage back.
- A Trainer or attack that names "a Basic Energy card" does not also
  find a Special Energy sitting in the same zone.

## Implementation decisions

Every Special Energy's effect is a value the engine executes, the
same discipline `AttackEffect` and `AbilityEffect` already hold —
`EnergyEffect`, matched by print name through `known_energy` in
`src/import.rs`, since the artifact carries no field naming what type
an Energy provides or what its text does. See [ADR
0080](../../docs/adr/0080-a-special-energy-carries-its-own-effect.md).

## Out of scope

- Every Special Energy print this milestone's own seven names do not
  cover — refused the same way an unbuilt Trainer or attack is,
  `Refusal::IsASpecialEnergy` with the reason now "not yet built"
  rather than "cannot exist."
- `Enhanced Hammer` ("discard a Special Energy from 1 of your
  opponent's Pokémon") — needs a discard-any-Special-Energy shape
  this milestone does not build, and Special Energy actually in play
  to have a real target. Revisit once this milestone closes.
