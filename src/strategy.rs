//! A Strategy: the decision a seat's own player makes each turn.
//!
//! [`legal_actions`](crate::action::legal_actions) is the engine's own menu;
//! a `Strategy` is what picks among it. One per seat, reading only that
//! seat's own masked [`PlayerView`] — never the opponent's hand, either
//! deck, or either Prize pile — plus the shared [`CardDb`], since a
//! card's own printed rules are never hidden, only which cards sit where.
//! See ADR 0095 for why this reads a view rather than the raw `GameState`.

use crate::action::Action;
use crate::card::{CardDb, TrainerEffect, TrainerKind, Zone};
use crate::rng::Rng;
use crate::view::PlayerView;

/// The decision a seat's own player makes each turn: which legal `Action`
/// to take, having seen only its own side of the board.
pub trait Strategy {
    fn choose(&mut self, view: &PlayerView, db: &CardDb, legal: &[Action]) -> Action;
}

/// A first pass at sequencing, expected to be revisited: attack whenever
/// legal; then accept a "may" Ability prompt that never costs anything to
/// take; otherwise take the highest-priority legal action kind, first
/// legal option within a tier, falling back to a uniform-random pick among
/// whatever remains once nothing above matches anything.
///
/// The full order: `Attack → AcceptPsychicDraw/AcceptJewelSeeker →
/// PlayBasic → UseAbility → Evolve → Stadium →` a Trainer ranked first by
/// what it does (draws a card, searches the deck, or neither) and second
/// by its own kind (Item before Supporter) `→ Tool → AttachEnergy →
/// Retreat → random`.
///
/// Every other `Accept*`/`Decline*` pair (attaching Energy, discarding,
/// switching) trades something away for its benefit, so choosing between
/// them needs real evaluation this first pass doesn't attempt yet — those
/// still fall through to the random tier. `Psychic Draw` and `Jewel
/// Seeker` (ADR 0072) are the two exceptions: drawing or searching for
/// free never has a downside, so there is nothing to weigh.
pub struct HeuristicStrategy {
    rng: Box<dyn Rng>,
}

impl HeuristicStrategy {
    pub fn new(rng: Box<dyn Rng>) -> HeuristicStrategy {
        HeuristicStrategy { rng }
    }
}

impl Strategy for HeuristicStrategy {
    fn choose(&mut self, view: &PlayerView, db: &CardDb, legal: &[Action]) -> Action {
        if let Some(attack) = legal.iter().find(|a| matches!(a, Action::Attack { .. })) {
            return *attack;
        }
        if let Some(free) = legal
            .iter()
            .find(|a| matches!(a, Action::AcceptPsychicDraw | Action::AcceptJewelSeeker))
        {
            return *free;
        }

        let tiers: [fn(&Action, &PlayerView, &CardDb) -> bool; 12] = [
            is_play_basic,
            is_use_ability,
            is_evolve,
            is_stadium,
            is_drawing_item,
            is_drawing_supporter,
            is_searching_item,
            is_searching_supporter,
            is_other_item,
            is_other_supporter,
            is_tool,
            is_attach_energy,
        ];
        for tier in tiers {
            if let Some(action) = legal.iter().find(|a| tier(a, view, db)) {
                return *action;
            }
        }
        if let Some(retreat) = legal.iter().find(|a| matches!(a, Action::Retreat { .. })) {
            return *retreat;
        }

        let index = self.rng.next_below(legal.len() as u32) as usize;
        legal[index]
    }
}

fn is_play_basic(action: &Action, _view: &PlayerView, _db: &CardDb) -> bool {
    matches!(action, Action::PlayBasic { .. })
}

fn is_use_ability(action: &Action, _view: &PlayerView, _db: &CardDb) -> bool {
    matches!(action, Action::UseAbility { .. })
}

fn is_evolve(action: &Action, _view: &PlayerView, _db: &CardDb) -> bool {
    matches!(action, Action::Evolve { .. })
}

fn is_tool(action: &Action, _view: &PlayerView, _db: &CardDb) -> bool {
    matches!(action, Action::PlayTool { .. })
}

