import { describe, expect, it } from "vitest";
import {
  AUTO_ADVANCE_CAP,
  gateAfterSeat,
  shouldAutoAdvance,
  sides,
  type AutoAdvanceInput,
} from "./session";

describe("sides", () => {
  it("maps the viewer to their own side and the other to the opponent", () => {
    expect(sides(0)).toEqual({ mine: 0, opponent: 1 });
    expect(sides(1)).toEqual({ mine: 1, opponent: 0 });
  });
});

describe("gateAfterSeat", () => {
  it("closes the reveal gate the first time a seat is shown", () => {
    expect(gateAfterSeat(undefined, 0)).toEqual({ shown: 0, reveal: false });
  });

  it("closes the gate when the acting seat changes", () => {
    expect(gateAfterSeat(0, 1)).toEqual({ shown: 1, reveal: false });
  });

  it("leaves the gate open while the same seat keeps acting", () => {
    expect(gateAfterSeat(1, 1)).toEqual({ shown: 1, reveal: true });
  });

  it("closes the gate when the game ends (no seat to act)", () => {
    expect(gateAfterSeat(1, undefined)).toEqual({
      shown: undefined,
      reveal: false,
    });
  });
});

describe("shouldAutoAdvance", () => {
  const base: AutoAdvanceInput = {
    actionCount: 1,
    playing: true,
    revealed: true,
    over: false,
    busy: false,
    steps: 0,
  };

  it("advances when there is exactly one legal action and the seat is revealed", () => {
    expect(shouldAutoAdvance(base)).toBe(true);
  });

  it("does not advance when the player still has a choice", () => {
    expect(shouldAutoAdvance({ ...base, actionCount: 2 })).toBe(false);
  });

  it("does not advance before the seat is revealed", () => {
    expect(shouldAutoAdvance({ ...base, revealed: false })).toBe(false);
  });

  it("does not advance once the game is over", () => {
    expect(shouldAutoAdvance({ ...base, over: true })).toBe(false);
  });

  it("does not advance while a move is being applied", () => {
    expect(shouldAutoAdvance({ ...base, busy: true })).toBe(false);
  });

  it("stops at the forced-loop cap", () => {
    expect(shouldAutoAdvance({ ...base, steps: AUTO_ADVANCE_CAP })).toBe(false);
    expect(shouldAutoAdvance({ ...base, steps: AUTO_ADVANCE_CAP - 1 })).toBe(true);
  });

  it("does not advance while the engine is still loading", () => {
    expect(shouldAutoAdvance({ ...base, playing: false })).toBe(false);
  });
});
