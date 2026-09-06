# A fact read from last turn

Type: task
Status: ready-for-agent

`Fezandipiti ex`'s Ability, "Flip the Script": *"Once during your turn, if
any of your Pokémon were Knocked Out during your opponent's last turn, you
may draw 3 cards."* 54 slots.

Every Ability built so far reads the board as it stands. This one reads
history: not "is a Pokémon of mine currently missing" (true for many reasons
that are not a knockout, and true long after the turn that caused it) but
specifically "did the opponent knock one out, last turn." Whether that is a
fact worth its own field on `GameState`, cleared at the start of a turn, or
a fact derivable from what the engine already keeps (`history: Vec<Action>`,
`log: Vec<String>`) is this ticket's first question — check before
building either.

- [ ] "A Pokémon of mine was Knocked Out during the opponent's last turn"
      can be asked, and answers correctly across a turn boundary
- [ ] The fact does not linger past the turn it describes
- [ ] `Fezandipiti ex` plays
