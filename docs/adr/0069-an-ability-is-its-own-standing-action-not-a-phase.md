# An Ability is its own standing `Action`, not a phase a card opens

**Status:** Accepted — 2026-09-07

`Mega Kangaskhan ex`'s `Run Errand` — "Once during your turn, if this
Pokémon is in the Active Spot, you may use this Ability. Draw 2 cards.
You can't use more than 1 Run Errand Ability each turn" — is the first
Ability the engine runs. Every effect built so far starts from a
player-initiated moment already in the turn: playing a card, attacking,
retreating. An Ability is different — it is a standing option a
Pokémon in play simply carries, available whenever its own gates hold,
with no card played and no attack declared to trigger it. `Action::UseAbility { pokemon }`
is offered directly out of `Phase::Main`'s own action-building tail,
alongside `Attack` and `Retreat`, rather than opening a phase the way
a Trainer's effect does — nothing about using it needs a follow-up
choice, so a phase would exist only to be left immediately.

`Limit::AbilityUsed(PlayerId, &'static str)` is keyed by the player and
the Ability's own name, not by which Pokémon carries it: the printed
restriction reads "You can't use more than 1 Run Errand Ability each
turn," covering every copy the player controls at once, the same
question ADR 0020 answered for a Trainer's name (there, whether a
print collides; here, whether a shared name shares its own limit).
`Pokemon` gains a single `ability: Option<Ability>` field — every
printed card carries at most one, so no name-and-print-override table
is needed yet, unlike Trainer's `known_trainer_by_print`. `Mega
Kangaskhan ex`'s own attack (`Rapid-Fire Combo`, a coin-flip-until-tails
count) needed a new `AttackEffect` too, admitted alongside the Ability
in the same ticket — ADR 0008 refuses a card until every printed piece
of it runs, Ability included.
