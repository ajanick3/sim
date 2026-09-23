import { fireEvent, render, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DeckPicker } from "./DeckPicker";
import type { DeckEntry } from "./decks";

const decks: DeckEntry[] = [
  {
    key: "2026-worlds/001-andrew-hedrick",
    player: "Andrew Hedrick",
    tournament: "2026 Worlds",
    headline: "Dragapult ex",
  },
  {
    key: "2026-worlds/002-diego-cassiraga",
    player: "Diego Cassiraga",
    tournament: "2026 Worlds",
    headline: "Alakazam",
  },
  {
    key: "2026-worlds/005-rune-heiremans",
    player: "Rune Heiremans",
    tournament: "2026 Worlds",
    headline: "Mega Kangaskhan ex",
  },
  {
    key: "2026-baltimore/025-andrew-hedrick",
    player: "Andrew Hedrick",
    tournament: "2026 Baltimore",
    headline: "Dragapult ex",
  },
];

const SEARCH_PLACEHOLDER = "Search player, Pokémon, or tournament…";

describe("DeckPicker", () => {
  it("shows the selected deck's player, headline, and tournament on the closed button", () => {
    const { getByText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={vi.fn()} />,
    );
    expect(getByText("Andrew Hedrick")).toBeTruthy();
    expect(getByText(/Dragapult ex/)).toBeTruthy();
    expect(getByText(/2026 Worlds/)).toBeTruthy();
  });

  it("opens a search sheet listing every deck", () => {
    const { getByText, getByRole, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    expect(getByPlaceholderText(SEARCH_PLACEHOLDER)).toBeTruthy();
    const list = within(getByRole("list", { name: "Deck results" }));
    expect(list.getAllByText(/Andrew Hedrick/)).toHaveLength(2);
    for (const other of decks.filter((d) => d.player !== "Andrew Hedrick")) {
      expect(list.getByText(new RegExp(other.player))).toBeTruthy();
    }
  });

  it("disambiguates two decks from the same player by tournament", () => {
    const { getByText, getByRole } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    const list = within(getByRole("list", { name: "Deck results" }));
    const rows = list.getAllByText(/Andrew Hedrick/).map((el) => el.closest("button")!.textContent);
    expect(rows.some((r) => r?.includes("2026 Worlds"))).toBe(true);
    expect(rows.some((r) => r?.includes("2026 Baltimore"))).toBe(true);
  });

  it("filters by player name", () => {
    const { getByText, getByRole, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.change(getByPlaceholderText(SEARCH_PLACEHOLDER), {
      target: { value: "diego" },
    });
    const list = within(getByRole("list", { name: "Deck results" }));
    expect(list.getByText(/Diego Cassiraga/)).toBeTruthy();
    expect(list.queryByText(/Rune Heiremans/)).toBeNull();
  });

  it("filters by headline (the Pokémon), not just the player", () => {
    const { getByText, getByRole, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.change(getByPlaceholderText(SEARCH_PLACEHOLDER), {
      target: { value: "kangaskhan" },
    });
    const list = within(getByRole("list", { name: "Deck results" }));
    expect(list.getByText(/Rune Heiremans/)).toBeTruthy();
    expect(list.queryByText(/Diego Cassiraga/)).toBeNull();
  });

  it("filters by tournament", () => {
    const { getByText, getByRole, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.change(getByPlaceholderText(SEARCH_PLACEHOLDER), {
      target: { value: "baltimore" },
    });
    const list = within(getByRole("list", { name: "Deck results" }));
    expect(list.getAllByText(/Andrew Hedrick/)).toHaveLength(1);
    expect(list.queryByText(/Diego Cassiraga/)).toBeNull();
  });

  it("shows a no-match message rather than an empty list", () => {
    const { getByText, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={vi.fn()} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.change(getByPlaceholderText(SEARCH_PLACEHOLDER), {
      target: { value: "zzz-no-such-deck" },
    });
    expect(getByText(/No deck matches/)).toBeTruthy();
  });

  it("picks a deck on click and closes the sheet", () => {
    const onPick = vi.fn();
    const { getByText, queryByPlaceholderText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={onPick} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.click(getByText(/Diego Cassiraga/));
    expect(onPick).toHaveBeenCalledWith("2026-worlds/002-diego-cassiraga");
    expect(queryByPlaceholderText(SEARCH_PLACEHOLDER)).toBeNull();
  });

  it("picks the highlighted deck on Enter", () => {
    const onPick = vi.fn();
    const { getByText, getByPlaceholderText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={onPick} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    const input = getByPlaceholderText(SEARCH_PLACEHOLDER);
    fireEvent.change(input, { target: { value: "diego" } });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onPick).toHaveBeenCalledWith("2026-worlds/002-diego-cassiraga");
  });

  it("closes without picking on Escape", () => {
    const onPick = vi.fn();
    const { getByText, getByPlaceholderText, queryByPlaceholderText } = render(
      <DeckPicker decks={decks} value="2026-worlds/001-andrew-hedrick" onPick={onPick} />,
    );
    fireEvent.click(getByText("Andrew Hedrick"));
    fireEvent.keyDown(getByPlaceholderText(SEARCH_PLACEHOLDER), { key: "Escape" });
    expect(onPick).not.toHaveBeenCalled();
    expect(queryByPlaceholderText(SEARCH_PLACEHOLDER)).toBeNull();
  });
});
