//! Reading the card artifact.
//!
//! The engine plays a small part of the game, so it can run a small part of
//! the cards. A card whose text the engine cannot execute is refused by name
//! and by reason, never half-loaded: a card that silently loses its effect
//! would play a game that is not Pokémon.
//!
//! Reading the file is the caller's job. This module takes the JSON as a
//! string, so the engine keeps no I/O.

use std::collections::HashMap;

use serde_json::Value;

use crate::card::{
    Attack, CardDb, CardDef, CardFilter, Destination, Energy, Pokemon, PromoteFollowUp,
    Requirement, Slot, Stage, TargetFilter, Then, Trainer, TrainerEffect, TrainerKind,
    TurnBonusTarget, Type, Zone,
};
use crate::ids::CardDefId;

/// Why the engine cannot run a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// A Trainer. The engine plays no card that carries rules text.
    IsATrainer(TrainerKind),
    /// A special Energy, which carries rules text. Basic Energy is not in the
    /// artifact at all.
    IsASpecialEnergy,
    /// A Stage 1 or Stage 2 Pokémon missing the name of the card it evolves
    /// from, so nothing could ever evolve into it.
    IsAnEvolution,
    /// A Pokémon card the engine cannot place, and nothing above fits.
    NotABasicPokemon,
    /// The card carries a held item, which has rules text of its own. No card
    /// in Standard does today; the guard keeps a future one from being
    /// admitted with its item ignored.
    HasAHeldItem,
    /// An ability is text the engine cannot execute.
    HasAnAbility,
    /// The attack carries an effect line, which is English, not a rule.
    AttackHasText,
    /// Damage such as `30+` or `60×` depends on an effect.
    DamageIsNotANumber,
    /// The card has nothing to do on a turn.
    HasNoAttack,
    /// A type, a Weakness, or a Resistance the engine does not model.
    UnknownSymbol,
}

/// One refused card, named so a reader can go and look at it.
#[derive(Debug, Clone)]
pub struct Refused {
    pub id: String,
    pub name: String,
    pub because: Refusal,
}

/// One card in the artifact, whether or not the engine can play it. It is
/// what a decklist matches against.
#[derive(Debug, Clone)]
pub struct CardRef {
    pub id: String,
    pub name: String,
    /// The set's TCGdex id, such as `me01`.
    pub set: String,
    /// The number printed on the card, without the leading zeroes a decklist
    /// leaves off.
    pub number: String,
    pub mark: String,
    /// The playable card, when the engine can run this one.
    pub playable: Option<CardDefId>,
    /// The card exactly as the artifact printed it, refused or not. A card
    /// the engine cannot run still has HP, attacks, and rules text a deck
    /// builder or a card browser needs; this is where they live. A card
    /// plays only when the engine can run all of it, regardless of what this
    /// field holds — ADR 0008 decided that, and this field does not revisit
    /// it.
    pub raw: Value,
}

/// A set, with the abbreviation a decklist prints.
#[derive(Debug, Clone)]
pub struct SetRef {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
}

/// The result of reading the artifact: the cards the engine can play, and an
/// account of every card it cannot.
#[derive(Debug)]
pub struct Import {
    pub db: CardDb,
    pub admitted: Vec<CardDefId>,
    pub refused: Vec<Refused>,
    /// Every card read, in artifact order.
    pub cards: Vec<CardRef>,
    pub sets: Vec<SetRef>,
    basic_energy: Vec<(Type, CardDefId)>,
}

impl Import {
    /// The basic Energy of a type, added to the database on first ask.
    ///
    /// Basic Energy is not printed in the Standard sets — it comes from a
    /// separate product and carries no regulation mark — so the artifact holds
    /// none. It is rules furniture rather than a card to import, so the engine
    /// supplies it.
    pub fn basic_energy(&mut self, kind: Type) -> CardDefId {
        if let Some(id) = self.basic_energy.iter().find(|(k, _)| *k == kind) {
            return id.1;
        }
        let id = self.db.add(CardDef::Energy(Energy {
            print_id: basic_energy_print_id(kind),
            name: basic_energy_name(kind),
            kind,
        }));
        self.basic_energy.push((kind, id));
        id
    }

    /// How many cards of those read the engine can play.
    pub fn coverage(&self) -> (usize, usize) {
        (
            self.admitted.len(),
            self.admitted.len() + self.refused.len(),
        )
    }

