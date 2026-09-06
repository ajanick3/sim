//! How much memory keeping every card's raw record costs.
//!
//! ```sh
//! cargo run --release --bin memcheck
//! ```
//!
//! Reads `/proc/self/status`, so this only reports a real number on Linux;
//! elsewhere it prints what it read and lets the reader judge.

use sim::import::load;

fn main() {
    let before = resident_kb();

    let json = std::fs::read_to_string("data/cards.json").expect("the artifact is committed");
    let import = load(&json).expect("the artifact loads");
    // Touch every raw record, so nothing about the measurement is lazy.
    let bytes: usize = import.cards.iter().map(|c| c.raw.to_string().len()).sum();

    let after = resident_kb();

    println!(
        "{} cards, raw JSON re-serializes to {bytes} bytes",
        import.cards.len()
    );
    match (before, after) {
        (Some(before), Some(after)) => {
            println!(
                "resident memory: {before} KiB before load, {after} KiB after, +{} KiB",
                after.saturating_sub(before)
            );
        }
        _ => println!("could not read /proc/self/status on this platform"),
    }
}

fn resident_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status
        .lines()
        .find(|line| line.starts_with("VmRSS:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|n| n.parse().ok())
}
