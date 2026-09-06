//! Reading a decklist, and checking it against deck construction.
//!
//! The format is the one the official client exports: a count, a name, a set
//! abbreviation, and the number printed on the card.
//!
//! ```text
//! Pokémon: 8
//! 4 Mega Venusaur ex MEG 3
//! Total Cards: 60
//! ```
//!
//! The checker reads rules 1, 2, and 4 — sixty cards, four by name, a Standard
//! regulation mark. Rule 3, one ACE SPEC per deck, is not in the data, and the
//! report says so rather than guessing.

use crate::import::{CardRef, Import};

/// The nine basic Energy. Rule 2 exempts them from the four-copy limit, and
/// they carry no regulation mark, so the artifact holds none of them.
const BASIC_ENERGY: [&str; 9] = [
    "Grass",
    "Fire",
    "Water",
    "Lightning",
    "Psychic",
    "Fighting",
    "Darkness",
    "Metal",
    "Dragon",
];

/// One line of a decklist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub count: u32,
    pub name: String,
    pub set_code: String,
    pub number: String,
}

impl Line {
    /// Whether this line is a basic Energy, which the four-copy limit exempts.
    pub fn is_basic_energy(&self) -> bool {
        let name = self.name.trim_start_matches("Basic ");
        BASIC_ENERGY
            .iter()
            .any(|kind| name == format!("{kind} Energy"))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Decklist {
    pub lines: Vec<Line>,
}

impl Decklist {
    /// How many cards the list holds.
    pub fn total(&self) -> u32 {
        self.lines.iter().map(|line| line.count).sum()
    }
}

/// A line matched to the card it names.
#[derive(Debug, Clone)]
pub struct Matched {
    pub line: Line,
    pub card: CardRef,
}

/// What is wrong with a decklist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    /// Rule 1: a deck is exactly 60 cards.
    WrongTotal { held: u32 },
    /// Rule 2: at most 4 of a card by name, basic Energy excepted.
    TooManyCopies { name: String, held: u32 },
    /// Rule 4: the card must carry a Standard regulation mark.
    NotStandard { name: String, mark: String },
    /// No card in the artifact matches the line.
    NoSuchCard { line: Line },
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub total: u32,
    /// How many cards of the deck the engine can play.
    pub playable: u32,
    pub matched: Vec<Matched>,
    pub problems: Vec<Problem>,
    /// Rules the data cannot answer. Named, so a reader is not misled by a
    /// clean report.
    pub uncheckable: Vec<String>,
}

impl Report {
    pub fn is_legal(&self) -> bool {
        self.problems.is_empty()
    }
}

/// Read a decklist. A line the parser cannot read at all is an error naming it.
pub fn parse(text: &str) -> Result<Decklist, Vec<String>> {
    let mut list = Decklist::default();
    let mut errors = Vec::new();

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || is_heading(line) {
            continue;
        }
        match read_line(line) {
            Some(entry) => list.lines.push(entry),
            None => errors.push(format!("cannot read: {line}")),
        }
    }

    if errors.is_empty() {
        Ok(list)
    } else {
        Err(errors)
    }
}

/// A section header or the total, neither of which is a card.
fn is_heading(line: &str) -> bool {
    line.ends_with(':')
        || line
            .split_once(':')
            .is_some_and(|(_, rest)| rest.trim().parse::<u32>().is_ok())
}

/// `4 Mega Venusaur ex MEG 3` — a count, a name of any length, a set code, a
/// number. The name is whatever lies between.
fn read_line(line: &str) -> Option<Line> {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() < 4 {
        return None;
    }
    let count = words[0].parse().ok()?;
    let number = words[words.len() - 1];
    let set_code = words[words.len() - 2];
    if !number.chars().next()?.is_ascii_digit() {
        return None;
    }
    if !set_code.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    Some(Line {
        count,
        name: words[1..words.len() - 2].join(" "),
        set_code: set_code.to_string(),
        number: number.to_string(),
    })
}

/// Check a decklist against the cards in the artifact.
pub fn check(list: &Decklist, import: &Import) -> Report {
    let mut report = Report {
        total: list.total(),
        uncheckable: vec![
            "Rule 3, one ACE SPEC per deck: the card data carries no ACE SPEC field.".to_string(),
        ],
        ..Report::default()
    };

    if report.total != 60 {
        report
            .problems
            .push(Problem::WrongTotal { held: report.total });
    }

    for line in &list.lines {
        if line.count > 4 && !line.is_basic_energy() {
            report.problems.push(Problem::TooManyCopies {
                name: line.name.clone(),
                held: line.count,
            });
        }

        if line.is_basic_energy() {
            // Basic Energy is not in the artifact and the engine supplies it.
            continue;
        }

        match find(import, line) {
            Some(card) => {
                if !matches!(card.mark.as_str(), "H" | "I" | "J") {
                    report.problems.push(Problem::NotStandard {
                        name: card.name.clone(),
                        mark: card.mark.clone(),
                    });
                }
                if card.playable.is_some() {
                    report.playable += line.count;
                }
                report.matched.push(Matched {
                    line: line.clone(),
                    card: card.clone(),
                });
            }
            None => report
                .problems
                .push(Problem::NoSuchCard { line: line.clone() }),
        }
    }

    report
}

/// Find the card a line names, by its set abbreviation and its number.
fn find(import: &Import, line: &Line) -> Option<CardRef> {
    let set = import
        .sets
        .iter()
        .find(|set| set.abbreviation.eq_ignore_ascii_case(&line.set_code))?;
    import
        .cards
        .iter()
        .find(|card| card.set == set.id && card.number == line.number.trim_start_matches('0'))
        .cloned()
}
