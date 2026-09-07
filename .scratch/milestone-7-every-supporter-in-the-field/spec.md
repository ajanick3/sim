# Spec: every Supporter in the field

## Problem

Milestone 6 closed with 390 slots still blocked on an unbuilt Trainer,
spread across 62 names, none large. Rather than pick the highest-value
cards again, this milestone takes a whole kind complete: every Supporter
any committed deck plays that the engine does not yet run.

17 names, 83 slots — a small, closed set. Their effects:

| Card                        | Slots | Effect                                        |
| ---------------------------- | ----- | ------------------------------------------------ |
| Eri                          | 20    | Reveal the opponent's hand; discard up to 2 Items found |
| Black Belt's Training        | 10    | This turn, +40 damage to the opponent's Active ex |
| Rosa's Encouragement         | 9     | Requires more Prizes than the opponent; attach 2 Energy from discard to a Stage 2 |
| Brock's Scouting             | 8     | Search for up to 2 Basic, or 1 Evolution — one count for each |
| Lana's Aid                   | 6     | Up to 3, any mix of a rule-boxless Pokémon and Basic Energy, from discard |
| Xerosic's Machinations       | 5     | The opponent discards down to 3 in hand, their own choice |
| Kieran                       | 4     | Choose one: switch your Active, or +30 damage this turn |
| Gladion's Final Battle       | 3     | Requires this be the last card in hand; +80 damage this turn |
| Pokémon Center Lady          | 3     | Heal 60 from a chosen Pokémon; clear its Special Conditions |
| Wally's Compassion           | 3     | Heal all damage from a chosen Mega ex; if healed, its Energy to hand |
| Rust Syndicate Grunt         | 3     | Requires a Knockout last turn; discard an Energy from a chosen opposing Pokémon |
| N's Plan                     | 3     | Move up to 2 Energy from the Bench to the Active |
| Janine's Secret Art          | 2     | For up to 2 Darkness Pokémon, search and attach a Basic Darkness Energy each; Poison the Active if it got one |
| AZ's Tranquility              | 1     | Switch your Active; if a Pokémon ex moved to Bench, heal 80 from it |
| Briar                         | 1     | Requires the opponent at exactly 2 Prizes; a Knockout by your Tera Pokémon this turn takes 1 extra Prize |
| Surfer                        | 1     | Switch your Active; if you did, draw to 5 in hand |
| Morty's Conviction            | 1     | Requires discarding another card; draw one per the opponent's Benched Pokémon |

## Solution

Build all seventeen. Four new primitives cover most of them:

- **The opponent's own zone, read or chosen from.** `Eri` reveals the
  opponent's hand and the player chooses what to discard from it;
  `Xerosic's Machinations` has the *opponent* choose what they discard.
  Nothing built reaches into a zone the opponent controls; these two
  need it in both directions — the player's choice over it, and the
  opponent's own.
- **A bonus that lasts only this turn.** `Black Belt's Training`,
  `Gladion's Final Battle`, and one branch of `Kieran` all add damage
  "this turn" against a restricted target, gone once the turn ends —
  distinct from a static effect that lasts as long as a card stays in
  play, which is still out of scope.
- **A choice between two named effects.** `Kieran` prints two options and
  the player picks one; nothing built offers a card whose own text
  branches.
- **A search with two filters, each its own count.** `Brock's Scouting`
  is "up to 2 Basic, or 1 Evolution" — not one filter admitting two kinds
  (already built for `Bug Catching Set`), but two independent counts.

The rest reuse primitives already built: a search from discard
(`Lana's Aid`, `Rosa's Encouragement`), a Requirement already read
(`Rust Syndicate Grunt`'s last-turn Knockout, reused verbatim from
`Unfair Stamp`), a switch already built (`AZ's Tranquility`, `Surfer`,
one branch of `Kieran`), and a cost already paid
(`Morty's Conviction`'s self-discard, the shape `Ultra Ball` established).
Healing a *chosen* Pokémon, rather than the fixed Active `Jumbo Ice Cream`
reads, is new but small: a target choice in front of the same arithmetic.

## User stories

- A player reveals their opponent's hand and discards the Items they find.
- A player's attacks hit harder this turn, against a Pokémon ex only.
- A player picks one of two things a card offers, not both.
- A player searches for a Basic or an Evolution, capped differently for
  each.
- A player heals a Pokémon of their choice, not only the Active.

## Implementation decisions

Every effect is a value the engine executes (ADR 0009); a card is admitted
only when the engine runs all of it (ADR 0008). Each ticket checks the
pool before it builds.

## Testing decisions

Test first. The coverage count and the committed decks are measured after
each ticket.

## Out of scope

Nothing from this list. All seventeen are in scope — that is the point of
taking a whole kind complete rather than a slice of it. A card here that
turns out to need more than its ticket predicts gets its own ticket, not a
deferral.
