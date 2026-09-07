# Secret Box

Type: task
Status: resolved

*"You can use this card only if you discard 3 other cards from your
hand. Search your deck for an Item card, a Pokémon Tool card, a
Supporter card, and a Stadium card, reveal them, and put them into your
hand. Then, shuffle your deck."*

No new primitive: `Requirement::DiscardOtherCardsFromHand(3)` (`Ultra
Ball`) pays the cost, then four independent one-count slots, one
`CardFilter::TrainerOfKind` per kind, run in sequence — the same
independence `Brock's Scouting` already proved a slot's `remaining`
carries.

- [x] The cost is paid before the search runs
- [x] Each slot offers only its own kind

## Resolution

No new primitive. Coverage: `admitted` 477 -> 478 (1 print); `trainers`
(refused) 314 -> 313.
