//! Ticket 06: the engine names every kind of card, even one it cannot play.

use sim::card::TrainerKind;
use sim::import::{Refusal, load};

fn artifact() -> String {
    std::fs::read_to_string("data/cards.json").expect("the artifact is committed")
}

#[test]
fn a_refusal_names_the_kind_of_card_it_refused() {
    let import = load(&artifact()).unwrap();
    let reason_for = |id: &str| {
        import
            .refused
            .iter()
            .find(|r| r.id == id)
            .map(|r| r.because)
    };

    assert_eq!(
        reason_for("me01-113"),
        Some(Refusal::IsATrainer(TrainerKind::Supporter)),
        "a Supporter is refused as a Supporter"
    );
    assert_eq!(
        reason_for("me01-116"),
        Some(Refusal::IsATrainer(TrainerKind::Item)),
        "Fighting Gong is an Item"
    );
    assert_eq!(
        reason_for("me01-122"),
        Some(Refusal::IsATrainer(TrainerKind::Stadium)),
        "Mystery Garden is a Stadium"
    );
    assert_eq!(
        reason_for("me02-124"),
        Some(Refusal::IsASpecialEnergy),
        "Ignition Energy is a special Energy"
    );
    assert_eq!(
        reason_for("me02.5-072"),
        Some(Refusal::HasAnAbility),
        "Iono's Kilowattrel's Flashing Draw is the same cost-gated \
         Ability shape ADR 0070 deferred, still unbuilt"
    );
}

#[test]
fn the_kinds_add_up_to_the_pool() {
    let import = load(&artifact()).unwrap();
    let count = |reason: Refusal| {
        import
            .refused
            .iter()
            .filter(|r| r.because == reason)
            .count()
    };

    let trainers: usize = [
        TrainerKind::Supporter,
        TrainerKind::Item,
        TrainerKind::Tool,
        TrainerKind::Stadium,
    ]
    .into_iter()
    .map(|kind| count(Refusal::IsATrainer(kind)))
    .sum();

    // Ticket 03 of the record effort admitted 22 prints of the eight
    // Trainers it built (each name in one, sometimes several, printings), so
    // the pool of 445 split: the rest were still refused, named by kind. The
    // second batch of Trainers takes 48 more: Cyrano, Buddy-Buddy Poffin,
    // Ultra Ball, Special Red Card, Energy Switch, Hilda, Dawn, Crispin,
    // Rare Candy, Team Rocket's Petrel, N's PP Up, Wondrous Patch, Pokégear
    // 3.0, Bug Catching Set, Ciphermaniac's Codebreaking, Unfair Stamp,
    // Switch, Jumbo Ice Cream, Lana's Aid, Rust Syndicate Grunt, N's Plan,
    // Pokémon Center Lady, Rosa's Encouragement, AZ's Tranquility, Surfer,
    // Black Belt's Training, Gladion's Final Battle, Kieran, Morty's
    // Conviction, Xerosic's Machinations, Eri, Brock's Scouting, Wally's
    // Compassion, Janine's Secret Art, Energy Search, Energy Retrieval,
    // Energy Recycler, Team Rocket's Transceiver, Hand Trimmer, Secret
    // Box, Dusk Ball, Prime Catcher, Strange Timepiece, Transformation
    // Tome, Air Balloon, Hero's Cape, Brave Bangle, Binding Mochi,
    // Lillie's Pearl, Punk Helmet, Lucky Helmet, Handheld Fan,
    // Powerglass, Gravity Mountain, N's Castle, Academy at Night,
    // Team Rocket's Factory, Lumiose City, Jamming Tower, Risky Ruins,
    // Forest of Vitality, and Festival Grounds. Tool Scrapper, built
    // this session (`TrainerEffect::MayDiscardUpToTwoToolsAnywhere`),
    // takes it down to 273.
    // Tera Orb took it to 272; Briar (four prints) took it to 268;
    // Enhanced Hammer (two prints) took it to 266; Glass Trumpet
    // (four prints) took it to 262; Nighttime Mine took it to 261;
    // Area Zero Underdepths (three prints) took it to 258; Battle
    // Cage (two prints) took it to 256; Team Rocket's Watchtower
    // (two prints) took it to 254; Colress's Tenacity (two prints)
    // takes it to 252.
    //
    // The standard-trainers effort opens beyond the field: Cheren,
    // Friends in Paldea, and Urbain (`Draw(3)`) took it to 247; Cook,
    // Fennel, and Clemont's Quick Wit (heals) took it to 241; Picnicker, Jett, and Iris's
    // Fighting Spirit (draw variants) took it to 235; Carmine, Team
    // Rocket's Archer, and Cassiopeia take it to 225.
    // Firebreather and Canari take it to 220.
    // Tarragon and Philippe take it to 216.
    // Emcee's Hype, Billy & O'Nare, and Emma take it to 211.
    // Drasna, Harlequin, and Naveen take it to 205.
    // Hassel and Drayton take it to 199.
    // Potion, Poké Vital A, and Jacinthe take it to 194.
    // The six search Items take it to 188.
    // Hole-Digging Shovel, Repel, and Brilliant Blender take it to 185.
    // Dangerous Laser and Dark Bell take it to 182.
    // The six "-Berry" Tools take it to 176.
    assert_eq!(trainers, 176, "the Trainers still refused, by kind");
    // Milestone 12 (Special Energy), now closed, admitted Growing
    // Grass Energy, Enriching Energy, Telepathic Psychic Energy,
    // Spiky Energy (two prints), Mist Energy, Boomerang Energy, and
    // Prism Energy (two prints), taking this from 21 to 12.
    // The standard-trainers effort's type-plus-rider Special Energy
    // (Rocky Fighting, Magnetic Metal, Bubbly Water) take this to 9.
    assert_eq!(count(Refusal::IsASpecialEnergy), 9, "every Energy card");
    assert_eq!(
        count(Refusal::IsAnEvolution),
        0,
        "every Stage 1 and 2 card in Standard names what it evolves from"
    );
    assert_eq!(
        import.admitted.len() + import.refused.len(),
        3051,
        "and nothing is lost"
    );
}

