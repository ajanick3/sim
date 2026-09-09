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
//! The checker reads all four rules: sixty cards, four by name, one ACE SPEC
//! total, a Standard regulation mark.

use crate::import::{self, CardRef, Import};

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
    /// Lines the parser could not read at all, kept verbatim. One bad line
    /// does not spoil a list: the rest still reads, and the report names
    /// what it could not.
    pub unreadable: Vec<String>,
}

impl Decklist {
    /// How many cards the list holds.
    ///
    /// A line the parser could not read still counts toward the deck if its
    /// leading number reads, which it usually does: the count is the one
    /// part of a line that never varies by dialect. Rule 1 stays checkable
    /// even where a card does not resolve.
    pub fn total(&self) -> u32 {
        let read: u32 = self.lines.iter().map(|line| line.count).sum();
        let unread: u32 = self
            .unreadable
            .iter()
            .filter_map(|line| line.split_whitespace().next())
            .filter_map(|count| count.parse::<u32>().ok())
            .sum();
        read + unread
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
    /// Rule 3: at most one ACE SPEC card, of any kind, in the whole deck.
    TooManyAceSpecs { held: u32 },
    /// No card in the artifact matches the line.
    NoSuchCard { line: Line },
    /// The line's set and number name one card, and its printed name another.
    /// A decklist carries only a name, a set, and a number, so the two must
    /// agree or one of them is a typo.
    NameMismatch { printed: String, found: String },
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

/// Read a decklist.
///
/// Nothing here fails: a line the parser cannot read is kept verbatim in
/// [`Decklist::unreadable`], and [`check`] names it. A promo printed with no
/// number is the case that taught this — one such line used to cost the
/// whole list.
pub fn parse(text: &str) -> Decklist {
    let mut list = Decklist::default();

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || is_heading(line) {
            continue;
        }
        match read_line(line) {
            Some(entry) => list.lines.push(entry),
            None => list.unreadable.push(line.to_string()),
        }
    }

    list
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
        ..Report::default()
    };

    for line in &list.unreadable {
        report.uncheckable.push(format!("could not read: {line}"));
    }

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

        // A set this artifact does not hold cannot be resolved, and that is
        // not a fault in the deck. It is the same kind of fact as the ACE
        // SPEC rule: something the data cannot answer, said out loud.
        if !import
            .sets
            .iter()
            .any(|set| set.abbreviation.eq_ignore_ascii_case(&line.set_code))
        {
            report.uncheckable.push(format!(
                "{} {}: this artifact holds no set {}, so the card is unresolved.",
                line.count, line.name, line.set_code
            ));
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
                if !same_name(&line.name, &card.name) {
                    report.problems.push(Problem::NameMismatch {
                        printed: line.name.clone(),
                        found: card.name.clone(),
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

    // Rule 3: at most one ACE SPEC card, of any kind, in the whole deck.
    let ace_specs: u32 = report
        .matched
        .iter()
        .filter(|matched| import::is_ace_spec(&matched.card.id))
        .map(|matched| matched.line.count)
        .sum();
    if ace_specs > 1 {
        report
            .problems
            .push(Problem::TooManyAceSpecs { held: ace_specs });
    }

    report
}

/// Whether two names are the same card.
///
/// A list is typed and exported by many tools, so an accent, an apostrophe,
/// and a hyphen all vary. Compare the letters and digits and nothing else.
fn same_name(printed: &str, found: &str) -> bool {
    fn letters(name: &str) -> String {
        name.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(fold)
            .collect()
    }
    letters(printed) == letters(found)
}

/// Fold the accented letters a card name uses down to their plain form.
fn fold(c: char) -> char {
    match c {
        'é' | 'è' | 'ê' => 'e',
        'á' | 'à' | 'â' => 'a',
        'í' | 'ì' | 'î' => 'i',
        'ó' | 'ò' | 'ô' => 'o',
        'ú' | 'ù' | 'û' => 'u',
        other => other,
    }
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
