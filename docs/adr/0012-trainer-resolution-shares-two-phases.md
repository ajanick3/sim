# A Trainer effect's resolution shares two phases, not one per card

**Status:** Accepted — 2026-09-06

Every mid-action choice built so far — retreat, the checkup — got its own `Phase` variant, one per shape. Ticket 02 named nine primitives a Trainer effect needs, and one bespoke phase per primitive was the naive reading of that pattern. It does not hold up: six of the eight Trainers scoped for this ticket — a deck search, a discard-pile recovery, a hand thinned for a bigger draw, an opponent's attached Energy discarded — are the same shape underneath. Each is "move up to some number of cards matching a filter from one zone to another, the player's choice each time." One phase, `Phase::Deciding { chooser, from, to, filter, remaining }`, covers all six; `Zone` and `CardFilter` are the values that tell it apart, not new phases.

A second overlap was found rather than built for: `Promoting`, which already existed for the choice after a knockout, is the same shape a card that switches an opponent's Active needs — "choose a Pokémon from a bench to become someone's Active." The only difference is who is doing the choosing. `Promoting(PlayerId)` became `Promoting { of, chooser }`, where `of == chooser` is today's only case and a card that acts on the opponent's bench is the other.

So two Trainers among the eight need no phase at all — `Judge` and `Lillie's Determination` resolve fully inside `apply` the moment they are played, with no player choice in between.

## Consequences

The engine gained one new `Phase` variant (`Deciding`) and one generalized existing one (`Promoting`), not six new ones. `Zone` covers Hand, Discard, and Library — never a Pokémon's attachments, which move through a different mechanism entirely (`attached: Vec<CardId>`), and never the Bench or Active, which hold Pokémon in play rather than loose cards.

`CardFilter` holds one variant today, `AnyPokemon`, because that is what the machinery's own tests needed to prove the shape works. The filters the eight committed Trainers actually require — "doesn't have a Rule Box" (`prizes == 1`, per [ADR 0010](0010-a-prize-value-is-read-from-the-card-name.md)), "a Pokémon or a basic Energy card" — are additions to this enum when the cards that need them are built, not part of this record.

`Deciding`'s `remaining` reaching zero does not auto-finish the phase: `legal_actions` stops offering `TakeCard` but still offers `FinishDeciding`, matching how `PlacingBench` already stops offering a placement once the Bench is full while still asking the player to say when they are done. A card that names an exact count rather than an "up to" one is the same phase with the same shape; nothing here forces every take to be optional.
