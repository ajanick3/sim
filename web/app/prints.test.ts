import { describe, expect, it } from "vitest";
import {
  bucketOf,
  catalogEntries,
  defaultPrint,
  identityOf,
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

describe("identityOf", () => {
  it("is the same for two prints that share every rules-relevant field", () => {
    const a = card({
      id: "me01-113",
      name: "Rare Candy",
      category: "Trainer",
      trainerType: "Item",
      effect: "Evolve a Basic.",
    });
    const b = card({
      id: "sv06-100",
      name: "Rare Candy",
      category: "Trainer",
      trainerType: "Item",
      effect: "Evolve a Basic.",
    });
    expect(identityOf(a)).toBe(identityOf(b));
  });

  it("differs for two cards that only share a name — TCGdex relates cards by name alone, and two unrelated Pokémon prints can carry it", () => {
    const a = card({
      id: "me01-054",
      name: "Abra",
      category: "Pokemon",
      hp: 50,
      attacks: [{ name: "Teleportation Attack", damage: 10 }],
    });
    const b = card({
      id: "sv06-080",
      name: "Abra",
      category: "Pokemon",
      hp: 40,
      attacks: [{ name: "Beam", damage: 10 }],
    });
    expect(identityOf(a)).not.toBe(identityOf(b));
  });

  it("is stable across different id/set/localId — those never enter the signature", () => {
    const a = card({ id: "me01-054", name: "Abra", category: "Pokemon", hp: 50 });
    const b = card({ id: "sv06-080", name: "Abra", category: "Pokemon", hp: 50 });
    expect(identityOf(a)).toBe(identityOf(b));
  });
});

describe("catalogEntries", () => {
  it("groups by identity and orders by category then name", () => {
    const cards: CatalogCard[] = [
      card({ id: "e1", name: "Ignition Energy", category: "Energy" }),
      card({ id: "p2", name: "Zubat", category: "Pokemon" }),
      card({ id: "p1a", name: "Bulbasaur", category: "Pokemon", hp: 60 }),
      card({ id: "p1b", name: "Bulbasaur", category: "Pokemon", hp: 60 }),
      card({ id: "t1", name: "Rare Candy", category: "Trainer", trainerType: "Item" }),
    ];
    expect(catalogEntries(cards).map((e) => e.name)).toEqual([
      "Bulbasaur",
      "Zubat",
      "Rare Candy",
      "Ignition Energy",
    ]);
  });

  it("carries every print id sharing one identity, sorted", () => {
    const cards: CatalogCard[] = [
      card({ id: "me01-003", name: "Mega Venusaur ex", category: "Pokemon", hp: 340 }),
      card({ id: "mep-037", name: "Mega Venusaur ex", category: "Pokemon", hp: 340 }),
    ];
    const entries = catalogEntries(cards);
    expect(entries).toHaveLength(1);
    expect(entries[0]).toMatchObject({
      name: "Mega Venusaur ex",
      bucket: "pokemon",
      prints: ["me01-003", "mep-037"],
    });
  });

  it("keeps two same-named but different cards as two separate entries", () => {
    const cards: CatalogCard[] = [
      card({ id: "me01-054", name: "Abra", category: "Pokemon", hp: 50 }),
      card({ id: "sv06-080", name: "Abra", category: "Pokemon", hp: 40 }),
    ];
    const entries = catalogEntries(cards);
    expect(entries).toHaveLength(2);
    expect(entries.map((e) => e.name)).toEqual(["Abra", "Abra"]);
    expect(entries[0].prints).toHaveLength(1);
    expect(entries[1].prints).toHaveLength(1);
    expect(entries[0].key).not.toBe(entries[1].key);
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
    expect(resolvePrint("key-1", prints, { "key-1": "mep-037" }, "me01-003")).toBe("mep-037");
  });

  it("falls back to the context print when no preference is stored", () => {
    expect(resolvePrint("key-1", prints, {}, "mep-037")).toBe("mep-037");
  });

  it("falls back to the deterministic first print when neither is set", () => {
    expect(resolvePrint("key-1", prints, {}, null)).toBe("me01-003");
  });

  it("ignores a stored preference that no longer names a known print", () => {
    expect(resolvePrint("key-1", prints, { "key-1": "stale-999" }, "mep-037")).toBe("mep-037");
  });
});