#[test]
fn markers_read_ex_mega_and_tera_by_print_not_species() {
    use sim::card::Marker;
    use sim::import::markers_for;

    // Dragapult ex, sv06-130 — a confirmed Tera print, also ex.
    assert_eq!(markers_for("sv06-130", "Dragapult ex"), vec![Marker::Ex, Marker::Tera]);
    // Meowth ex, me03-062 — an ordinary ex, never Tera.
    assert_eq!(markers_for("me03-062", "Meowth ex"), vec![Marker::Ex]);
    // Mega Chandelure ex, me05-038 — worth Ex and Mega together.
    assert_eq!(
        markers_for("me05-038", "Mega Chandelure ex"),
        vec![Marker::Ex, Marker::Mega]
    );
    // A plain Basic carries no markers at all.
    assert_eq!(markers_for("sv06.5-032", "Zoroark"), Vec::<Marker>::new());
}

#[test]
fn hydreigon_ex_has_both_a_tera_print_and_a_plain_one() {
    use sim::card::Marker;
    use sim::import::markers_for;

    // The same name, sv08 (Surging Sparks) is Tera and sv10.5w is not —
    // the reason Tera cannot be read from the name alone.
    assert_eq!(markers_for("sv08-119", "Hydreigon ex"), vec![Marker::Ex, Marker::Tera]);
    assert_eq!(markers_for("sv10.5w-067", "Hydreigon ex"), vec![Marker::Ex]);
}

#[test]
fn koraidon_ex_has_an_ancient_print_and_two_plain_ones() {
    use sim::card::Marker;
    use sim::import::markers_for;

    // The same Koraidon ex split Tera already forced print-id
    // precision for: only the Temporal Forces print is Ancient.
    assert_eq!(
        markers_for("sv05-120", "Koraidon ex"),
        vec![Marker::Ex, Marker::Ancient]
    );
    // me02.5-121 is Koraidon ex's own Tera print, not Ancient.
    assert_eq!(markers_for("me02.5-121", "Koraidon ex"), vec![Marker::Ex, Marker::Tera]);
    assert_eq!(markers_for("svp-197", "Koraidon ex"), vec![Marker::Ex]);
}

#[test]
fn iron_crown_ex_carries_future_on_every_print() {
    use sim::card::Marker;
    use sim::import::markers_for;

    for print_id in ["sv05-081", "sv05-191", "sv05-206", "sv05-216", "sv08.5-158", "svp-146"] {
        assert_eq!(
            markers_for(print_id, "Iron Crown ex"),
            vec![Marker::Ex, Marker::Future],
            "every Iron Crown ex print in the artifact is Future"
        );
    }
}

#[test]
fn prizes_is_computed_from_markers_not_the_name_directly() {
    use sim::import::{markers_for, prizes_for};

    assert_eq!(prizes_for(&markers_for("sv06.5-032", "Zoroark")), 1, "no markers");
    assert_eq!(prizes_for(&markers_for("me03-062", "Meowth ex")), 2, "Marker::Ex");
    assert_eq!(
        prizes_for(&markers_for("sv06-130", "Dragapult ex")),
        2,
        "Ex and Tera together, still worth 2"
    );
    assert_eq!(
        prizes_for(&markers_for("me05-038", "Mega Chandelure ex")),
        3,
        "Ex and Mega together, worth 3"
    );
}

#[test]
fn a_supporter_and_a_stadium_are_once_per_turn() {
    use sim::cards::{milestone1, starter_decklist};
    use sim::rng::SeededRng;
    use sim::state::GameState;

    let set = milestone1();
    let decklist = starter_decklist(&set);
    let state = GameState::new(
        set.db,
        [decklist.clone(), decklist],
        Box::new(SeededRng::new(9)),
    );
    let one = sim::ids::PlayerId::One;
    assert!(
        !state.is_spent(sim::state::Limit::SupporterPlayed(one)),
        "rule 13: one Supporter a turn"
    );
    assert!(
        !state.is_spent(sim::state::Limit::StadiumPlayed(one)),
        "rule 13: one Stadium a turn"
    );
}
