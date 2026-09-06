# The engine refuses a card it cannot run, and counts what it refused

**Status:** Accepted — 2026-09-06

`data/cards.json` holds 3051 Standard cards and the engine can execute a fraction of them: most carry an English effect line, an ability, or damage such as `30+` that only an effect can resolve. Two ways to read the artifact were live. The engine could load every card and ignore the text it cannot run, which is the tempting one — it makes the card count look finished. It was rejected: a card that silently loses its effect plays a game that is not Pokémon, and a bot trained against it learns the wrong game. So the engine admits a card only when it can run all of it, and refuses the rest by name and by reason.

Refusal is not a failure state. `Import` returns the admitted cards and a `Refused` entry for every other card, each naming the card and one of six reasons, and `cargo run --bin coverage` prints the count and the breakdown. On the day of this record the engine plays **274 of 3051 Standard cards, 9.0%** — refused: 1614 not a Basic Pokémon, 929 for an attack's text, 234 for an ability.

Basic Energy is the exception the rule needed. It is not printed in the Standard sets, so the artifact holds none, and without it no imported card can pay for an attack. It is rules furniture rather than a card to import, so `Import::basic_energy` supplies it.

## Consequences

The coverage number is the project's progress metric, and it says what to build next: the largest refusal reason is the next feature worth having. Evolution would move a large part of the 1614, and an effect system the 929.

[ADR 0004](0004-a-deep-engine-and-a-tiny-card-set.md) stands. The engine still plays on the literals in `src/cards.rs`; the artifact is a source to measure against, not a card set to implement. Nothing here commits the project to raising the number.

Reading the file stays outside the engine. `load` takes the JSON as a string, so the crate keeps no I/O and a test can hand it a literal.
