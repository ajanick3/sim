# A search to the Bench from an attach

Type: task
Status: open

`Telepathic Psychic Energy`: *"As long as this card is attached to a
Pokémon, it provides {P} Energy. When you attach this card from your
hand to a {P} Pokémon, search your deck for up to 2 Basic {P} Pokémon
and put them onto your Bench. Then, shuffle your deck."* 55 slots —
the single largest name behind this milestone.

Reuses ticket 02's attach-from-hand trigger, narrowed to a Psychic
carrier, and opens a search phase the same shape `Call for Family`'s
own `Phase::SearchingLibraryForBasics` already is, read from the
trigger instead of an attack.

- [ ] The attach-from-hand trigger only fires when the Pokémon
      receiving the Energy is Psychic-type
- [ ] Attaching it opens a search for up to 2 Basic Psychic Pokémon,
      chosen one at a time, then shuffles
- [ ] No qualifying Basic Psychic Pokémon in the library still
      attaches the Energy — only the search is skipped
- [ ] `Telepathic Psychic Energy` plays

Blocked by: 02
