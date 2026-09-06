# The decklist dialects

Type: task
Status: resolved

Read the decklist dialects real lists are written in.

Three of the 64 committed decks do not parse, and both reasons are known: two
name cards by a Japanese-region set code, and one carries a promo printed
with no number at all. `tests/decklist.rs` names them as known exceptions.

A tabletop simulator surveyed for this milestone reads six shapes before
falling back to a service that resolves a whole list at once. The engine
holds no I/O, so a service belongs to a tool, not to `decklist.rs`.

- [x] A line with no set number parses, and says what it could not resolve
- [x] A decision on the Japanese-region set codes: translate, or refuse by
      name and say so
- [x] The known-exception list in `tests/decklist.rs` shrinks, and what
      remains is there for a recorded reason — it is gone entirely

## Answer

Resolved 2026-09-06 on branch `feat/the-decklist-dialects`. Written test
first, `tests/decklist.rs`.

**`parse` no longer fails a list.** It returns a `Decklist` rather than a
`Result`: a line it cannot read is kept verbatim in `Decklist::unreadable`,
and `check` names it in the report. One promo printed with no number used to
cost the whole list. `Decklist::total` still counts such a line, because its
leading number reads even when the rest does not — the count is the one part
of a line that never varies by dialect — so rule 1 stays checkable.

**An unknown set code is uncheckable, not a problem.** It is the same kind of
fact as the ACE SPEC rule: something the data cannot answer, said out loud
rather than guessed. A set the artifact *does* hold, at a number that is not
in it, is still a real fault.

**The decision on the Japanese-region codes was measured before it was
made.** Matching those lines by name alone would have resolved 24 of 43,
*guessed* on 15 more where the prints disagree, and still failed on 4 names
absent from the pool. Guessing from a source the repository does not hold is
what the Lost Zone taught, one ticket earlier. So: refuse, and say which set
is missing.

**The operator then removed the question.** All three affected decks were
deleted rather than kept as exceptions, so `decks/2026-worlds/` holds 61 of
the 64 placements and the known-exception list is gone entirely. The
behaviour built for them stays: any future list with an unreadable line or an
unknown set still reads as far as it can and says what it could not.
