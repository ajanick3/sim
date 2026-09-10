import { fireEvent, render } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { card, noArt } from "./fixtures";
import { asDecision } from "./shared";
import { DecisionBar } from "./DecisionBar";
import type { WireCard } from "../view";

const library: WireCard[] = [
  card({ name: "Ralts", category: "pokemon" }),
  card({ name: "Kirlia", category: "pokemon" }),
  card({ name: "Ultra Ball", category: "item" }),
];
const actions = ["Take Ralts", "Stop searching"];
const meta = [
  { kind: "TakeCard", card: library[0].id, target: null },
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
      library={library}
      {...overrides}
    />,
  );
}

describe("DecisionBar with a whole-library search", () => {
  it("draws every library card, not only the takeable ones", () => {
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

  it("falls back to the plain choice row without a library", () => {
    const { getByText, queryByText } = bar({ library: null });
    expect(getByText("Ralts")).toBeTruthy();
    expect(queryByText("Kirlia")).toBeNull();
  });
});
