# A Pokémon that returns to the deck

Type: task
Status: ready-for-agent

`Dudunsparce`'s Ability, "Run Away Draw": *"Once during your turn, you may
draw 3 cards. If you drew any cards in this way, shuffle this Pokémon and
all attached cards into your deck."* 27 slots.

Nothing built so far removes a Pokémon from play except a knockout, and a
knockout sends the stack to discard, never back into the Library. This
Ability's own Pokémon — cards and attachments together — leaves play by its
own choice, mid-game, and rejoins the deck to be drawn again. Whatever a
Bench or an Active empties into does not apply here: there is no zone this
belongs to on the way out, only the one it lands in.

- [ ] A Pokémon in play, its whole card stack and every attachment, can be
      shuffled into its owner's Library
- [ ] Whatever `Retreat`, evolution's `Limit::Evolved`, or an attack's
      spent state remembered about this Pokémon Id does not survive it,
      since the Id itself stops meaning anything
- [ ] `Dudunsparce` plays