    /// How many cards each reason refused.
    pub fn refusals_by_reason(&self) -> Vec<(Refusal, usize)> {
        let reasons = [
            Refusal::IsATrainer(TrainerKind::Supporter),
            Refusal::IsATrainer(TrainerKind::Item),
            Refusal::IsATrainer(TrainerKind::Tool),
            Refusal::IsATrainer(TrainerKind::Stadium),
            Refusal::IsASpecialEnergy,
            Refusal::IsAnEvolution,
            Refusal::NotABasicPokemon,
            Refusal::HasAnAbility,
            Refusal::AttackHasText,
            Refusal::DamageIsNotANumber,
            Refusal::HasNoAttack,
            Refusal::UnknownSymbol,
        ];
        reasons
            .into_iter()
            .map(|reason| {
                let count = self.refused.iter().filter(|r| r.because == reason).count();
                (reason, count)
            })
            .filter(|(_, count)| *count > 0)
            .collect()
    }
}

/// Read the artifact. The error is for a file that is not the artifact at all.
pub fn load(json: &str) -> Result<Import, String> {
    let root: Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let cards = root["cards"]
        .as_array()
        .ok_or("the artifact holds no card list")?;

    let mut import = Import {
        db: CardDb::new(),
        admitted: Vec::new(),
        refused: Vec::new(),
        cards: Vec::new(),
        sets: read_sets(&root),
        basic_energy: Vec::new(),
    };

    // A chain from a Stage 2 to the Basic beneath it walks two links of
    // `evolveFrom`, and the middle print need not be admitted — only in the
    // pool. This reads every Pokémon record once, admitted or not, so the
    // walk in `read_card` never depends on what got refused.
    let lineage: HashMap<&str, &str> = cards
        .iter()
        .filter(|c| c["category"].as_str() == Some("Pokemon"))
        .filter_map(|c| Some((c["name"].as_str()?, c["evolveFrom"].as_str()?)))
        .collect();

    for card in cards {
        let playable = match read_card(card, &lineage) {
            Ok(def) => {
                let id = import.db.add(def);
                import.admitted.push(id);
                Some(id)
            }
            Err(because) => {
                import.refused.push(Refused {
                    id: card["id"].as_str().unwrap_or("?").to_string(),
                    name: card["name"].as_str().unwrap_or("?").to_string(),
                    because,
                });
                None
            }
        };
        import.cards.push(CardRef {
            id: card["id"].as_str().unwrap_or("?").to_string(),
            name: card["name"].as_str().unwrap_or("?").to_string(),
            set: card["set"].as_str().unwrap_or("?").to_string(),
            // A decklist prints 3 where the artifact holds 001.
            number: card["localId"]
                .as_str()
                .unwrap_or("")
                .trim_start_matches('0')
                .to_string(),
            mark: card["regulationMark"].as_str().unwrap_or("?").to_string(),
            playable,
            raw: card.clone(),
        });
    }

    Ok(import)
}

