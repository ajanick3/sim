# The effect vocabulary

What a Trainer has to be able to say, taken from the cards the committed decks
actually play rather than from the card pool front to back. Nineteen Trainers,
57 copies across two decks.

## What the decks play

| Copies | Kind      | Card                     |
| ------ | --------- | ------------------------ |
| 8      | Item      | Poké Pad                 |
| 7      | Item      | Buddy-Buddy Poffin       |
| 4      | Item      | Ultra Ball               |
| 4      | Item      | Crushing Hammer          |
| 4      | Item      | Night Stretcher          |
| 4      | Item      | Rare Candy               |
| 4      | Supporter | Lillie's Determination   |
| 4      | Supporter | Boss's Orders            |
| 4      | Supporter | Gwynn                    |
| 4      | Supporter | Hilda                    |
| 3      | Supporter | Crispin                  |
| 3      | Item      | Strange Timepiece        |
| 2      | Stadium   | Team Rocket's Watchtower |
| 2      | Supporter | Dawn                     |
| 2      | Item      | Special Red Card         |
| 1      | Item      | Unfair Stamp             |
| 1      | Item      | Sacred Ash               |
| 1      | Item      | Prime Catcher            |
| 1      | Supporter | Judge                    |

## The primitives

Nine, and one of them carries most of the weight.

**Move cards.** From a zone to a destination, one slot at a time, by a
filter, in a count, chosen by a player and sometimes revealed. The
destination is usually another zone. The Bench is one exception — it holds
Pokémon in play rather than loose cards, so a card sent there comes into
play instead of moving — and attaching is the other, which needs a target
Pokémon as well. A card that wants one of several different things, such as
an Evolution and an Energy (Hilda), carries a slot for each; a card whose
second choice depends on its first, such as two Energy of different types
(Crispin), marks the later slot to read what an earlier one took. This one
primitive covers fourteen of the nineteen: a deck search to the hand (Poké
Pad, Ultra Ball, Hilda, Dawn), a deck search to the Bench (Buddy-Buddy
Poffin), a deck search split between the hand and an attachment (Crispin),
the discard pile to the hand (Night Stretcher), the discard pile to the deck
(Sacred Ash), and the hand to the deck (Judge, Lillie's Determination,
Unfair Stamp) or to the bottom of it (Special Red Card).

**Shuffle** a player's deck. Almost every search ends with one.

**Draw** a number of cards, for either player.

**Discard** from the hand, chosen by the player (Ultra Ball, Gwynn).

**Discard an attached card** from a Pokémon in play (Crushing Hammer).

**Flip a coin** and run an effect on heads (Crushing Hammer).

**Switch the Active** Pokémon, either player's, chosen (Boss's Orders, and
Prime Catcher, which does it to both sides).

**Attach an Energy** from a place that is not the hand (Crispin).

**Evolve and devolve** outside the normal turn step (Rare Candy, Strange
Timepiece). Both wait on evolution itself.

## The three things a primitive is not enough for

**A card filter.** "A Basic Pokémon with 70 HP or less", "a Pokémon that
doesn't have a Rule Box", "a Pokémon or a Basic Energy card". A filter is a
value in its own right, and two of its terms need data the engine does not hold
yet: a card's stage, and whether it has a Rule Box, which is the same fact as
what a knockout of it is worth.

**A requirement.** "You can use this card only if you discard 2 other cards
from your hand" (Ultra Ball), "only if any of your Pokémon were Knocked Out
during your opponent's last turn" (Unfair Stamp), "only if your opponent has 3
or fewer Prize cards remaining" (Special Red Card). A requirement is checked
before the card is legal to play, not while it resolves, so it belongs to
`legal_actions`. The Unfair Stamp one also needs the state to remember
something a turn later than it happens.

**A count that depends on the game.** Gwynn draws 3 for each card discarded,
and Lillie's Determination draws 8 instead of 6 at exactly 6 Prizes.

## What this leaves out

`Team Rocket's Watchtower` turns off every Ability of a Colourless Pokémon in
play. It is a continuous rule rather than an effect that resolves, and it needs
Abilities to exist first. Nothing here covers it.
