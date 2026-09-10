// Pure decisions for a game session, kept out of the component so they can
// be tested without a DOM or the wasm engine.

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
