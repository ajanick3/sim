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

/// Three of the 2026 Worlds decks are known exceptions, not parser bugs:
///
/// - `35-minho-song.txt` and `44-kazuki-yuasa.txt` name cards by a
///   Japanese-region set code (`SV6a`, `M1S`, `MC`, ...) rather than the
///   English one this artifact holds. Matching a card across regional
///   databases is an unsolved problem, recorded in
///   `docs/architecture/sources.md`.
/// - `55-joji-koyama.txt` carries a promo card printed with no number at
///   all (`1 Flutter Mane SV P`), which the format has nowhere to put.
///
/// Each is the source data disagreeing with what this parser can read, not a
/// defect in the parser. They are named here so a fix to either problem is a
/// one-line removal, not a rediscovery.
const KNOWN_EXCEPTIONS: &[&str] = &[
    "35-minho-song.txt",
    "44-kazuki-yuasa.txt",
    "55-joji-koyama.txt",
];

/// Every `.txt` file anywhere under `decks/`, deepest first is not required.
fn every_deck_file(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            every_deck_file(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("txt") {
            out.push(path);
        }
    }
}

#[test]
fn every_committed_deck_is_legal_and_every_line_matches() {
    let import = load(&artifact()).unwrap();
    let mut paths = Vec::new();
    every_deck_file(std::path::Path::new("decks"), &mut paths);

    let mut checked = 0;
    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap();
        if KNOWN_EXCEPTIONS.contains(&name) {
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

    assert!(checked >= 60, "the decks are being read: {checked} found");
}

#[test]
fn the_known_exceptions_still_fail_for_the_reason_recorded() {
    let import = load(&artifact()).unwrap();

    for name in KNOWN_EXCEPTIONS {
        let path = std::path::Path::new("decks/2026-worlds").join(name);
        let text = std::fs::read_to_string(&path).unwrap();
        match parse(&text) {
            Ok(list) => {
                let report = check(&list, &import);
                assert!(
                    !report.problems.is_empty(),
                    "{name} was expected to disagree with the artifact, and now agrees: \
                     remove it from KNOWN_EXCEPTIONS"
                );
            }
            Err(_) => {
                // A line the parser cannot read at all, e.g. joji-koyama's
                // promo card with no number. Failing to parse is the
                // expected exception for this file.
            }
        }
    }
}

#[test]
fn a_line_whose_name_disagrees_with_the_card_is_reported() {
    let import = load(&artifact()).unwrap();
    // MEG 3 is Mega Venusaur ex. The number decides the card; the name is
    // checked against it, so a typo in either is caught.
    let list = parse("4 Mega Charizard ex MEG 3\n").unwrap();
    let report = check(&list, &import);
    assert!(
        report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::NameMismatch { .. })),
        "the printed name must agree with the card the number names: {:?}",
        report.problems
    );
}

#[test]
fn the_name_check_allows_the_punctuation_a_list_drops() {
    let import = load(&artifact()).unwrap();
    // Lists are exported with varying accents and apostrophes.
    let list = parse("4 Poke Pad ASC 198\n").unwrap();
    let report = check(&list, &import);
    assert!(
        !report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::NameMismatch { .. })),
        "an accent is not a disagreement: {:?}",
        report.problems
    );
}
