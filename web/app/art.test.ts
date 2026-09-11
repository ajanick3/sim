import { describe, expect, it } from "vitest";
import { artUrl } from "./art";

describe("artUrl", () => {
  const index = { "sv6-160": "https://assets.tcgdex.net/en/sv/sv06/160" };

  it("defaults to the low-quality suffix", () => {
    expect(artUrl(index, "sv6-160")).toBe("https://assets.tcgdex.net/en/sv/sv06/160/low.webp");
  });

  it("takes the high-quality suffix when asked", () => {
    expect(artUrl(index, "sv6-160", "high")).toBe(
      "https://assets.tcgdex.net/en/sv/sv06/160/high.webp",
    );
  });

  it("returns null for a print id with no art", () => {
    expect(artUrl(index, "me05-999")).toBeNull();
    expect(artUrl({}, "sv6-160")).toBeNull();
  });
});
