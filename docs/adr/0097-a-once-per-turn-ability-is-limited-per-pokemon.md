# A once-per-turn Ability is limited per Pokémon, unless its own text scopes the limit to the name

**Status:** Accepted — 2026-09-10

`Limit::AbilityUsed` was keyed by the player and the Ability's name. So the
first `Drakloak` to use `Recon Directive` spent the limit for every other
`Drakloak` that player controlled that turn. The Dragapult deck stacks three
`Drakloak` and uses `Recon Directive` from each one every turn, so the engine
blocked ordinary play. The rulebook makes "Once during your turn" a
per-Pokémon limit: each `Drakloak` gets its own use. A separate clause,
"You can't use more than 1 [Name] Ability each turn", is what scopes a limit
to the name across every copy; four cards in the pool print it —
`Lunatone`'s `Lunar Cycle`, `Mega Kangaskhan ex`'s `Run Errand`,
`Fezandipiti ex`'s `Flip the Script`, and `Pecharunt ex`'s
`Subjugating Chains`.

## Decision

`Limit::AbilityUsed` carries the `PokemonId` as well: `(PlayerId, PokemonId,
&'static str)`. A new variant `Limit::AbilityUsedByName(PlayerId,
&'static str)` keeps the old, name-wide key for the four cards whose text
asks for it. `Limit::for_ability_use` picks the variant, reading
`card::ability_is_scoped_to_its_name`, a name match over those four — the
same real-world-knowledge-by-name discipline `known_ability` and
`known_energy` already use. Every `spend` and `is_spent` site goes through
the helper.

The `PokemonId` was already in scope at almost every site, either from
`Action::UseAbility { pokemon }` or a `trigger_*` parameter. Two deferred
phases, `DecidingToUsePsychicDraw` and `DecidingToUseJewelSeeker`, carried
only the name; each gains a `pokemon` field, set from the evolving Pokémon
the trigger already holds. `DecidingToUseSubjugatingChains` is unchanged:
its Ability is one of the four, so its limit stays name-wide.

## Consequences

- The `state.pokemon` arena only grows — a Knocked Out Pokémon keeps its
  slot — so a `PokemonId` is never reused and the per-turn key cannot
  collide with a later Pokémon.
- Scope was read from the Ability's name, not a new field on `Ability`. A
  field would have touched every `Ability` literal in the tests. The name
  is unique to the print in every one of the four cases.
- No card's playable state changes; this is a limit-keying fix, not a new
  card. The card-progress table is untouched.
