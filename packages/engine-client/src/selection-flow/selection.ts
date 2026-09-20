// The reference tap-to-choose interaction model: narrows a Wire client's
// legal actions to what a player has tapped, groups the rest for display,
// and decides when a forced single choice advances on its own. See ADR
// 0104 and the "Selection" / "Selection flow" glossary entries — this is
// one particular pointer/tap paradigm built on a Wire client, not
// required by one.
import type { WireActionMeta } from "../wire/view";

/** What the player has tapped on the board, if anything. */
export type Selection = { kind: "pokemon"; id: number } | { kind: "hand"; card: number } | null;

/**
 * The `legal_actions` indices that concern the current selection: a tapped
 * Pokémon matches every move aimed at it (`target`); a tapped hand card
 * matches every move that plays it (`card`). No selection matches nothing —
 * the caller shows the whole list instead.
 */
export function movesForSelection(meta: WireActionMeta[], sel: Selection): number[] {
  if (!sel) return [];
  return meta
    .map((m, i) => ({ m, i }))
    .filter(({ m }) => (sel.kind === "pokemon" ? m.target === sel.id : m.card === sel.card))
    .map(({ i }) => i);
}

/**
 * When a hand card is selected, the Pokémon ids a move with that card can be
 * aimed at — the ones to highlight as drop targets. Maps each target id to
 * the `legal_actions` index that lands the card there.
 */
export function targetsForHandCard(meta: WireActionMeta[], card: number): Map<number, number> {
  const out = new Map<number, number>();
  meta.forEach((m, i) => {
    if (m.card === card && m.target !== null && !out.has(m.target)) out.set(m.target, i);
  });
  return out;
}

// The engine hands the UI a flat list of move labels. Until the wasm
// boundary reports structured actions, the label prefix is the only clue
// to what kind of move each one is. These rules are matched in order; the
// first hit wins, and anything unmatched lands in "Other".
const ACTION_GROUPS: { group: string; test: RegExp }[] = [
  { group: "Attack", test: /^Attack/ },
  { group: "Attach Energy", test: /^Attach .* Energy /i },
  { group: "Attach", test: /^Attach / },
  { group: "Evolve", test: /^Evolve / },
  { group: "Bench", test: /^Bench / },
  { group: "Play", test: /^Play / },
  { group: "Retreat", test: /^Retreat/ },
  { group: "Promote", test: /^Promote / },
  { group: "Move Energy", test: /^(Move .* to|Stop moving Energy)/ },
  { group: "Heal", test: /^Heal / },
  { group: "Take cards", test: /^(Take |Stop taking)/ },
  { group: "Discard", test: /^Discard /i },
  { group: "Choose", test: /^Choose /i },
];
const FINISH_GROUP = "Finish";
const OTHER_GROUP = "Other";

export const ACTION_GROUP_ORDER = [...ACTION_GROUPS.map((g) => g.group), OTHER_GROUP, FINISH_GROUP];

export interface GroupedAction {
  /** Index into the engine's own `legal_actions()` list — what `apply` takes. */
  index: number;
  label: string;
  /** Set when this exact label appears more than once; its 0-based rank. */
  copy?: number;
}

export interface ActionGroup {
  group: string;
  items: GroupedAction[];
}

function groupOf(label: string): string {
  if (label === "End turn" || label === "End your turn") return FINISH_GROUP;
  for (const { group, test } of ACTION_GROUPS) {
    if (test.test(label)) return group;
  }
  return OTHER_GROUP;
}

/**
 * Sort action labels into named groups for display, keeping each action's
 * original index for `apply`. When one label repeats — two same-named
 * Pokémon give identical text — each copy gets a 0-based `copy` rank so the
 * panel can mark them apart, the way the board does.
 */
export function groupActions(labels: string[]): ActionGroup[] {
  return groupActionsAt(
    labels,
    labels.map((_, i) => i),
  );
}

/**
 * As `groupActions`, but over a subset: `indices` picks which of `labels` to
 * show (and in what order), each keeping its own index for `apply`.
 */
export function groupActionsAt(labels: string[], indices: number[]): ActionGroup[] {
  const chosen = indices.map((index) => labels[index]);
  const total = new Map<string, number>();
  for (const l of chosen) total.set(l, (total.get(l) ?? 0) + 1);
  const seen = new Map<string, number>();

  const byGroup = new Map<string, GroupedAction[]>();
  indices.forEach((index) => {
    const label = labels[index];
    const g = groupOf(label);
    const item: GroupedAction = { index, label };
    if ((total.get(label) ?? 0) > 1) {
      const rank = seen.get(label) ?? 0;
      seen.set(label, rank + 1);
      item.copy = rank;
    }
    const bucket = byGroup.get(g) ?? [];
    bucket.push(item);
    byGroup.set(g, bucket);
  });

  return ACTION_GROUP_ORDER.filter((g) => byGroup.has(g)).map((group) => ({
    group,
    items: byGroup.get(group)!,
  }));
}

export const AUTO_ADVANCE_CAP = 100;

export interface AutoAdvanceInput {
  /** How many legal actions the acting player has. */
  actionCount: number;
  /** The engine has finished loading and a game is in progress. */
  playing: boolean;
  /** The seat has been revealed (past the pass-the-device screen), if the
   *  UI has such a screen at all — a UI with no privacy gate always
   *  passes `true`. */
  revealed: boolean;
  over: boolean;
  /** A move is mid-apply. */
  busy: boolean;
  /** Consecutive auto-advances taken since the last real choice. */
  steps: number;
  /** The wire's current phase tag, e.g. "Main", "PlacingActive". */
  phase?: string;
}

/** Phases where a sole legal action is still a real choice the player
 *  should see land, not a forced step to skip past. Setup's opening
 *  Active is the one place a single Basic in hand still deserves its
 *  own tap — a mulligan hand of exactly one Basic is not "no choice",
 *  it's the choice. */
const NEVER_AUTO_ADVANCE = new Set(["PlacingActive"]);

/**
 * Whether the UI should apply the sole legal action for the player. Only
 * when there is exactly one — no choice to make — and never before the
 * seat is revealed, so a pass-the-device gate (if the UI has one) is not
 * skipped.
 */
export function shouldAutoAdvance(i: AutoAdvanceInput): boolean {
  return (
    i.actionCount === 1 &&
    i.playing &&
    i.revealed &&
    !i.over &&
    !i.busy &&
    i.steps < AUTO_ADVANCE_CAP &&
    !(i.phase != null && NEVER_AUTO_ADVANCE.has(i.phase))
  );
}
