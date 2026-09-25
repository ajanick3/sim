import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DREEPY_ART, CRUSHING_HAMMER_ART } from "../../_fixtures/cardArt";
import { PokemonCard } from "./PokemonCard";
import { CardPile } from "./CardPile";
import { HandGrid } from "../regions/HandGrid";
import { PrizeZone } from "../regions/PrizeZone";

describe("composed game pieces", () => {
  it("uses a Pokemon card for selectable board positions", () => {
    const select = vi.fn();
    render(
      <PokemonCard name="Dreepy" imageUrl={DREEPY_ART} hp={70} damage={30} onSelect={select} />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Dreepy, 70 HP remaining" }));
    expect(select).toHaveBeenCalledOnce();
    expect(screen.getByLabelText("30 damage")).toBeInTheDocument();
  });
  it("shows only the numeric health value", () => {
    render(<PokemonCard name="Dreepy" imageUrl={DREEPY_ART} hp={70} />);
    expect(screen.getByLabelText("70 HP remaining")).toHaveTextContent("70");
    expect(screen.getByLabelText("70 HP remaining")).not.toHaveTextContent("HP");
  });
  it("names and counts both pile types", () => {
    render(
      <>
        <CardPile kind="deck" count={24} />
        <CardPile
          kind="discard"
          count={6}
          topCardName="Crushing Hammer"
          topCardImageUrl={CRUSHING_HAMMER_ART}
        />
      </>,
    );
    expect(screen.getByRole("group", { name: "Deck, 24 cards" })).toBeInTheDocument();
    expect(screen.getByRole("group", { name: "Discard pile, 6 cards" })).toBeInTheDocument();
  });
  it("renders exactly six prize markers without a visible count", () => {
    render(<PrizeZone remaining={4} />);
    expect(screen.getAllByRole("img")).toHaveLength(6);
    expect(screen.queryByText("4")).not.toBeInTheDocument();
  });
  it("lays cards into the hand and blocks unavailable cards", () => {
    const select = vi.fn();
    render(
      <HandGrid
        cards={[
          { id: 1, name: "Dreepy", imageUrl: DREEPY_ART },
          { id: 2, name: "Crushing Hammer", imageUrl: CRUSHING_HAMMER_ART, playable: false },
        ]}
        onSelect={select}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Crushing Hammer" }));
    expect(select).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Dreepy" }));
    expect(select).toHaveBeenCalledWith(1);
  });
});
