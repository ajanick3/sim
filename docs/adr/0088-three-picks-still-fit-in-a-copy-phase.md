# A 3-target damage choice keeps `Phase` `Copy` with a fixed array

**Status:** Accepted — 2026-09-09

`Kyurem`'s `Trifrost` needed a choice like `Twin Shotels`'
`ChoosingTwoOpponentPokemonDamageTargets`, but for 3 picks instead of
2. That phase tracks its one possible exclusion as
`excluding: Option<PokemonId>`; a `Vec<PokemonId>` reads as the
obvious way to widen it to 3, but `Phase` is `Copy` everywhere in this
engine, and a `Vec` field would break that for every variant, not
only this one.

`ChoosingThreeOpponentPokemonDamageTargets` instead carries
`excluding: [Option<PokemonId>; 2]` — the choice never needs to
remember more than 2 prior picks, since a 3rd pick always ends it, so
a fixed array holds every reachable state and keeps `Copy`. The apply
handler fills the first `None` slot it finds each pick, then checks
whether the array is full or the opponent has nothing left unpicked.

Trifrost's own cost — discard all Energy from the attacker, then this
choice — reuses the exact discard-then-open-phase shape
`DiscardsOwnEnergyThenDamagesChosenBenchedEx` (Zeraora's `Thunder
Raid`) already takes, narrowed the same way that one narrows
`DamageChosenOpponentPokemon`: to a fixed damage amount and no
Weakness or Resistance.
