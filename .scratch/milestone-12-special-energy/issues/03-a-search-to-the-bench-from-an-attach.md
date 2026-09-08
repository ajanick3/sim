# A search to the Bench from an attach

Type: task
Status: resolved

`Telepathic Psychic Energy`: *"As long as this card is attached to a
Pokémon, it provides {P} Energy. When you attach this card from your
hand to a {P} Pokémon, search your deck for up to 2 Basic {P} Pokémon
and put them onto your Bench. Then, shuffle your deck."* 55 slots —
the single largest name behind this milestone.

Reuses ticket 02's attach-from-hand trigger, narrowed to a Psychic
carrier, and opens a search phase the same shape `Call for Family`'s
own `Phase::SearchingLibraryForBasics` already is, read from the
trigger instead of an attack.

- [x] The attach-from-hand trigger only fires when the Pokémon
      receiving the Energy is Psychic-type
- [x] Attaching it opens a search for up to 2 Basic Psychic Pokémon,
      chosen one at a time, then shuffles
- [x] No qualifying Basic Psychic Pokémon in the library still
      attaches the Energy — only the search is skipped
- [x] `Telepathic Psychic Energy` plays

Blocked by: 02

## Resolution

`EnergyEffect::WhenAttachedToTypeSearchesBasicPokemonOfTypeToBench(Type,
Type, u32)` carries both the carrier's required type and the type of
Basic Pokémon to search for as separate parameters — same symbol on
this print, but kept distinct in case a future card needs them apart.
Read inside `Action::AttachEnergy`'s own apply handler, right after
ticket 02's draw check, gated on `state.pokemon_def(target).kind ==
carrier_kind`.

A new `CardFilter::BasicPokemonOfType(Type)` and
`Phase::SearchingLibraryForBasicsOfType` mirror `Call for Family`'s
own `Phase::SearchingLibraryForBasics` exactly, narrowed by type —
kept as a separate phase/action pair rather than adding a type filter
to the existing one, the same "narrower sibling phase" shape
`ChoosingBenchedExDamageTarget` already took from
`ChoosingAnyOpponentPokemonDamageTarget`.

Admits `Telepathic Psychic Energy`, the single largest name behind
this milestone. Coverage moves from 678 to 679.
