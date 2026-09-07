# Briar is refused: the artifact carries no Tera concept

**Status:** Accepted — 2026-09-07

## Context

`Briar` reads: *"During this turn, if your opponent's Active Pokémon is
Knocked Out by damage from an attack used by your Tera Pokémon, take 1
more Prize card."* Its "this turn" conditional and its extra-Prize
mechanic are both shapes the engine already has — ticket 07 built the
first, and `Refusal::HasAnAbility` already refuses on missing mechanics
rather than guessing. What is missing is the concept the condition names:
"Tera Pokémon." The imported artifact (`data/cards.json`, from TCGdex)
carries no field marking a print as Tera — its Pokémon schema holds only
`types`, `stage`, `suffix` (`ex`/`EX`), `abilities`, and `attacks`, none
of which distinguish a Tera print from an ordinary one.

Two shapes were live. First: admit the card and read "Tera Pokémon" as
some proxy already in the data — an `ex` suffix, say, or a name pattern.
Second: refuse it, the same way a card naming an Ability or a held item
is refused, because the engine has nothing to check the condition
against.

## Decision

Refuse `Briar`. A proxy would answer the condition with a guess the
artifact cannot back — an `ex` Pokémon is not a Tera Pokémon, and no
field or naming convention in this artifact says which prints are. ADR
0008 already settled this shape: refuse a card the engine cannot run in
full, rather than run part of it under an assumption the data does not
support. `Briar` needs no new `known_trainer` entry; it falls through to
the same `Refusal::IsATrainer(Supporter)` every unbuilt Supporter already
gets.

## Consequences

Coverage does not move. `Briar` stays refused until either TCGdex's
export gains a Tera marker or another source supplies one; the day it
does, this record is the one to revisit, not to reopen from scratch.
