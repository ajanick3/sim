import { describe, expect, it } from "vitest";
import {
  decodeRecipe,
  encodeRecipe,
  newRecipe,
  readRecipeParam,
  writeRecipeParam,
  type Recipe,
} from "./recipe";

const sample: Recipe = {
  v: 1,
  seed: 123456,
  a: "dragapult",
  b: "alakazam",
  moves: [0, 1, 0, 3, 2],
};

describe("encode / decode", () => {
  it("round-trips a recipe", () => {
    expect(decodeRecipe(encodeRecipe(sample))).toEqual(sample);
  });

  it("round-trips a recipe with no moves yet", () => {
    const fresh = newRecipe(7, "dragapult", "alakazam");
    expect(decodeRecipe(encodeRecipe(fresh))).toEqual(fresh);
  });

  it("produces a URL-safe string (no +, /, =)", () => {
    const s = encodeRecipe(sample);
    expect(s).not.toMatch(/[+/=]/);
  });

  it("returns null for a non-base64 string", () => {
    expect(decodeRecipe("!!!not base64!!!")).toBeNull();
  });

  it("returns null for base64 that is not JSON", () => {
    expect(decodeRecipe(btoa("hello"))).toBeNull();
  });

  it("returns null for JSON of the wrong shape", () => {
    expect(decodeRecipe(btoaUrl(JSON.stringify({ seed: 1 })))).toBeNull();
  });

  it("returns null for an unknown recipe version", () => {
    expect(decodeRecipe(encodeRecipe({ ...sample, v: 2 as 1 }))).toBeNull();
  });

  it("returns null when moves is not an array of numbers", () => {
    const bad = btoaUrl(JSON.stringify({ ...sample, moves: ["x"] }));
    expect(decodeRecipe(bad)).toBeNull();
  });
});

describe("newRecipe", () => {
  it("starts with the given seed and decks and no moves", () => {
    expect(newRecipe(42, "a", "b")).toEqual({
      v: 1,
      seed: 42,
      a: "a",
      b: "b",
      moves: [],
    });
  });
});

describe("readRecipeParam / writeRecipeParam", () => {
  it("reads a recipe back out of a query string it wrote", () => {
    const search = writeRecipeParam(sample);
    expect(search.startsWith("?g=")).toBe(true);
    expect(readRecipeParam(search)).toEqual(sample);
  });

  it("returns null when there is no g param", () => {
    expect(readRecipeParam("")).toBeNull();
    expect(readRecipeParam("?seed=1")).toBeNull();
  });

  it("returns null when the g param is malformed", () => {
    expect(readRecipeParam("?g=notarecipe")).toBeNull();
  });
});

// A minimal URL-safe base64 for building test fixtures.
function btoaUrl(s: string): string {
  return btoa(s).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}
