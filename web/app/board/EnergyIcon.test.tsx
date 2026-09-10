import { render } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import {
  basicEnergyKinds,
  EnergyIcon,
  energyDefinitions,
  energyKind,
  specialEnergyKinds,
} from "./EnergyIcon";

describe("EnergyIcon", () => {
  it("labels itself as an image from the kind's definition", () => {
    const { container } = render(<EnergyIcon kind="fire" />);
    const svg = container.querySelector("svg")!;
    expect(svg.getAttribute("role")).toBe("img");
    expect(svg.getAttribute("aria-label")).toBe(energyDefinitions.fire.label);
  });

  it("takes a label override", () => {
    const { container } = render(<EnergyIcon kind="water" label="Ocean" />);
    expect(container.querySelector("svg")!.getAttribute("aria-label")).toBe("Ocean");
  });

  it("hides from assistive tech when decorative", () => {
    const { container } = render(<EnergyIcon kind="metal" decorative />);
    const svg = container.querySelector("svg")!;
    expect(svg.getAttribute("aria-hidden")).toBe("true");
    expect(svg.getAttribute("role")).toBeNull();
    expect(svg.getAttribute("aria-label")).toBeNull();
  });

  it("sizes the svg", () => {
    const { container } = render(<EnergyIcon kind="grass" size={24} />);
    const svg = container.querySelector("svg")!;
    expect(svg.getAttribute("width")).toBe("24");
    expect(svg.getAttribute("height")).toBe("24");
  });

  it("takes a fill override", () => {
    const { container } = render(<EnergyIcon kind="psychic" color="#123456" />);
    expect(container.querySelector("svg")!.style.getPropertyValue("--energy-fill")).toBe("#123456");
  });

  it("draws a glyph for every basic and special kind", () => {
    for (const kind of [...basicEnergyKinds, ...specialEnergyKinds]) {
      const { container } = render(<EnergyIcon kind={kind} />);
      // circle backdrop + at least one glyph shape
      expect(container.querySelectorAll("svg path, svg circle").length).toBeGreaterThan(1);
    }
  });
});

describe("energyKind", () => {
  it("maps the eight wire Energy types to icon kinds", () => {
    expect(energyKind("Fire")).toBe("fire");
    expect(energyKind("Lightning")).toBe("lightning");
    expect(energyKind("Darkness")).toBe("darkness");
  });

  it("is null for a type it does not draw, or for nothing", () => {
    expect(energyKind("Colorless")).toBeNull();
    expect(energyKind("Fairy")).toBeNull();
    expect(energyKind(null)).toBeNull();
    expect(energyKind(undefined)).toBeNull();
  });
});
