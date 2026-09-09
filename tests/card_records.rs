//! Card-records ticket 01: every card in the artifact stays readable.

use sim::import::load;

fn artifact() -> String {
    std::fs::read_to_string("data/cards.json").expect("the artifact is committed")
}

#[test]
fn a_refused_card_can_be_read_back_in_full() {
    let import = load(&artifact()).unwrap();

    // me01-113 is a Supporter, refused outright: its rules text has nowhere
    // to go in a CardDef. The raw record still holds it.
    let card = import
        .cards
        .iter()
        .find(|c| c.id == "me01-113")
        .expect("the artifact holds this card");
    assert!(card.playable.is_none(), "a Supporter is never admitted");
    let effect = card.raw["effect"]
        .as_str()
        .expect("a Trainer's raw record keeps its printed text");
    assert!(!effect.is_empty());
    assert_eq!(card.raw["category"].as_str(), Some("Trainer"));
}

#[test]
fn an_admitted_card_keeps_its_raw_record_too() {
    let import = load(&artifact()).unwrap();
    let card = import
        .cards
        .iter()
        .find(|c| c.name == "Chikorita")
        .expect("Chikorita is in the artifact");
    assert!(card.playable.is_some());
    assert_eq!(card.raw["hp"].as_u64(), Some(70));
}

