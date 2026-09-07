# A search bounded to the top of the deck

Type: task
Status: resolved

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

- [x] A search can read a bounded prefix of a zone, not only the whole of
      it
- [x] `Pokégear 3.0` plays
- [x] `Bug Catching Set` plays: "a Grass Pokémon or a Basic Grass Energy"
      is one filter admitting two kinds of card, the same shape
      `PokemonOrBasicEnergy` already is, narrowed to one type

## Resolution

`Slot` grows `peek: Option<u32>` — read only the top `n` cards of the zone,
which are the last `n` entries of the `Vec<CardId>` since `draw` pops from
the end. [ADR 0022](../../../docs/adr/0022-a-search-can-peek-a-bounded-prefix.md)
records why it lives on `Slot`, a fact about the search, rather than on
`Zone`, a fact about the card — and why the untaken cards need no new
shuffle rule: a plain whole-Library shuffle already lands on the same
distribution as "shuffle only the cards seen back in."

`Pokégear 3.0` needed `CardFilter::TrainerOfKind`, narrower than
`AnyTrainer`. `Bug Catching Set` needed
`CardFilter::PokemonOfTypeOrBasicEnergyOfType`, the same shape
`PokemonOrBasicEnergy` already is, narrowed to one type.

Coverage went 402 → 405 (3 prints), and the field went 1467 → 1503
playable slots of 3660 — 41.1%.
