# Team Rocket's Transceiver

Type: task
Status: resolved

*"Search your deck for a Supporter card that has 'Team Rocket' in its
name, reveal it, and put it into your hand. Then, shuffle your deck."*

New: `CardFilter::SupporterNameContains(&'static str)` — a family of
cards named by a shared word, where every filter built so far names a
kind or a stat, not a substring of the printed name.

- [x] Finds only a Supporter whose name holds "Team Rocket"

## Resolution

One new filter variant. Coverage: `admitted` 473 -> 476 (3 prints);
`trainers` (refused) 318 -> 315.
