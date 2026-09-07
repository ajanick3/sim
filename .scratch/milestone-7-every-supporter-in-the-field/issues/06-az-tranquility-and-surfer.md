# AZ's Tranquility and Surfer

Type: task
Status: resolved

`AZ's Tranquility`: *"Switch your Active Pokémon with 1 of your Benched
Pokémon. If you moved a Pokémon ex to your Bench in this way, heal 80
damage from that Pokémon."* `Surfer`: *"Switch your Active Pokémon with 1
of your Benched Pokémon. If you do, draw cards until you have 5 cards in
your hand."* 1 slot each.

`SwitchOwnActive` (`Switch`) already performs the switch. Both cards add
a follow-up conditioned on what the switch actually did — one on what the
displaced Pokémon was, the other unconditionally once the switch
happened at all. Neither follow-up is a player's choice; both run the
moment `Action::Promote` completes a `SwitchOwnActive`.

- [x] A switch's resolution can trigger a follow-up effect, read from
      what the switch just did
- [x] `AZ's Tranquility` plays: an ex moved to the Bench heals
- [x] `Surfer` plays: the player draws to 5 in hand

## Resolution

`Phase::Promoting` grows a fourth field, `then: Option<PromoteFollowUp>`,
read in the one place `Action::Promote` already computes what was
displaced — not a second action after `Promote`, which would let the
engine stop between a switch and its own follow-up, a state no printed
card can produce.
[ADR 0027](../../../docs/adr/0027-a-switch-can-carry-a-follow-up.md)
records why. `TrainerEffect::SwitchOwnActiveWithFollowUp` is a second
variant beside `SwitchOwnActive`, not a field on it, so `Switch` itself
costs nothing for a follow-up it never has.

Coverage went 425 → 432 (7 prints), and the field went 1613 → 1615
playable slots of 3660 — 44.1%.
