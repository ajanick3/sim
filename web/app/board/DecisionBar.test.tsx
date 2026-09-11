import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { card, noArt } from "./fixtures";
import { asDecision } from "./shared";
import { DecisionBar } from "./DecisionBar";
import type { WireCard } from "../view";

const deck: WireCard[] = [
  card({ name: "Ralts", category: "pokemon" }),
  card({ name: "Kirlia", category: "pokemon" }),
  card({ name: "Ultra Ball", category: "item" }),
];
const actions = ["Take Ralts", "Stop searching"];
const meta = [
  { kind: "TakeCard", card: deck[0].id, target: null },
  { kind: "FinishDeciding", card: null, target: null },
];

function bar(overrides = {}) {
  return render(
    <DecisionBar
      actions={actions}
      decision={asDecision(actions)!}
      busy={false}
      onAct={vi.fn()}
      art={noArt}
      meta={meta}
      deck={deck}
      {...overrides}
    />,
  );
}

describe("DecisionBar with a whole-deck search", () => {
  it("draws every deck card, not only the takeable ones", () => {
    const { getByText } = bar();
    for (const name of ["Ralts", "Kirlia", "Ultra Ball"]) {
      expect(getByText(name)).toBeTruthy();
    }
  });

  it("disables the cards this step cannot take", () => {
    const { getByText } = bar();
    const takeable = getByText("Ralts").closest("button")!;
    const dimmed = getByText("Kirlia").closest("button")!;
    expect(takeable.disabled).toBe(false);
    expect(dimmed.disabled).toBe(true);
  });

  it("acts with the takeable card's action index", () => {
    const onAct = vi.fn();
    const { getByText } = bar({ onAct });
    fireEvent.click(getByText("Ralts").closest("button")!);
    expect(onAct).toHaveBeenCalledWith(0);
  });

  it("falls back to the plain choice row without a deck", () => {
    const { getByText, queryByText } = bar({ deck: null });
    expect(getByText("Ralts")).toBeTruthy();
    expect(queryByText("Kirlia")).toBeNull();
  });
});

describe("DecisionBar for a slot bound to attach", () => {
  // Crispin's second slot: the same Energy could land on any Pokémon in
  // play, so `action_meta` carries one `TakeCardOnto` per legal target.
  const energy = card({ name: "Fire Energy", category: "energy" });
  const attachActions = [
    "Take Fire Energy and attach it to Charmander",
    "Take Fire Energy and attach it to Squirtle",
    "Stop taking cards",
  ];
  const attachMeta = [
    { kind: "TakeCardOnto", card: energy.id, target: 10 },
    { kind: "TakeCardOnto", card: energy.id, target: 11 },
    { kind: "FinishDeciding", card: null, target: null },
  ];

  it("selects the card instead of guessing a target, when more than one is legal", () => {
    const onAct = vi.fn();
    const onSelect = vi.fn();
    const { getByText } = render(
      <DecisionBar
        actions={attachActions}
        decision={asDecision(attachActions)!}
        busy={false}
        onAct={onAct}
        art={noArt}
        meta={attachMeta}
        deck={[energy]}
        selection={null}
        onSelect={onSelect}
      />,
    );
    fireEvent.click(getByText("Fire Energy").closest("button")!);
    expect(onAct).not.toHaveBeenCalled();
    expect(onSelect).toHaveBeenCalledWith({ kind: "hand", card: energy.id });
  });
});