fn read_sets(root: &Value) -> Vec<SetRef> {
    root["sets"]
        .as_array()
        .map(|sets| {
            sets.iter()
                .map(|set| SetRef {
                    id: set["id"].as_str().unwrap_or("?").to_string(),
                    name: set["name"].as_str().unwrap_or("?").to_string(),
                    abbreviation: set["abbreviation"].as_str().unwrap_or("").to_string(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn read_card(card: &Value, lineage: &HashMap<&str, &str>) -> Result<CardDef, Refusal> {
    match card["category"].as_str() {
        Some("Trainer") => {
            let kind = trainer_kind(card);
            let name = card["name"].as_str().unwrap_or("?");
            let id = card["id"].as_str().unwrap_or("?");
            // A print is matched by its own id first, and only falls back
            // to its name where nothing more specific claims it. Safe
            // today because nothing does — see `known_trainer_by_print`.
            return match known_trainer_by_print(id).or_else(|| known_trainer(name)) {
                Some((requirement, effect)) => Ok(CardDef::Trainer(Trainer {
                    print_id: leak(id),
                    name: leak(name),
                    kind,
                    requirement,
                    effect,
                })),
                None => Err(Refusal::IsATrainer(kind)),
            };
        }
        Some("Energy") => return Err(Refusal::IsASpecialEnergy),
        Some("Pokemon") => {}
        _ => return Err(Refusal::NotABasicPokemon),
    }
    let stage = match card["stage"].as_str() {
        Some("Basic") => Stage::Basic,
        Some("Stage1") => Stage::Stage1,
        Some("Stage2") => Stage::Stage2,
        _ => return Err(Refusal::NotABasicPokemon),
    };
    let evolve_from = match card["stage"].as_str() {
        Some("Basic") => None,
        // Rule 19 matches an evolution to the Pokémon it names, by name. A
        // Stage 1 or 2 card missing that name cannot be evolved into, so it
        // is refused rather than admitted with no way to play it.
        Some(_) => match card["evolveFrom"].as_str() {
            Some(from) => Some(leak(from)),
            None => return Err(Refusal::IsAnEvolution),
        },
        None => return Err(Refusal::NotABasicPokemon),
    };
    if card["abilities"].as_array().is_some_and(|a| !a.is_empty()) {
        return Err(Refusal::HasAnAbility);
    }
    if card["item"].is_object() {
        return Err(Refusal::HasAHeldItem);
    }

    let attacks_json = card["attacks"].as_array().ok_or(Refusal::HasNoAttack)?;
    if attacks_json.is_empty() {
        return Err(Refusal::HasNoAttack);
    }
    let mut attacks = Vec::new();
    for attack in attacks_json {
        attacks.push(read_attack(attack)?);
    }

    let kind = card["types"]
        .as_array()
        .and_then(|types| types.first())
        .and_then(Value::as_str)
        .and_then(read_type)
        .ok_or(Refusal::UnknownSymbol)?;

    // The Basic two links beneath a Stage 2. `evolve_from` already named the
    // Stage 1; its own `evolveFrom`, read from the lineage table rather than
    // from this printing, names the Basic. A Stage 1 in the middle that
    // this engine refused (an ability, say) still answers, since the table
    // holds every printed Pokémon, not only the admitted ones.
    let evolves_from_basic = match stage {
        Stage::Stage2 => evolve_from
            .and_then(|parent| lineage.get(parent))
            .map(|b| leak(b)),
        Stage::Basic | Stage::Stage1 => None,
    };

    Ok(CardDef::Pokemon(Pokemon {
        print_id: leak(card["id"].as_str().unwrap_or("?")),
        name: leak(card["name"].as_str().unwrap_or("?")),
        hp: card["hp"].as_u64().ok_or(Refusal::UnknownSymbol)? as u32,
        kind,
        weakness: read_modifier(&card["weaknesses"], &["×2", "x2"])?,
        resistance: read_modifier(&card["resistances"], &["-30"])?,
        retreat_cost: card["retreat"].as_u64().unwrap_or(0) as u8,
        prizes: prizes_for(card["name"].as_str().unwrap_or("")),
        stage,
        evolve_from,
        evolves_from_basic,
        attacks,
    }))
}

/// A print whose behaviour cannot be read from its name, because the name
/// is shared with a print whose behaviour differs. Checked before
/// `known_trainer`, and keyed on the print's own id rather than its name.
///
/// Empty of any real card, deliberately. `tools/check_trainer_name_safety.py`
/// checks every Trainer name in the artifact, ignoring whitespace, and finds
/// each one carries a single effect — a name-based match is safe for every
/// card built so far ([ADR 0020](../docs/adr/0020-a-trainer-name-is-matched-unless-a-print-overrides-it.md)).
/// The day a name stops being safe, its prints are named here instead of
/// matched by name, one arm at a time; `test-print-a` and `test-print-b`
/// exist only so `tests/import.rs` can prove the override wins, the same
/// way [`crate::cards`] ships a small synthetic pool for the engine's own
/// tests to play with.
fn known_trainer_by_print(print_id: &str) -> Option<(Option<Requirement>, TrainerEffect)> {
    Some(match print_id {
        "test-print-a" => (None, TrainerEffect::Nothing),
        "test-print-b" => (None, TrainerEffect::SwitchOpponentActive),
        _ => return None,
    })
}

/// Every Trainer the engine has built, by printed name, with what the card
/// demands before it may be played and what it does once it is. Each name
/// has one distinct behaviour across every printing in the artifact today,
/// so a name match is safe; a card with the same name and a genuinely
/// different effect is matched by print id instead, in
/// `known_trainer_by_print`.
///
/// The requirement and the effect share this one table on purpose. A card's
/// whole behaviour is one fact, and a second table keyed by the same name
/// would let a card gain an effect here and lose its requirement there.
fn known_trainer(name: &str) -> Option<(Option<Requirement>, TrainerEffect)> {
    // Most cards demand nothing to be played.
    let free = None;
    Some(match name {
        "Boss's Orders" => (free, TrainerEffect::SwitchOpponentActive),
        "Judge" => (free, TrainerEffect::BothShuffleHandThenDraw { you: 4, opponent: 4 }),
        "Lillie's Determination" => (
            free,
            TrainerEffect::ShuffleHandThenDraw {
                normal: 6,
                at_six_prizes: 8,
            },
        ),
        "Night Stretcher" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::PokemonOrBasicEnergy,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Poké Pad" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::PokemonWithoutRuleBox,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Crushing Hammer" => (free, TrainerEffect::CoinFlipDiscardOpponentEnergy),
        "Buddy-Buddy Poffin" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::BasicPokemonWithHpAtMost(70),
                    to: Destination::Bench,
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Cyrano" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::PokemonEx,
                    to: Destination::Zone(Zone::Hand),
                    limit: 3,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Gwynn" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Hand,
                slots: vec![Slot {
                    filter: CardFilter::PokemonWithoutRuleBox,
                    to: Destination::Zone(Zone::Discard),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: Some(Then::DrawPerCardMoved(3)),
            },
        ),
        "Sacred Ash" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::AnyPokemon,
                    to: Destination::Zone(Zone::Library),
                    limit: 5,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Ultra Ball" => (
            Some(Requirement::DiscardOtherCardsFromHand(2)),
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::AnyPokemon,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Energy Switch" => (free, TrainerEffect::MoveAttachedEnergy),
        "Hilda" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![
                    Slot {
                        filter: CardFilter::EvolutionPokemon,
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::BasicEnergy,
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                ],
                then: None,
            },
        ),
        "Dawn" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![
                    Slot {
                        filter: CardFilter::PokemonOfStage(Stage::Basic),
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::PokemonOfStage(Stage::Stage1),
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::PokemonOfStage(Stage::Stage2),
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                ],
                then: None,
            },
        ),
        "Special Red Card" => (
            Some(Requirement::OpponentPrizesAtMost(3)),
            TrainerEffect::OpponentHandToBottomThenDraw { count: 3 },
        ),
        "Crispin" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![
                    Slot {
                        filter: CardFilter::BasicEnergy,
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::BasicEnergy,
                        to: Destination::Attach(TargetFilter::AnyInPlay),
                        limit: 1,
                        excludes_type_of_previous: true,
                        peek: None,
                    },
                ],
                then: None,
            },
        ),
        "Rare Candy" => (free, TrainerEffect::EvolveSkippingOneStage),
        "Team Rocket's Petrel" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::AnyTrainer,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "N's PP Up" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Attach(TargetFilter::BenchedNameStartsWith("N's")),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Wondrous Patch" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergyOfType(Type::Psychic),
                    to: Destination::Attach(TargetFilter::BenchedOfType(Type::Psychic)),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Pokégear 3.0" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::TrainerOfKind(TrainerKind::Supporter),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: Some(7),
                }],
                then: None,
            },
        ),
        "Bug Catching Set" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::PokemonOfTypeOrBasicEnergyOfType(Type::Grass),
                    to: Destination::Zone(Zone::Hand),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: Some(7),
                }],
                then: None,
            },
        ),
        "Ciphermaniac's Codebreaking" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::AnyCard,
                    to: Destination::TopOfLibraryInOrder,
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Unfair Stamp" => (
            Some(Requirement::KnockedOutDuringOpponentsLastTurn),
            TrainerEffect::BothShuffleHandThenDraw {
                you: 5,
                opponent: 2,
            },
        ),
        "Switch" => (free, TrainerEffect::SwitchOwnActive),
        "Jumbo Ice Cream" => (
            Some(Requirement::ActiveHasAtLeastEnergy(3)),
            TrainerEffect::HealActive(80),
        ),
        "Lana's Aid" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::PokemonWithoutRuleBoxOrBasicEnergy,
                    to: Destination::Zone(Zone::Hand),
                    limit: 3,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Rust Syndicate Grunt" => (
            Some(Requirement::KnockedOutDuringOpponentsLastTurn),
            TrainerEffect::DiscardOpponentEnergy,
        ),
        "N's Plan" => (free, TrainerEffect::MoveEnergyFromBenchToActive { limit: 2 }),
        "Pokémon Center Lady" => (free, TrainerEffect::HealChosen(60)),
        "Rosa's Encouragement" => (
            Some(Requirement::MorePrizesThanOpponent),
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Attach(TargetFilter::OfStage(Stage::Stage2)),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "AZ's Tranquility" => (
            free,
            TrainerEffect::SwitchOwnActiveWithFollowUp(PromoteFollowUp::HealDisplacedIfEx(80)),
        ),
        "Surfer" => (
            free,
            TrainerEffect::SwitchOwnActiveWithFollowUp(PromoteFollowUp::DrawUpTo(5)),
        ),
        "Black Belt's Training" => (
            free,
            TrainerEffect::BonusDamageThisTurn(40, TurnBonusTarget::OpponentActiveEx),
        ),
        "Gladion's Final Battle" => (
            Some(Requirement::HandSizeIs(1)),
            TrainerEffect::BonusDamageThisTurn(80, TurnBonusTarget::OpponentActiveWithoutRuleBox),
        ),
        "Kieran" => (
            free,
            TrainerEffect::ChooseOneOf(
                Box::new(TrainerEffect::SwitchOwnActive),
                Box::new(TrainerEffect::BonusDamageThisTurn(
                    30,
                    TurnBonusTarget::OpponentActiveEx,
                )),
            ),
        ),
        "Morty's Conviction" => (
            Some(Requirement::DiscardOtherCardsFromHand(1)),
            TrainerEffect::DrawPerOpponentBenched,
        ),
        "Xerosic's Machinations" => (free, TrainerEffect::OpponentDiscardsDownTo(3)),
        "Eri" => (
            free,
            TrainerEffect::DiscardFromOpponentsHand {
                filter: CardFilter::TrainerOfKind(TrainerKind::Item),
                limit: 2,
            },
        ),
        "Brock's Scouting" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![
                    Slot {
                        filter: CardFilter::PokemonOfStage(Stage::Basic),
                        to: Destination::Zone(Zone::Hand),
                        limit: 2,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::EvolutionPokemon,
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                ],
                then: None,
            },
        ),
        "Wally's Compassion" => (free, TrainerEffect::HealMegaExAndTakeEnergyIfHealed),
        "Janine's Secret Art" => (free, TrainerEffect::JaninesSecretArt),
        "Energy Search" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Energy Retrieval" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Zone(Zone::Hand),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Hand Trimmer" => (free, TrainerEffect::BothDiscardDownTo(5)),
        "Dusk Ball" => (free, TrainerEffect::LookAtBottomOfLibrary { count: 7 }),
        "Prime Catcher" => (free, TrainerEffect::SwitchOpponentActiveThenOwn),
        "Strange Timepiece" => (free, TrainerEffect::DevolveChosen),
        "Transformation Tome" => (
            Some(Requirement::SecondCopyOfThisInHand),
            TrainerEffect::SwapBasicWithDiscard,
        ),
        "Secret Box" => (
            Some(Requirement::DiscardOtherCardsFromHand(3)),
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![
                    Slot {
                        filter: CardFilter::TrainerOfKind(TrainerKind::Item),
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::TrainerOfKind(TrainerKind::Tool),
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::TrainerOfKind(TrainerKind::Supporter),
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::TrainerOfKind(TrainerKind::Stadium),
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                ],
                then: None,
            },
        ),
        "Team Rocket's Transceiver" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Library,
                slots: vec![Slot {
                    filter: CardFilter::SupporterNameContains("Team Rocket"),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Energy Recycler" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Zone(Zone::Library),
                    limit: 5,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        _ => return None,
    })
}

