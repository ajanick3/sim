# Plain-draw Supporters

Type: task
Status: resolved

`Cheren`, `Friends in Paldea`, and `Urbain` each read only "Draw 3
cards." The engine has `DrawCards` for attacks and Abilities but no plain
draw for a Supporter — every draw Supporter built so far shuffles the
hand in first (`Judge`, `Lillie's Determination`) or has a rider.

## Acceptance criteria

- [ ] `TrainerEffect::Draw(u32)` — draw that many, no cost, no target.
- [ ] `resolve_trainer` runs it; `legal_actions` offers it like any other
      Supporter with no target.
- [ ] `known_trainer` maps `Cheren`, `Friends in Paldea`, `Urbain` to
      `Draw(3)`.
- [ ] A test plays a `Draw(3)` Supporter and the hand grows by three;
      an `..._is_admitted_from_the_artifact` check for one of the names.
- [ ] README progress table updated.

## Answer

Resolved 2026-09-10 on `feat/plain-draw-supporters`. `TrainerEffect::Draw(u32)`
in `card.rs`, run by `resolve_trainer`, mapped for `Cheren`,
`Friends in Paldea`, `Urbain` in `known_trainer`. Tests `cheren_draws_three`
and `the_plain_draw_supporters_are_admitted_from_the_artifact` in
`tests/supporters.rs`. `progress_table` gained a Standard-coverage summary;
README regenerated (Supporters 31/78). Guard tests: `card_records` 803→808,
`card_types` trainers 252→247.
