# Map: every Item in the field

01. Energy Search & Energy Retrieval — a Basic Energy from Library or Discard
02. Energy Recycler — a `Destination` that shuffles into the Library
03. Team Rocket's Transceiver — a Supporter filtered by a name substring
04. Enhanced Hammer — refused: Special Energy is structurally absent
05. Hand Trimmer — both players discard down to 5, opponent first
06. Secret Box — one of each Trainer kind, paid for in hand
07. Glass Trumpet & Tera Orb — refused: no Tera concept in the artifact
08. Dusk Ball — read the bottom of the Library
09. Prime Catcher — a two-sided switch
10. Strange Timepiece — devolve a Pokémon, and block its evolving this turn
11. Transformation Tome — a cost paid in a second copy of itself

`Tool Scrapper` moved to the Tools milestone: it discards a Tool
*attached* to a Pokémon, and nothing built attaches a Tool at all yet —
`PlayTrainer` sends every `TrainerKind::Tool` straight to the discard,
the same as an Item. That attach mechanism belongs to the milestone that
builds it first, not to an Item that only consumes it.
