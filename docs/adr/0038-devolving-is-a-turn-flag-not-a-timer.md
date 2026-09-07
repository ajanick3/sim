# Devolving is a per-Pokémon flag cleared at the next turn, not a timer

**Status:** Accepted — 2026-09-07

## Context

`Strange Timepiece` reads: *"Devolve 1 of your evolved Pokémon by
putting any number of Evolution cards on it into your hand. (That
Pokémon can't evolve this turn.)"* Two new facts: a Pokémon's card
stack can shrink, not only grow, and a specific Pokémon — not a whole
side, the way `turn_bonus` restricts damage — cannot take a specific
action for the rest of the turn.

`turn_bonus: Option<(u32, TurnBonusTarget)>` already carries "this
turn" for a damage bonus, cleared in `begin_turn` regardless of whose
turn is starting. The same lifetime fits "cannot evolve this turn":
once opened, it lasts until the *next* `begin_turn`, which is always
after the current player's own turn ends — never mid-turn.

## Decision

`PokemonInPlay` grows `cannot_evolve_this_turn: bool`, set wherever a
card is removed from that Pokémon's stack, cleared for every Pokémon in
`begin_turn`. `Evolve` and `EvolveSkippingOneStage` (`Rare Candy`) both
read it alongside the existing `played_on_turn` and `Limit::Evolved`
checks — the same two sites, since both name eligibility the same way.

Removing a card reads `PokemonInPlay.cards` directly:
`Phase::Devolving` names only the target and whether one is chosen yet;
`Action::RemoveOneEvolutionCard` pops the stack's last card into hand,
one at a time, "any number" read as the player stopping whenever they
choose (including zero).

## Consequences

A per-Pokémon "not this turn" restriction is now a field on
`PokemonInPlay`, the same shape a per-side one already is on
`GameState`. A card restricting a different action the same way reuses
the field only if the actions it blocks are also read at the same two
or so eligibility sites; otherwise it gets its own flag, named for what
it blocks.