/// A Trainer's kind. TCGdex writes `Tool` for a Pokémon Tool.
fn trainer_kind(card: &Value) -> TrainerKind {
    match card["trainerType"].as_str() {
        Some("Supporter") => TrainerKind::Supporter,
        Some("Stadium") => TrainerKind::Stadium,
        Some("Tool") => TrainerKind::Tool,
        // TCGdex leaves the kind off a handful of Trainers. An Item is the
        // kind with no limit of its own, so it is the safe reading.
        _ => TrainerKind::Item,
    }
}

/// What a knockout of this card is worth, read from its name.
///
/// TCGdex's `suffix` field cannot be trusted for this: it is absent on 21 ex
/// cards, `Mega Charizard X ex` among them, and it uses both `ex` and `EX`.
/// The name is exact — every card carrying a suffix also ends in ` ex`, and 21
/// more do — so the name is what this reads. A Trainer named `Mega Signal` is
/// why the caller must already know this is a Pokémon.
fn prizes_for(name: &str) -> u32 {
    if !name.to_lowercase().ends_with(" ex") {
        return 1;
    }
    if name.starts_with("Mega ") { 3 } else { 2 }
}

fn read_attack(attack: &Value) -> Result<Attack, Refusal> {
    if attack["effect"].as_str().is_some_and(|e| !e.is_empty()) {
        return Err(Refusal::AttackHasText);
    }
    let base_damage = match &attack["damage"] {
        Value::Number(damage) => damage.as_u64().ok_or(Refusal::DamageIsNotANumber)? as u32,
        Value::Null => 0,
        _ => return Err(Refusal::DamageIsNotANumber),
    };

    let mut cost = Vec::new();
    for symbol in attack["cost"].as_array().unwrap_or(&Vec::new()) {
        let kind = symbol
            .as_str()
            .and_then(read_type)
            .ok_or(Refusal::UnknownSymbol)?;
        cost.push(kind);
    }

    Ok(Attack {
        name: leak(attack["name"].as_str().unwrap_or("?")),
        cost,
        base_damage,
        inflicts: None,
    })
}

