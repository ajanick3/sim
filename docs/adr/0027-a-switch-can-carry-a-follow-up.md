# A switch can carry a follow-up read from what it did

**Status:** Accepted — 2026-09-07

`AZ's Tranquility` and `Surfer` both switch the player's own Active, the
plain thing `Switch` already does, then do something more: one heals the
Pokémon the switch just displaced, if it was worth more than a Prize;
the other draws until the player holds five cards, unconditionally, once
the switch has happened. Neither follow-up is a player's choice, and
neither can run until `Action::Promote` has actually completed the swap —
"the Pokémon ex that moved to the Bench" and "once you do" both name a
fact that does not exist until the promotion itself is over.

`Phase::Promoting` gained a fourth field, `then: Option<PromoteFollowUp>`,
rather than a new phase for each follow-up or a second action after
`Promote`. A second action would let the engine stop between the switch
and its own follow-up — a state no printed card can produce, and one
`legal_actions` would then have to pretend was a real choice.
`PromoteFollowUp` is checked in the one place `Action::Promote` already
computes what was displaced, so the fact the follow-up reads never has to
be reconstructed or asked for twice.

## Consequences

`TrainerEffect::SwitchOwnActiveWithFollowUp` is a second variant beside
`SwitchOwnActive`, not a field added to it: `Switch` itself carries no
follow-up, and giving it an `Option` it always sets to `None` would cost
every future one-shape reader a case to ignore. A knockout and `Boss's
Orders` both open `Phase::Promoting` with `then: None` — the same phase,
untouched, for the two switches that were never asked to do more.
