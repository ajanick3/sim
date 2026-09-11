# A search

Type: task
Status: resolved

*"Search your deck for up to 2 Basic Pokémon and put them onto your
Bench. Then, shuffle your deck."* — `Drilbur` and `Toxel`'s
`Call for Family`.

- [x] `AttackEffect::SearchDeckForBasicPokemonToBench(u32)`
- [x] New `Phase::SearchingDeckForBasics { player, remaining }`, an
      attack-driven search that names no card to read slots back from
- [x] `Action::TakeBasicPokemonForCallForFamily` and
      `Action::FinishCallForFamily` (an "up to N" search can stop
      early, unlike ticket 07's mandatory placement)

Recorded in [ADR 0063](../../../docs/adr/0063-an-attack-search-gets-its-own-phase-not-deciding.md).

## Resolution

One new `AttackEffect` variant, one new `Phase` variant, two new
`Action` variants.

`Drilbur`'s me05-046 print and both `Toxel` prints are admitted; their
other attacks have no printed text.

`Duskull`'s `Come and Get You` (search the discard for up to 3 copies
of its own name) was deferred here: it needs a name-matching
`CardFilter` this pool had not needed yet. See ADR 0063. Built later,
beyond the spec's own ticket order, once `blockers` showed it was
still real weight: `CardFilter::PokemonNamed` and
`AttackEffect::SearchDiscardForNamedToBench` admit all 3 prints.

Coverage: `admitted` 539 -> 542 (3 prints).
