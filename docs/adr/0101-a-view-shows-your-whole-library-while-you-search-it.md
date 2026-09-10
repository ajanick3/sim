# A view shows your whole library while you search the whole of it

**Status:** Accepted — 2026-09-10

## Context

A [`PlayerView`](../../src/view.rs) hides the cards in either library and
their order. A deck search runs against that mask: the board could show
only the cards the search may take, read from `action_meta`, because those
are the only library cards the view named. A player doing a real deck
search sees more than that. They hold the whole deck and read every card,
takeable or not, and decide from the whole. The board showed less than the
table does.

Three options were live:

- **No change.** Show only the takeable cards. Rejected: it hides
  information the player has at the table, and the "why can I not take
  this card" question has no answer on screen.
- **Show only the reachable subset.** Widen `action_meta` to name every
  library card a slot's filter could match across the search, not only
  this step's. Rejected: still not what the player sees, and it grows the
  action-meta contract to carry non-actions.
- **A scoped mask exception.** While the phase is a search of the whole
  library, the view shows that player their whole library, sorted by kind
  then name so the shuffle order is gone. Chosen.

## Decision

`PlayerView::library_in_search` is `Some` only for the searching player,
and only while the phase searches the whole library — never a `peek` that
reads the top few cards, which would leak the rest of the order. The list
is sorted, so it carries card identity and count, never order. The board
dims the cards the current step cannot take.

## Consequences

The `src/view.rs` module doc gains this exception in the same commit that
adds the field. The wasm `WireView` and the web `view.ts` gain a matching
optional field. A `peek` search keeps the old behaviour: the board shows
only its actions.
