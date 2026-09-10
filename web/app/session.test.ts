import { describe, expect, it } from "vitest";
import {
  AUTO_ADVANCE_CAP,
  copyBadges,
  gateAfterSeat,
  groupActions,
  movesForSelection,
  shouldAutoAdvance,
  sides,
  targetsForHandCard,
  type AutoAdvanceInput,
} from "./session";

describe("groupActions", () => {
  it("sorts labels into named groups and keeps their engine index", () => {
    const groups = groupActions([
      "End turn",
      "Attack: Phantom Dive (200 damage)",
      "Bench Dreepy",
      "Attach Fire Energy to Dragapult ex",
    ]);
    expect(groups.map((g) => g.group)).toEqual(["Attack", "Attach Energy", "Bench", "Finish"]);
    const attack = groups.find((g) => g.group === "Attack")!;
    expect(attack.items[0].index).toBe(1);
  });

  it("puts an unrecognised label in Other, before Finish", () => {
    const groups = groupActions(["Reshuffle the mystery pile", "End turn"]);
    expect(groups.map((g) => g.group)).toEqual(["Other", "Finish"]);
  });

  it("ranks repeated labels so identical moves can be told apart", () => {
    const groups = groupActions([
      "Evolve Dreepy into Drakloak",
      "Evolve Dreepy into Drakloak",
      "Bench Dreepy",
    ]);
    const evolve = groups.find((g) => g.group === "Evolve")!;
    expect(evolve.items.map((i) => i.copy)).toEqual([0, 1]);
    const bench = groups.find((g) => g.group === "Bench")!;
    expect(bench.items[0].copy).toBeUndefined();
  });
});

describe("copyBadges", () => {
  it("leaves a unique name without a badge", () => {
    expect(copyBadges(["Pikachu", "Snorlax", null])).toEqual([undefined, undefined, undefined]);
  });

  it("numbers each copy of a repeated name from zero", () => {
    expect(copyBadges(["Dreepy", "Dreepy", "Rotom", "Dreepy"])).toEqual([0, 1, undefined, 2]);
  });

  it("counts names independently and skips empty slots", () => {
    expect(copyBadges(["A", null, "A", "B", "B"])).toEqual([0, undefined, 1, 0, 1]);
  });
});

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

describe("movesForSelection / targetsForHandCard", () => {
  const meta = [
    { kind: "AttachEnergy", card: 10, target: 1 },
    { kind: "AttachEnergy", card: 10, target: 2 },
    { kind: "PlayTrainer", card: 11, target: null },
    { kind: "Retreat", card: null, target: 2 },
    { kind: "EndTurn", card: null, target: null },
  ];

  it("returns nothing when nothing is selected", () => {
    expect(movesForSelection(meta, null)).toEqual([]);
  });

  it("matches every move aimed at a selected Pokemon", () => {
    expect(movesForSelection(meta, { kind: "pokemon", id: 2 })).toEqual([1, 3]);
  });

  it("matches every move that plays a selected hand card", () => {
    expect(movesForSelection(meta, { kind: "hand", card: 10 })).toEqual([0, 1]);
    expect(movesForSelection(meta, { kind: "hand", card: 11 })).toEqual([2]);
  });

  it("maps a hand card's drop targets to the action that lands it there", () => {
    const t = targetsForHandCard(meta, 10);
    expect([...t.entries()]).toEqual([
      [1, 0],
      [2, 1],
    ]);
    expect(targetsForHandCard(meta, 11).size).toBe(0);
  });
});
