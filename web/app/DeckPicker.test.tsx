import { fireEvent, render, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DeckPicker } from "./DeckPicker";
import type { DeckEntry } from "./decks";

const decks: DeckEntry[] = [
  { key: "001-andrew-hedrick", player: "Andrew Hedrick", headline: "Dragapult ex" },
  { key: "002-diego-cassiraga", player: "Diego Cassiraga", headline: "Alakazam" },
  { key: "005-rune-heiremans", player: "Rune Heiremans", headline: "Mega Kangaskhan ex" },
];

describe("DeckPicker", () => {
  it("shows the selected deck's player and headline on the closed button", () => {
    const { getByText } = render(
      <DeckPicker decks={decks} value="001-andrew-hedrick" onPick={vi.fn()} />,
    );
    expect(getByText("Andrew Hedrick")).toBeTruthy();
    expect(getByText(/Dragapult ex/)).toBeTruthy();
  });

  it("opens a search sheet listing every deck", () => {
    const { getByText, getByRole, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    expect(getByPlaceholderText("Search player or Pokémon…")).toBeTruthy();
    const list = within(getByRole("list", { name: "Deck results" }));
    for (const d of decks) expect(list.getByText(new RegExp(d.player))).toBeTruthy();
  });

  it("filters by player name", () => {
    const { getByText, getByRole, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.change(getByPlaceholderText("Search player or Pokémon…"), {
      target: { value: "diego" },
    });
    const list = within(getByRole("list", { name: "Deck results" }));
    expect(list.getByText(/Diego Cassiraga/)).toBeTruthy();
    expect(list.queryByText(/Rune Heiremans/)).toBeNull();
  });

  it("filters by headline (the Pokémon), not just the player", () => {
    const { getByText, getByRole, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.change(getByPlaceholderText("Search player or Pokémon…"), {
      target: { value: "kangaskhan" },
    });
    const list = within(getByRole("list", { name: "Deck results" }));
    expect(list.getByText(/Rune Heiremans/)).toBeTruthy();
    expect(list.queryByText(/Andrew Hedrick/)).toBeNull();
  });

  it("shows a no-match message rather than an empty list", () => {
    const { getByText, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.change(getByPlaceholderText("Search player or Pokémon…"), {
      target: { value: "zzz-no-such-deck" },
    });
    expect(getByText(/No deck matches/)).toBeTruthy();
  });

  it("picks a deck on click and closes the sheet", () => {
    const onPick = vi.fn();
    const { getByText, queryByPlaceholderText } = render(
      <DeckPicker decks={decks} value="001-andrew-hedrick" onPick={onPick} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.click(getByText(/Diego Cassiraga/));
    expect(onPick).toHaveBeenCalledWith("002-diego-cassiraga");
    expect(queryByPlaceholderText("Search player or Pokémon…")).toBeNull();
  });

  it("picks the highlighted deck on Enter", () => {
    const onPick = vi.fn();
    const { getByText, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="001-andrew-hedrick" onPick={onPick} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    const input = getByPlaceholderText("Search player or Pokémon…");
    fireEvent.change(input, { target: { value: "diego" } });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onPick).toHaveBeenCalledWith("002-diego-cassiraga");
  });

  it("closes without picking on Escape", () => {
    const onPick = vi.fn();
    const { getByText, getByPlaceholderText, queryByPlaceholderText } = render(
      <DeckPicker decks={decks} value="001-andrew-hedrick" onPick={onPick} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.keyDown(getByPlaceholderText("Search player or Pokémon…"), { key: "Escape" });
    expect(onPick).not.toHaveBeenCalled();
    expect(queryByPlaceholderText("Search player or Pokémon…")).toBeNull();
  });
});
