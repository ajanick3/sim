import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { BenchRegion } from "./BenchRegion";
import type { PlayCard } from "./types";

const mon = (id: number, name: string): PlayCard => ({ id, name, hp: 70, damage: 0 });

describe("BenchRegion", () => {
  it("pads a short Bench out to five slots", () => {
    render(<BenchRegion mons={[mon(1, "Dreepy"), mon(2, "Toxel")]} />);
    // Named cards, plus three empty pads named "empty".
    expect(screen.getAllByText(/Dreepy|Toxel|empty/)).toHaveLength(5);
  });

  it("keeps every Pokémon when a raised limit holds more than five", () => {
    const six = Array.from({ length: 6 }, (_, i) => mon(i + 1, `Mon${i + 1}`));
    render(<BenchRegion mons={six} />);
    for (const m of six) {
      expect(screen.getByText(m.name)).toBeInTheDocument();
    }
  });

  it("calls onSelect with the tapped Pokémon's id, not an empty slot", () => {
    const onSelect = vi.fn();
    render(<BenchRegion mons={[mon(7, "Bidoof")]} onSelect={onSelect} />);
    fireEvent.click(screen.getByRole("button", { name: /Bidoof/ }));
    expect(onSelect).toHaveBeenCalledWith(7);
    // The four empty pads carry no click handler at all.
    expect(screen.queryAllByRole("button")).toHaveLength(1);
  });
});
