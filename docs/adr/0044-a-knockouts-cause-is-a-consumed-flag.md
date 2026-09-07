# A knockout's cause is a flag `settle` consumes, not stores

**Status:** Accepted — 2026-09-08

## Context

`Lillie's Pearl` reads: *"If the Lillie's Pokémon this card is attached
to is Knocked Out by damage from an attack from your opponent's
Pokémon, that player takes 1 fewer Prize card."* `knock_out_the_dead`
(inside `settle`) finds every knockout the same way, whatever caused
it — an attack, or a checkup's Poison or Burn damage — and nothing
before this card needed to tell those apart. ADR 0010 already
anticipated a card adjusting the Prize count at knockout time, naming
`Lillie's Pearl` explicitly; this is that card.

Two shapes were live. First: move the knockout check for the attacked
Pokémon inline into `attack()` itself, bypassing `settle`'s generic
loop for that one case. Second: a transient field, `attacking_defender:
Option<PokemonId>`, set by `attack()` and consumed by
`knock_out_the_dead` — the same "pending" shape `pending_end_turn` and
`checkup_pending` already are.

## Decision

`attacking_defender`. `attack()` sets it to the defender right after
applying damage; `knock_out_the_dead` takes it — reads and clears it —
at the top of every call, whether or not a knockout happened, before
looping over both players' Pokémon. Taking it unconditionally, every
call, is what keeps it from leaking into a later, unrelated `settle`
call: `Action::Attack` calls `attack` then `settle` in the same step,
so the first `knock_out_the_dead` inside that `settle` is the only one
that can ever see a non-`None` value.

Moving the check into `attack()` directly was rejected: it would split
knockout handling — discard, Prize count, win condition — across two
functions instead of one, for the sake of one card's one condition.
The flag keeps `knock_out_the_dead` the single place a knockout is
decided and paid for, the same way `pending_end_turn` keeps turn
transition decided in one place despite being set from several actions.

## Consequences

A future card conditioned on *how* a knockout happened (an Ability's
damage, a Special Condition, a Trainer effect) extends this shape:
a transient field set at the moment of the cause, taken once at the
top of `knock_out_the_dead`. Two such causes active in the same
`settle` call would collide on a single `Option` field; that has not
happened yet; a card that needs it would decide between growing this
into a small enum of causes or giving that cause its own field.
