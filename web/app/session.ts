// Pure decisions for a game session, kept out of the component so they can
// be tested without a DOM or the wasm engine.

import type { WireActionMeta } from "./view";

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

/** Which board side belongs to the viewer, and which to the opponent. */
export function sides(you: number): { mine: number; opponent: number } {
  return { mine: you, opponent: you === 1 ? 0 : 1 };
}

/**
 * The reveal gate after the acting seat is read again. It closes whenever
 * the seat changes — including the first read and the game ending — and
 * stays open while the same seat keeps acting.
 */
export function gateAfterSeat(
  shown: number | undefined,
  next: number | undefined,
): { shown: number | undefined; reveal: boolean } {
  if (next === shown) return { shown, reveal: true };
  return { shown: next, reveal: false };
}

/**
 * A subtle per-copy colour, so two Pokémon of the same name on one side
 * can be told apart. Cycles red, green, blue, yellow — enough for a full
 * four-of playset.
 */
export const COPY_COLORS = ["#E4593E", "#63B95B", "#5AA7E4", "#F4D023"];

/**
 * For each Pokémon on a side (active first, then the Bench; `null` for an
 * empty slot), the 0-based index of that Pokémon among the copies that
 * share its name — or `undefined` when the name is unique on the side, so
 * no badge is drawn.
 */
export function copyBadges(names: (string | null)[]): (number | undefined)[] {
  const total = new Map<string, number>();
  for (const n of names) {
    if (n !== null) total.set(n, (total.get(n) ?? 0) + 1);
  }
  const seen = new Map<string, number>();
  return names.map((n) => {
    if (n === null || (total.get(n) ?? 0) < 2) return undefined;
    const i = seen.get(n) ?? 0;
    seen.set(n, i + 1);
    return i;
  });
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
  /** The seat has been revealed (past the pass-the-device screen). */
  revealed: boolean;
  over: boolean;
  /** A move is mid-apply. */
  busy: boolean;
  /** Consecutive auto-advances taken since the last real choice. */
  steps: number;
}

/**
 * Whether the UI should apply the sole legal action for the player. Only
 * when there is exactly one — no choice to make — and never before the
 * seat is revealed, so the pass-the-device gate is not skipped.
 */
export function shouldAutoAdvance(i: AutoAdvanceInput): boolean {
  return (
    i.actionCount === 1 &&
    i.playing &&
    i.revealed &&
    !i.over &&
    !i.busy &&
    i.steps < AUTO_ADVANCE_CAP
  );
}
