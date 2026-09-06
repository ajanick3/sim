# A search that puts a pokemon into play

Type: task
Status: ready-for-agent

`Buddy-Buddy Poffin` searches the deck for up to 2 Basic Pokémon with 70 HP
or less and puts them **onto the Bench**. 169 slots, the most-played unbuilt
Trainer in the field.

`Phase::Deciding` moves a card between zones. The Bench is not a zone: it
holds Pokémon in play, and putting a card there makes a `PokemonInPlay`
rather than moving a `CardId`. That is the whole of this ticket.

- [ ] A search can put a Pokémon into play, not only into a zone
- [ ] The Bench limit of 5 is respected, and a full Bench ends the choice
- [ ] `Buddy-Buddy Poffin` plays, and the decks are measured after
