# Spec: the Trainers still standing

## Problem

Milestone 5 took unbuilt Trainers from 1230 slots to 591, across the 61
committed decks' 3660 slots. Ability then measured larger (703), and
Milestone 7 was drafted for it — but Trainers is not finished, and the
operator asked to finish it first.

591 slots, 62 distinct names, no single card carrying much of it — the
largest, `Area Zero Underdepths`, is 42 slots and already out of scope
(milestone 5 named it: it changes the Bench size for a player with a Tera
Pokémon in play, and needs continuous rules and a Tera concept neither of
which exists). The rest spread thin. Seven cards, chosen for the primitives
they share rather than for being the single largest, cover 201 slots — 34%
of what remains, a smaller share than milestone 5 or 7 claimed of theirs,
because this pool has a longer tail and no single card worth more than 33
slots.

## Solution

| Card                          | Slots | Needs                                      |
| ------------------------------ | ----- | -------------------------------------------- |
| Team Rocket's Petrel           | 33    | A search for any Trainer card                |
| N's PP Up / Wondrous Patch     | 46    | Attaching to a target chosen by a filter, not any Pokémon in play |
| Pokégear 3.0 / Bug Catching Set| 36    | A search bounded to the top of the deck, not the whole of it |
| Ciphermaniac's Codebreaking    | 24    | A search with no filter at all, returned to the top in a chosen order |
| Unfair Stamp                   | 25    | A requirement read from last turn, and an asymmetric draw |
| Switch                         | 11    | The player's own Active switched by choice, mid-turn |
| Jumbo Ice Cream                | 26    | Healing — nothing built has ever reduced damage |

Two of these directly de-risk questions Milestone 7 (Abilities) left open
rather than answered. `Unfair Stamp`'s requirement — "a Pokémon of mine was
Knocked Out during the opponent's last turn" — is the exact fact
`Fezandipiti ex`'s Ability needs; building it here for a Trainer settles
whether it is new `GameState` field or something read from the log, before
an Ability has to ask the same question. `Switch`'s effect — the player's
own Active swapped with a Benched Pokémon, by choice, mid-turn — is the
plain form of what `Subjugating Chains` (Pecharunt ex) needs with a
condition added on top; `Phase::Promoting` today opens only after a
knockout or a card that switches the *opponent's* Active, so this is the
first real card that opens it for a player switching their own.

## User stories

- A player searches their deck for any Trainer card, not only a Pokémon or
  an Energy.
- A player attaches an Energy from the discard pile to one of their own
  Benched Pokémon, matching a Trainer's name or type — not any Pokémon in
  play.
- A player looks at the top of their deck without searching all of it, and
  keeps what matches.
- A player searches for two cards of any kind and puts them back on top, in
  the order they choose.
- A player whose Pokémon was Knocked Out last turn shuffles both hands away
  and draws more than their opponent does.
- A player switches their own Active with a Benched Pokémon whenever they
  choose, not only after a knockout.
- A player heals damage from their Active Pokémon.

## Implementation decisions

Every effect is a value the engine executes (ADR 0009), and a card is
admitted only when the engine can run all of it (ADR 0008). Attaching a
card straight from a search already exists
([ADR 0016](../../docs/adr/0016-a-search-names-a-destination-not-a-zone.md),
[ADR 0018](../../docs/adr/0018-a-slot-can-read-what-a-search-already-took.md));
what it has never done is refuse a target by anything but being in play.

Each ticket checks the pool before it builds.

## Testing decisions

Test first. The coverage count and the committed decks are measured after
each ticket.

## Out of scope

**`Area Zero Underdepths`** (42 slots, the largest single name still
unbuilt) carries over from milestone 5, unchanged: it changes the Bench
size for a player with a Tera Pokémon in play, and needs continuous rules
and a Tera concept, neither of which exists.

**Every static or continuous Trainer** — `Air Balloon` and `Handheld Fan`
(Tools, 20 and 14 slots), `Nighttime Mine`, `Team Rocket's Watchtower`,
`N's Castle`, `Risky Ruins`, and `Battle Cage` (Stadiums, 16, 16, 13, 16,
13 slots), and `Binding Mochi` (a Tool, 14 slots). Every one of these reads
a modified rule — Retreat Cost, attack cost, whether an Ability runs at
all, whether a Bench placement or a Benched Pokémon takes damage — for as
long as the card holding it stays in play. Milestone 7 named the same
class of problem for static Abilities and deferred it for the same reason:
a real Stadium was already refused at import for carrying a continuous
rule the engine cannot run. This is one mechanism serving Tools, Stadiums,
and static Abilities together, not three or four cards' worth of one-offs,
and it earns its own effort once enough of them are ready to share it.

**`Academy at Night`** (a Stadium, 12 slots) grants a repeatable action —
"once during each player's turn, that player may put a card from hand on
top of their deck" — for as long as it stays in play. Not a modified rule
like the cards above, but the same shape of problem: an effect that
outlives the moment it was played, rather than resolving once. Deferred
with the Stadiums it will likely share a mechanism with.

**`Eri`** (a Supporter, 20 slots) reveals the opponent's hand and discards
up to 2 Item cards found there. Nothing built reads or discards from a
zone the opponent controls; `DiscardOpponentEnergy` reaches only into an
opponent's attachments, never their hand. A real primitive, deferred until
a second card asks for it — the same standard `Adrena-Brain`'s damage-move
was held to in Milestone 7.

**`Transformation Tome`** (an Item, 12 slots) must be played two at once,
and swaps a discarded Basic with one in play, carrying every attachment,
condition, and turn-in-play across. A combo mechanic with no second card
like it in the pool; low value for its complexity, and deferred.

390 of the 591 remaining Trainer slots — 66% — are still out of reach
after this milestone: 209 named above (`Area Zero Underdepths` included),
and 181 more spread across the roughly 41 Trainer names this milestone
never looked at.
