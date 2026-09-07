# A search can return its cards known, on top of the deck

**Status:** Accepted — 2026-09-07

`Ciphermaniac's Codebreaking` reads *"Search your deck for 2 cards, shuffle
your deck, then put those cards on top of it in any order."* Every search
built before it put its cards into a zone the player then draws from
blind — including the one case that returns to the Library, which shuffles
them in. This card's two chosen cards are not shuffled in: they sit on top,
known, in whichever order the player picked, on top of a deck that was
shuffled *before* they landed there.

`Destination::TopOfLibraryInOrder` is that placement. `CardFilter::AnyCard`
is its filter — the card names no kind at all, matching a Pokémon, an
Energy, or a Trainer alike, where every filter before it named something.

The shuffle itself needed no new rule, only a different moment to run at.
This engine tracks no hidden information — a `PlayerView` masks what a bot
may see (ADR 0006), but the state a search reads and writes is always
complete — so a full shuffle of the Library, run once when the slot opens
rather than once when the search ends, lands on the same distribution: the
untouched cards are uniformly random either way, and the chosen two are
placed afterward, undisturbed. `enter_slot`'s end-of-search shuffle, which
every Library-sourced search ran before this, is skipped for this
destination — running it again after the cards were placed would scramble
the very order the card promises to keep.

## Consequences

A card taken toward `TopOfLibraryInOrder` moves within its own zone rather
than out of it, so the zone `legal_actions` searches still contains it,
repositioned. `previous` — the field `Crispin`'s cross-slot type
constraint already carries — is reused to keep the just-taken card from
being offered a second time. It remembers only the last card taken, which
is exactly enough for the two `Ciphermaniac's Codebreaking` ever asks for;
a card with a higher limit toward this same destination would need every
card taken in the slot remembered, not only the last one.
