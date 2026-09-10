# Map: every Standard Trainer and Special Energy, beyond the field

## Destination

`coverage` shows 0 refused Trainers and 0 refused Special Energy.

## Notes

`known_trainer` in `src/import.rs` matches a card by name to a
`(Requirement, TrainerEffect)`. `known_trainer_by_print` overrides by
print id where two prints of one name differ. `resolve_trainer` in
`src/engine.rs` runs the effect; `legal_actions` gates the card. Adding a
plain card is a `known_trainer` line plus, where the effect is new, a
`TrainerEffect` variant and its `resolve_trainer` arm.

The refused Supporters split roughly into: plain draw, shuffle-and-draw
variants, deck searches, discard-pile retrieval, heals, switches, and
opponent-hand disruption — most map onto an effect the engine already
has, in a new shape.

## Tickets

- 01 — plain-draw Supporters (`Cheren`, `Friends in Paldea`, `Urbain`):
  a `TrainerEffect::Draw(u32)`.

## Decisions so far

Ticket 01 resolved 2026-09-10: `TrainerEffect::Draw(u32)` admits the plain-draw
Supporters (`Cheren`, `Friends in Paldea`, `Urbain`), and `progress_table` now
prints a Standard-coverage summary; details under [the ticket's Answer](issues/01-plain-draw-supporters.md).

Supporter clusters merged (each a PR, TDD, guards + README moved):
- #248 plain draw — `Draw(u32)`.
- #249 heals — `HealActive` reskin, `HealEachYours { amount, of_type }`.
- #250 draw variants — `CoinFlipDraw`, `DrawPerOpponentMegaEx`, `DrawUpToHandSize`.
- #251 hand refresh — `DiscardHandThenDraw`, `BothShuffleHandThenDraw` reskin, `Decide` last-card.
- #252 deck search — `Decide` reskin, new `CardFilter::PokemonOfType`.
- #253 discard retrieval — `Decide` reskin, new `TargetFilter::OfType`.
- #254 conditional draw — `DrawThenBonusIf*`, `DrawPerPokemonInOpponentHand`.

More clusters merged:
- #255 shuffle-then-coin draw — `ShuffleHandThenCoinFlipDraw`, `BothShuffleHandThenCoinFlipDraw`.
- #256 peek search — Hassel, Drayton (`Decide` + `peek` reskins).
- #257 Special Energy — `PreventsAttackEffectsOnCarrier` reskin, `CarrierHasNoRetreatCost`, `CarrierImmuneToSpecialConditions`.
- #258 heal-chosen — `HealChosenPlain { amount, of_type }`; `Phase::HealingChosen` gains `clear_conditions` + `of_type`.
- #259 Item searches — six `Decide` reskins, `Then::EndTurnIfMoved`.
- #260 Item deck-manip — `DiscardTopOfDeck`, `SwitchOutOpponentActive`, `Decide` to discard.
- #261 Item conditions — `InflictOnOpponentActive`, `ConfuseBothActivesExceptType`.
- #262 "-Berry" Tools — `ReducesDamageFromType`, a defender-Tool read in `damage_dealt_with`.
- #263 conditional-damage Tools — `BonusDamageVsActiveEx`, `ReducesDamageFromAbilityHolders`.

- #264 static Stadiums — `StadiumBoostsBasicHp`, `StadiumReducesDamageToType`.
- #265 name-prefix — `CardFilter::BasicPokemonNameContains`, `IncreasesHpForNamePrefix`, `StadiumReducesDamageForNamePrefix`.
- #266 coin-gated + retreat — `CoinFlipThen(Box)`, `RaisesBothActiveRetreatWhileCarrierActive`.
- #267 markers + Checkup Stadium — `DrawPerOwnPokemonWithMarker`, `AttachBasicEnergyFromDiscardToEachFuture`, `StadiumExtraPoisonDamage`.
- #268 next-turn side shield — ADR 0098, `GameState::side_shield_next_turn`, `GrantSideShieldNextTurn`.
- #270 Items — Lumiose Galette, Dragon Elixir (`HealActiveAndClearConditions`, `HealChosenPlain` of type).
- #272 type-matched Special Energy — Nitro Fire (`ReattachesAfterOwnDiscardByAttackEffect` reskin), Voltaic Lightning (`CarrierAttacksHitOpponentActiveHarder`, a step-32e read in `damage_dealt_with`).
- #273 Shadowy Darkness Energy — `PreventsBenchDamageWhileCarrierTypeMatches`, a second clause in `bench_attack_damage_blocked`.
- #274 unlimited searches — Precious Trolley (`Decide`, Basic Pokemon to Bench), Energy Search Pro (`Decide`, Basic Energy of distinct types to hand via `excludes_type_of_previous`).

Coverage at #274: 917 / 3051 prints (30.1%). Refused, by kind:
Supporter 60, Item 44, Tool 20, Stadium 20, Special Energy 6.

Special Energy still refused: Ignition, Legacy, Neo Upper (a
conditional-provision family — provides X, or Y off an Evolution /
Stage 2 — plus Ignition's end-of-turn self-discard); Team Rocket's
Energy (needs the deferred name-prefix Pokemon filter).

Deferred — the tier that needs its own design/ADR before it is cheap:
- **Trainer-as-Pokémon**: the eight "Antique … Fossil" Items play as a
  60-HP Basic. A whole mechanic; no seam for it yet.
- **Next-turn player-wide effects**: Roxie's Performance, Jasmine's Gaze,
  Acerola's Mischief, Iron Defender. `opponent_next_turn_restriction`
  is keyed to one `PokemonId` + an `AttackEffect`; a Supporter/Item
  shield over a whole side needs a new store and clear/arm logic.
- **Name-prefix Pokémon filters**: "Ethan's Pokémon", "Team Rocket's
  Pokémon", "Hop's Pokémon" — a `PokemonNameContains` filter (mirror
  `SupporterNameContains`), then Ethan's Adventure, Team Rocket's Proton,
  Hop's Bag, Team Rocket's Great Ball, Cynthia's Power Weight, Light Ball.
- **Peek-then-discard/reorder the rest**: Explorer's Guidance, Deduction
  Kit, Roto-Stick, Grimsley's Move — the `Decide`/`peek` leftover is
  always "shuffle back" today.
- **Interactive opponent choice**: Lt. Surge's Bargain, Meddling Memo,
  Team Rocket's Bother-Bot, Tyme's HP-guess minigame.
- **Ancient / Future markers**: Awakening Drum, Reboot Pod — the `Marker`
  enum has Ex/Mega/Tera only.
- **"Playable on the first turn" allowance**: Carmine's rider, Team
  Rocket's Proton, Call Bell, Chill Teaser Toy.
- **Retreat-cost modifier Tools**: Rescue Board, Gravity Gemstone,
  Heavy Baton, Sparkling Crystal, Counter Gain.
- **Attack-granting Tools**: Core Memory, Technical Machine: Fluorite.
- Coin-gated searches (Poké Ball, Energy Coin, Team Rocket's Great Ball),
  the many single-effect one-offs (Scoop Up Cyclone, Megaton Blower,
  Great Haul Net, Precious Trolley, Redeemable Ticket, …).
