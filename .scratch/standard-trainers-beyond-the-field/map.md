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
- #276 filtered searches — Mega Signal (`CardFilter::MegaPokemon`), TM Machine (`CardFilter::ToolNameContains`).
- #386 Larry's Skill — `DiscardHandThenDecide { slots }`, a hand discard ahead of the existing one-slot-per-kind search.
- Ignition and Neo Upper Energy — `ProvidesMoreColorlessIfCarrierIsEvolutionThenDiscardsAtEndOfTurn`,
  `ProvidesMoreOfAnyTypeIfCarrierIsStage2`; the first end-of-turn
  self-discard an Energy card has needed, read in `end_the_turn`.
- Sparkling Crystal and Counter Gain — `ReducesAttackCostByAnyTypeIfCarrierMarked`,
  `ReducesAttackCostIfMorePrizesRemaining`; the first Tools to
  discount their own carrier's attack cost, read in `legal_actions`'
  attack-cost loop alongside the Ability discounts already there.
- Super Potion and Misty's Vitality — `HealChosenThenDiscardEnergyIfHealed`
  reuses the attack-cost energy-discard phase for a Trainer's own
  effect; `Then::EndTurnAlways` joins `EndTurnIfMoved` for a search
  that ends the turn whether or not it found anything.
- Ethan's Adventure — `CardFilter::PokemonNameContainsOrBasicEnergyOfType`,
  a name-substring match paired with a typed-Energy alternative in one
  filter, the same "one filter, two kinds of card" shape
  `PokemonOfTypeOrBasicEnergyOfType` already is. Checking the rest of
  the name-prefix cluster found Team Rocket's Proton, Hop's Bag, and
  Cynthia's Power Weight already built (an earlier cluster used
  `BasicPokemonNameContains` and `IncreasesHpForNamePrefix` for them,
  before this map's own notes caught up); only Proton's separate
  "playable on your first turn" clause remains open, filed below.
- Team Rocket's Great Ball — `CardFilter::EvolutionPokemonNameContains`,
  paired with the existing `BasicPokemonNameContains` inside a new
  `TrainerEffect::CoinFlipEitherThen`, the first coin flip to choose
  between two different searches rather than "the same search or
  nothing." Its `Decide` phase opens directly from `resolve_trainer`
  once the coin lands, bypassing `Trainer::slots()` — that accessor
  is a pure function of the card's own printed effect and cannot know
  which side of a flip already happened.
- Light Ball — `BonusDamageVsActiveExForCarrierNamed`, `BonusDamageVsActiveEx`
  narrowed to an exact carrier name, read in `damage_dealt_with` the
  same way `Cobalt Command`'s own name check already is.