#[test]
fn the_admitted_cards_still_play_and_coverage_does_not_move() {
    let import = load(&artifact()).unwrap();
    let (admitted, total) = import.coverage();
    assert_eq!(total, 3051);
    // The coverage canary. This number moves only when a ticket admits a
    // card on purpose, and the ticket that moves it says so here. Ticket 03
    // of the record effort took it to 368; the second batch of Trainers
    // takes it up from there — 2 prints of Cyrano, 5 of Buddy-Buddy Poffin,
    // then 3 of Ultra Ball, 2 of Special Red Card, 2 of Energy Switch, 3 of
    // Hilda, 3 of Dawn, 4 of Crispin, 2 of Rare Candy, 3 of Team Rocket's
    // Petrel, 3 of N's PP Up, 2 of Wondrous Patch, 1 of Pokégear 3.0, 2 of
    // Bug Catching Set, 3 of Ciphermaniac's Codebreaking, 1 of Unfair
    // Stamp, 2 of Switch, 2 of Jumbo Ice Cream, 3 of Lana's Aid, 2 of Rust
    // Syndicate Grunt, 3 of N's Plan, 1 of Pokémon Center Lady, 3 of Rosa's
    // Encouragement, 3 of AZ's Tranquility, 4 of Surfer, 10 of Black
    // Belt's Training, 3 of Gladion's Final Battle, 5 of Kieran, 3 of
    // Morty's Conviction, 2 of Xerosic's Machinations, 4 of Eri, and 2 of
    // Brock's Scouting, 3 of Wally's Compassion, 4 of Janine's Secret
    // Art, 1 of Energy Search, 2 of Energy Retrieval, 2 of Energy
    // Recycler, 3 of Team Rocket's Transceiver, 1 of Hand Trimmer, 1 of
    // Secret Box, 1 of Dusk Ball, 2 of Prime Catcher, 1 of Strange
    // Timepiece, 1 of Transformation Tome, 3 of Air Balloon, 1 of
    // Hero's Cape, 2 of Brave Bangle, 2 of Binding Mochi, 1 of
    // Lillie's Pearl, 2 of Punk Helmet, 1 of Lucky Helmet, 1 of
    // Handheld Fan, 2 of Powerglass, 2 of Gravity Mountain, 1 of
    // N's Castle, 1 of Academy at Night, 2 of Team Rocket's Factory,
    // 2 of Lumiose City, 3 of Jamming Tower, 1 of Risky Ruins, 3 of
    // Forest of Vitality, and 2 of Festival Grounds. Milestone 11
    // (Pokémon attacks) starts here: 1 of Carvanha, 2 of Tapu Bulu, and
    // 1 of Rellor (sv05-023), each a plain recoil attack. Damage
    // multipliers take it up 5 more: 3 of N's Reshiram, 1 of Passimian,
    // and 1 of Paldean Tauros (me02-048, both its attacks now read).
    // Ignoring the defender's own effects completes Dudunsparce ex
    // (2 prints): its other attack already read from ticket 02. A
    // direct or coin-flipped Special Condition, and a coin-flipped
    // damage bonus, admit Applin's sv06-017 print (its only attack).
    // A restriction through the opponent's next turn (can't retreat)
    // admits Yveltal's me01-088 print (its other attack has no text).
    // A restriction on the attacker's own next turn (can't attack)
    // completes N's Zekrom (2 prints): its other attack (Shred) was
    // already read in ticket 03. Damage placed on the opponent's
    // Bench, in the player's own choice of split, admits Dragapult ex
    // (5 prints; its other attack, Jet Headbutt, has no printed text).
    // A cost paid in the attacker's own Energy — discarding it all,
    // then damaging a chosen Benched Pokémon — completes N's
    // Darmanitan (3 prints): its other attack (Back Draft) was
    // already read in ticket 02. A switch, reusing SwitchOwnActive
    // outright, admits Abra's me01-054 print (its other print's only
    // attack has no printed text). A search, reusing the Decide-to-
    // Bench shape from an attack, admits Drilbur's me05-046 print and
    // both Toxel prints (their other attacks have no printed text).
    // A hand read — the opponent reveals their hand, which changes no
    // state the engine tracks separately — admits Hoothoot's sv05-126
    // print. A fixed draw, and a bonus read once from a boolean board
    // fact ("if this Pokémon has any damage counters on it"), complete
    // Mega Sharpedo ex (3 prints). Ticket 13, the deferred-on-
    // inspection cards: Dwebble's Flail (an existing damage-per-count
    // shape) admits its sv10.5b prints (2); Slowking's Wash the Slate
    // Clean (an optional move of the defender's own Energy to its
    // owner's hand) admits sv08.5-019; Dedenne's Electromagnetic Sonar
    // (a Trainer taken from the discard pile) admits sv08-087; and
    // Dwebble's Ascension (a search straight to evolution, no hand
    // step) admits sv10-011.
    // Beyond the spec's own ticket list: Beldum's Iron Tackle (Recoil,
    // already built) and Dunsparce's Trading Places (SwitchOwnActive,
    // already built) each admit one more print with no new shape.
    // Moltres's Fighting Wings (a bonus read once against a boolean
    // defender fact, the mirror of BonusDamageIfOwnDamaged) admits
    // its one print.
    // Slowpoke's Dangle Tail (a Pokemon taken from the discard pile,
    // the mirror of TakeTrainerFromDiscard) admits its sv07-057 print.
    // Duskull's Come and Get You (a named search from the player's own
    // discard pile, CardFilter::PokemonNamed) completes all 3 prints.
    // Torchic's and Celebi's Collect (DrawCards, already built) and
    // Buneary's Run Around (SwitchOwnActive, already built) each admit
    // one more print with no new shape.
    // Bayleef's Push Down admits its one print with the
    // SwitchOpponentActive mirror of ticket 09's SwitchOwnActive.
    // A damage-reduction restriction through the opponent's next
    // turn (before Weakness and Resistance), the mirror of
    // DefenderCannotRetreatNextTurn's own lifetime, admits Chikorita's
    // me02.5-008 print (Growl).
    // Buneary's Charm (the same shape, printed word for word) admits
    // its me01-107 print too.
    // A coin-flipped invulnerability through the opponent's next turn
    // — every effect of an attack against this Pokemon prevented
    // outright, a full short-circuit at the top of `attack` rather
    // than a `damage_dealt_with` read — admits Dunsparce's Dig prints
    // (2) and Elgyem's Hide print (1).
    // A restriction on the opponent's own Item plays, through their
    // next turn only — the same opponent_next_turn_restriction
    // lifetime, read at the Item-offering site instead of retreat's
    // — completes all 3 Budew prints.
    // An optional fixed-count Energy shuffle-back paid for bench
    // damage — the player's choice is whether to pay at all, not
    // which cards, since the Energy is interchangeable for this
    // effect — completes all 5 Wellspring Mask Ogerpon ex prints
    // (Sob was already read in ticket 05).
    // Paldean Tauros's Spirited Tackle (BonusDamageIfDefenderIsStage,
    // the mirror of BonusDamageIfDefenderIsEx) admits its sv08-018
    // print.
    // Combusken's Double Kick (damage per heads across several coin
    // flips, the same pre-damage slot DamagePerCount already
    // occupies) admits its sv10-041 print.
    // Elgyem's Slight Shift moves an Energy between two of the
    // opponent's own Pokemon, the mirror of MoveAttachedEnergy played
    // on the opponent's board instead of the player's own — completes
    // both Elgyem prints (Hide was already read this session).
    // Zeraora's Combat Thunder (Count::OpponentBenchedPokemonCount,
    // the same DamagePerCount shape with a new counted fact) admits
    // both sv07-055 and sv07-151 prints.
    // Milestone 8 (Pokémon abilities) starts here: Mega Kangaskhan
    // ex's Run Errand (an Ability a player opts into once a turn, the
    // milestone's first primitive) admits all 4 prints — its own
    // attack (Rapid-Fire Combo, a coin-flip-until-tails count) needed
    // a new AttackEffect too, since a card admits only once every
    // printed piece of it reads.
    // Meowth ex's Last-Ditch Catch (an Ability tied to a moment — the
    // instant it is played from hand onto the Bench, not a standing
    // choice) and its own attack, Tuck Tail (returns the attacker and
    // its attachments to hand, the same "moves together" rule 22
    // already keeps for a knockout) complete all 3 Meowth ex prints.
    // Kadabra's and Alakazam's Psychic Draw (the same Ability name,
    // resolved by species alone — known_ability's (species, name)
    // keying already distinguishes them, no print-override table
    // needed) admits Kadabra's me01-055 print (its own attack, Super
    // Psy Bolt, has no printed text).
    // Fezandipiti ex's Flip the Script (a fact read from last turn —
    // the same knocked_out_last_turn field Unfair Stamp's own
    // requirement already reads) and its own attack, Cruel Arrow (flat
    // damage to any chosen opponent Pokemon, Active or Benched) admit
    // all 5 prints.
    // Teal Mask Ogerpon ex's Teal Dance (an Ability that attaches
    // Energy from hand as the effect itself, then may draw) and its
    // own attack, Myriad Leaf Shower (damage per Energy on both
    // Active Pokemon combined) admit all 8 prints.
    // Dudunsparce's Run Away Draw (a Pokemon that shuffles itself
    // into the deck as part of its own Ability, the "moves together"
    // rule 22 already keeps but into the library) admits both prints
    // (Land Crush has no printed text).
    // Beyond Milestone 8's own map: Abra's Teleporter (the same
    // shuffle-self shape as Run Away Draw, but no draw and Active-
    // only) admits its sv06-080 print (its other print's only attack
    // has no printed text and no Ability at all).
    // Dusclops's and Dusknoir's Cursed Blast (an Ability that damages
    // a chosen opponent Pokemon, then Knocks itself out — modeled by
    // raising its own damage to its effective HP rather than a
    // separate forced-knockout primitive) complete all 3 Dusclops
    // prints and all 3 Dusknoir prints (Shadow Bind, already built).
    // Genesect ex's Metallic Signal (a standing search for Evolution
    // Pokemon of a type, CardFilter::EvolutionPokemonOfType) and its
    // own attack, Protect Charge (reduced damage taken through the
    // opponent's next turn, after Weakness and Resistance — the
    // mirror of DefenderDealsLessDamageNextTurn's own lifetime and
    // reduction) complete all 3 prints. Fixed along the way: the
    // opponent_next_turn_restriction clearing bug this uncovered — a
    // self-targeted restriction was cleared one turn too early,
    // inferred from the wrong owner (ADR 0077).
    // Blaziken ex's Seething Spirit (attaching Energy from the
    // discard pile to a chosen own Pokemon) and its own attack,
    // Smolder-sault (AttackerCannotAttackNextTurn, already built)
    // admit its one print.
    // Chien-Pao's Strafe/Rising Blade print reuses SwitchOwnActive and
    // BonusDamageIfDefenderIsEx outright. Its other prints pair
    // Icicle Loop (puts an attached Energy into hand,
    // MoveOwnAttachedEnergyToHand) with Snow Sink (a play-triggered
    // Ability that may discard whichever Stadium is in play).
    // Completes all 3 Chien-Pao prints.
    // Iron Leaves ex's Rapid Vernier (a play-triggered switch, then
    // moving any amount of Energy from the player's other Pokemon to
    // the switched-in one) and its own attack, Prism Edge
    // (AttackerCannotAttackNextTurn, already built) complete all 6
    // Iron Leaves ex prints.
    // Toxtricity's Sinister Surge (search for a Basic Energy of a
    // type, attach it to a Benched Pokemon of the same type, then
    // damage it) admits all 3 prints (Gentle Slap has no printed
    // text).
    // Fan Rotom's Fan Call (a standing search restricted to the
    // player's own first turn, CardFilter::PokemonOfTypeWithHpAtMost)
    // and its own attack, Assault Landing (fizzles outright with no
    // Stadium in play) complete all 4 prints.
    // Pecharunt ex's Subjugating Chains (a standing switch of a
    // Benched Pokemon of a type, excluding a name, then Poisoning the
    // new Active) and its own attack, Irritated Outburst (damage per
    // Prize the opponent has taken) complete all 5 prints.
    // Genesect's Bug's Cannon (damage per a counted board fact, to a
    // chosen opponent Pokemon — the mirror of DamageChosenOpponentPokemon
    // opening the same phase) admits its me02-008 print.
    // Tool Scrapper (an Item: discard up to 2 Pokemon Tools attached
    // anywhere in play, either player's) admits all 3 of its prints —
    // the first Item admitted since the Items milestone's own ticket
    // order closed.
    // Alakazam's Powerful Hand (placing damage counters per a
    // counted board fact, bypassing the ordinary damage order
    // entirely — the same shape DamageCountersToOpponentBenchAnyWay
    // already takes) completes all 3 Alakazam prints.
    assert_eq!(admitted, 769, "coverage moves only on purpose");
}
