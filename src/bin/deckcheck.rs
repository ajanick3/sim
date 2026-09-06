//! Check a decklist against deck construction, and say how much of it the
//! engine can play.
//!
//! ```sh
//! cargo run --bin deckcheck -- my-deck.txt
//! ```

use sim::decklist::{Problem, check, parse};
use sim::import::load;

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: deckcheck <decklist.txt>");
        std::process::exit(2);
    };

    let text = read_or_exit(&path);
    let artifact = read_or_exit("data/cards.json");
    let import = load(&artifact).unwrap_or_else(|error| {
        eprintln!("data/cards.json: {error}");
        std::process::exit(1);
    });

    let list = parse(&text);

    let report = check(&list, &import);
    println!("{} cards, {} lines", report.total, list.lines.len());

    if report.is_legal() {
        println!("Legal, as far as the data can say.");
    } else {
        println!("\nProblems:");
        for problem in &report.problems {
            println!("  {}", describe(problem));
        }
    }

    println!(
        "\nThe engine can play {} of {} cards ({:.0}%).",
        report.playable,
        report.total,
        100.0 * report.playable as f64 / report.total as f64
    );
    for matched in &report.matched {
        let mark = if matched.card.playable.is_some() {
            "plays"
        } else {
            "     "
        };
        println!(
            "  {mark}  {:>2} {:<28} {}",
            matched.line.count, matched.card.name, matched.card.id
        );
    }

    println!("\nNot checked:");
    for note in &report.uncheckable {
        println!("  {note}");
    }
}

fn describe(problem: &Problem) -> String {
    match problem {
        Problem::WrongTotal { held } => format!("Rule 1: a deck is 60 cards, this holds {held}"),
        Problem::TooManyCopies { name, held } => {
            format!("Rule 2: {held} copies of {name}, the limit is 4")
        }
        Problem::NotStandard { name, mark } => {
            format!("Rule 4: {name} carries mark {mark}, which is not Standard")
        }
        Problem::NameMismatch { printed, found } => {
            format!("{printed} does not match the card at that number, {found}")
        }
        Problem::NoSuchCard { line } => format!(
            "No card matches: {} {} {} {}",
            line.count, line.name, line.set_code, line.number
        ),
    }
}

fn read_or_exit(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| {
        eprintln!("{path}: {error}");
        std::process::exit(1);
    })
}
