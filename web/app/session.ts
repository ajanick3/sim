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