fn is_attach_energy(action: &Action, _view: &PlayerView, _db: &CardDb) -> bool {
    matches!(action, Action::AttachEnergy { .. })
}

fn is_stadium(action: &Action, view: &PlayerView, db: &CardDb) -> bool {
    trainer_kind(action, view, db) == Some(TrainerKind::Stadium)
}

fn is_drawing_item(action: &Action, view: &PlayerView, db: &CardDb) -> bool {
    trainer_tier(action, view, db) == Some((TrainerKind::Item, EffectCategory::Draws))
}

fn is_drawing_supporter(action: &Action, view: &PlayerView, db: &CardDb) -> bool {
    trainer_tier(action, view, db) == Some((TrainerKind::Supporter, EffectCategory::Draws))
}

fn is_searching_item(action: &Action, view: &PlayerView, db: &CardDb) -> bool {
    trainer_tier(action, view, db) == Some((TrainerKind::Item, EffectCategory::Searches))
}

fn is_searching_supporter(action: &Action, view: &PlayerView, db: &CardDb) -> bool {
    trainer_tier(action, view, db) == Some((TrainerKind::Supporter, EffectCategory::Searches))
}

fn is_other_item(action: &Action, view: &PlayerView, db: &CardDb) -> bool {
    trainer_tier(action, view, db) == Some((TrainerKind::Item, EffectCategory::Other))
}

fn is_other_supporter(action: &Action, view: &PlayerView, db: &CardDb) -> bool {
    trainer_tier(action, view, db) == Some((TrainerKind::Supporter, EffectCategory::Other))
}

/// The `TrainerKind` an `Action::PlayTrainer` names, read from the card's
/// own definition — `None` for every other kind of `Action`.
fn trainer_kind(action: &Action, view: &PlayerView, db: &CardDb) -> Option<TrainerKind> {
    let Action::PlayTrainer { card } = action else {
        return None;
    };
    let card_view = view.your_hand.iter().find(|c| c.id == *card)?;
    Some(db.get(card_view.def).as_trainer()?.kind)
}

/// The `(kind, category)` pair an `Action::PlayTrainer` ranks by, for every
/// kind but Stadium (`is_stadium` reads that one on its own, ranked above
/// this whole tier).
fn trainer_tier(
    action: &Action,
    view: &PlayerView,
    db: &CardDb,
) -> Option<(TrainerKind, EffectCategory)> {
    let Action::PlayTrainer { card } = action else {
        return None;
    };
    let card_view = view.your_hand.iter().find(|c| c.id == *card)?;
    let trainer = db.get(card_view.def).as_trainer()?;
    if trainer.kind == TrainerKind::Stadium {
        return None;
    }
    Some((trainer.kind, classify(&trainer.effect)))
}

/// What a Trainer's own effect does, for ranking purposes — draws a card,
/// searches the deck, or neither. Not every `TrainerEffect` variant is
/// named here: most belong to a Tool or a Stadium, which never reach this
/// (Tools attach through `Action::PlayTool`, not `PlayTrainer`; Stadiums
/// are ranked by `is_stadium` before this ever runs), so a new variant
/// defaults to `Other` rather than failing to compile. A future Item or
/// Supporter effect that draws or searches needs adding here by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EffectCategory {
    Draws,
    Searches,
    Other,
}

fn classify(effect: &TrainerEffect) -> EffectCategory {
    match effect {
        TrainerEffect::BothShuffleHandThenDraw { .. }
        | TrainerEffect::ShuffleHandThenDraw { .. }
        | TrainerEffect::DrawPerOpponentBenched
        | TrainerEffect::MayDrawTwoIfPlayedTeamRocketSupporter
        | TrainerEffect::OpponentHandToBottomThenDraw { .. } => EffectCategory::Draws,
        TrainerEffect::Decide { from: Zone::Deck, .. }
        | TrainerEffect::LookAtBottomOfDeck { .. }
        | TrainerEffect::JaninesSecretArt => EffectCategory::Searches,
        // `Kieran`'s own dual-mode effect: rank it by whichever of its two
        // branches would rank best, since either might actually run.
        TrainerEffect::ChooseOneOf(a, b) => classify(a).min(classify(b)),
        _ => EffectCategory::Other,
    }
}
