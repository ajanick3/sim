# Is the action list outside the masked view by design, or a leak?

Type: research
Status: needs-triage

The senior review (Spec axis, fix-first 3 and unrecorded-decision 1) found
that `PlayerView` does not mask its `phase` field, and that no ADR says the
action list is unmasked.

## What the code does

`PlayerView::of` copies `phase: state.phase`. `Phase` is `Copy` and
several variants carry raw `CardId`s into hidden zones:

- `Phase::Deciding { card, previous, .. }` — `state.rs:189`
- `Phase::ChoosingOneOf { card }` — `state.rs:235`
- `Phase::LookingAtBottomOfDeck`, the `SearchingDeckFor…` phases

During a deck search, `previous` is a deck `CardId` and the `legal`
list names `Action::TakeCard { card }` for specific deck cards.

## Why it is latent, not live

`play.rs:88` and `selfplay.rs:44` build the view only for
`player_to_act` — the player who is searching their own deck, and may see
it. No caller builds an opponent view mid-search today.

## The conflict

`docs/architecture/glossary.md` / ADR 0006: a view hides "the cards in
either deck and their order". ADR 0095: a strategy reads a masked view,
not the state. ADR 0086 carved out one specific case (a choice reading the
opponent's hand for Claw of Darkness). The general rule — that
`legal_actions` output and `Phase` fields expose card identities the zone
lists hide — has no record.

## The decision

Two live alternatives:

1. **The list is unmasked by design.** The decision-maker is always the
   player the list belongs to, so a `CardId` in the list or the phase is
   never a leak. Write the ADR that says so, and note the invariant every
   view caller must keep (build a view only for `player_to_act`).
2. **The `phase` field is a leak.** Sanitize `phase` in `PlayerView::of`
   — drop or redact the `CardId`-bearing variants for a view that is not
   the actor's. Add a test that an opponent view carries no deck id.

## Acceptance criteria

- [ ] An ADR records whether the action list and `Phase` payload are
      inside or outside the mask.
- [ ] If unmasked by design, the ADR states the caller invariant.
- [ ] If a leak, `PlayerView::of` redacts `phase` for a non-actor view and
      a test covers it.
