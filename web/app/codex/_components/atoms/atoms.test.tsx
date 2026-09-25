import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { CardFrame } from "./CardFrame";
import { CardArt } from "./CardArt";
import { DREEPY_ART } from "../../_fixtures/cardArt";
import { CardSlot } from "./CardSlot";
import { IconButton } from "./IconButton";
import { DamageCounter } from "./DamageCounter";

describe("card interaction boundaries", () => {
  it("exposes selection and blocks unavailable cards", () => {
    const act = vi.fn();
    render(
      <>
        <CardFrame label="Selected card" interactive state="selected" onClick={act}>
          Art
        </CardFrame>
        <CardFrame label="Unavailable card" interactive state="unavailable" onClick={act}>
          Art
        </CardFrame>
      </>,
    );
    expect(screen.getByRole("button", { name: "Selected card" })).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    fireEvent.click(screen.getByRole("button", { name: "Unavailable card" }));
    expect(act).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Selected card" }));
    expect(act).toHaveBeenCalledTimes(1);
  });

  it("keeps passive cards out of the tab order and has no redundant image name", () => {
    render(
      <CardFrame label="Opponent card">
        <CardArt src={DREEPY_ART} />
      </CardFrame>,
    );
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
    expect(screen.getByRole("group", { name: "Opponent card" })).not.toHaveAttribute("tabindex");
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
  });

  it("recovers from an image error when the source changes", () => {
    const { container, rerender } = render(<CardArt src="/broken.png" />);
    fireEvent.error(container.querySelector("img")!);
    expect(screen.getByText("Image unavailable")).toBeInTheDocument();
    rerender(<CardArt src="/working.png" />);
    expect(container.querySelector("img")).toHaveAttribute("src", "/working.png");
  });

  it("allows placement only in enabled target slots", () => {
    const place = vi.fn();
    render(
      <>
        <CardSlot label="Empty slot" />
        <CardSlot label="Legal target" onPlace={place} />
        <CardSlot label="Busy target" onPlace={place} disabled />
      </>,
    );
    expect(screen.getByRole("img", { name: "Empty slot" })).not.toHaveAttribute("tabindex");
    fireEvent.click(screen.getByRole("button", { name: "Busy target" }));
    expect(place).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Legal target" }));
    expect(place).toHaveBeenCalledTimes(1);
  });

  it("names icon buttons and prevents form submission", () => {
    render(<IconButton label="Close dialog">×</IconButton>);
    expect(screen.getByRole("button", { name: "Close dialog" })).toHaveAttribute("type", "button");
  });

  it("omits damage when none has been applied", () => {
    const { container } = render(<DamageCounter damage={0} />);
    expect(container).toBeEmptyDOMElement();
  });
});
