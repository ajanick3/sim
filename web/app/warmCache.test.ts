import { describe, expect, it, vi } from "vitest";
import { warmCache } from "./warmCache";

describe("warmCache", () => {
  it("fetches every url and reports progress up to the total", async () => {
    const urls = ["a", "b", "c", "d"];
    const fetchOne = vi.fn(async () => undefined);
    const progress: [number, number][] = [];
    await warmCache(urls, fetchOne, (done, total) => progress.push([done, total]));
    expect(fetchOne).toHaveBeenCalledTimes(4);
    for (const url of urls) expect(fetchOne).toHaveBeenCalledWith(url);
    expect(progress[0]).toEqual([0, 4]);
    expect(progress[progress.length - 1]).toEqual([4, 4]);
  });

  it("never fetches the same url twice, even with more workers than urls", async () => {
    const urls = ["a", "b"];
    const fetchOne = vi.fn(async () => undefined);
    await warmCache(urls, fetchOne, () => {}, 10);
    expect(fetchOne).toHaveBeenCalledTimes(2);
  });

  it("keeps going when one fetch rejects", async () => {
    const urls = ["a", "b", "c"];
    const fetchOne = vi.fn(async (url: string) => {
      if (url === "b") throw new Error("network error");
    });
    const progress: [number, number][] = [];
    await warmCache(urls, fetchOne, (done, total) => progress.push([done, total]), 1);
    expect(fetchOne).toHaveBeenCalledTimes(3);
    expect(progress[progress.length - 1]).toEqual([3, 3]);
  });

  it("reports [0, 0] and fetches nothing for an empty list", async () => {
    const fetchOne = vi.fn(async () => undefined);
    const progress: [number, number][] = [];
    await warmCache([], fetchOne, (done, total) => progress.push([done, total]));
    expect(fetchOne).not.toHaveBeenCalled();
    expect(progress).toEqual([[0, 0]]);
  });
});
