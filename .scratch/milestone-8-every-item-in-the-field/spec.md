# Spec: every Item in the field

## Problem

Milestone 7 closed every Supporter the committed decks need but one
(`Briar`, refused for a concept the artifact lacks). This milestone takes
the next Trainer kind complete, per the standing order: Items, then
Tools, then Stadiums.

14 names, 71 slots:

| Card                       | Slots | Effect                                                       |
| -------------------------- | ----- | ------------------------------------------------------------- |
| Transformation Tome        | 12    | Play 2 at once; swap a Basic in play for one in discard, attachments and all |
| Secret Box                 | 11    | Requires discarding 3; search one each of Item, Tool, Supporter, Stadium |
| Glass Trumpet              | 10    | Requires a Tera Pokémon in play; attach discard Energy to up to 2 Benched Colorless |
| Dusk Ball                  | 9     | Look at the bottom 7 of the deck; may take a Pokémon found there |
| Prime Catcher              | 7     | Switch an opponent's Benched Pokémon in; if you do, switch your own too |
| Enhanced Hammer            | 6     | Discard a Special Energy from an opponent's Pokémon, no coin flip |
| Strange Timepiece          | 6     | Devolve one of your own Pokémon; the Evolution cards go to hand |
| Tool Scrapper              | 2     | Discard up to 2 Tools in play, yours or the opponent's |
| Team Rocket's Transceiver  | 2     | Search for a Supporter whose name holds "Team Rocket" |
| Energy Recycler            | 2     | Shuffle up to 5 Basic Energy from discard into the deck |
| Energy Search              | 1     | Search for a Basic Energy |
| Energy Retrieval           | 1     | Up to 2 Basic Energy from discard to hand |
| Tera Orb                   | 1     | Search for a Tera Pokémon |
| Hand Trimmer               | 1     | Each player discards down to 5, opponent first |

## Solution

Three reuse existing primitives outright:

- `Energy Search` and `Energy Retrieval` are `Decide` with a
  `CardFilter::BasicEnergy` slot, from `Zone::Library` and
  `Zone::Discard` respectively — the same shape `Ultra Ball` and
  `Lana's Aid` already run.
- `Secret Box` pays `Requirement::DiscardOtherCardsFromHand(3)` (already
  built for `Ultra Ball`), then runs four independent one-count slots,
  one `CardFilter::TrainerOfKind` per kind — sequencing already proven
  correct by `Brock's Scouting`'s two independent counts.

Four are small variations on a built shape:

- `Energy Recycler` needs a `Destination` that shuffles into the
  Library unordered, distinct from `TopOfLibraryInOrder` — a `Library`
  destination, shuffled once the whole `Decide` ends.
- `Team Rocket's Transceiver` needs `CardFilter::SupporterNameContains`,
  alongside the `BenchedNameStartsWith` filter already built.
- `Enhanced Hammer` reuses `Rust Syndicate Grunt`'s "discard from a
  chosen opposing Pokémon" shape, filtered to `CardFilter` naming a
  Special Energy specifically rather than any Energy.
- `Tool Scrapper` reads Tools attached to *either* side, a new
  `TargetFilter` that does not restrict by owner — everything built so
  far restricts to the chooser's own board.
- `Hand Trimmer` is `TrainerEffect::OpponentDiscardsDownTo` twice, once
  for the opponent, once for the player, in that order — `Xerosic's
  Machinations` already builds the single-player half.

Two are refused outright, the way `Briar` was: `Glass Trumpet` and
`Tera Orb` both gate on "a Tera Pokémon," a concept `data/cards.json`
carries no field for (ADR 0033 covers the reasoning; it applies here
unchanged).

Three are genuinely novel and get their own tickets:

- `Dusk Ball` reads the *bottom* of the Library, not the top — nothing
  built reads a Library from that end.
- `Prime Catcher` is a two-sided switch: the opponent's Bench to their
  Active, then (conditioned on that happening) the player's own Active
  to their Bench. Nothing built moves a Pokémon on the opponent's side
  of the board directly; `Boss's Orders` and `Switch` only ever choose
  which of the opponent's Benched Pokémon becomes Active, never place
  one there outright without the opponent's current Active being
  knocked out or displaced by the player's own switch.
- `Strange Timepiece` reverses evolution — moving Evolution cards off a
  Pokémon's stack back to hand, and it needs "cannot evolve this turn,"
  a restriction nothing built tracks yet (`Limit`'s once-a-turn shape
  covers a resource spent, not a Pokémon flagged for the rest of the
  turn).

`Transformation Tome` needs "play 2 copies at once" — a cost paid in a
second copy of the same card, not in cards discarded — before its
switch-preserving-attachments effect even runs. Nothing built pays a
cost in a second physical card. Ticket order takes the straightforward
cards first; each novel card gets its own ticket, and each records
either how it was built or why it could not be, the way `Briar` did.
