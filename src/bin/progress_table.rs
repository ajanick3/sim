//! Regenerate the README's card-progress table from the artifact and the
//! committed decks. AGENTS.md asks for this table to move with every
//! commit that changes what plays, so this stays a committed tool rather
//! than a one-off script.
//!
//! ```sh
//! cargo run --bin progress_table
//! ```

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use sim::decklist::parse;
use sim::import::load;

fn every_deck_file(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("the decks directory is committed") {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir() {
            every_deck_file(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("txt") {
            out.push(path);
        }
    }
}

/// The line a function named `fn_name` starts at, and the line the next
/// top-level `fn` after it starts at (or the file's own end) — the span
/// `source_line` searches within, so `known_attack` and `known_ability`
/// each answer only from their own arms, despite sharing the exact same
/// `("Name", "...") =>` tuple shape in their own match arms.
fn function_span(import_rs: &[&str], fn_name: &str) -> (usize, usize) {
    let start = import_rs
        .iter()
        .position(|line| line.starts_with(&format!("fn {fn_name}(")))
        .expect("the named function is committed");
    let end = import_rs[start + 1..]
        .iter()
        .position(|line| line.starts_with("fn "))
        .map(|offset| start + 1 + offset)
        .unwrap_or(import_rs.len());
    (start, end)
}

/// The GitHub line a built card's own logic sits at, if the card has a
/// named entry in `src/import.rs` within `[start, end)` (a Trainer's
/// `known_trainer` arm, a Pokémon's `known_attack` arm, or its
/// `known_ability` arm). A card with no such entry — a plain Pokémon
/// with no attack effect to encode — links to `read_card`, the general
/// mechanism that admits it instead.
fn source_line(import_rs: &[&str], name: &str, (start, end): (usize, usize)) -> usize {
    let trainer_arm = format!("\"{name}\" =>");
    let named_arm = format!("(\"{name}\",");
    for (index, line) in import_rs[start..end].iter().enumerate() {
        if line.contains(&trainer_arm) || line.contains(&named_arm) {
            return start + index + 1;
        }
    }
    234 // fn read_card: the general admission path a plain card takes.
}

fn main() {
    let import_rs_text = std::fs::read_to_string("src/import.rs").expect("import.rs is committed");
    let import_rs: Vec<&str> = import_rs_text.lines().collect();

    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = load(&json).unwrap_or_else(|error| {
        eprintln!("data/cards.json: {error}");
        std::process::exit(1);
    });

    let mut paths = Vec::new();
    every_deck_file(Path::new("decks"), &mut paths);

    let mut deck_names: HashSet<String> = HashSet::new();
    for path in &paths {
        let text = std::fs::read_to_string(path).expect("a committed deck is readable");
        for line in parse(&text).lines {
            deck_names.insert(line.name.clone());
        }
    }

    let known_trainer_span = function_span(&import_rs, "known_trainer");
    let known_attack_span = function_span(&import_rs, "known_attack");
    let known_ability_span = function_span(&import_rs, "known_ability");
    let known_energy_span = function_span(&import_rs, "known_energy");

    // A name's category and kind, and whether any of its prints is
    // admitted. A name with several prints (an errata, a reprint) is
    // built the moment one print plays. A Pokémon tracks its Attacks
    // and its Ability separately — Milestone 11 and Milestone 8 make
    // separate progress on the same species, and `playable` alone
    // cannot tell the two apart.
    let mut kind_of: BTreeMap<String, &'static str> = BTreeMap::new();
    let mut built: HashSet<String> = HashSet::new();
    let mut attacks_built: HashSet<String> = HashSet::new();
    let mut has_ability: HashSet<String> = HashSet::new();
    let mut abilities_built: HashSet<String> = HashSet::new();
    for card in &import.cards {
        if !deck_names.contains(&card.name) {
            continue;
        }
        let kind = card.raw["category"].as_str().unwrap_or("");
        let label = if kind == "Pokemon" {
            "Pokemon"
        } else if kind == "Trainer" {
            match card.raw["trainerType"].as_str().unwrap_or("") {
                "Supporter" => "Supporter",
                "Item" => "Item",
                "Tool" => "Tool",
                "Stadium" => "Stadium",
                _ => continue,
            }
        } else if kind == "Energy" {
            "Energy"
        } else {
            continue;
        };
        kind_of.insert(card.name.clone(), label);
        if card.playable.is_some() {
            built.insert(card.name.clone());
        }
        if kind == "Pokemon" {
            if sim::import::attacks_read(&card.raw) {
                attacks_built.insert(card.name.clone());
            }
            if card.raw["abilities"].as_array().is_some_and(|a| !a.is_empty()) {
                has_ability.insert(card.name.clone());
                if sim::import::ability_reads(&card.raw) {
                    abilities_built.insert(card.name.clone());
                }
            }
        }
    }

    for kind in ["Supporter", "Item", "Tool", "Stadium", "Energy", "Pokemon"] {
        let mut names: Vec<&String> = kind_of
            .iter()
            .filter(|(_, k)| **k == kind)
            .map(|(n, _)| n)
            .collect();
        names.sort();
        let total = names.len();
        let done = names.iter().filter(|n| built.contains(**n)).count();
        let heading = if kind == "Pokemon" {
            "Pokémon".to_string()
        } else if kind == "Energy" {
            "Special Energy".to_string()
        } else {
            format!("{kind}s")
        };
        println!("### {heading} ({done}/{total} built)\n");
        if kind == "Pokemon" {
            println!("| Card | Attacks | Ability |");
            println!("| --- | --- | --- |");
            for name in names {
                let attacks_mark = if attacks_built.contains(name) { "✅" } else { "❌" };
                let attacks_cell = if attacks_built.contains(name) {
                    let line = source_line(&import_rs, name, known_attack_span);
                    format!("[{attacks_mark}](src/import.rs#L{line})")
                } else {
                    attacks_mark.to_string()
                };
                let ability_cell = if !has_ability.contains(name) {
                    "—".to_string()
                } else if abilities_built.contains(name) {
                    let line = source_line(&import_rs, name, known_ability_span);
                    format!("[✅](src/import.rs#L{line})")
                } else {
                    "❌".to_string()
                };
                let name_cell = if built.contains(name) {
                    let span = if attacks_built.contains(name) { known_attack_span } else { known_ability_span };
                    let line = source_line(&import_rs, name, span);
                    format!("[{name}](src/import.rs#L{line})")
                } else {
                    name.clone()
                };
                println!("| {name_cell} | {attacks_cell} | {ability_cell} |");
            }
        } else {
            let span = if kind == "Energy" { known_energy_span } else { known_trainer_span };
            println!("| Card | Status |");
            println!("| --- | --- |");
            for name in names {
                let mark = if built.contains(name) { "✅" } else { "❌" };
                let cell = if built.contains(name) {
                    let line = source_line(&import_rs, name, span);
                    format!("[{name}](src/import.rs#L{line})")
                } else {
                    name.clone()
                };
                println!("| {cell} | {mark} |");
            }
        }
        println!();
    }
}
