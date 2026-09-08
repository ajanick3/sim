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
    // Enhanced Hammer discards a Special Energy — refused outright,
    // since this pool never admits a special Energy at all
    // (Refusal::IsASpecialEnergy), so no target could ever exist.
    assert_eq!(
        reason_for("sv06-148"),
        Some(Refusal::IsATrainer(TrainerKind::Item)),
        "Enhanced Hammer is an Item"
    );
    assert_eq!(
        reason_for("sv10-180"),
        Some(Refusal::IsATrainer(TrainerKind::Stadium)),
        "Team Rocket's Watchtower is a Stadium"
    );
    assert_eq!(
        reason_for("me02-124"),
        Some(Refusal::IsASpecialEnergy),
        "Ignition Energy is a special Energy"
    );
    assert_eq!(
        reason_for("me02.5-137"),
        Some(Refusal::HasAnAbility),
        "N's Zoroark ex's Ability cost is deferred (no sample-deck card \
         pairs one with a buildable attack yet — ADR 0070)"
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
    assert_eq!(trainers, 273, "the Trainers still refused, by kind");
    // Milestone 12 (Special Energy) admitted Growing Grass Energy,
    // the first print, taking this from 21 to 20.
    assert_eq!(count(Refusal::IsASpecialEnergy), 20, "every Energy card");
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
