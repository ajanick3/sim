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
    // Mega Sharpedo ex (3 prints).
    assert_eq!(admitted, 546, "coverage moves only on purpose");
}
