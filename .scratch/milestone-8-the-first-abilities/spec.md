# Spec: the first Abilities

## Problem

An Ability is the largest single reason the engine refuses a card. Counted
by each card's first blocker, across the 61 committed decks, 3660 slots:

| Blocker                 | Slots | Share |
| ----------------------- | ----- | ----- |
| Plays                   | 1388  | 37.9% |
| Ability                 | 703   | 19.2% |
| Trainer, not yet built  | 591   | 16.1% |
| Basic Energy, plays     | 480   | 13.1% |
| Attack text             | 370   | 10.1% |
| Special Energy          | 128   | 3.5%  |

Milestone 5 took unbuilt Trainers from the largest blocker (1230 slots) to
the third largest (591). Ability is now the largest, and the engine runs
none of it: `Refusal::HasAnAbility` is a blanket refusal, the same for every
card regardless of what its Ability actually says.

42 distinct Ability names appear across the field. Seven account for 344
slots — just under half — and share primitives closely enough to fit one
milestone; the rest need mechanisms this milestone deliberately leaves for
later, named in Out of scope.

## Solution

Build the Ability primitive, and the first seven cards that need it:

| Card                  | Slots | Needs                                        |
| ---------------------- | ----- | --------------------------------------------- |
| Mega Kangaskhan ex     | 63    | An Ability a player opts into, once a turn     |
| N's Zoroark ex         | 36    | A cost paid to use an Ability                  |
| Meowth ex              | 56    | An Ability that fires the moment it enters play|
| Kadabra / Alakazam     | 70    | The same Ability name, two different effects   |
| Fezandipiti ex         | 54    | An Ability that reads what happened last turn  |
| Teal Mask Ogerpon ex   | 38    | An Ability that attaches Energy from hand      |
| Dudunsparce            | 27    | An Ability that returns its own Pokémon to the deck |

Kadabra and Alakazam both print an Ability named "Psychic Draw" — Kadabra
draws 2, Alakazam draws 3. The two engine-side conveniences milestone 5
built for exactly this shape, name-based matching with a print-id override
([ADR 0020](../../docs/adr/0020-a-trainer-name-is-matched-unless-a-print-overrides-it.md)),
were built for Trainers; this is the first real Ability that needs the same
choice made for Abilities.

## User stories

- A player with a Pokémon carrying an Ability may use it once a turn, from
  the Active Spot, and draws the cards it promises.
- A player discards a card from hand to use an Ability that demands it.
- A player benches a Pokémon and is offered its Ability the moment it
  arrives, not only later in the turn.
- A player's Pokémon with an Ability was knocked out last turn, and this
  turn's copy of a different Ability reads that fact.

## Implementation decisions

Every Ability effect is a value the engine executes, the same discipline
[ADR 0009](../../docs/adr/0009-an-effect-is-a-value-the-engine-executes.md)
holds Trainers to, and a card is admitted only when the engine can run all
of it ([ADR 0008](../../docs/adr/0008-the-engine-refuses-a-card-it-cannot-run.md)).
A cost paid to use an Ability is not the effect itself, the same distinction
[ADR 0017](../../docs/adr/0017-a-requirement-is-not-an-effect.md) drew for a
Trainer's requirement — ticket 02 answers whether an Ability's cost reuses
that mechanism or needs its own.

Each ticket checks the pool before it builds, the way milestone 5's did.

## Testing decisions

Test first. The coverage count and the committed decks are measured after
each ticket.

## Out of scope

**Static and continuous Abilities** — a Weakness or Retreat Cost changed by
an Ability that stays in play doing nothing else (`Lillie's Clefairy ex`,
`Latias ex`), and damage prevented from reaching the Bench (`Shaymin`).
81 slots between the three. None of them is a player's choice or a phase:
each reads a modified rule at the moment that rule runs — attack damage,
retreat cost — for as long as the Pokémon holding it stays in play and
un-Knocked-Out. `docs/architecture/rules.md` already named a Pokémon Tool
limit with the same shape ("a continuous invariant re-checked when an
Ability is lost"), and a real Stadium was refused at import for the same
reason (a continuous rule the engine cannot run). This is one mechanism,
not three cards' worth of one-offs, and it touches Stadiums as much as
Abilities — it earns its own milestone once both are ready to share it.

**`Adrena-Brain` (Munkidori, 44 slots)** moves damage counters from one
Pokémon to another. Nothing built so far moves damage; every card that
touches it so far only adds damage, through an attack. A real primitive,
deferred until a second card asks for it.

**`Recon Directive` (Drakloak, 60 slots) and `Metal Maker` (Metang,
16 slots)** each look at the top few cards of the Deck, not the whole
deck. `Phase::Deciding` always offers every match in the whole zone it
searches; a bounded peek at the top is a different shape, and
`Metal Maker`'s "attach any number you find, in any way you like" adds a
second: many cards to many targets, not one card to one destination.

**`Subjugating Chains` (Pecharunt ex, 14 slots)** switches the player's own
Active with a Benched Pokémon outside of a knockout, and poisons the new
Active outside of an attack. `Phase::Promoting` today only opens after a
knockout or a card that switches the *opponent's* Active; a player
switching their own Active by choice, mid-turn, is a new call site, and a
Special Condition applied outside attack resolution is a second one.

359 of the 703 Ability slots — 51% — are still out of reach after this
milestone: 215 named above, and 144 more spread across the roughly 35
Ability names this milestone never looked at. That is the honest cost of
Abilities being the broadest mechanic in the game rather than the
shallowest — Trainers are one kind of thing with many shapes; Abilities are
many kinds of thing.
