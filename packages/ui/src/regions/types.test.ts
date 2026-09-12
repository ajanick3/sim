import { describe, expect, it } from "vitest";
import { attachedParts, type PlayCard } from "./types";

const mon = (attached: PlayCard["attached"]): PlayCard => ({
  id: 1,
  name: "Munkidori",
  hp: 110,
  damage: 0,
  attached,
});

describe("attachedParts", () => {
  it("splits energies from the Tool, told apart by category — not two separate fields", () => {
    const { energies, tool } = attachedParts(
      mon([
        { id: 1, name: "Psychic Energy", energyType: "Psychic", category: "special-energy" },
        { id: 2, name: "Rescue Board", category: "tool" },
      ]),
    );
    expect(energies).toEqual([
      { id: 1, name: "Psychic Energy", energyType: "Psychic", category: "special-energy" },
    ]);
    expect(tool?.name).toBe("Rescue Board");
  });

  it("finds no Tool among energies only", () => {
    const { tool } = attachedParts(
      mon([{ id: 1, name: "Colorless Energy", energyType: "Colorless", category: "energy" }]),
    );
    expect(tool).toBeNull();
  });

  it("handles a Pokémon with nothing attached", () => {
    expect(attachedParts(mon(undefined))).toEqual({ energies: [], tool: null });
  });
});
