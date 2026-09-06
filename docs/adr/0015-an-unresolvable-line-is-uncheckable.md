# A decklist line the artifact cannot resolve is uncheckable, not a fault

**Status:** Accepted — 2026-09-06

Three of the 64 decklists fetched from the operator's tournament site name their cards by Japanese-region set codes — `SV6a`, `M1S`, `MC` — which the English pool in `data/cards.json` does not hold. A fourth kind of line, a promo printed with no number at all, could not be read by the parser at all and cost the whole list it appeared in.

Two ways to resolve the regional codes were live, and the choice was measured before it was made. Matching those lines by printed name alone would have resolved 24 of the 43 unresolvable lines, **guessed** on 15 more where the prints of that name disagree on what matters, and still failed on 4 names absent from the pool entirely. Guessing a card from a source this repository does not hold is what the Lost Zone taught one ticket earlier: a survey of somewhere else is not evidence about this pool. So a line naming a set the artifact does not hold is refused, and the report says which set is missing.

Where such a line goes in the report is the decision that matters. It is not a `Problem`: nothing is wrong with the deck, and a report that called it a fault would be lying about a legal list. It joins `uncheckable`, beside the ACE SPEC rule — the shape this project already uses for a fact the data cannot answer. A set the artifact *does* hold, at a number that is not in it, stays a real fault.

## Consequences

`parse` returns a `Decklist` rather than a `Result` and never fails. A line it cannot read is kept verbatim, and `Decklist::total` still counts it, because the leading number reads even when the rest does not — the count is the one part of a line that does not vary by dialect, so rule 1 stays checkable around an unresolved card.

The operator deleted the three affected decks rather than keep them as exceptions, so the corpus is 61 of the 64 placements and the known-exception list in the tests is gone. The behaviour outlives them: a future list with an unreadable line or an unknown set reads as far as it can and says what it could not.

Nothing here reaches for a service that could resolve the codes. The engine holds no I/O ([ADR 0007](0007-card-data-comes-from-tcgdex-into-this-repository.md)), and a resolver belongs to a tool if it is ever wanted.
