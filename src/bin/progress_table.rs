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

/// The GitHub line a built card's own logic sits at, if the card has a
/// named entry in `src/import.rs` (a Trainer's `known_trainer` arm, or a
/// Pokémon's `known_attack` arm). A card with no such entry — a plain
/// Pokémon with no attack effect to encode — links to `read_card`, the
/// general mechanism that admits it instead.
fn source_line(import_rs: &[&str], name: &str) -> usize {
    let trainer_arm = format!("\"{name}\" =>");
    let attack_arm = format!("(\"{name}\",");
    for (index, line) in import_rs.iter().enumerate() {
        if line.contains(&trainer_arm) || line.contains(&attack_arm) {
            return index + 1;
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

    // A name's category and kind, and whether any of its prints is
    // admitted. A name with several prints (an errata, a reprint) is
    // built the moment one print plays.
    let mut kind_of: BTreeMap<String, &'static str> = BTreeMap::new();
    let mut built: HashSet<String> = HashSet::new();
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
        } else {
            continue;
        };
        kind_of.insert(card.name.clone(), label);
        if card.playable.is_some() {
            built.insert(card.name.clone());
        }
    }

    for kind in ["Supporter", "Item", "Tool", "Stadium", "Pokemon"] {
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
        } else {
            format!("{kind}s")
        };
        println!("### {heading} ({done}/{total} built)\n");
        println!("| Card | Status |");
        println!("| --- | --- |");
        for name in names {
            let mark = if built.contains(name) { "✅" } else { "❌" };
            let cell = if built.contains(name) {
                let line = source_line(&import_rs, name);
                format!("[{name}](src/import.rs#L{line})")
            } else {
                name.clone()
            };
            println!("| {cell} | {mark} |");
        }
        println!();
    }
}
