# Energy Search & Energy Retrieval

Type: task
Status: resolved

*"Search your deck for a Basic Energy card, reveal it, and put it into
your hand. Then, shuffle your deck."* (`Energy Search`) and *"Put up to
2 Basic Energy cards from your discard pile into your hand."*
(`Energy Retrieval`).

Both reuse `TrainerEffect::Decide` with a `CardFilter::BasicEnergy` slot,
already built for `Ultra Ball` (Library) and `Lana's Aid` (Discard).

- [x] `Energy Search` finds one Basic Energy from the Library
- [x] `Energy Retrieval` takes up to two from the Discard

## Resolution

No new primitive. Coverage: `admitted` 468 -> 471 (1 print of `Energy
Search`, 2 of `Energy Retrieval`); `trainers` (refused) 323 -> 320.
