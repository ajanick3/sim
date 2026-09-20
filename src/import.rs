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
    Ability, AbilityEffect, Attack, AttackEffect, CardDb, CardDef, CardFilter, Condition, Count,
    Destination, Energy, Marker, Pokemon, PromoteFollowUp, Requirement, Slot, Stage, TargetFilter,
    Then, Trainer, TrainerEffect, TrainerKind, TurnBonusTarget, Type, Zone,
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
            effect: None,
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
        Some("Energy") => {
            let name = card["name"].as_str().unwrap_or("?");
            let id = card["id"].as_str().unwrap_or("?");
            return match known_energy(name) {
                Some((kind, effect)) => Ok(CardDef::Energy(Energy {
                    print_id: leak(id),
                    name: leak(name),
                    kind,
                    effect: Some(effect),
                })),
                None => Err(Refusal::IsASpecialEnergy),
            };
        }
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
    let name_for_ability = card["name"].as_str().unwrap_or("?");
    let ability = read_ability(name_for_ability, card["abilities"].as_array())?;
    if card["item"].is_object() {
        return Err(Refusal::HasAHeldItem);
    }

    let attacks_json = card["attacks"].as_array().ok_or(Refusal::HasNoAttack)?;
    if attacks_json.is_empty() {
        return Err(Refusal::HasNoAttack);
    }
    let name = card["name"].as_str().unwrap_or("?");
    let mut attacks = Vec::new();
    for attack in attacks_json {
        attacks.push(read_attack(name, attack)?);
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

    let print_id = card["id"].as_str().unwrap_or("?");
    let markers = markers_for(print_id, name);
    Ok(CardDef::Pokemon(Pokemon {
        print_id: leak(print_id),
        name: leak(name),
        hp: card["hp"].as_u64().ok_or(Refusal::UnknownSymbol)? as u32,
        kind,
        weakness: read_modifier(&card["weaknesses"], &["×2", "x2"])?,
        resistance: read_modifier(&card["resistances"], &["-30"])?,
        retreat_cost: card["retreat"].as_u64().unwrap_or(0) as u8,
        prizes: prizes_for(&markers),
        stage,
        evolve_from,
        evolves_from_basic,
        ability,
        attacks,
        markers,
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
        "Cheren" | "Friends in Paldea" | "Urbain" => (free, TrainerEffect::Draw(3)),
        "Cook" => (free, TrainerEffect::HealActive(70)),
        "Hole-Digging Shovel" => (free, TrainerEffect::DiscardTopOfDeck(2)),
        "Maximum Belt" => (free, TrainerEffect::BonusDamageVsActiveEx(50)),
        "Sacred Charm" => (free, TrainerEffect::ReducesDamageFromAbilityHolders(30)),
        "Lively Stadium" => (free, TrainerEffect::StadiumBoostsBasicHp(30)),
        "Rescue Board" => (free, TrainerEffect::ReducesRetreatCost(1)),
        "Jasmine's Gaze" => (
            free,
            TrainerEffect::GrantSideShieldNextTurn(crate::card::SideShield::DamageReduction(30)),
        ),
        "Iron Defender" => (
            free,
            TrainerEffect::GrantSideShieldNextTurn(
                crate::card::SideShield::DamageReductionForType(Type::Metal, 30),
            ),
        ),
        "Roxie's Performance" => (
            free,
            TrainerEffect::GrantSideShieldNextTurn(
                crate::card::SideShield::OpponentPoisonedCannotRetreat,
            ),
        ),
        "Awakening Drum" => (
            free,
            TrainerEffect::DrawPerOwnPokemonWithMarker(crate::card::Marker::Ancient),
        ),
        "Reboot Pod" => (free, TrainerEffect::AttachBasicEnergyFromDiscardToEachFuture),
        "Perilous Jungle" => (free, TrainerEffect::StadiumExtraPoisonDamage(20)),
        "Gravity Gemstone" => (
            free,
            TrainerEffect::RaisesBothActiveRetreatWhileCarrierActive(1),
        ),
        "Poké Ball" => (
            free,
            TrainerEffect::CoinFlipThen(Box::new(TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::AnyPokemon,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            })),
        ),
        "Team Rocket's Proton" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::BasicPokemonNameContains("Team Rocket's"),
                    to: Destination::Zone(Zone::Hand),
                    limit: 3,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Hop's Bag" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::BasicPokemonNameContains("Hop's"),
                    to: Destination::Bench,
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Cynthia's Power Weight" => (
            free,
            TrainerEffect::IncreasesHpForNamePrefix { word: "Cynthia's", amount: 70 },
        ),
        "Granite Cave" => (
            free,
            TrainerEffect::StadiumReducesDamageForNamePrefix { word: "Steven's", amount: 30 },
        ),
        "Full Metal Lab" => (
            free,
            TrainerEffect::StadiumReducesDamageToType { kind: Type::Metal, amount: 30 },
        ),
        "Babiri Berry" => (
            free,
            TrainerEffect::ReducesDamageFromType { kind: Type::Metal, amount: 60 },
        ),
        "Colbur Berry" => (
            free,
            TrainerEffect::ReducesDamageFromType { kind: Type::Darkness, amount: 60 },
        ),
        "Occa Berry" => (
            free,
            TrainerEffect::ReducesDamageFromType { kind: Type::Fire, amount: 60 },
        ),
        "Passho Berry" => (
            free,
            TrainerEffect::ReducesDamageFromType { kind: Type::Water, amount: 60 },
        ),
        "Payapa Berry" => (
            free,
            TrainerEffect::ReducesDamageFromType { kind: Type::Psychic, amount: 60 },
        ),
        "Haban Berry" => (
            free,
            TrainerEffect::ReducesDamageFromType { kind: Type::Dragon, amount: 60 },
        ),
        "Dangerous Laser" => (
            free,
            TrainerEffect::InflictOnOpponentActive(
                crate::card::Condition::Burned,
                Some(crate::card::Condition::Confused),
            ),
        ),
        "Dark Bell" => (
            free,
            TrainerEffect::ConfuseBothActivesExceptType(Type::Darkness),
        ),
        "Repel" => (free, TrainerEffect::SwitchOutOpponentActive),
        "Brilliant Blender" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::AnyCard,
                    to: Destination::Zone(Zone::Discard),
                    limit: 5,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Master Ball" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
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
        "Hyper Aroma" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::PokemonOfStage(Stage::Stage1),
                    to: Destination::Zone(Zone::Hand),
                    limit: 3,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Treasure Tracker" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::TrainerOfKind(TrainerKind::Tool),
                    to: Destination::Zone(Zone::Hand),
                    limit: 5,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Boxed Order" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::TrainerOfKind(TrainerKind::Item),
                    to: Destination::Zone(Zone::Hand),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: Some(Then::EndTurnIfMoved),
            },
        ),
        "Max Rod" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::PokemonOrBasicEnergy,
                    to: Destination::Zone(Zone::Hand),
                    limit: 5,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Miracle Headset" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::TrainerOfKind(TrainerKind::Supporter),
                    to: Destination::Zone(Zone::Hand),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Lumiose Galette" => (free, TrainerEffect::HealActiveAndClearConditions(20)),
        "Dragon Elixir" => (
            free,
            TrainerEffect::HealChosenPlain { amount: 60, of_type: Some(Type::Dragon) },
        ),
        "Potion" => (
            free,
            TrainerEffect::HealChosenPlain { amount: 30, of_type: None },
        ),
        "Poké Vital A" => (
            free,
            TrainerEffect::HealChosenPlain { amount: 150, of_type: None },
        ),
        "Jacinthe" => (
            free,
            TrainerEffect::HealChosenPlain {
                amount: 150,
                of_type: Some(Type::Psychic),
            },
        ),
        "Picnicker" => (free, TrainerEffect::CoinFlipDraw { heads: 4, tails: 2 }),
        "Carmine" => (free, TrainerEffect::DiscardHandThenDraw(5)),
        "Tarragon" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::PokemonOfTypeOrBasicEnergyOfType(Type::Fighting),
                    to: Destination::Zone(Zone::Hand),
                    limit: 4,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Philippe" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergyOfType(Type::Metal),
                    to: Destination::Attach(TargetFilter::OfType(Type::Metal)),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Firebreather" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergyOfType(Type::Fire),
                    to: Destination::Zone(Zone::Hand),
                    limit: 7,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Canari" => (
            Some(Requirement::DiscardOtherCardsFromHand(1)),
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::PokemonOfType(Type::Lightning),
                    to: Destination::Zone(Zone::Hand),
                    limit: 4,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Team Rocket's Archer" => (
            Some(Requirement::KnockedOutDuringOpponentsLastTurn),
            TrainerEffect::BothShuffleHandThenDraw { you: 5, opponent: 3 },
        ),
        "Cassiopeia" => (
            Some(Requirement::HandSizeIs(1)),
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::AnyCard,
                    to: Destination::Zone(Zone::Hand),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Jett" => (free, TrainerEffect::DrawPerOpponentMegaEx),
        "Emcee's Hype" => (
            free,
            TrainerEffect::DrawThenBonusIfOpponentPrizesAtMost {
                base: 2,
                bonus: 2,
                at_most: 3,
            },
        ),
        "Billy & O'Nare" => (
            free,
            TrainerEffect::DrawThenBonusIfHandAtLeast {
                base: 2,
                bonus: 2,
                at_least: 10,
            },
        ),
        "Emma" => (free, TrainerEffect::DrawPerPokemonInOpponentHand),
        "Drasna" => (
            free,
            TrainerEffect::ShuffleHandThenCoinFlipDraw { heads: 8, tails: 3 },
        ),
        "Hassel" => (
            Some(Requirement::KnockedOutDuringOpponentsLastTurn),
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::AnyCard,
                    to: Destination::Zone(Zone::Hand),
                    limit: 3,
                    excludes_type_of_previous: false,
                    peek: Some(8),
                }],
                then: None,
            },
        ),
        "Explorer's Guidance" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::AnyCard,
                    to: Destination::Zone(Zone::Hand),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: Some(6),
                }],
                then: Some(Then::DiscardRestOfPeek),
            },
        ),
        "Drayton" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![
                    Slot {
                        filter: CardFilter::AnyPokemon,
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: Some(7),
                    },
                    Slot {
                        filter: CardFilter::AnyTrainer,
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: Some(7),
                    },
                ],
                then: None,
            },
        ),
        "Harlequin" => (
            free,
            TrainerEffect::BothShuffleHandThenCoinFlipDraw {
                you_heads: 5,
                opponent_heads: 3,
                you_tails: 3,
                opponent_tails: 5,
            },
        ),
        // The optional pre-draw discard is a player line this build skips;
        // the mandatory "draw up to five" is the whole of the rest.
        "Naveen" => (free, TrainerEffect::DrawUpToHandSize(5)),
        "Iris's Fighting Spirit" => (
            Some(Requirement::DiscardOtherCardsFromHand(1)),
            TrainerEffect::DrawUpToHandSize(6),
        ),
        "Fennel" => (
            free,
            TrainerEffect::HealEachYours { amount: 40, of_type: None },
        ),
        "Clemont's Quick Wit" => (
            free,
            TrainerEffect::HealEachYours {
                amount: 60,
                of_type: Some(Type::Lightning),
            },
        ),
        "Tool Scrapper" => (free, TrainerEffect::MayDiscardUpToTwoToolsAnywhere),
        "Briar" => (
            Some(Requirement::OpponentPrizesExactly(2)),
            TrainerEffect::GrantsBonusPrizeIfOwnTeraAttackerKnocksOutThisTurn,
        ),
        "Enhanced Hammer" => (free, TrainerEffect::DiscardOpponentSpecialEnergy),
        "Glass Trumpet" => (
            Some(Requirement::OwnTeraPokemonInPlay),
            TrainerEffect::Decide {
                from: Zone::Discard,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Attach(TargetFilter::BenchedOfType(Type::Colorless)),
                    limit: 2,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Nighttime Mine" => (free, TrainerEffect::TeraAttacksCostMore),
        "Area Zero Underdepths" => (free, TrainerEffect::TeraPokemonRaisesBenchLimit),
        "Battle Cage" => (free, TrainerEffect::PreventsDamageCountersOnBench),
        "Team Rocket's Watchtower" => (free, TrainerEffect::AbilitiesDisabled),
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
                from: Zone::Deck,
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
        "Precious Trolley" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::PokemonOfStage(Stage::Basic),
                    to: Destination::Bench,
                    limit: 5,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Energy Search Pro" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::BasicEnergy,
                    to: Destination::Zone(Zone::Hand),
                    limit: 9,
                    excludes_type_of_previous: true,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Mega Signal" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::MegaPokemon,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "TM Machine" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::ToolNameContains("Technical Machine"),
                    to: Destination::Zone(Zone::Hand),
                    limit: 3,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Buddy-Buddy Poffin" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
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
                from: Zone::Deck,
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
                    to: Destination::Zone(Zone::Deck),
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
                from: Zone::Deck,
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
        "Tera Orb" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::TeraPokemon,
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Hilda" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![
                    Slot {
                        filter: CardFilter::EvolutionPokemon,
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        // "an Energy card" — Basic or Special alike.
                        filter: CardFilter::AnyEnergy,
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                ],
                then: None,
            },
        ),
        "Colress's Tenacity" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![
                    Slot {
                        filter: CardFilter::TrainerOfKind(TrainerKind::Stadium),
                        to: Destination::Zone(Zone::Hand),
                        limit: 1,
                        excludes_type_of_previous: false,
                        peek: None,
                    },
                    Slot {
                        filter: CardFilter::AnyEnergy,
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
                from: Zone::Deck,
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
                from: Zone::Deck,
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
                from: Zone::Deck,
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
                from: Zone::Deck,
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
                from: Zone::Deck,
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
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::AnyCard,
                    to: Destination::TopOfDeckInOrder,
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
        "Bianca's Devotion" => (free, TrainerEffect::HealFullyIfRemainingHpAtMost(30)),
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
        "Premium Power Pro" => (
            free,
            TrainerEffect::BonusDamageThisTurn(30, TurnBonusTarget::AttackerIsType(Type::Fighting)),
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
                from: Zone::Deck,
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
                from: Zone::Deck,
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
        "Dusk Ball" => (free, TrainerEffect::LookAtBottomOfDeck { count: 7 }),
        "Prime Catcher" => (free, TrainerEffect::SwitchOpponentActiveThenOwn),
        "Pokémon Catcher" => (
            free,
            TrainerEffect::CoinFlipThen(Box::new(TrainerEffect::SwitchOpponentActive)),
        ),
        "Fighting Gong" => (
            free,
            TrainerEffect::Decide {
                from: Zone::Deck,
                slots: vec![Slot {
                    filter: CardFilter::BasicPokemonOfTypeOrBasicEnergyOfType(Type::Fighting),
                    to: Destination::Zone(Zone::Hand),
                    limit: 1,
                    excludes_type_of_previous: false,
                    peek: None,
                }],
                then: None,
            },
        ),
        "Strange Timepiece" => (free, TrainerEffect::DevolveChosen),
        "Air Balloon" => (free, TrainerEffect::ReducesRetreatCost(2)),
        "Hero's Cape" => (free, TrainerEffect::IncreasesHp(100)),
        "Brave Bangle" => (free, TrainerEffect::BonusDamageWithoutRuleBoxVsEx(30)),
        "Binding Mochi" => (free, TrainerEffect::BonusDamageIfPoisonedVsActive(40)),
        "Lillie's Pearl" => (free, TrainerEffect::FewerPrizeIfLilliesKnockedOutByAttack),
        "Punk Helmet" => (free, TrainerEffect::DamagesAttackerWhenDefenderIsHit(40)),
        "Lucky Helmet" => (free, TrainerEffect::DrawsWhenDefenderIsHit(2)),
        "Handheld Fan" => (free, TrainerEffect::MovesEnergyFromAttackerToTheirBench),
        "Powerglass" => (free, TrainerEffect::MayAttachBasicEnergyFromDiscardAtTurnEnd),
        "Gravity Mountain" => (free, TrainerEffect::ReducesHpForStage(Stage::Stage2, 30)),
        "N's Castle" => (free, TrainerEffect::RemovesRetreatCostForNamePrefix("N's")),
        "Team Rocket's Ariana" => (
            free,
            TrainerEffect::DrawUpToHandSizeOrMoreIfAllOwnNamePrefix {
                base: 5,
                bonus: 8,
                prefix: "Team Rocket's",
            },
        ),
        "Team Rocket's Giovanni" => (
            Some(Requirement::ActiveNamePrefix("Team Rocket's")),
            TrainerEffect::SwitchOwnNamePrefixThenOpponent("Team Rocket's"),
        ),
        "Salvatore" => (free, TrainerEffect::SearchDeckToEvolveNoAbility),
        "Lisia's Appeal" => (free, TrainerEffect::SwitchOpponentActiveBasicThenConfuse),
        "Scoop Up Cyclone" => (free, TrainerEffect::ReturnChosenToHand),
        "Team Rocket's Venture Bomb" => (
            free,
            TrainerEffect::CoinFlipDamageCountersOnChosenOpponentElseOwnActive(20),
        ),
        "Acerola's Mischief" => (
            Some(Requirement::OpponentPrizesAtMost(2)),
            TrainerEffect::ProtectChosenFromExNextTurn,
        ),
        "Academy at Night" => (free, TrainerEffect::MayPutHandCardOnTopOfDeck),
        "Team Rocket's Factory" => (
            free,
            TrainerEffect::MayDrawTwoIfPlayedTeamRocketSupporter,
        ),
        "Lumiose City" => (free, TrainerEffect::MaySearchBasicToBenchThenMaybeEndTurn),
        "Prism Tower" => (free, TrainerEffect::StadiumMayDiscardTwoToDrawOne),
        "Community Center" => (
            free,
            TrainerEffect::StadiumMayHealAllIfPlayedSupporter(10),
        ),
        "Jamming Tower" => (free, TrainerEffect::ToolsHaveNoEffect),
        "Risky Ruins" => (free, TrainerEffect::DamagesNonDarknessBasicBenched(20)),
        "Forest of Vitality" => (free, TrainerEffect::GrassCanEvolveTheTurnItIsPlayed),
        "Festival Grounds" => (
            free,
            TrainerEffect::EnergizedPokemonImmuneToSpecialConditions,
        ),
        "Transformation Tome" => (
            Some(Requirement::SecondCopyOfThisInHand),
            TrainerEffect::SwapBasicWithDiscard,
        ),
        "Secret Box" => (
            Some(Requirement::DiscardOtherCardsFromHand(3)),
            TrainerEffect::Decide {
                from: Zone::Deck,
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
                from: Zone::Deck,
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
                    to: Destination::Zone(Zone::Deck),
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

/// What a print carries beyond its species name, per
/// [`crate::card::Pokemon::markers`].
///
/// `ex` and `Mega` are read from the name — TCGdex's `suffix` field
/// cannot be trusted for this: it is absent on 21 ex cards, `Mega
/// Charizard X ex` among them, and it uses both `ex` and `EX`. The
/// name is exact — every card carrying a suffix also ends in ` ex`,
/// and 21 more do. `Tera` is read from `print_id` against
/// `TERA_PRINT_IDS` instead: the artifact carries no field for it at
/// all, and two prints can share an identical name where only one is
/// Tera (`Hydreigon ex`'s Surging Sparks print is Tera; its
/// Black Bolt/White Flare print is not), so the name alone can never
/// answer this one. `Ancient` and `Future` read the same way, against
/// `ANCIENT_PRINT_IDS`/`FUTURE_PRINT_IDS` — `Koraidon ex` prints the
/// same within-name split Tera does: only its Temporal Forces print
/// (`sv05-120`) is Ancient, not its me02.5 or SVP prints.
pub fn markers_for(print_id: &str, name: &str) -> Vec<Marker> {
    let mut markers = Vec::new();
    if name.to_lowercase().ends_with(" ex") {
        markers.push(Marker::Ex);
        if name.starts_with("Mega ") {
            markers.push(Marker::Mega);
        }
    }
    if TERA_PRINT_IDS.contains(&print_id) {
        markers.push(Marker::Tera);
    }
    if ANCIENT_PRINT_IDS.contains(&print_id) {
        markers.push(Marker::Ancient);
    }
    if FUTURE_PRINT_IDS.contains(&print_id) {
        markers.push(Marker::Future);
    }
    markers
}

/// What a knockout of a Pokémon carrying these markers is worth.
pub fn prizes_for(markers: &[Marker]) -> u32 {
    if markers.contains(&Marker::Mega) {
        3
    } else if markers.contains(&Marker::Ex) {
        2
    } else {
        1
    }
}

/// Every Tera print in the artifact, by its own `id` — cross-checked
/// by hand against pkmncards.com's `is:tera` listing (150 cards) and
/// this artifact's own H/I/J-regulation pool: 104 of the 150 are
/// present here, the rest predate this artifact's regulation floor.
/// Pokémon TCG stopped printing new Tera cards after the sets already
/// represented below — this list is closed and permanent, not one to
/// extend as later sets import. A name is not enough to identify one:
/// `Hydreigon ex`, `Pikachu ex`, `Terapagos ex`, `Lapras ex`,
/// `Greninja ex`, `Koraidon ex`, and `Miraidon ex` each print both a
/// Tera version (listed here) and a plain one (not listed) under the
/// exact same name.
const TERA_PRINT_IDS: &[&str] = &[
    "me02.5-038",
    "me02.5-057",
    "me02.5-073",
    "me02.5-121",
    "me02.5-160",
    "me02.5-179",
    "me02.5-277",
    "sv05-060",
    "sv05-108",
    "sv05-190",
    "sv05-194",
    "sv06-025",
    "sv06-029",
    "sv06-040",
    "sv06-064",
    "sv06-106",
    "sv06-112",
    "sv06-130",
    "sv06-190",
    "sv06-191",
    "sv06-192",
    "sv06-194",
    "sv06-198",
    "sv06-199",
    "sv06-200",
    "sv06-211",
    "sv06-212",
    "sv06-213",
    "sv06-214",
    "sv06-215",
    "sv06-221",
    "sv06.5-015",
    "sv06.5-081",
    "sv07-028",
    "sv07-032",
    "sv07-051",
    "sv07-128",
    "sv07-157",
    "sv07-158",
    "sv07-159",
    "sv07-168",
    "sv07-170",
    "sv07-173",
    "sv08-036",
    "sv08-057",
    "sv08-086",
    "sv08-091",
    "sv08-106",
    "sv08-119",
    "sv08-133",
    "sv08-142",
    "sv08-159",
    "sv08-219",
    "sv08-221",
    "sv08-222",
    "sv08-223",
    "sv08-225",
    "sv08-226",
    "sv08-228",
    "sv08-238",
    "sv08-240",
    "sv08-242",
    "sv08-247",
    "sv08-248",
    "sv08.5-006",
    "sv08.5-012",
    "sv08.5-014",
    "sv08.5-017",
    "sv08.5-023",
    "sv08.5-026",
    "sv08.5-027",
    "sv08.5-030",
    "sv08.5-034",
    "sv08.5-041",
    "sv08.5-058",
    "sv08.5-060",
    "sv08.5-073",
    "sv08.5-075",
    "sv08.5-144",
    "sv08.5-145",
    "sv08.5-146",
    "sv08.5-147",
    "sv08.5-148",
    "sv08.5-149",
    "sv08.5-150",
    "sv08.5-152",
    "sv08.5-153",
    "sv08.5-155",
    "sv08.5-156",
    "sv08.5-160",
    "sv08.5-161",
    "sv08.5-165",
    "sv08.5-167",
    "sv08.5-169",
    "sv08.5-177",
    "sv08.5-179",
    "sv08.5-180",
    "svp-163",
    "svp-164",
    "svp-165",
    "svp-166",
    "svp-174",
    "svp-175",
    "svp-176",
];

/// Every print carrying the Paradox Ancient trait, among the six
/// species names the sample decks actually use (`Brute Bonnet`,
/// `Flutter Mane`, `Koraidon ex`, `Raging Bolt ex`) — cross-checked
/// by hand against pkmncards.com's `is:ancient` listing, print by
/// print, against every print of each of those names this artifact
/// carries. Unlike `TERA_PRINT_IDS`, **this list is not known to be
/// closed** — nobody has confirmed Ancient stopped being printed —
/// and it is deliberately narrow: a Paradox name that isn't one of
/// the four above (there are others, `Great Tusk` and `Walking Wake`
/// among them) has simply never been checked, not confirmed absent.
/// Extend it the same way if a future card needs a name outside this
/// four to carry `Marker::Ancient`.
const ANCIENT_PRINT_IDS: &[&str] = &[
    "sv05-078", // Flutter Mane, Temporal Forces
    "sv05-120", // Koraidon ex, Temporal Forces
    "sv05-123", // Raging Bolt ex, Temporal Forces
    "sv05-196", // Raging Bolt ex, Temporal Forces
    "sv05-208", // Raging Bolt ex, Temporal Forces
    "sv05-218", // Raging Bolt ex, Temporal Forces
    "sv06-118", // Brute Bonnet, Twilight Masquerade
    "sv08-096", // Flutter Mane, Surging Sparks
    "sv08.5-043", // Flutter Mane, Prismatic Evolutions
    "sv08.5-166", // Raging Bolt ex, Prismatic Evolutions
    "svp-097",  // Flutter Mane, Scarlet & Violet Promos
    "svp-145",  // Raging Bolt ex, Scarlet & Violet Promos
];

/// Every print carrying the Paradox Future trait, among the two
/// species names the sample decks actually use (`Iron Crown ex`,
/// `Iron Leaves ex`) — the same hand cross-check against
/// pkmncards.com's `is:future` listing, and the same open-list,
/// deliberately-narrow caveat `ANCIENT_PRINT_IDS` carries: a Paradox
/// name outside these two (`Miraidon ex` and the rest of the `Iron`
/// line among them) has never been checked here.
const FUTURE_PRINT_IDS: &[&str] = &[
    "sv05-025", // Iron Leaves ex, Temporal Forces
    "sv05-081", // Iron Crown ex, Temporal Forces
    "sv05-186", // Iron Leaves ex, Temporal Forces
    "sv05-191", // Iron Crown ex, Temporal Forces
    "sv05-203", // Iron Leaves ex, Temporal Forces
    "sv05-206", // Iron Crown ex, Temporal Forces
    "sv05-213", // Iron Leaves ex, Temporal Forces
    "sv05-216", // Iron Crown ex, Temporal Forces
    "sv08.5-158", // Iron Crown ex, Prismatic Evolutions
    "sv08.5-176", // Iron Leaves ex, Prismatic Evolutions
    "svp-128",  // Iron Leaves ex, Scarlet & Violet Promos
    "svp-146",  // Iron Crown ex, Scarlet & Violet Promos
];

/// Whether `print_id` carries ACE SPEC — a classification of its own,
/// not a `Marker`, since `Marker` lives on `Pokemon` alone and ACE
/// SPEC spans two `CardDef` variants instead: `Trainer` (an Item,
/// Tool, or Stadium — never a Supporter) and `Energy` (a Special
/// Energy). Read at exactly two sites — Rule 3's one-per-deck limit
/// in `decklist::check`, and `Genesect`'s `ACE Nullifier` — so this
/// is a lookup function against `ACE_SPEC_PRINT_IDS`, not a stored
/// field threaded through both structs' own construction sites.
/// Cross-checked by hand against pkmncards.com's `is:ace-spec`
/// listing, scoped to the five names the sample decks actually use
/// (`Prime Catcher`, `Unfair Stamp`, `Enriching Energy`, `Hero's
/// Cape`, `Secret Box`); open, not closed, the same caveat
/// `ANCIENT_PRINT_IDS` and `FUTURE_PRINT_IDS` carry — ACE SPEC is
/// still being printed, and a name outside this five has simply
/// never been checked here.
pub fn is_ace_spec(print_id: &str) -> bool {
    ACE_SPEC_PRINT_IDS.contains(&print_id)
}

const ACE_SPEC_PRINT_IDS: &[&str] = &[
    "sv05-152",   // Hero's Cape, Temporal Forces
    "sv05-157",   // Prime Catcher, Temporal Forces
    "sv06-163",   // Secret Box, Twilight Masquerade
    "sv06-165",   // Unfair Stamp, Twilight Masquerade
    "sv08-191",   // Enriching Energy, Surging Sparks
    "sv08.5-119", // Prime Catcher, Prismatic Evolutions
];

/// Whether every attack this raw print carries would read on its own,
/// regardless of whether its Ability (if any) also reads. Milestone 11
/// and Milestone 8 track separate progress on the same species; the
/// README's own progress table reads this to tell them apart, since
/// `playable` alone answers only "does the whole card run."
pub fn attacks_read(raw: &Value) -> bool {
    let name = raw["name"].as_str().unwrap_or("?");
    raw["attacks"]
        .as_array()
        .is_some_and(|attacks| attacks.iter().all(|attack| read_attack(name, attack).is_ok()))
}

/// The Ability half of the same split `attacks_read` draws.
pub fn ability_reads(raw: &Value) -> bool {
    let name = raw["name"].as_str().unwrap_or("?");
    read_ability(name, raw["abilities"].as_array()).is_ok()
}

fn read_attack(pokemon_name: &str, attack: &Value) -> Result<Attack, Refusal> {
    let attack_name = attack["name"].as_str().unwrap_or("?");
    let effect = if attack["effect"].as_str().is_some_and(|e| !e.is_empty()) {
        match known_attack(pokemon_name, attack_name) {
            Some(effect) => Some(effect),
            None => return Err(Refusal::AttackHasText),
        }
    } else {
        None
    };
    let base_damage = match &attack["damage"] {
        Value::Number(damage) => damage.as_u64().ok_or(Refusal::DamageIsNotANumber)? as u32,
        Value::Null => 0,
        // "40×" (a per-count multiplier, `AttackEffect::DamagePerCount`)
        // or "10+" (a conditional bonus on top of a printed base, such as
        // `AttackEffect::CoinFlipBonusDamage`): either way the digits are
        // read only to confirm the print agrees with what `known_attack`
        // already matched, not as the source of truth for what the
        // number means.
        Value::String(s) if effect.is_some() && (s.ends_with('×') || s.ends_with('+')) => s
            .trim_end_matches(['×', '+'])
            .parse::<u32>()
            .map_err(|_| Refusal::DamageIsNotANumber)?,
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
        name: leak(attack_name),
        cost,
        base_damage,
        inflicts: None,
        effect,
    })
}

/// A Pokémon's own Ability, if it has one — refused unless
/// `known_ability` already matches it, the same "checked before the
/// unconditional refusal" shape `read_attack` already takes.
fn read_ability(pokemon_name: &str, abilities: Option<&Vec<Value>>) -> Result<Option<Ability>, Refusal> {
    let Some(ability_json) = abilities.and_then(|a| a.first()) else {
        return Ok(None);
    };
    let ability_name = ability_json["name"].as_str().unwrap_or("?");
    match known_ability(pokemon_name, ability_name) {
        Some(effect) => Ok(Some(Ability { name: leak(ability_name), effect })),
        None => Err(Refusal::HasAnAbility),
    }
}

/// An Ability's own effect, matched by the Pokémon's printed name and
/// the Ability's own name — the same shape `known_attack` matches an
/// attack by, mirrored here since an Ability dispatches through its
/// own machinery (`Action::UseAbility`), not `attack` or
/// `resolve_trainer`. Empty until this milestone's first ticket
/// admits a real card.
/// A Special Energy's type and effect, matched by its own print name —
/// the same discipline `known_attack` and `known_ability` already
/// hold. `data/cards.json` carries no field naming what type an Energy
/// provides, so it is read here, from real-world knowledge of the
/// print, the same way `known_attack`'s effects are.
fn known_energy(name: &str) -> Option<(Type, crate::card::EnergyEffect)> {
    Some(match name {
        "Growing Grass Energy" => (Type::Grass, crate::card::EnergyEffect::IncreasesCarrierHp(20)),
        "Enriching Energy" => {
            (Type::Colorless, crate::card::EnergyEffect::DrawCardsOnAttachFromHand(4))
        }
        "Telepathic Psychic Energy" => (
            Type::Psychic,
            crate::card::EnergyEffect::WhenAttachedToTypeSearchesBasicPokemonOfTypeToBench(
                Type::Psychic,
                Type::Psychic,
                2,
            ),
        ),
        "Spiky Energy" => {
            (Type::Colorless, crate::card::EnergyEffect::CountersAttackerOnDamageTakenWhileActive(20))
        }
        "Mist Energy" => {
            (Type::Colorless, crate::card::EnergyEffect::PreventsAttackEffectsOnCarrier)
        }
        "Boomerang Energy" => {
            (Type::Colorless, crate::card::EnergyEffect::ReattachesAfterOwnDiscardByAttackEffect)
        }
        "Prism Energy" => {
            (Type::Colorless, crate::card::EnergyEffect::ProvidesAnyTypeIfAttachedToBasic)
        }
        "Rocky Fighting Energy" => {
            (Type::Fighting, crate::card::EnergyEffect::PreventsAttackEffectsOnCarrier)
        }
        "Magnetic Metal Energy" => {
            (Type::Metal, crate::card::EnergyEffect::CarrierHasNoRetreatCost)
        }
        "Bubbly Water Energy" => {
            (Type::Water, crate::card::EnergyEffect::CarrierImmuneToSpecialConditions)
        }
        "Nitro Fire Energy" => {
            (Type::Fire, crate::card::EnergyEffect::ReattachesAfterOwnDiscardByAttackEffect)
        }
        "Voltaic Lightning Energy" => {
            (Type::Lightning, crate::card::EnergyEffect::CarrierAttacksHitOpponentActiveHarder(20))
        }
        "Shadowy Darkness Energy" => {
            (Type::Darkness, crate::card::EnergyEffect::PreventsBenchDamageWhileCarrierTypeMatches)
        }
        _ => return None,
    })
}

fn known_ability(pokemon_name: &str, ability_name: &str) -> Option<AbilityEffect> {
    Some(match (pokemon_name, ability_name) {
        ("Mega Kangaskhan ex", "Run Errand") => {
            AbilityEffect::OncePerTurnWhileActiveMayDrawCards(2)
        }
        ("Latias ex", "Skyliner") => AbilityEffect::PassiveOwnBasicPokemonHaveNoRetreatCost,
        ("Crustle", "Mysterious Rock Inn") => AbilityEffect::PassiveImmuneToDamageFromOpponentEx,
        ("Patrat", "Watchful Eye") => AbilityEffect::PassiveBlocksDamageCounterMovement,
        ("Psyduck", "Damp") => AbilityEffect::PassiveDisablesSelfKnockOutAbilities,
        ("Lillie's Clefairy ex", "Fairy Zone") => {
            AbilityEffect::PassiveSetsOpponentTypeWeaknessTo(Type::Dragon, Type::Psychic)
        }
        ("Shaymin", "Flower Curtain") => AbilityEffect::PassivePreventsAttackDamageToNonRuleBoxBench,
        ("Rabsca", "Spherical Shield") => AbilityEffect::PassivePreventsAttackEffectsOnBench,
        ("Iron Crown ex", "Cobalt Command") => {
            AbilityEffect::PassiveFutureAttacksDoBonusDamageToActiveExceptNamed(20)
        }
        ("Annihilape", "Lose Cool") => AbilityEffect::PassiveBonusDamageToActiveIfSelfDamaged(120),
        ("Hoothoot", "Insomnia") => AbilityEffect::PassiveImmuneToAsleep,
        ("Hydrapple ex", "Ripening Charge") => {
            AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeFromHandToChosenThenHeal(
                Type::Grass,
                30,
            )
        }
        ("Bloodmoon Ursaluna ex", "Seasoned Skill") => {
            AbilityEffect::PassiveNamedAttackCostsLessPerOpponentPrizeTaken("Blood Moon")
        }
        ("Flutter Mane", "Midnight Fluttering") => {
            AbilityEffect::PassiveDisablesOpponentActiveAbilityExceptSelf
        }
        ("Tatsugiri", "Attract Customers") => {
            AbilityEffect::OncePerTurnWhileActiveMayLookAtTopCardsTakeASupporter(6)
        }
        ("Drakloak", "Recon Directive") => {
            AbilityEffect::OncePerTurnMayLookAtTopCardsTakeOneRestToBottom(2)
        }
        ("Metang", "Metal Maker") => {
            AbilityEffect::OncePerTurnMayLookAtTopCardsAttachFoundBasicEnergyOfType(4, Type::Metal)
        }
        ("Munkidori", "Adrena-Brain") => {
            AbilityEffect::OncePerTurnIfEnergyOfTypeAttachedMayMoveDamageCountersToOpponent(
                Type::Darkness,
                3,
            )
        }
        ("Meowth ex", "Last-Ditch Catch") => AbilityEffect::WhenBenchedFromHandMaySearchSupporter,
        ("Kadabra", "Psychic Draw") => AbilityEffect::WhenEvolvedFromHandMayDrawCards(2),
        ("Noctowl", "Jewel Seeker") => {
            AbilityEffect::WhenEvolvedFromHandMaySearchTrainersIfOwnTeraInPlay(2)
        }
        ("Alakazam", "Psychic Draw") => AbilityEffect::WhenEvolvedFromHandMayDrawCards(3),
        ("Fezandipiti ex", "Flip the Script") => {
            AbilityEffect::OncePerTurnIfKnockedOutLastTurnMayDrawCards(3)
        }
        ("Teal Mask Ogerpon ex", "Teal Dance") => {
            AbilityEffect::OncePerTurnMayAttachBasicEnergyOfTypeThenDraw(Type::Grass)
        }
        ("Dudunsparce", "Run Away Draw") => {
            AbilityEffect::OncePerTurnMayDrawThenShuffleSelfIntoDeck(3)
        }
        ("Abra", "Teleporter") => AbilityEffect::OncePerTurnWhileActiveMayShuffleSelfIntoDeck,
        ("Dusclops", "Cursed Blast") => AbilityEffect::OncePerTurnMayDamageOpponentThenKnockOutSelf(5),
        ("Dusknoir", "Cursed Blast") => AbilityEffect::OncePerTurnMayDamageOpponentThenKnockOutSelf(13),
        ("Genesect ex", "Metallic Signal") => {
            AbilityEffect::OncePerTurnMaySearchEvolutionPokemonOfType(Type::Metal, 2)
        }
        ("Genesect", "ACE Nullifier") => {
            AbilityEffect::PassiveBlocksOpponentAceSpecPlaysIfSelfHasTool
        }
        ("N's Zoroark ex", "Trade") => {
            AbilityEffect::OncePerTurnMayDiscardFromHandThenDrawCards(2)
        }
        ("Meganium", "Wild Growth") => AbilityEffect::PassiveDoublesBasicGrassEnergyForCost,
        ("Pecharunt", "Toxic Subjugation") => {
            AbilityEffect::PassiveBonusCheckupDamageToOpponentsPoisonedWhileActive(50)
        }
        ("Goldeen", "Festival Lead") => AbilityEffect::PassiveFestivalLead,
        ("Seaking", "Festival Lead") => AbilityEffect::PassiveFestivalLead,
        ("Dipplin", "Festival Lead") => AbilityEffect::PassiveFestivalLead,
        ("Thwackey", "Boom Boom Groove") => {
            AbilityEffect::OncePerTurnMaySearchAnyCardIfActiveHasNamedAbility("Festival Lead")
        }
        ("Annihilape", "Durable Body") => {
            AbilityEffect::PassiveCoinFlipPreventsAttackKnockOutAtTenHp
        }
        ("Kyurem", "Plasma Bane") => {
            AbilityEffect::PassiveNamedAttackCostsJustColorlessIfOpponentDiscardNameContains(
                "Trifrost", "Colress",
            )
        }
        ("Blaziken ex", "Seething Spirit") => {
            AbilityEffect::OncePerTurnMayAttachBasicEnergyFromDiscardToChosen
        }
        ("Chien-Pao", "Snow Sink") => AbilityEffect::WhenBenchedFromHandMayDiscardStadium,
        ("Iron Leaves ex", "Rapid Vernier") => {
            AbilityEffect::WhenBenchedFromHandMaySwitchThenMoveAnyEnergy
        }
        ("Toxtricity", "Sinister Surge") => {
            AbilityEffect::OncePerTurnMaySearchBasicEnergyOfTypeAttachToBenchedThenDamage(
                Type::Darkness,
                20,
            )
        }
        ("Fan Rotom", "Fan Call") => {
            AbilityEffect::OnceDuringFirstTurnMaySearchPokemonOfTypeWithHpAtMost(
                Type::Colorless,
                100,
                3,
            )
        }
        ("Pecharunt ex", "Subjugating Chains") => {
            AbilityEffect::OncePerTurnMaySwitchBenchedOfTypeExcludingNamedThenPoison(
                Type::Darkness,
                "Pecharunt ex",
            )
        }
        _ => return None,
    })
}

/// An attack's own effect, matched by the Pokémon's printed name and the
/// attack's own name — the same shape `known_trainer` matches a Trainer
/// by, mirrored here rather than shared, since a Trainer and an attack
/// dispatch through different machinery (`resolve_trainer` vs `attack`).
/// Empty until this milestone's first ticket admits a real card.
fn known_attack(pokemon_name: &str, attack_name: &str) -> Option<AttackEffect> {
    Some(match (pokemon_name, attack_name) {
        ("Carvanha", "Reckless Charge") => AttackEffect::Recoil(10),
        ("Rellor", "Slight Intrusion") => AttackEffect::Recoil(10),
        ("Tapu Bulu", "Wood Hammer") => AttackEffect::Recoil(30),
        ("Paldean Tauros", "Double-Edge") => AttackEffect::Recoil(20),
        ("Paldean Tauros", "Raging Charge") => {
            AttackEffect::DamagePerCount(Count::OwnDamagedWithNamePrefix("Tauros"), 40)
        }
        ("N's Reshiram", "Powerful Rage") => {
            AttackEffect::DamagePerCount(Count::OwnDamageCounters, 20)
        }
        ("N's Darmanitan", "Back Draft") => {
            AttackEffect::DamagePerCount(Count::OpponentBasicEnergyInDiscard, 30)
        }
        ("Dudunsparce ex", "Tenacious Tail") => {
            AttackEffect::DamagePerCount(Count::OpponentPokemonExInPlay, 60)
        }
        ("Passimian", "Coordinated Throwing") => {
            AttackEffect::DamagePerCount(Count::OwnBasicPokemonInPlay, 20)
        }
        ("N's Zekrom", "Shred") => AttackEffect::IgnoresDefendersEffects,
        ("Mega Lopunny ex", "Spiky Hopper") => AttackEffect::IgnoresDefendersEffects,
        ("Dudunsparce ex", "Destructive Drill") => AttackEffect::IgnoresDefendersEffects,
        ("Brute Bonnet", "Poison Spray") => {
            AttackEffect::InflictsCondition(crate::card::Condition::Poisoned)
        }
        ("Zeraora", "Shocking Knuckle") => {
            AttackEffect::CoinFlipInflicts(crate::card::Condition::Paralyzed)
        }
        ("Zeraora", "Strong Volt") => AttackEffect::DiscardsFixedOwnEnergyChosen(1),
        ("Metagross", "Luster Blast") => AttackEffect::DiscardsFixedOwnEnergyChosen(2),
        ("Metagross", "Bounce Back") => AttackEffect::SwitchOpponentActive,
        ("Metagross", "Metallic Hammer") => {
            AttackEffect::MayDiscardUpToOwnEnergyOfTypeForFlatBonusDamage(Type::Metal, 3, 150)
        }
        ("Raging Bolt ex", "Burst Roar") => AttackEffect::DiscardsHandThenDrawsCards(6),
        ("Raging Bolt ex", "Bellowing Thunder") => {
            AttackEffect::MayDiscardAnyOwnBasicEnergyForDamagePerCard(70)
        }
        ("Dedenne", "Thunder Shock") => {
            AttackEffect::CoinFlipInflicts(crate::card::Condition::Paralyzed)
        }
        ("Applin", "Tumbling Attack") => AttackEffect::CoinFlipBonusDamage(20),
        ("Yveltal", "Clutch") => AttackEffect::DefenderCannotRetreatNextTurn,
        ("Wellspring Mask Ogerpon ex", "Sob") => AttackEffect::DefenderCannotRetreatNextTurn,
        ("N's Zekrom", "Rampaging Thunder") => AttackEffect::AttackerCannotAttackNextTurn,
        ("Dragapult ex", "Phantom Dive") => {
            AttackEffect::DamageCountersToOpponentBenchAnyWay(6)
        }
        ("N's Darmanitan", "Flamebody Cannon") => {
            AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenched(90)
        }
        ("Abra", "Teleportation Attack") => AttackEffect::SwitchOwnActive,
        ("Drilbur", "Call for Family") => AttackEffect::SearchDeckForBasicPokemonToBench(2),
        ("Toxel", "Call for Family") => AttackEffect::SearchDeckForBasicPokemonToBench(2),
        ("Hoothoot", "Silent Wing") => AttackEffect::RevealOpponentsHand,
        ("Mega Sharpedo ex", "Greedy Fang") => AttackEffect::DrawCards(2),
        ("Mega Sharpedo ex", "Hungry Jaws") => AttackEffect::BonusDamageIfOwnDamaged(150),
        ("Dwebble", "Flail") => AttackEffect::DamagePerCount(Count::OwnDamageCounters, 10),
        ("Slowking", "Wash the Slate Clean") => {
            AttackEffect::MayReturnOpponentsActiveEnergyToHand(2)
        }
        ("Dedenne", "Electromagnetic Sonar") => AttackEffect::TakeTrainerFromDiscard,
        ("Dwebble", "Ascension") => AttackEffect::SearchDeckToEvolveSelf,
        ("Beldum", "Iron Tackle") => AttackEffect::Recoil(10),
        ("Dunsparce", "Trading Places") => AttackEffect::SwitchOwnActive,
        ("Moltres", "Fighting Wings") => AttackEffect::BonusDamageIfDefenderIsEx(90),
        ("Slowpoke", "Dangle Tail") => AttackEffect::TakePokemonFromDiscard,
        ("Duskull", "Come and Get You") => AttackEffect::SearchDiscardForNamedToBench("Duskull", 3),
        ("Torchic", "Collect") => AttackEffect::DrawCards(1),
        ("Rabsca", "Triple Draw") => AttackEffect::DrawCards(3),
        ("Rabsca", "Psychic") => AttackEffect::DamagePerCount(Count::DefenderEnergyAttachedCount, 30),
        ("Rabsca", "Counterturn") => AttackEffect::BonusDamageIfOwnDeckAtMost(3, 200),
        ("Iron Crown ex", "Twin Shotels") => AttackEffect::DamageTwoChosenOpponentPokemon(50),
        ("Koraidon ex", "Retribution Strike") => {
            AttackEffect::BonusDamagePerCount(Count::OwnDamageCounters, 10)
        }
        ("Koraidon ex", "Kaiser Tackle") => AttackEffect::Recoil(60),
        ("Koraidon ex", "Orichalcum Fang") => AttackEffect::BonusDamageIfOwnKnockedOutLastTurn(120),
        ("Koraidon ex", "Impact Blow") => AttackEffect::CannotUseThisAttackNextTurn,
        ("Koraidon ex", "Revenge Buster") => AttackEffect::BonusDamageIfOwnBenchDamaged(120),
        ("Annihilape", "Impact Blow") => AttackEffect::CannotUseThisAttackNextTurn,
        ("Celebi", "Collect") => AttackEffect::DrawCards(1),
        ("Buneary", "Run Around") => AttackEffect::SwitchOwnActive,
        ("Bayleef", "Push Down") => AttackEffect::SwitchOpponentActive,
        ("Chikorita", "Growl") => AttackEffect::DefenderDealsLessDamageNextTurn(20),
        ("Buneary", "Charm") => AttackEffect::DefenderDealsLessDamageNextTurn(20),
        ("Dunsparce", "Dig") => AttackEffect::CoinFlipSelfInvulnerableNextTurn,
        ("Elgyem", "Hide") => AttackEffect::CoinFlipSelfInvulnerableNextTurn,
        ("Budew", "Itchy Pollen") => AttackEffect::OpponentCannotPlayItemsNextTurn,
        ("Wellspring Mask Ogerpon ex", "Torrential Pump") => {
            AttackEffect::MayShuffleFixedEnergyThenDamageChosenBenched {
                count: 3,
                damage: 120,
            }
        }
        ("Paldean Tauros", "Spirited Tackle") => {
            AttackEffect::BonusDamageIfDefenderIsStage(Stage::Stage1, 90)
        }
        ("Combusken", "Double Kick") => {
            AttackEffect::DamagePerCoinFlipHeads { flips: 2, per_head: 40 }
        }
        ("Hoothoot", "Triple Stab") => {
            AttackEffect::DamagePerCoinFlipHeads { flips: 3, per_head: 10 }
        }
        ("Brute Bonnet", "Relentless Punches") => {
            AttackEffect::BonusDamagePerCount(Count::DefenderDamageCounters, 50)
        }
        ("Enamorus", "Love Resonance") => AttackEffect::BonusDamageIfSharedTypeInPlay(120),
        ("Stunfisk", "Muddy Bolt") => {
            AttackEffect::BonusDamageIfOwnEnergyOfTypeAttached(Type::Fighting, 20)
        }
        ("Bloodmoon Ursaluna ex", "Blood Moon") => AttackEffect::AttackerCannotAttackNextTurn,
        ("Hydrapple ex", "Syrup Storm") => {
            AttackEffect::BonusDamagePerCount(Count::OwnEnergyOfTypeAttachedAcrossSideCount(Type::Grass), 30)
        }
        ("Mega Excadrill ex", "Undermine") => AttackEffect::DiscardsTopOfOpponentsDeck(2),
        ("Mega Lopunny ex", "Gale Thrust") => {
            AttackEffect::BonusDamageIfSelfPromotedThisTurn(170)
        }
        ("Mega Absol ex", "Terminal Period") => {
            AttackEffect::KnocksOutDefenderIfExactDamageCounters(6)
        }
        ("Mega Absol ex", "Claw of Darkness") => AttackEffect::DiscardsChosenFromOpponentsHand,
        ("Celebi", "Traverse Time") => {
            AttackEffect::SearchDeckForUpToPokemonOfTypeOrStadiumToHand(Type::Grass, 3)
        }
        ("Smoochum", "Delightful Kiss") => {
            AttackEffect::SearchesBasicEnergyOfTypeAttachToChosenBenched(Type::Psychic, 2)
        }
        ("Cofagrigus", "Law of the Underworld") => AttackEffect::DamagesEveryPokemonWithAnAbility(60),
        ("Cofagrigus", "Extended Damagriiigus") => {
            AttackEffect::MovesAllDamageFromChosenOwnBenchedToChosenOpponentPokemon
        }
        ("Mega Skarmory ex", "Sonic Ripper") => {
            AttackEffect::ShufflesOwnEnergyThenDamagesChosenOpponentPokemonWeaknessIfActive(220)
        }
        ("Mega Excadrill ex", "Maximum Drilling") => {
            AttackEffect::BonusDamageIfExtraEnergyAttached(2, 130)
        }
        ("Stunfisk", "Paralyzing Crackle") => {
            AttackEffect::CoinFlipInflictsAndDiscardsDefenderEnergy(Condition::Paralyzed)
        }
        ("Cofagrigus", "Perplex") => AttackEffect::InflictsCondition(Condition::Confused),
        ("Flutter Mane", "Hex Hurl") => AttackEffect::DamageCountersToOpponentBenchAnyWay(2),
        ("Elgyem", "Slight Shift") => AttackEffect::MoveOpponentsEnergyBetweenTheirPokemon,
        ("Dusknoir", "Shadow Bind") => AttackEffect::DefenderCannotRetreatNextTurn,
        ("Genesect ex", "Protect Charge") => AttackEffect::SelfDamageReductionNextTurn(30),
        ("Blaziken ex", "Smolder-sault") => AttackEffect::AttackerCannotAttackNextTurn,
        ("Chien-Pao", "Strafe") => AttackEffect::SwitchOwnActive,
        ("Chien-Pao", "Rising Blade") => AttackEffect::BonusDamageIfDefenderIsEx(80),
        ("Chien-Pao", "Icicle Loop") => AttackEffect::MoveOwnAttachedEnergyToHand,
        ("Iron Leaves ex", "Prism Edge") => AttackEffect::AttackerCannotAttackNextTurn,
        ("Fan Rotom", "Assault Landing") => AttackEffect::FizzlesWithNoStadiumInPlay,
        ("Pecharunt ex", "Irritated Outburst") => {
            AttackEffect::DamagePerCount(Count::OpponentPrizesTakenCount, 60)
        }
        ("Pecharunt", "Poison Chain") => {
            AttackEffect::InflictsConditionAndDefenderCannotRetreatNextTurn(
                crate::card::Condition::Poisoned,
            )
        }
        ("Goldeen", "Whirlpool") => AttackEffect::CoinFlipDiscardsDefenderEnergy,
        ("Seaking", "Rapid Draw") => AttackEffect::DrawCards(2),
        ("Dipplin", "Do the Wave") => {
            AttackEffect::DamagePerCount(Count::OwnBenchedPokemonCount, 20)
        }
        ("Annihilape", "Tantrum") => {
            AttackEffect::InflictsConditionOnSelf(crate::card::Condition::Confused)
        }
        ("Annihilape", "Destined Fight") => AttackEffect::KnocksOutBothActivePokemon,
        ("Annihilape", "Ghostly Blow") => {
            AttackEffect::PlacesDamageCountersOnChosenOpponentBenched(50)
        }
        ("Mega Slowbro ex", "Shellnado Spin") => {
            AttackEffect::GrantsSelfCountersAttackerIfDamagedNextTurn(120)
        }
        ("Stunfisk", "Pouncing Trap") => {
            AttackEffect::DefenderCannotRetreatAndTakesMoreDamageNextTurn(100)
        }
        ("Chi-Yu", "Scorching Earth") => {
            AttackEffect::DiscardsOpponentsStadiumThenOpponentCannotPlayStadiumsNextTurn
        }
        ("Chi-Yu", "Allure") => AttackEffect::DrawCards(2),
        ("Chi-Yu", "Whirling Envy") => {
            AttackEffect::BonusDamageIfOwnDamageCountersAtLeastIgnoringWeakness(2, 90)
        }
        ("Chi-Yu", "Ground Melter") => {
            AttackEffect::BonusDamageIfStadiumInPlayThenDiscardsIt(60)
        }
        ("Slowking", "Seek Inspiration") => {
            AttackEffect::DiscardsTopOfDeckThenCopiesItsAttackIfNoRuleBox
        }
        ("N's Zoroark ex", "Night Joker") => {
            AttackEffect::CopiesChosenBenchedPokemonAttackByNamePrefix("N's ")
        }
        ("Kyurem", "Trifrost") => {
            AttackEffect::DiscardsOwnEnergyThenDamagesThreeChosenOpponentPokemon(110)
        }
        ("Genesect", "Bug's Cannon") => AttackEffect::DamagePerCountToChosenOpponentPokemon(
            Count::OwnEnergyOfTypeAttachedCount(Type::Grass),
            20,
        ),
        ("Seaking", "Hydro Jet") => AttackEffect::DamagePerCountToChosenOpponentPokemon(
            Count::OwnEnergyOfTypeAttachedCount(Type::Water),
            30,
        ),
        ("Alakazam", "Powerful Hand") => {
            AttackEffect::PlaceDamageCountersOnDefenderPerCount(Count::OwnHandSizeCount, 2)
        }
        ("Shaymin", "Pinpoint Dive") => AttackEffect::DamageChosenOpponentBenchedEx(60),
        ("Shaymin", "Send Flowers") => AttackEffect::SearchEnergyAttachToBenchedOfType(Type::Grass),
        ("Shaymin", "Reflect Energy") => AttackEffect::MoveOwnAttachedEnergyToChosenBenched,
        ("Latias ex", "Eon Blade") => AttackEffect::AttackerCannotAttackNextTurn,
        ("Metang", "Guard Press") => AttackEffect::SelfDamageReductionNextTurn(30),
        ("Patrat", "Procurement") => AttackEffect::SearchDeckForItemCardToHand,
        ("Noctowl", "Talon Hunt") => AttackEffect::SearchDeckForUpToCardsOfAnyKindToHand(2),
        ("Dipplin", "Energy Loop") => AttackEffect::MoveOwnAttachedEnergyToHand,
        ("Seaking", "Peck Off") => AttackEffect::DiscardsDefendersTools,
        ("Crustle", "Superb Scissors") => AttackEffect::IgnoresDefendersEffects,
        ("Munkidori", "Mind Bend") => {
            AttackEffect::InflictsCondition(crate::card::Condition::Confused)
        }
        ("Mega Kangaskhan ex", "Rapid-Fire Combo") => {
            AttackEffect::DamagePerCoinFlipUntilTails(50)
        }
        ("Meowth ex", "Tuck Tail") => AttackEffect::ReturnSelfAndAttachedToHand,
        ("Fezandipiti ex", "Cruel Arrow") => AttackEffect::DamageChosenOpponentPokemon(100),
        ("Teal Mask Ogerpon ex", "Myriad Leaf Shower") => {
            AttackEffect::DamagePerCount(Count::EnergyOnBothActivesCount, 30)
        }
        ("Zeraora", "Thunder Raid") => {
            AttackEffect::DiscardsOwnEnergyThenDamagesChosenBenchedEx(210)
        }
        ("Zeraora", "Combat Thunder") => {
            AttackEffect::DamagePerCount(Count::OpponentBenchedPokemonCount, 20)
        }
        ("Lillie's Clefairy ex", "Full Moon Rondo") => {
            AttackEffect::DamagePerCount(Count::BothBenchedPokemonCount, 20)
        }
        _ => return None,
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
