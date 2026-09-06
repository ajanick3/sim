# A search bounded to the top of the deck

Type: task
Status: ready-for-agent

`Pokégear 3.0`: *"Look at the top 7 cards of your deck. You may reveal a
Supporter card you find there and put it into your hand. Shuffle the other
cards back into your deck."* `Bug Catching Set`: the same shape, up to 2
in any combination of Grass Pokémon and Basic Grass Energy. 23 and 13
slots.

Every search built so far reads the whole zone it searches. These two read
only the first 7 cards of the Library — a search bounded to a prefix, not
the zone entire.

Whatever is left of the top 7 once the choice ends is shuffled back exactly
where an ordinary search already shuffles the deck when it ends: since
nothing below the top 7 was ever inspected, a plain shuffle of the whole
Library is indistinguishable from shuffling only the untaken cards back
into it. Check that reasoning before building a second shuffle rule.

- [ ] A search can read a bounded prefix of a zone, not only the whole of
      it
- [ ] `Pokégear 3.0` plays
- [ ] `Bug Catching Set` plays: "a Grass Pokémon or a Basic Grass Energy"
      is one filter admitting two kinds of card, the same shape
      `PokemonOrBasicEnergy` already is, narrowed to one type

