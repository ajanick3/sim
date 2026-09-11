import { describe, expect, it } from "vitest";
import {
  bucketOf,
  catalogEntries,
  defaultPrint,
  printsFor,
  resolvePrint,
  type CatalogCard,
} from "./prints";

const card = (
  over: Partial<CatalogCard> & Pick<CatalogCard, "id" | "name" | "category">,
): CatalogCard => ({
  ...over,
});

describe("bucketOf", () => {
  it("reads Pokemon as the pokemon bucket", () => {
    expect(bucketOf(card({ id: "p1", name: "Bulbasaur", category: "Pokemon" }))).toBe("pokemon");
  });

  it("reads a Trainer by its trainerType", () => {
    expect(
      bucketOf(card({ id: "t1", name: "X", category: "Trainer", trainerType: "Supporter" })),
    ).toBe("supporter");
    expect(bucketOf(card({ id: "t2", name: "X", category: "Trainer", trainerType: "Item" }))).toBe(
      "item",
    );
    expect(bucketOf(card({ id: "t3", name: "X", category: "Trainer", trainerType: "Tool" }))).toBe(
      "tool",
    );
    expect(
      bucketOf(card({ id: "t4", name: "X", category: "Trainer", trainerType: "Stadium" })),
    ).toBe("stadium");
  });

  it("reads Energy as special-energy — basic Energy never appears in this catalog", () => {
    expect(bucketOf(card({ id: "e1", name: "Ignition Energy", category: "Energy" }))).toBe(
      "special-energy",
    );
  });
});

describe("printsFor", () => {
  const cards: CatalogCard[] = [
    card({ id: "me01-003", name: "Mega Venusaur ex", category: "Pokemon" }),
    card({ id: "me01-001", name: "Bulbasaur", category: "Pokemon" }),
    card({ id: "mep-037", name: "Mega Venusaur ex", category: "Pokemon" }),
  ];

  it("collects every print sharing a name, sorted", () => {
    expect(printsFor(cards, "Mega Venusaur ex")).toEqual(["me01-003", "mep-037"]);
  });

  it("is a single-element array for a name with one print", () => {
    expect(printsFor(cards, "Bulbasaur")).toEqual(["me01-001"]);
  });

  it("is empty for a name not in the catalog", () => {
    expect(printsFor(cards, "Nobody")).toEqual([]);
  });
});

describe("catalogEntries", () => {
  it("groups by name and orders by category then name", () => {
    const cards: CatalogCard[] = [
      card({ id: "e1", name: "Ignition Energy", category: "Energy" }),
      card({ id: "p2", name: "Zubat", category: "Pokemon" }),
      card({ id: "p1a", name: "Bulbasaur", category: "Pokemon" }),
      card({ id: "p1b", name: "Bulbasaur", category: "Pokemon" }),
      card({ id: "t1", name: "Rare Candy", category: "Trainer", trainerType: "Item" }),
    ];
    expect(catalogEntries(cards).map((e) => e.name)).toEqual([
      "Bulbasaur",
      "Zubat",
      "Rare Candy",
      "Ignition Energy",
    ]);
  });

  it("carries every print id for a name, sorted", () => {
    const cards: CatalogCard[] = [
      card({ id: "me01-003", name: "Mega Venusaur ex", category: "Pokemon" }),
      card({ id: "mep-037", name: "Mega Venusaur ex", category: "Pokemon" }),
    ];
    expect(catalogEntries(cards)).toEqual([
      { name: "Mega Venusaur ex", bucket: "pokemon", prints: ["me01-003", "mep-037"] },
    ]);
  });
});

describe("defaultPrint", () => {
  it("is the first print id, sorted", () => {
    expect(defaultPrint(["mep-037", "me01-003"])).toBe("me01-003");
  });
});

describe("resolvePrint", () => {
  const prints = ["me01-003", "mep-037"];

  it("prefers the player's stored preference", () => {
    expect(
      resolvePrint("Mega Venusaur ex", prints, { "Mega Venusaur ex": "mep-037" }, "me01-003"),
    ).toBe("mep-037");
  });

  it("falls back to the context print when no preference is stored", () => {
    expect(resolvePrint("Mega Venusaur ex", prints, {}, "mep-037")).toBe("mep-037");
  });

  it("falls back to the deterministic first print when neither is set", () => {
    expect(resolvePrint("Mega Venusaur ex", prints, {}, null)).toBe("me01-003");
  });

  it("ignores a stored preference that no longer names a known print", () => {
    expect(
      resolvePrint("Mega Venusaur ex", prints, { "Mega Venusaur ex": "stale-999" }, "mep-037"),
    ).toBe("mep-037");
  });
});
