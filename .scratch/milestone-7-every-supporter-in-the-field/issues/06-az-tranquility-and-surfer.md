# AZ's Tranquility and Surfer

Type: task
Status: ready-for-agent

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

- [ ] A switch's resolution can trigger a follow-up effect, read from
      what the switch just did
- [ ] `AZ's Tranquility` plays: an ex moved to the Bench heals
- [ ] `Surfer` plays: the player draws to 5 in hand