/// A Weakness or a Resistance, but only in the one shape the engine applies.
/// TCGdex writes the multiplier with either a multiplication sign or an x.
fn read_modifier(field: &Value, allowed: &[&str]) -> Result<Option<Type>, Refusal> {
    let Some(entries) = field.as_array() else {
        return Ok(None);
    };
    let Some(first) = entries.first() else {
        return Ok(None);
    };
    let value = first["value"].as_str().unwrap_or("");
    if !allowed.contains(&value) {
        return Err(Refusal::UnknownSymbol);
    }
    first["type"]
        .as_str()
        .and_then(read_type)
        .ok_or(Refusal::UnknownSymbol)
        .map(Some)
}

fn basic_energy_name(kind: Type) -> &'static str {
    match kind {
        Type::Grass => "Grass Energy",
        Type::Fire => "Fire Energy",
        Type::Water => "Water Energy",
        Type::Lightning => "Lightning Energy",
        Type::Psychic => "Psychic Energy",
        Type::Fighting => "Fighting Energy",
        Type::Darkness => "Darkness Energy",
        Type::Metal => "Metal Energy",
        Type::Dragon => "Dragon Energy",
        Type::Colorless => "Colorless Energy",
    }
}

/// A stable id for the supplied basic Energy, since it has no printed one.
fn basic_energy_print_id(kind: Type) -> &'static str {
    match kind {
        Type::Grass => "basic-grass-energy",
        Type::Fire => "basic-fire-energy",
        Type::Water => "basic-water-energy",
        Type::Lightning => "basic-lightning-energy",
        Type::Psychic => "basic-psychic-energy",
        Type::Fighting => "basic-fighting-energy",
        Type::Darkness => "basic-darkness-energy",
        Type::Metal => "basic-metal-energy",
        Type::Dragon => "basic-dragon-energy",
        Type::Colorless => "basic-colorless-energy",
    }
}

fn read_type(name: &str) -> Option<Type> {
    Some(match name {
        "Grass" => Type::Grass,
        "Fire" => Type::Fire,
        "Water" => Type::Water,
        "Lightning" => Type::Lightning,
        "Psychic" => Type::Psychic,
        "Fighting" => Type::Fighting,
        "Darkness" => Type::Darkness,
        "Metal" => Type::Metal,
        "Dragon" => Type::Dragon,
        "Colorless" => Type::Colorless,
        _ => return None,
    })
}

/// A card name outlives the game that reads it. The literals in
/// [`crate::cards`] are `&'static str`, and an imported name joins them by
/// being leaked once, at load, and never freed.
fn leak(name: &str) -> &'static str {
    Box::leak(name.to_string().into_boxed_str())
}
