import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DeckSearchDialog } from "./DeckSearchDialog";
import { ActionDialog } from "./ActionDialog";

const cards = [
  { id: 1, name: "Dreepy", imageUrl: "/dreepy.png", eligible: true },
  { id: 2, name: "Blocked", imageUrl: "/blocked.png", eligible: false },
];

describe("game dialogs", () => {
  it("selects an eligible deck card and confirms it", () => {
    const onConfirm = vi.fn();
    render(<DeckSearchDialog cards={cards} onConfirm={onConfirm} />);
    fireEvent.click(screen.getByRole("button", { name: "Dreepy" }));
    fireEvent.click(screen.getByRole("button", { name: "Add to hand" }));
    expect(onConfirm).toHaveBeenCalledWith([1]);
    expect(screen.getByRole("button", { name: "Blocked" })).toBeDisabled();
  });

  it("puts the finish-search action in the header, not a second dialog", () => {
    const onDone = vi.fn();
    render(<DeckSearchDialog cards={cards} onDone={onDone} doneLabel="Stop taking cards" />);
    fireEvent.click(screen.getByRole("button", { name: "Stop taking cards" }));
    expect(onDone).toHaveBeenCalled();
  });

  it("sorts eligible cards before ineligible cards without mutating the input", () => {
    const unsorted = [cards[1], cards[0]];
    render(<DeckSearchDialog cards={unsorted} />);
    const choices = screen.getByLabelText("Cards in deck").querySelectorAll("button");
    expect(choices[0]).toHaveAccessibleName("Dreepy");
    expect(choices[1]).toHaveAccessibleName("Blocked");
    expect(unsorted[0].name).toBe("Blocked");
  });

  it("reports the chosen game action", () => {
    const onChoose = vi.fn();
    render(
      <ActionDialog title="Choose" actions={[{ id: 4, label: "Bite" }]} onChoose={onChoose} />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Bite" }));
    expect(onChoose).toHaveBeenCalledWith(4);
  });
});
