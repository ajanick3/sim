# Card identity

Type: task
Status: ready-for-agent

412 Pokemon names are printed with differing behaviour, and a card
definition keeps only a name. Nothing can say which printed card an
implementation came from.

`CardRef` already carries the print id for every card read. This ticket puts
identity on the definition itself, so a game can name the card it is playing.

- [ ] A card definition carries its print id
- [ ] Two cards of the same name and different text stay separate
- [ ] The text interface names the card a player is looking at
