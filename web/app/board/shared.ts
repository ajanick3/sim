// Pure helpers and shared types for the board components.

import type { Selection } from "../session";
import type { WireActionMeta, WireCard, WirePokemon } from "../view";

/** Resolve a print id to an image URL, or null when there is no art. */
export type Art = (printId: string) => string | null;

/** Seat 0 is the first player, seat 1 the second — shown as medals. */
export const SEAT_NAME = ["🥇", "🥈"];

/** A stable pseudo-random spot for a Pokémon's damage counter, as
 *  `top` / `left` percentages over the illustration. Seeded by the
 *  Pokémon's id so it stays put across renders. */
export function damageSpot(id: number): { top: string; left: string } {
  const h = (id * 2654435761) >>> 0;
  return { top: `${28 + (h % 30)}%`, left: `${24 + ((h >>> 8) % 44)}%` };
}

/** The props `monHooks` fills in so a Pokémon card can be tapped. */
export type MonHooks = {
  onSelect?: () => void;
  selectable?: boolean;
  selected?: boolean;
  dropTarget?: boolean;
};

/** Wire a Pokémon card to the selection flow: tappable when a legal move
 *  names it (or `extraSelectable` forces it), ringed when selected, and
 *  a drop target when a held card can land on it. */
export function monHooks(
  m: WirePokemon | null,
  meta: WireActionMeta[],
  selection: Selection,
  dropTargets: Map<number, number>,
  onPokemon: (id: number) => void,
  extraSelectable = false,
): MonHooks {
  if (!m) return {};
  return {
    onSelect: () => onPokemon(m.id),
    selectable: extraSelectable || meta.some((x) => x.target === m.id) || dropTargets.has(m.id),
    selected: selection?.kind === "pokemon" && selection.id === m.id,
    dropTarget: dropTargets.has(m.id),
  };
}

export const HAND_ORDER = [
  "pokemon",
  "supporter",
  "item",
  "tool",
  "stadium",
  "special-energy",
  "energy",
] as const;

/** Sort the hand by category, then break it into at most two rows at the
 *  category boundary nearest the middle. One row until it would hold more
 *  than six; then two rows, each at least three wide. */
export function splitHandRows(hand: WireCard[]): WireCard[][] {
  const order = HAND_ORDER as readonly string[];
  const rank = (c: string) => {
    const i = order.indexOf(c);
    return i < 0 ? order.length : i;
  };
  const sorted = [...hand].sort((a, b) => rank(a.category) - rank(b.category));
  const n = sorted.length;
  if (n <= 6) return [sorted];
  const target = Math.ceil(n / 2);
  const boundaries: number[] = [];
  for (let i = 1; i < n; i++) {
    if (sorted[i].category !== sorted[i - 1].category) boundaries.push(i);
  }
  let split = target;
  if (boundaries.length) {
    const best = boundaries.reduce((p, c) => (Math.abs(c - target) < Math.abs(p - target) ? c : p));
    if (Math.abs(best - target) <= 2) split = best;
  }
  split = Math.max(3, Math.min(split, n - 3));
  return [sorted.slice(0, split), sorted.slice(split)];
}

// --- Decision bar / prompt shapes ----------------------------------------

export type DecisionKind = "take" | "discard" | "choose" | "pay";

/** When the whole legal-action set is a search / discard / pick prompt,
 *  describe it; otherwise null and the normal panel shows. */
export function asDecision(actions: string[]): { kind: DecisionKind; verb: string } | null {
  if (actions.length === 0) return null;
  const isFinish = (l: string) => /^(Stop |Finish|Take no more|Move on|Decline)/.test(l);
  const body = actions.filter((l) => !isFinish(l));
  if (body.length === 0) return null;
  const test = (re: RegExp) => body.every((l) => re.test(l));
  // The setup bonus draw is a yes / no, not a card to pick — asPrompt
  // handles it.
  if (test(/bonus card$/)) return null;
  if (test(/^Take /)) return { kind: "take", verb: "Choose cards to take" };
  if (test(/^Bench /)) return { kind: "take", verb: "Choose Pokémon to Bench" };
  // Before the plain "Discard …" test, which would otherwise swallow it.
  if (test(/^Discard .* to pay/)) return { kind: "pay", verb: "Discard to pay the cost" };
  if (test(/^Discard /)) return { kind: "discard", verb: "Choose cards to discard" };
  if (test(/^Choose /)) return { kind: "choose", verb: "Make a choice" };
  return null;
}

export type Prompt = {
  verb: string;
  accepts: { label: string; index: number }[];
  decline: number;
  declineLabel?: string;
};

/** A yes / no the engine is waiting on: a "may" Ability, or the setup
 *  bonus draw. Null when the action set is not that shape. */
export function asPrompt(actions: string[]): Prompt | null {
  // Setup: take the bonus cards the opponent's mulligans owe you.
  const takeBonus = actions.indexOf("Take a bonus card");
  if (takeBonus >= 0) {
    return {
      verb: "Take a bonus card?",
      accepts: [{ label: "Take one", index: takeBonus }],
      decline: actions.findIndex((a) => /^Take no more/.test(a)),
      declineLabel: "No more",
    };
  }
  const decline = actions.findIndex((a) => /^Decline /.test(a));
  if (decline < 0) return null;
  const accepts = actions
    .map((label, index) => ({ label, index }))
    .filter(({ index }) => index !== decline);
  if (accepts.length === 0) return null;
  const name = actions[decline].replace(/^Decline (the )?/, "");
  return { verb: `Use ${name}?`, accepts, decline };
}

export type CoinResult = "heads" | "tails";

/** The coin-flip results in a run of log lines, oldest first. The engine
 *  logs each as "<player> flips heads." / "tails." */
export function coinFlipsIn(lines: string[]): CoinResult[] {
  const out: CoinResult[] = [];
  for (const line of lines) {
    const m = /\bflips (heads|tails)\.$/.exec(line);
    if (m) out.push(m[1] as CoinResult);
  }
  return out;
}
