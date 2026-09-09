# `Festival Lead`'s second attack skips ending the turn, once

**Status:** Accepted — 2026-09-09

`Goldeen`'s and `Seaking`'s `Festival Lead` — "If Festival Grounds is
in play, this Pokémon may use an attack it has twice. If the first
attack Knocks Out your opponent's Active Pokémon, you may attack again
after your opponent chooses a new Active Pokémon" — was flagged early
this session as needing its own turn-structure record, since nothing
in the engine had ever let one Pokémon attack twice in a turn.

The reason turned out narrower than the flag suggested. `Action::Attack`
already runs `attack()` and then unconditionally sets `pending_end_turn`,
which is the only reason a second attack was ever impossible — `Main`
phase, and `legal_actions`, do not otherwise care how many times the
current player has attacked this turn. Skipping that one assignment,
once, is the entire mechanism: `legal_actions` re-offers the attack
naturally, including through a Knockout's own `Promoting` phase (which
never changes whose turn it is), so the card's second sentence — attack
again only after the opponent chooses a new Active — falls out of the
existing phase machinery for free, rather than needing its own case.

A single `festival_lead_extra_swing_used: bool`, cleared at
`begin_turn`, is enough state: only one Pokémon can attack in a turn to
begin with, so there is never more than one carrier's extra swing to
track. The check reads the attacker *after* `attack()` runs, not
before — an attack that removes its own attacker from play entirely
(`Dudunsparce`'s shuffle-self-into-deck shape) must not be read as
"still eligible," and a stale `PokemonId` captured beforehand doesn't
know that.

`Thwackey`'s `Boom Boom Groove` — "if your Active Pokémon has the
Festival Lead Ability, you may search your deck for a card" — is the
first Ability gated on a *different* Pokémon's Ability by name rather
than a standing board fact; `OncePerTurnMaySearchAnyCardIfActiveHasNamedAbility`
reads `side.active`'s own printed Ability name directly, then opens
the same shape `OncePerTurnMaySearchEvolutionPokemonOfType` already
takes, narrowed to `CardFilter::AnyCard`.
