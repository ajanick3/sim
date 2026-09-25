import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { Board, type BoardProps } from "./Board";
import { CRUSHING_HAMMER_ART, DREEPY_ART } from "../../_fixtures/cardArt";

const pokemon = (id: number) => ({ id, name: "Dreepy", imageUrl: DREEPY_ART, hp: 70 });
const props: BoardProps = {
  active: pokemon(1),
  opponentActive: pokemon(2),
  bench: [pokemon(3)],
  opponentBench: [pokemon(4)],
  hand: Array.from({ length: 10 }, (_, id) => ({
    id,
    name: "Crushing Hammer",
    imageUrl: CRUSHING_HAMMER_ART,
  })),
  deckCount: 24,
  opponentDeckCount: 31,
  discardCount: 6,
  opponentDiscardCount: 3,
  discardImageUrl: CRUSHING_HAMMER_ART,
  opponentDiscardImageUrl: CRUSHING_HAMMER_ART,
  prizesRemaining: 4,
  opponentPrizesRemaining: 5,
};

describe("Board", () => {
  it("renders mirrored zones and ten hand cards", () => {
    render(<Board {...props} />);
    expect(screen.getByLabelText("Your bench")).toBeInTheDocument();
    expect(screen.getByLabelText("Opponent bench")).toBeInTheDocument();
    expect(screen.getByLabelText("4 prizes remaining")).toBeInTheDocument();
    expect(screen.getByLabelText("5 prizes remaining")).toBeInTheDocument();
    expect(within(screen.getByLabelText("Hand, 10 cards")).getAllByRole("button")).toHaveLength(10);
  });

  it("selects a playable hand card", () => {
    render(<Board {...props} />);
    const card = within(screen.getByLabelText("Hand, 10 cards")).getAllByRole("button")[0];
    fireEvent.click(card);
    expect(card).toHaveAttribute("aria-pressed", "true");
  });
});
