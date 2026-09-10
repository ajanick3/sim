import { describe, expect, it, vi } from "vitest";
import type { WireActionMeta, WireCard, WirePokemon } from "../view";
import { asDecision, asPrompt, coinFlipsIn, damageSpot, monHooks, splitHandRows } from "./shared";

const card = (category: string, id = 0): WireCard => ({
  id,
  name: `c${id}`,
  def: 0,
  print_id: `p${id}`,
  energy_type: null,
  category,
});

describe("damageSpot", () => {
  it("is stable for an id and varies between ids", () => {
    expect(damageSpot(3)).toEqual(damageSpot(3));
    expect(damageSpot(3)).not.toEqual(damageSpot(4));
  });

  it("stays inside the illustration", () => {
    for (const id of [0, 1, 2, 7, 41, 999]) {
      const { top, left } = damageSpot(id);
      const t = Number(top.replace("%", ""));
      const l = Number(left.replace("%", ""));
      expect(t).toBeGreaterThanOrEqual(28);
      expect(t).toBeLessThan(58);
      expect(l).toBeGreaterThanOrEqual(24);
      expect(l).toBeLessThan(68);
    }
  });
});

describe("splitHandRows", () => {
  it("keeps a hand of six or fewer in one row, sorted by category", () => {
    const hand = [card("energy", 1), card("pokemon", 2), card("item", 3)];
    const rows = splitHandRows(hand);
    expect(rows).toHaveLength(1);
    expect(rows[0].map((c) => c.category)).toEqual(["pokemon", "item", "energy"]);
  });

  it("splits a larger hand into two rows of at least three", () => {
    const hand = [
      ...Array.from({ length: 5 }, (_, i) => card("pokemon", i)),
      ...Array.from({ length: 4 }, (_, i) => card("energy", 10 + i)),
    ];
    const [a, b] = splitHandRows(hand);
    expect(a.length).toBeGreaterThanOrEqual(3);
    expect(b.length).toBeGreaterThanOrEqual(3);
    expect(a.length + b.length).toBe(9);
  });

  it("prefers a category boundary near the middle for the split", () => {
    // 4 pokemon then 4 items — the boundary at 4 is the midpoint.
    const hand = [
      ...Array.from({ length: 4 }, (_, i) => card("pokemon", i)),
      ...Array.from({ length: 4 }, (_, i) => card("item", 10 + i)),
    ];
    const [a] = splitHandRows(hand);
    expect(a.every((c) => c.category === "pokemon")).toBe(true);
    expect(a).toHaveLength(4);
  });
});

describe("asDecision", () => {
  it("names a take / bench / discard / choose / pay prompt", () => {
    expect(asDecision(["Take Ralts", "Take Kirlia", "Stop taking"])).toEqual({
      kind: "take",
      verb: "Choose cards to take",
    });
    expect(asDecision(["Bench Ralts", "Bench Budew", "Stop searching"])).toMatchObject({
      kind: "take",
    });
    expect(asDecision(["Discard Rare Candy", "Stop discarding"])).toMatchObject({
      kind: "discard",
    });
    expect(asDecision(["Choose the first", "Choose the second"])).toMatchObject({ kind: "choose" });
    expect(asDecision(["Discard 2 Energy to pay the cost", "Stop discarding"])).toMatchObject({
      kind: "pay",
    });
  });

  it("is null for an ordinary turn and for the bonus draw", () => {
    expect(
      asDecision(["Attack: Itchy Pollen", "Retreat, promoting Dreepy", "End turn"]),
    ).toBeNull();
    expect(asDecision(["Take a bonus card", "Take no more bonus cards"])).toBeNull();
    expect(asDecision([])).toBeNull();
  });
});

describe("asPrompt", () => {
  it("reads the setup bonus draw as a yes / no", () => {
    expect(asPrompt(["Take a bonus card", "Take no more bonus cards"])).toEqual({
      verb: "Take a bonus card?",
      accepts: [{ label: "Take one", index: 0 }],
      decline: 1,
      declineLabel: "No more",
    });
  });

  it("reads a may-ability from its Decline line", () => {
    const p = asPrompt(["Use Psychic Draw", "Decline Psychic Draw"]);
    expect(p).toMatchObject({ verb: "Use Psychic Draw?", decline: 1 });
    expect(p?.accepts).toEqual([{ label: "Use Psychic Draw", index: 0 }]);
  });

  it("carries every non-decline option as an accept", () => {
    const p = asPrompt([
      "Attach Fire (Powerglass)",
      "Attach Water (Powerglass)",
      "Decline Powerglass",
    ]);
    expect(p?.accepts).toHaveLength(2);
    expect(p?.decline).toBe(2);
  });

  it("is null with no Decline line", () => {
    expect(asPrompt(["Attack: Tackle", "End turn"])).toBeNull();
  });
});

describe("monHooks", () => {
  const mon: WirePokemon = {
    id: 5,
    name: "Dreepy",
    print_id: "p5",
    hp: 70,
    damage: 0,
    remaining_hp: 70,
    conditions: [],
    attached: [],
  };
  const meta = (target: number | null): WireActionMeta[] => [
    { kind: "Attack", card: null, target },
  ];

  it("returns nothing for an empty slot", () => {
    expect(monHooks(null, [], null, new Map(), () => {})).toEqual({});
  });

  it("is selectable when a legal move names it, or a card can drop on it, or forced", () => {
    expect(monHooks(mon, meta(5), null, new Map(), () => {}).selectable).toBe(true);
    expect(monHooks(mon, meta(9), null, new Map(), () => {}).selectable).toBe(false);
    expect(monHooks(mon, meta(9), null, new Map([[5, 2]]), () => {}).selectable).toBe(true);
    expect(monHooks(mon, meta(9), null, new Map(), () => {}, true).selectable).toBe(true);
  });

  it("reports selected and dropTarget from the selection and drop map", () => {
    const h = monHooks(mon, [], { kind: "pokemon", id: 5 }, new Map([[5, 1]]), () => {});
    expect(h.selected).toBe(true);
    expect(h.dropTarget).toBe(true);
  });

  it("routes onSelect through the id", () => {
    const onPokemon = vi.fn();
    monHooks(mon, meta(5), null, new Map(), onPokemon).onSelect?.();
    expect(onPokemon).toHaveBeenCalledWith(5);
  });
});

describe("coinFlipsIn", () => {
  it("pulls each flip result from a run of log lines", () => {
    expect(
      coinFlipsIn([
        "One benches Budew.",
        "One flips heads.",
        "One flips tails.",
        "Two flips heads.",
        "One draws 2.",
      ]),
    ).toEqual(["heads", "tails", "heads"]);
  });

  it("is empty when nothing flipped", () => {
    expect(coinFlipsIn(["One wins the coin flip.", "One draws 7."])).toEqual([]);
  });
});
