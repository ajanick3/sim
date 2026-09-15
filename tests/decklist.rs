//! Ticket 01 of the decklists effort: read a decklist and check it.

use sim::decklist::{Problem, check, parse, to_deck};
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
    let list = parse(LIST);
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
    let list = parse(LIST);
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
    let list = parse("48 Basic Grass Energy SVE 1\n");
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
    // A set this artifact does hold, at a card that is not in it.
    let list = parse("4 Not A Real Card MEG 999\n");
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
    let list = parse("4 Chikorita MEG 8\n");
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

    let list = parse("5 Chikorita MEG 8\n");
    let report = check(&list, &import);
    assert!(
        report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::TooManyCopies { .. })),
        "rule 2: at most 4 by name"
    );

    let list = parse("48 Basic Grass Energy SVE 1\n");
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
    let list = parse(LIST);
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
fn a_deck_may_hold_one_ace_spec() {
    let import = load(&artifact()).unwrap();
    let list = parse("1 Prime Catcher TEF 157\n");
    let report = check(&list, &import);
    assert!(
        !report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::TooManyAceSpecs { .. })),
        "one ACE SPEC is legal: {:?}",
        report.problems
    );
}

#[test]
fn two_ace_specs_of_different_names_still_break_rule_3() {
    let import = load(&artifact()).unwrap();
    let list = parse("1 Prime Catcher TEF 157\n1 Unfair Stamp TWM 165\n");
    let report = check(&list, &import);
    assert!(
        report
            .problems
            .contains(&Problem::TooManyAceSpecs { held: 2 }),
        "rule 3 counts every ACE SPEC together, not one limit per name: {:?}",
        report.problems
    );
}

/// Every `.txt` file anywhere under `decks/`.
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
        let text = std::fs::read_to_string(&path).unwrap();
        let list = parse(&text);
        let report = check(&list, &import);

        assert_eq!(report.total, 60, "{path:?} holds 60 cards");
        assert!(
            report.problems.is_empty(),
            "{path:?} checks clean: {:?}",
            report.problems
        );
        checked += 1;
    }

    assert!(checked >= 143, "every deck is read: {checked} found");
}

#[test]
fn a_line_the_parser_cannot_read_does_not_fail_the_whole_list() {
    // A promo printed with no number at all. The rest of the list still
    // reads, and the line it could not read is named.
    let list = parse("4 Chikorita MEG 8\n1 Flutter Mane SV P\n");
    assert_eq!(list.lines.len(), 1, "the readable line still read");
    assert_eq!(list.unreadable.len(), 1);
    assert!(list.unreadable[0].contains("Flutter Mane"));

    let import = load(&artifact()).unwrap();
    let report = check(&list, &import);
    assert!(
        report
            .uncheckable
            .iter()
            .any(|n| n.contains("Flutter Mane")),
        "and the report says what it could not read: {:?}",
        report.uncheckable
    );
}

#[test]
fn an_unknown_set_code_is_uncheckable_rather_than_a_problem() {
    let import = load(&artifact()).unwrap();
    // A Japanese-region set code. This artifact holds the English pool, so
    // the line cannot be resolved — which is not a fault in the deck.
    let list = parse("3 Slowking SV7 039\n");
    let report = check(&list, &import);

    assert!(
        !report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::NoSuchCard { .. })),
        "an unknown set code is not the same as no such card: {:?}",
        report.problems
    );
    assert!(
        report.uncheckable.iter().any(|n| n.contains("SV7")),
        "the report names the set it does not hold: {:?}",
        report.uncheckable
    );
}

#[test]
fn an_unknown_set_code_is_a_missing_line() {
    let import = load(&artifact()).unwrap();
    let list = parse("3 Slowking SV7 039\n");
    let report = check(&list, &import);

    assert_eq!(report.missing.len(), 1, "{:?}", report.missing);
    assert_eq!(report.missing[0].name, "Slowking");
    assert_eq!(report.missing[0].count, 3);
}

#[test]
fn a_wrong_number_in_a_set_we_do_hold_is_also_a_missing_line() {
    let import = load(&artifact()).unwrap();
    let list = parse("1 Chikorita MEG 9999\n");
    let report = check(&list, &import);

    assert_eq!(report.missing.len(), 1, "{:?}", report.missing);
    assert_eq!(report.missing[0].name, "Chikorita");
}

#[test]
fn a_wrong_number_in_a_set_we_do_hold_is_still_a_problem() {
    let import = load(&artifact()).unwrap();
    let list = parse("1 Chikorita MEG 9999\n");
    let report = check(&list, &import);
    assert!(
        report
            .problems
            .iter()
            .any(|p| matches!(p, Problem::NoSuchCard { .. })),
        "a set we hold, at a number that is not in it, is a real fault"
    );
}

#[test]
fn a_line_whose_name_disagrees_with_the_card_is_reported() {
    let import = load(&artifact()).unwrap();
    // MEG 3 is Mega Venusaur ex. The number decides the card; the name is
    // checked against it, so a typo in either is caught.
    let list = parse("4 Mega Charizard ex MEG 3\n");
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
    let list = parse("4 Poke Pad ASC 198\n");
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

#[test]
fn basic_energy_counts_toward_playable_not_only_the_rest_of_the_deck() {
    let import = load(&artifact()).unwrap();
    let list = parse(LIST);
    let report = check(&list, &import);
    // LIST holds 48 Basic Grass Energy; Mega Venusaur ex has an Ability
    // and is refused. Chikorita plays, and so does whatever card MEG 113
    // actually prints (Acerola's Mischief, not the "Professor's
    // Research" this fixture's own line names — a stale label, not a
    // fault `playable` reads).
    assert_eq!(
        report.playable,
        4 + 4 + 48,
        "Chikorita, MEG 113's real card, and every Basic Energy — not Mega Venusaur ex"
    );
}

#[test]
fn a_fully_playable_real_deck_reports_is_playable() {
    let import = load(&artifact()).unwrap();
    let text = std::fs::read_to_string("decks/2026-worlds/009-ojvind-svinhufvud.txt")
        .expect("a committed deck");
    let list = parse(&text);
    let report = check(&list, &import);
    assert!(report.is_legal(), "{:?}", report.problems);
    assert!(
        report.is_playable(),
        "{} of {} play",
        report.playable,
        report.total
    );
}

#[test]
fn to_deck_builds_sixty_cards_including_basic_energy() {
    let mut import = load(&artifact()).unwrap();
    let text = std::fs::read_to_string("decks/2026-worlds/009-ojvind-svinhufvud.txt")
        .expect("a committed deck");
    let list = parse(&text);
    let report = check(&list, &import);
    assert!(report.is_legal() && report.is_playable());

    let deck = to_deck(&list, &report, &mut import);
    assert_eq!(deck.len(), 60, "one CardDefId per physical card");
}

#[test]
fn to_deck_resolves_basic_energy_through_import_not_the_artifact() {
    let mut import = load(&artifact()).unwrap();
    // Unlike LIST, every card here plays — Mega Venusaur ex does not.
    let list = parse(
        "4 Chikorita MEG 8\n4 Boss's Orders MEG 114\n52 Basic Grass Energy SVE 1\n",
    );
    let report = check(&list, &import);
    assert!(report.is_legal() && report.is_playable(), "{:?}", report.problems);

    let deck = to_deck(&list, &report, &mut import);
    assert_eq!(deck.len(), 60);
    let grass = import.basic_energy(sim::card::Type::Grass);
    let grass_count = deck.iter().filter(|id| **id == grass).count();
    assert_eq!(grass_count, 52, "this list's own 52 Basic Grass Energy");
}
