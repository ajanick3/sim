import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { PokeBall } from "./PokeBall";

describe("PokeBall", () => {
  it("reads as remaining by default", () => {
    render(<PokeBall />);
    expect(screen.getByRole("img", { name: "Prize remaining" })).toBeInTheDocument();
  });

  it("reads as taken, and dims, when taken", () => {
    render(<PokeBall taken />);
    const el = screen.getByRole("img", { name: "Prize taken" });
    expect(el).toBeInTheDocument();
    expect(el.style.opacity).not.toBe("1");
  });
});
