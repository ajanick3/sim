# An Ability tied to a moment, not the whole turn

Type: task
Status: ready-for-agent

`Meowth ex`'s Ability, "Last-Ditch Catch": *"Once during your turn, when
you play this Pokémon from your hand onto your Bench, you may use this
Ability. Search your deck for a Supporter card..."* 56 slots.

Every Ability this milestone has built so far may be used any time during
the player's turn. This one may not: the window is the instant the Pokémon
is placed onto the Bench, and closes the moment something else happens.
`Action::PlayBasic` today ends with nothing left to decide; this ticket
gives it somewhere to offer a choice right after, without making every
ordinary placement stop and ask. Ticket 04 gives `Action::Evolve` the same
shape of window for a different trigger — build this one narrow enough that
the next ticket is an addition, not a rewrite.

- [ ] Placing a Basic that carries a triggered Ability offers it once, right
      after, and never again once the turn moves on
- [ ] Declining it is a real choice, not an oversight — the Ability is not
      forced
- [ ] `Meowth ex` plays: benching it offers the search for a Supporter
