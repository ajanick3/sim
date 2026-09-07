# Briar

Type: task
Status: ready-for-agent

*"You can use this card only if your opponent has exactly 2 Prize cards
remaining. During this turn, if your opponent's Active Pokémon is
Knocked Out by damage from an attack used by your Tera Pokémon, take 1
more Prize card."* 1 slot — the smallest card in the milestone, and among
the most involved.

A this-turn conditional, the shape ticket 07 builds, but reading a
knockout's cause rather than a raw damage number: it must know the
knockout was caused by an attack, from a Tera Pokémon specifically, and
add to the Prize count that same knockout already takes — not a bonus
read at damage time, one read at the moment `take_prizes` runs. Nothing
built has a Tera concept at all; check what "Tera Pokémon" would even
mean here before deciding how to admit this card, or whether it is
admitted at all.

- [ ] A recorded decision on whether this card can be run in full, or
      must be refused the way `Refusal::HasAnAbility` already refuses
      what the engine cannot run
- [ ] If admitted: an extra Prize taken, conditioned on this turn's
      knockout coming from a Tera Pokémon's attack
