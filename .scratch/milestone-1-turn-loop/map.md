# Map: the Milestone 1 turn loop

## Destination

An engine that plays one whole game of the Pokémon TCG with two synthetic
Basics, driven by a person in the terminal or by a bot through one interface.

## Notes

The effort ran on 2026-09-05, before the tracker existed here. The tickets
below were written from the work after it landed, so each one opens resolved.
Rust was installed at the start of the effort; the toolchain is 1.98.1.

## Decisions so far

Ticket 01 resolved 2026-09-05: the state holds every object in an arena and names it by typed index; details under [the ticket's Answer](issues/01-arena-state-model.md).
Ticket 02 resolved 2026-09-05: `legal_actions` became the engine's one interface and `apply` refuses anything outside it; details under [the ticket's Answer](issues/02-legal-actions-interface.md).
Ticket 03 resolved 2026-09-05: setup deals, mulligans, places Pokémon, and sets Prizes; details under [the ticket's Answer](issues/03-setup-and-mulligans.md).
Ticket 04 resolved 2026-09-05: the damage order follows the rulebook's numbered steps; details under [the ticket's Answer](issues/04-damage-order.md).
Ticket 05 resolved 2026-09-05: the turn loop ends a game on all three win conditions; details under [the ticket's Answer](issues/05-turn-loop-and-win-conditions.md).
Ticket 06 resolved 2026-09-05: a text interface plays a whole game from the same action list a bot reads; details under [the ticket's Answer](issues/06-text-interface.md).

## Fog

Nothing. Every ticket resolved. The effort waits on the operator to close it,
which deletes this directory and leaves the git history as the record.
