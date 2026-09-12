import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { Card } from "./Card";

describe("Card", () => {
  it("shows the plain name when there is no art", () => {
    render(<Card size="hand" name="Ultra Ball" />);
    expect(screen.getByText("Ultra Ball")).toBeInTheDocument();
  });

  it("draws the art as a background instead of the name fallback when src is given", () => {
    render(<Card size="hand" name="Munkidori" src="https://example.com/munkidori.png" />);
    expect(screen.queryByText("Munkidori")).not.toBeInTheDocument();
    const card = screen.getByRole("img", { name: "Munkidori" });
    // A CSS background, not an <img> — so children (a CardOverlay) can
    // sit on top of it via plain flexbox, no position: absolute
    // anywhere in the stack.
    expect(card.tagName).not.toBe("IMG");
    expect(card).toHaveStyle({ backgroundImage: 'url(https://example.com/munkidori.png)' });
  });

  it("fires onClick when tapped", () => {
    const onClick = vi.fn();
    render(<Card size="bench" name="Dreepy" onClick={onClick} />);
    fireEvent.click(screen.getByRole("button"));
    expect(onClick).toHaveBeenCalledOnce();
  });

  it("renders as a plain, non-interactive element with no onClick", () => {
    render(<Card size="bench" name="Dreepy" />);
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
  });
});
