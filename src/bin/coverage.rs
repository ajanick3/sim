//! How much of Standard the engine can play.
//!
//! ```sh
//! cargo run --bin coverage            # the count and the reasons
//! cargo run --bin coverage -- refused # every refused card, by reason
//! ```

use sim::import::load;

fn main() {
    let path = "data/cards.json";
    let json = match std::fs::read_to_string(path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("{path}: {error}");
            eprintln!("Run: python3 tools/import_cards.py");
            std::process::exit(1);
        }
    };

    let import = match load(&json) {
        Ok(import) => import,
        Err(error) => {
            eprintln!("{path}: {error}");
            std::process::exit(1);
        }
    };

    let (admitted, total) = import.coverage();
    println!(
        "The engine plays {admitted} of {total} Standard cards ({:.1}%).",
        100.0 * admitted as f64 / total as f64
    );
    println!("\nRefused:");
    for (reason, count) in import.refusals_by_reason() {
        println!("{count:6}  {reason:?}");
    }

    if std::env::args().any(|a| a == "refused") {
        println!();
        for card in &import.refused {
            println!("{:<12} {:<28} {:?}", card.id, card.name, card.because);
        }
    }
}