- Lucian and Lacey — `BothHandToBottomThenEachCoinFlipDraw` (both
  hands to the bottom of their own deck, then each player flips their
  own coin, unlike `BothShuffleHandThenCoinFlipDraw`'s one shared
  flip) and `ShuffleHandThenDrawBonusIfOpponentPrizesAtMost` (the same
  shuffle-then-draw shape, conditioned on the opponent's Prizes
  instead of the player's own).

- Amarys — `DrawThenDiscardHandAtEndOfTurnIfAtLeast`, a new
  per-player `GameState` fact set at resolve time and read once by
  `end_the_turn`, the same lifetime shape `Ignition Energy`'s own
  self-discard already established, generalized off one card's
  attachment onto a player's own hand.
- Waitress — no new code at all: `Decide` with `peek: Some(6)` and
  `Destination::Attach(TargetFilter::AnyInPlay)` already shuffles
  the rest of the peek back by default, which is the whole of what
  the card asks for.

- Thick Scale — `ReducesDamageFromTypes`, `ReducesDamageFromType`
  widened to several attacker types at once.
- Team Rocket's Hypnotizer — `InflictsConditionOnAttackerIfDefenderNamed`,
  the same "while the defender is hit, even by a knockout" trigger
  `DamagesAttackerWhenDefenderIsHit`/`DrawsWhenDefenderIsHit` already
  read, inflicting a Special Condition instead, gated on the
  defender's own name.
- Adversity Policy — `DrawsWhenDefenderWeakToAttackerIsHit`, the same
  trigger again, gated on `effective_weakness` matching the
  attacker's type instead of a name.

- Postwick — `StadiumBoostsDamageForNamePrefix`, the attacker-side
  mirror of `StadiumReducesDamageForNamePrefix`.
- Paradise Resort — `StadiumReducesRetreatCostForName`, an exact-name,
  partial-amount sibling of `RemovesRetreatCostForNamePrefix`'s
  prefix-match, whole-cost removal.
- Energy Coin — `CoinFlipAllThen(u32, Box<TrainerEffect>)`, `CoinFlipThen`'s
  many-coin sibling: flip several coins and resolve the wrapped effect
  only if every one lands heads.
- Kofu, Perrin, Caretaker — three Supporter shapes, each new: Kofu
  bottom-decks exactly 2 chosen cards then draws 4, gated by a new
  `Requirement::PutOtherCardsOnBottomOfDeck` and a new
  `Phase::PayingToBottomOfDeck` (the same shape `Paying` already is,
  paid to the bottom of the deck instead of the discard pile). Perrin
  reveals up to 2 Pokémon from hand into the deck, then searches for
  up to that many back — a new `Then::SearchPokemonUpToMoved` opens a
  second search phase by hand once the first ends, sized to what it
  moved, since no fixed slot can print a limit read from another
  slot's own result. Caretaker draws 2, then — only if it drew any
  and Community Center is in play — shuffles itself out of the
  discard pile it already went to and back into the deck, the first
  effect to read a specific Stadium's presence at resolve time.

- Levincia, Spikemuth Gym, Mystery Garden, Surfing Beach — four
  once-a-turn Stadium actions, each opening `Phase::Deciding` or
  `Phase::DiscardingFromHand` directly from its own `Action::Use*`
  handler rather than through the card's own `slots()`, the same way
  `Prism Tower` and `Community Center` already do: Levincia moves up
  to 2 Basic Lightning Energy from discard to hand; Spikemuth Gym
  searches the deck for a Marnie's Pokémon (a new
  `CardFilter::PokemonNameContains`, the un-Basic-restricted sibling
  of `BasicPokemonNameContains`) to hand; Mystery Garden discards an
  Energy card to draw up to the discarder's own Psychic-in-play count
  (a new `DiscardFollowUp::DrawUpToInPlayCountOfType`); Surfing Beach
  is the first Stadium to grant a free switch action, gated on both
  the Active and the chosen Bench Pokémon sharing its type.
- Hop's Choice Band — `ReducesAttackCostAndBonusDamageForCarrierNamePrefix`,
  the named-carrier attack-cost discount `ReducesAttackCostIfMore-
  PrizesRemaining`'s shape already is, gated on a name instead of a
  Prize count, combined in one Tool with the named-carrier bonus
  damage `BonusDamageVsActiveExForCarrierNamed`'s shape already is,
  without the ex restriction. Read at both existing sites: the
  attack-cost loop in `legal_actions`, and the Tool-bonus step of
  `damage_dealt_with`.

Coverage: 1012 / 3051 prints (33.2%). Refused, by kind:
Supporter 15, Item 26, Tool 9, Stadium 10, Special Energy 3.

Special Energy still refused: Legacy Energy (a wildcard-plus-prize-count
card, not the conditional-provision-by-stage shape this cluster built —
its prize-count clause wants milestone-3's deferred prize-count support
instead); Team Rocket's Energy — checked its real text and it is not a
name-prefix search at all: it is a Special Energy restricted to attach
only to a Team Rocket's Pokémon (discarding itself immediately if
attached to anything else) that then provides 2 Energy in any
combination of two named types. Nothing in the engine validates an
attach against the carrier's name, or discards a card the instant it
lands somewhere illegal — a real new mechanic, moved to Deferred below
rather than bundled as a "cheap filter reuse."

