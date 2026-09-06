# The decklist dialects

Type: task
Status: ready-for-agent

Read the decklist dialects real lists are written in.

Three of the 64 committed decks do not parse, and both reasons are known: two
name cards by a Japanese-region set code, and one carries a promo printed
with no number at all. `tests/decklist.rs` names them as known exceptions.

A tabletop simulator surveyed for this milestone reads six shapes before
falling back to a service that resolves a whole list at once. The engine
holds no I/O, so a service belongs to a tool, not to `decklist.rs`.

- [ ] A line with no set number parses, and says what it could not resolve
- [ ] A decision on the Japanese-region set codes: translate, or refuse by
      name and say so
- [ ] The known-exception list in `tests/decklist.rs` shrinks, and what
      remains is there for a recorded reason
