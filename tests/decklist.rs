//! Ticket 01 of the decklists effort: read a decklist and check it.

use sim::decklist::{Problem, check, parse};
use sim::import::load;

const LIST: &str = "\
Pokémon: 8
4 Mega Venusaur ex MEG 3
4 Chikorita MEG 8

Trainer: 4
4 Professor's Research MEG 113

Energy: 48
48 Basic Grass Energy SVE 1

Total Cards: 60
";

fn artifact() -> String {
    std::fs::read_to_string("data/cards.json").expect("the artifact is committed")
}

#[test]
fn a_line_of_the_export_becomes_a_card() {
    let list = parse(LIST).expect("the export parses");
    assert_eq!(list.lines.len(), 4, "headers and totals are not cards");

    let first = &list.lines[0];
    assert_eq!(first.count, 4);
    assert_eq!(first.name, "Mega Venusaur ex");
    assert_eq!(first.set_code, "MEG");
    assert_eq!(first.number, "3");
    assert_eq!(list.total(), 60, "the counts add up to the deck");
}

#[test]
fn a_card_resolves_to_the_artifact_by_set_and_number() {
    let import = load(&artifact()).unwrap();
    let list = parse(LIST).unwrap();
    let report = check(&list, &import);

    let venusaur = report
        .matched
        .iter()
        .find(|m| m.line.number == "3")
        .expect("MEG 3 is a card");
    assert_eq!(venusaur.card.id, "me01-003");
    assert_eq!(venusaur.card.name, "Mega Venusaur ex");
}

#[test]
fn basic_energy_resolves_even_though_the_artifact_holds_none() {
    let import = load(&artifact()).unwrap();
    let list = parse("48 Basic Grass Energy SVE 1\n").unwrap();
    let report = check(&list, &import);
    assert!(
        report.problems.is_empty()
            || !report
                .problems
                .iter()
                .any(|p| matches!(p, Problem::NoSuchCard { .. })),
        "basic Energy is known by name: {:?}",
        report.problems
    );
}

#[test]
fn a_line_that_matches_nothing_is_reported() {
    let import = load(&artifact()).unwrap();
    let list = parse("4 Not A Real Card ZZZ 999\n").unwrap();
    let report = check(&list, &import);
    assert!(
        report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::NoSuchCard { .. })),
        "an unmatched line is named, never skipped"
    );
}

#[test]
fn a_deck_must_hold_exactly_sixty_cards() {
    let import = load(&artifact()).unwrap();
    let list = parse("4 Chikorita MEG 8\n").unwrap();
    let report = check(&list, &import);
    assert!(
        report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::WrongTotal { held: 4 })),
        "rule 1: exactly 60 cards"
    );
}

#[test]
fn five_copies_of_a_card_is_too_many_but_energy_is_exempt() {
    let import = load(&artifact()).unwrap();

    let list = parse("5 Chikorita MEG 8\n").unwrap();
    let report = check(&list, &import);
    assert!(
        report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::TooManyCopies { .. })),
        "rule 2: at most 4 by name"
    );

    let list = parse("48 Basic Grass Energy SVE 1\n").unwrap();
    let report = check(&list, &import);
    assert!(
        !report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::TooManyCopies { .. })),
        "rule 2: basic Energy has no limit"
    );
}

#[test]
fn the_report_says_how_much_of_the_deck_the_engine_can_play() {
    let import = load(&artifact()).unwrap();
    let list = parse(LIST).unwrap();
    let report = check(&list, &import);

    // Chikorita is admitted; Mega Venusaur ex has an ability and is refused.
    assert!(report.playable > 0, "some of the deck is playable");
    assert!(
        report.playable < report.total,
        "and some of it is not: {} of {}",
        report.playable,
        report.total
    );
}

#[test]
fn the_uncheckable_rule_is_named() {
    let import = load(&artifact()).unwrap();
    let list = parse(LIST).unwrap();
    let report = check(&list, &import);
    assert!(
        report
            .uncheckable
            .iter()
            .any(|note| note.contains("ACE SPEC")),
        "rule 3 cannot be checked from this data, and the report says so"
    );
}

#[test]
fn every_committed_deck_is_legal_and_every_line_matches() {
    let import = load(&artifact()).unwrap();
    let mut checked = 0;

    for entry in std::fs::read_dir("decks").expect("the decks directory is committed") {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        let list = parse(&text).unwrap_or_else(|errors| panic!("{path:?}: {errors:?}"));
        let report = check(&list, &import);

        assert_eq!(report.total, 60, "{path:?} holds 60 cards");
        assert!(
            report.problems.is_empty(),
            "{path:?} checks clean: {:?}",
            report.problems
        );
        checked += 1;
    }

    assert!(checked >= 2, "the decks are being read: {checked} found");
}
