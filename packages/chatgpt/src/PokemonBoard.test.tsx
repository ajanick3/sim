import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { CardSurface, CARD_ASPECT_RATIO } from "./CardSurface";
import {
  MediumDesktopPokemonBoard,
  MobilePokemonBoard,
  PokemonBoard,
  TabletPokemonBoard,
} from "./PokemonBoard";
import type { PokemonBoardState } from "./types";

const state: PokemonBoardState = {
  turn: 4,
  isPlayerTurn: true,
  stadiumName: "Beach Court",
  phaseLabel: "Main phase",
  player: {
    name: "Nick",
    active: { id: 1, name: "Player active", damage: 30 },
    bench: [
      { id: 2, name: "Player bench 1" },
      { id: 3, name: "Player bench 2" },
      { id: 4, name: "Player bench 3" },
      null,
    ],
    hand: Array.from({ length: 7 }, (_, index) => ({
      id: 10 + index,
      name: `Hand card ${index + 1}`,
    })),
    deckCount: 31,
    discardCount: 4,
    prizesRemaining: 4,
  },
  opponent: {
    name: "Rival",
    active: { id: 20, name: "Opponent active", damage: 80 },
    bench: [{ id: 21, name: "Opponent bench 1" }],
    deckCount: 28,
    discardCount: 7,
    prizesRemaining: 3,
  },
  log: ["Nick drew a card", "Rival ended their turn"],
};

function setViewportWidth(width: number) {
  Object.defineProperty(window, "matchMedia", {
    configurable: true,
    value: vi.fn().mockImplementation((query: string) => {
      const max = query.match(/max-width:\s*(\d+)px/);
      const min = query.match(/min-width:\s*(\d+)px/);
      const matches = (!max || width <= Number(max[1])) && (!min || width >= Number(min[1]));

      return {
        matches,
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      };
    }),
  });
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe("CardSurface", () => {
  it("keeps the source card aspect ratio", () => {
    render(<CardSurface card={{ id: 1, name: "Crushing Hammer" }} />);

    const card = screen.getByRole("img", { name: "Crushing Hammer" });
    expect(card).toHaveStyle({ aspectRatio: CARD_ASPECT_RATIO });
    expect(card).toHaveAttribute("data-card-ratio", "245 / 337");
    expect(card).toHaveStyle({ backgroundImage: expect.stringContaining("crushing-hammer.png") });
  });

  it("reports the selected card", () => {
    const onSelect = vi.fn();
    render(<CardSurface card={{ id: 1, name: "Crushing Hammer" }} onSelect={onSelect} />);

    fireEvent.click(screen.getByRole("button", { name: "Crushing Hammer" }));
    expect(onSelect).toHaveBeenCalledWith({ id: 1, name: "Crushing Hammer" });
  });
});

describe("responsive board layouts", () => {
  it.each([
    [MobilePokemonBoard, "mobile", "5"],
    [TabletPokemonBoard, "tablet", "7"],
    [MediumDesktopPokemonBoard, "medium-desktop", "7"],
  ] as const)("renders the %s component", (Component, layout, handColumns) => {
    const { container } = render(<Component state={state} />);

    expect(container.firstElementChild).toHaveAttribute("data-layout", layout);
    expect(container.querySelector(".chatgpt-hand__grid")).toHaveAttribute("data-columns", handColumns);
    container.querySelectorAll("[data-card-ratio]").forEach((card) => {
      expect(card).toHaveAttribute("data-card-ratio", "245 / 337");
    });
  });

  it("shows compact actions on tablet and the inspector on desktop", () => {
    const { rerender } = render(<TabletPokemonBoard state={state} selectedCardId={10} />);
    expect(screen.getByLabelText("Card actions")).toHaveClass("chatgpt-actions--compact");
    expect(screen.queryByText("Game log")).not.toBeInTheDocument();

    rerender(<MediumDesktopPokemonBoard state={state} selectedCardId={10} />);
    expect(screen.getByLabelText("Card actions")).not.toHaveClass("chatgpt-actions--compact");
    expect(screen.getByText("Game log")).toBeInTheDocument();
  });

  it("keeps the primary turn action available on mobile", () => {
    const onEndTurn = vi.fn();
    render(<MobilePokemonBoard state={state} onEndTurn={onEndTurn} />);

    fireEvent.click(screen.getByRole("button", { name: "End turn" }));
    expect(onEndTurn).toHaveBeenCalledOnce();
  });

  it.each([
    [390, "mobile"],
    [768, "tablet"],
    [1024, "medium-desktop"],
  ])("selects the correct layout at %ipx", (width, layout) => {
    setViewportWidth(width);
    const { container } = render(<PokemonBoard state={state} />);

    expect(container.firstElementChild).toHaveAttribute("data-layout", layout);
  });
});