Confirmed built already, despite an earlier version of this map still
listing them as deferred: Roxie's Performance, Jasmine's Gaze, and Iron
Defender (the next-turn-shield cluster resolved as #268, ADR 0098) —
checked directly against `src/import.rs` and the artifact's own refusal
list rather than trusted from an older note.

Deferred — the tier that needs its own design/ADR before it is cheap:
- **Trainer-as-Pokémon**: the seven "Antique … Fossil" Items (Armor,
  Cover, Jaw, Plume, Root, Sail, Skull — the pool holds no eighth) play
  as a 60-HP Basic. A whole mechanic; no seam for it yet.
- **Team Rocket's Energy**: attaches only to a Team Rocket's Pokémon,
  discarding itself the instant it lands anywhere else, then provides
  2 Energy in any combination of two named types. Needs an attach-time
  name check plus an immediate self-discard on a failed one — nothing
  in the engine validates an attach against the carrier's identity
  today; `AttachEnergy`'s handler assumes every attach succeeds.
- **Peek-then-discard/reorder the rest**: Explorer's Guidance, Deduction
  Kit, Roto-Stick, Grimsley's Move — the `Decide`/`peek` leftover is
  always "shuffle back" today.
- **Interactive opponent choice**: Lt. Surge's Bargain, Meddling Memo,
  Team Rocket's Bother-Bot, Tyme's HP-guess minigame.
- **Ancient / Future markers**: Awakening Drum, Reboot Pod — the `Marker`
  enum has Ex/Mega/Tera only.
- **"Playable on the first turn" allowance**: Carmine's rider, Team
  Rocket's Proton, Call Bell, Chill Teaser Toy.
- **Retreat-cost modifier Tools**: mostly resolved — Rescue Board
  (`ReducesRetreatCost`, though its own "no Retreat Cost at 30 HP or
  less" clause is still unread) and Gravity Gemstone
  (`RaisesBothActiveRetreatWhileCarrierActive`) already play, and
  Sparkling Crystal and Counter Gain turned out to be attack-cost,
  not retreat-cost, Tools — built above.
- **Defender-Tool-on-knockout**: Heavy Baton, Survival Brace, Amulet of
  Hope, Deluxe Bomb — each a knockout-triggered "move Energy (or heal,
  or search) off the Pokémon that was just Knocked Out, before its
  cards go to discard" moment nothing in the engine has a hook for
  yet — `knock_out_the_dead` moves straight to the Prize count, with
  no pause for a choice first.
- **Attack-granting Tools**: Core Memory, Technical Machine: Fluorite
  — the Tool itself carries an `Attack` and grants it to the carrier;
  today an `Attack` only ever comes from the Pokémon's own printed
  list, so `legal_actions`' attack loop has no seam for one sourced
  from an attached card.
- **Same-Pokémon dual discard**: Ruffian discards a Tool *and* a
  Special Energy, but both from one opponent Pokémon the player
  picks first — `DiscardOpponentSpecialEnergy`'s existing phase
  offers a Special Energy from anywhere on the board, not scoped to
  a target chosen up front. Confirmed tractable, just bigger than
  this cluster's other entries: a `Phase::ChoosingRuffianTarget
  { chooser }`, legal only when the opponent has an in-play Pokémon
  carrying both a Tool and a Special Energy (the same "no legal
  target, card unplayable" shape `enhanced_hammer`'s own Requirement
  already reads by); on choosing it, discard the one attached Tool
  (a Pokémon carries at most one, by rule) and the first attached
  Special Energy found, no further choice needed. Left unbuilt this
  pass for a session with room for a new phase and Action variant.
- **Named-target heal**: Arven's Sandwich heals 30, or 100 if the
  *healed* Pokémon's own name is an Arven's Pokémon — every named-bonus
  shape built so far reads the carrier's name, not the target's.
- **Hand-card-as-price Items**: Blowtorch is playable only if a Basic
  Fire Energy is discarded from hand as its own cost — no Item's
  legality spends a hand card as its price today; `Requirement` checks
  board state, never pays for itself out of the hand.
- **Opponent's deck/hand as a search or reveal source**: Accompanying
  Flute reveals the opponent's top 5 and fills the *opponent's* Bench
  with Basics found there; Energy Swatter has the opponent reveal
  their hand and lets the player choose an Energy card there. Both
  need a search or reveal that reads and edits across the table, not
  just the player's own side — the interactive-opponent-choice bucket
  above is the closer cousin, but these two touch zones instead of
  choices.
- **Discard-to-deck shuffles keyed to a chosen set**: Great Haul Net
  shuffles chosen cards from the discard pile back into the deck (of
  either of two categories) — today's shuffles always return the
  *unchosen* remainder of a peek, never a chosen discard-pile subset.
- **Board-state-keyed tutor**: Love Ball searches the deck for a
  Pokémon sharing a name with one the opponent already has in play —
  every tutor filter built so far matches a fixed name or prefix, not
  the live board.
- **Discard-everything-from-everyone**: Megaton Blower discards every
  Tool and every Special Energy from every one of the opponent's
  Pokémon at once, plus a Stadium — the existing discard effects
  (`DiscardOpponentTool`, `DiscardOpponentSpecialEnergy`) each resolve
  against one target.
- **Discard-to-active swap**: Ogre's Mask swaps a Pokémon ex sitting in
  the discard pile with one in play, carrying over every attachment,
  damage counter, condition, and turn count — a whole mechanic; no
  seam moves a card from discard into an active board slot today.
- **Prize-pile manipulation**: Redeemable Ticket reshuffles the Prize
  cards and redraws that many from the deck; Anthea & Concordia grants
  3 extra Prizes on a knockout by an N's Pokémon (also gated on six
  named N's Pokémon being in play at once) — both want the same
  deferred prize-count support Legacy Energy already named above.
- **Switch-and-move-Energy**: Scramble Switch switches Active and
  Bench, then may move all Energy from the Pokémon just benched onto
  the new Active — switching exists; carrying Energy along with one
  does not.
- **Coin re-flip**: Backtrack Badge lets its carrier re-flip an
  attack's coins after seeing the results — no Tool can intercept and
  redo flips that already landed.
- **Attacker-category-gated retaliation**: Tremendous Bomb retaliates
  only against a Mega Evolution Pokémon ex specifically, only past a
  240-damage threshold, then discards itself — the reactive-damage
  Tools built so far gate on the defender's name or type, never the
  attacker's category, and none self-discard after firing.
- **Stadium consumes another Stadium**: Ange Floette can be put into
  play only by discarding Prism Tower, in play, that same turn — no
  rule lets one Stadium's play condition consume a different Stadium
  already on the field.
- **Both-sides heal that ends the turn**: Celebratory Fanfare heals
  every Pokémon on both sides at once and ends the turn only if
  healing happened — the "ends the turn if it did anything" clause has
  only ever paired with a search before, never a heal, and never both
  sides at once.
- **Evolution skips a condition clear**: Dizzying Valley stops
  Confusion from clearing when a confused Pokémon evolves or
  devolves — evolving always clears conditions today; nothing hooks
  that step to skip it conditionally.
- **Antique Fossil support**: Fossil Quarry searches for up to 2
  "Antique …" Items and benches them as Pokémon — the same
  Trainer-as-Pokémon mechanic the seven Antique Fossils themselves
  are deferred under, above.
- **Chained forced evolution**: Grand Tree searches for a Stage 1,
  evolves it onto a Basic, then chains into a Stage 2 of that same
  Pokémon — a two-step forced-evolution search nothing performs yet.
- **Rule-Box-aware damage shield**: Neutralization Zone blocks damage
  from the opponent's Pokémon ex or Pokémon V onto non-Rule-Box
  Pokémon, and can't be returned from the discard pile to hand or
  deck — needs a Rule-Box category read and a discard-pile lockout,
  neither tracked today.

Triaged but cheap, left unbuilt this pass for a session with room —
needed, just the time to wire and test it. Kofu, Perrin, Caretaker,
Levincia, Spikemuth Gym, Mystery Garden, Surfing Beach, and Hop's
Choice Band, all listed here in an earlier pass, are now built and
recorded under Decisions above instead — nothing is left in this list.

Not yet triaged: none. Every refused Trainer and Special Energy in the
pool is now either built or filed under a Deferred mechanic above.
