# Brock's Scouting

Type: task
Status: resolved

*"Search your deck for up to 2 Basic Pokémon or 1 Evolution Pokémon,
reveal them, and put them into your hand. Then, shuffle your deck."*
8 slots.

Not one filter admitting two kinds of card at the same count
(`Bug Catching Set`'s shape), and not a sequence of slots each wanting one
of a different kind (`Dawn`'s shape): this card offers two counts for two
kinds *at once*, and taking from one does not reduce what the other
allows. Check whether the pool has a second card shaped like this before
deciding how general the answer needs to be.

- [x] A search can offer two filters at once, each with its own limit,
      independent of the other
- [x] `Brock's Scouting` plays

## A planning finding: nothing new needed

The milestone map worried a shared sequence of slots — `Dawn`'s shape —
could not express "two counts, each independent of the other." Checking
it against the existing mechanism found otherwise: taking from an earlier
slot never reduces what a later slot allows, since each slot's own
`remaining` is set fresh when `enter_slot` opens it. Walking "up to 2
Basic, then up to 1 Evolution" in sequence reaches exactly the same final
hand as offering both at once would — the only difference is the order
the choices are presented in, not what is reachable. `Brock's Scouting`
plays as an ordinary two-slot `Decide`, the same shape `Dawn` and `Hilda`
already use, with `CardFilter::PokemonOfStage(Stage::Basic)` and
`CardFilter::EvolutionPokemon`, both already built.

No new primitive, no ADR. Coverage went 459 → 461 (2 prints), and the
field went 1658 → 1666 playable slots of 3660 — 45.5%.
