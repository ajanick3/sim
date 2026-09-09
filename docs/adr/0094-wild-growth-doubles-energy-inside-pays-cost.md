# `Wild Growth` doubles Energy inside `pays_cost`, not by moving cards

**Status:** Accepted — 2026-09-09

`Meganium`'s `Wild Growth` — "Each Basic Grass Energy attached to all
of your Pokémon provides GG Energy. The effect of Wild Growth doesn't
stack" — was flagged early this session as needing its own
whole-side Energy-doubling semantics. The card never actually
duplicates an Energy card or attaches a second one: it changes what
one attached card is *worth* when a cost is checked.

`pays_cost` already builds `available: Vec<Type>` from a Pokémon's
own attached Energy before matching it against a printed cost. Wild
Growth reads once, per call, whether any of the player's own Pokémon
carries it (undisabled) — a single board-wide `bool`, not a per-Pokémon
one, which is what "doesn't stack" means here: a second carrier in
play changes nothing, since the check was never counting carriers to
begin with. When that flag is set, each Basic Grass Energy (`effect:
None`, ruling out a Special Energy no printed text ever asks this to
touch) pushes into `available` twice instead of once. No new phase,
no new `Attack` value, and no change to `attached_energy_types` or
Retreat Cost — Wild Growth's own printed text names only "provides
Energy," which this engine reads as a cost-paying fact, the same way
every other Energy already is.
