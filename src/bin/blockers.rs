//! Count each committed deck's cards by their first blocker: a Trainer not
//! yet built, an Ability, attack text, and so on. `coverage` counts the
//! whole 3051-card artifact; this counts the 3660 slots the field actually
//! plays, which is where a milestone's target cards are chosen from.
//!
//! ```sh
//! cargo run --bin blockers
//! ```

use std::collections::HashMap;
use std::path::Path;

use sim::decklist::{check, parse};
use sim::import::{Refusal, load};

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

fn refusal_label(reason: Refusal) -> &'static str {
    match reason {
        Refusal::IsATrainer(_) => "Trainer, not yet built",
        Refusal::IsASpecialEnergy => "Special Energy",
        Refusal::IsAnEvolution => "Evolution missing its parent",
        Refusal::NotABasicPokemon => "Not a Basic Pokemon",
        Refusal::HasAHeldItem => "Held item",
        Refusal::HasAnAbility => "Ability",
        Refusal::AttackHasText => "Attack text",
        Refusal::DamageIsNotANumber => "Variable damage",
        Refusal::HasNoAttack => "No attack",
        Refusal::UnknownSymbol => "Unknown symbol",
    }
}

fn main() {
    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = load(&json).unwrap_or_else(|error| {
        eprintln!("data/cards.json: {error}");
        std::process::exit(1);
    });
    let refusal_by_id: HashMap<&str, Refusal> = import
        .refused
        .iter()
        .map(|r| (r.id.as_str(), r.because))
        .collect();

    let mut paths = Vec::new();
    every_deck_file(Path::new("decks"), &mut paths);

    let mut slots: HashMap<&'static str, u32> = HashMap::new();
    let mut total = 0u32;
    // Named by blocker, so a card blocked for a reason worth planning
    // around can be picked out without re-deriving the whole table.
    let mut by_name: HashMap<&'static str, HashMap<String, u32>> = HashMap::new();

    for path in &paths {
        let text = std::fs::read_to_string(path).expect("a committed deck is readable");
        let list = parse(&text);
        let report = check(&list, &import);
        total += report.total;

        for line in &list.lines {
            if line.is_basic_energy() {
                *slots.entry("Basic Energy, plays").or_default() += line.count;
            }
        }

        for matched in &report.matched {
            let count = matched.line.count;
            let label = if matched.card.playable.is_some() {
                "Plays"
            } else {
                refusal_by_id
                    .get(matched.card.id.as_str())
                    .map(|r| refusal_label(*r))
                    .unwrap_or("Unmatched to a refusal")
            };
            *slots.entry(label).or_default() += count;
            *by_name
                .entry(label)
                .or_default()
                .entry(matched.card.name.clone())
                .or_default() += count;
        }
    }

    println!("{} decks, {total} slots\n", paths.len());

    let mut rows: Vec<_> = slots.into_iter().collect();
    rows.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    println!("{:<24} {:>6} {:>7}", "Blocker", "Slots", "Share");
    for (label, count) in &rows {
        println!(
            "{:<24} {:>6} {:>6.1}%",
            label,
            count,
            100.0 * *count as f64 / total as f64
        );
    }

    for label in ["Ability", "Trainer, not yet built", "Attack text"] {
        let Some(names) = by_name.get(label) else {
            continue;
        };
        let mut named: Vec<_> = names.iter().collect();
        named.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
        println!("\nTop {label} by slot count:");
        for (name, count) in named.iter().take(40) {
            println!("  {count:>4}  {name}");
        }
        println!("({} distinct names)", named.len());
    }
}
