# Dusk Ball

Type: task
Status: resolved

*"Look at the bottom 7 cards of your deck. You may reveal a Pokémon you
find there and put it into your hand. Shuffle the other cards back into
your deck."*

New: `TrainerEffect::LookAtBottomOfLibrary` and
`Phase::LookingAtBottomOfLibrary`, reading the bottom of the Library
live — the opposite end from every peeked search built so far, which
all read the top.

- [x] Offers only a Pokémon from the bottom 7
- [x] Taking one shuffles the rest of the library

## Resolution

One new `TrainerEffect`, one new `Phase`, two new `Action`s. Recorded in
[ADR 0036](../../../docs/adr/0036-a-bottom-search-reads-the-library-live.md).

Coverage: `admitted` 478 -> 479 (1 print); `trainers` (refused)
313 -> 312.
