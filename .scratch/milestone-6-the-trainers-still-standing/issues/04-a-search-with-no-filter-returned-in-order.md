# A search with no filter, returned in a chosen order

Type: task
Status: resolved

`Ciphermaniac's Codebreaking`: *"Search your deck for 2 cards, shuffle your
deck, then put those cards on top of it in any order."* 24 slots.

Two things nothing built has done. First, the filter admits every card in
the game — Pokémon, Energy, and Trainer alike — where every search so far
has named some kind of card to look for. Second, the cards taken go back on
top of the deck, in an order the player chooses, rather than being shuffled
in: the shuffle in this card's own text happens *before* the two cards are
placed, not after, so the two land in a known, chosen order on top of an
otherwise-scrambled deck.

- [x] `CardFilter` can match any card at all
- [x] Two chosen cards can be put back on top of the Library in the order
      the player picks, distinct from being shuffled in
- [x] `Ciphermaniac's Codebreaking` plays

## Resolution

`CardFilter::AnyCard` matches every kind at once. `Destination::TopOfLibraryInOrder`
places a card on top of its own zone, in the order taken, rather than
shuffling it in.

The shuffle needed no new rule, only a different moment: it runs once when
the slot opens (before any card is taken) instead of once when the search
ends, which lands on the same distribution since this engine tracks no
hidden information — [ADR 0023](../../../docs/adr/0023-a-search-can-return-its-cards-known-on-top.md)
records the reasoning. The end-of-search shuffle every other Library
search runs is skipped for this destination, since running it again after
the cards were placed would scramble the very order the card promises.

A card taken toward this destination moves within its own zone, so it is
still there to be offered again; `previous` — the field `Crispin`'s
cross-slot constraint already added — is reused to exclude the one just
taken, which is exactly enough for the two this card ever asks for.

Coverage went 405 → 408 (3 prints), and the field went 1503 → 1527
playable slots of 3660 — 41.7%.
