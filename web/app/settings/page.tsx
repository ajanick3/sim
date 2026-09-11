"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { loadRecent } from "../recent";
import { artUrl, loadArtIndex, type ArtIndex } from "../art";
import { catalogEntries, resolvePrint, type CatalogCard, type CatalogEntry } from "../prints";
import { loadPrintPrefs, savePrintPref } from "../printPrefs";
import { PlayingCard } from "../board/PlayingCard";

/** The catalog's buckets a player can filter by, in `CATALOG_ORDER`.
 *  Plain Energy is left out — `public/cards.json` never carries a
 *  Basic Energy, so that bucket is always empty. */
const CATEGORY_FILTERS = [
  { bucket: "pokemon", label: "Pokémon" },
  { bucket: "supporter", label: "Supporters" },
  { bucket: "item", label: "Items" },
  { bucket: "tool", label: "Tools" },
  { bucket: "stadium", label: "Stadiums" },
  { bucket: "special-energy", label: "Energy" },
] as const;

export default function SettingsPage() {
  const [recentCount, setRecentCount] = useState(0);
  const [cards, setCards] = useState<CatalogCard[]>([]);
  const [artIndex, setArtIndex] = useState<ArtIndex>({});
  const [query, setQuery] = useState("");
  const [activeBuckets, setActiveBuckets] = useState<Set<string>>(
    () => new Set(CATEGORY_FILTERS.map((f) => f.bucket)),
  );
  const [prefs, setPrefs] = useState<Record<string, string>>({});
  const [openEntry, setOpenEntry] = useState<CatalogEntry | null>(null);

  useEffect(() => {
    setRecentCount(loadRecent().length);
    setPrefs(loadPrintPrefs());
    void fetch("/cards.json")
      .then((r) => (r.ok ? r.json() : { cards: [] }))
      .then((data: { cards: CatalogCard[] }) => setCards(data.cards ?? []))
      .catch(() => setCards([]));
    void loadArtIndex().then(setArtIndex);
  }, []);

  const entries = useMemo(() => catalogEntries(cards), [cards]);
  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    return entries.filter(
      (e) => activeBuckets.has(e.bucket) && (!q || e.name.toLowerCase().includes(q)),
    );
  }, [entries, query, activeBuckets]);

  const toggleBucket = (bucket: string) => {
    setActiveBuckets((prev) => {
      const next = new Set(prev);
      if (next.has(bucket)) next.delete(bucket);
      else next.add(bucket);
      return next;
    });
  };

  const clearRecent = () => {
    try {
      localStorage.removeItem("sim.recent");
    } catch {
      // storage disabled — nothing to clear
    }
    setRecentCount(0);
  };

  return (
    <main className="px-4 py-8">
      <div className="mx-auto max-w-[560px]">
        <header className="flex items-baseline justify-between">
          <h1 className="m-0 text-[20px]">Settings</h1>
          <Link href="/" className="text-[13px] text-dim no-underline hover:text-text">
            Decks
          </Link>
        </header>

        <section className="mt-6 flex flex-col gap-2">
          <h2 className="m-0 text-[13px] uppercase tracking-widest text-dim">Recent games</h2>
          <p className="m-0 text-[13px] text-dim">
            {recentCount} saved in this browser. They are keyed by seed and decks, and hold the
            latest position of each.
          </p>
          <button
            onClick={clearRecent}
            disabled={recentCount === 0}
            className="w-fit rounded-md border border-edge px-3 py-1.5 text-[13px] hover:border-accent disabled:opacity-40"
          >
            Clear recent games
          </button>
        </section>

        <section className="mt-8 flex flex-col gap-1">
          <h2 className="m-0 text-[13px] uppercase tracking-widest text-dim">Motion</h2>
          <p className="m-0 text-[13px] text-dim">
            Card and coin animations follow your system “reduce motion” setting.
          </p>
        </section>
      </div>

      <section className="mt-8 flex flex-col gap-2">
        <div className="mx-auto flex max-w-[560px] flex-col gap-2">
          <h2 className="m-0 text-[13px] uppercase tracking-widest text-dim">Card prints</h2>
          <p className="m-0 text-[13px] text-dim">
            Pick which art a card shows. A card with only one known print can’t be changed.
          </p>
          <input
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search cards…"
            className="rounded-md border border-edge bg-transparent px-3 py-1.5 text-[13px] outline-none focus:border-accent"
          />
        </div>
        <div className="mx-auto flex max-w-[560px] flex-wrap items-center gap-2">
          {CATEGORY_FILTERS.map(({ bucket, label }) => {
            const active = activeBuckets.has(bucket);
            return (
              <button
                key={bucket}
                onClick={() => toggleBucket(bucket)}
                aria-pressed={active}
                className={`rounded-full border px-3 py-1 text-[12px] transition-colors ${
                  active
                    ? "border-accent bg-accent text-bg font-semibold"
                    : "border-edge text-dim opacity-50 hover:opacity-80"
                }`}
              >
                {label}
              </button>
            );
          })}
          <span className="ml-auto text-[12px] text-dim">
            {filtered.length} of {entries.length}
          </span>
        </div>
        <div className="mt-1 flex flex-wrap justify-center gap-3">
          {filtered.map((entry) => {
            const single = entry.prints.length === 1;
            const printId = resolvePrint(entry.name, entry.prints, prefs, null);
            return (
              <PlayingCard
                key={entry.name}
                size="picker"
                crop="full"
                src={artUrl(artIndex, printId)}
                name={entry.name}
                dimmed={single}
                disabled={single}
                interactive={!single}
                onClick={single ? undefined : () => setOpenEntry(entry)}
                className={
                  single
                    ? "cursor-default"
                    : "cursor-pointer transition-transform hover:scale-110 active:scale-110 active:shadow-card-raised"
                }
              />
            );
          })}
        </div>
      </section>

      {openEntry && (
        <div
          className="fixed inset-0 z-50 flex items-end justify-center bg-black/50 p-3 sm:items-center"
          onClick={() => setOpenEntry(null)}
        >
          <div
            className="flex max-h-[80vh] w-full max-w-lg flex-col overflow-hidden rounded-lg border border-edge bg-bg"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center justify-between border-b border-edge p-3">
              <h3 className="m-0 text-[14px] font-semibold">{openEntry.name}</h3>
              <button
                onClick={() => setOpenEntry(null)}
                className="text-[13px] text-dim hover:text-text"
              >
                Close
              </button>
            </div>
            <div className="flex flex-wrap justify-center gap-3 overflow-y-auto p-3">
              {openEntry.prints.map((printId) => (
                <PlayingCard
                  key={printId}
                  size="picker"
                  crop="full"
                  src={artUrl(artIndex, printId)}
                  name={openEntry.name}
                  interactive
                  onClick={() => {
                    setPrefs(savePrintPref(openEntry.name, printId));
                    setOpenEntry(null);
                  }}
                  className="cursor-pointer transition-transform hover:scale-110 active:scale-110 active:shadow-card-raised"
                />
              ))}
            </div>
          </div>
        </div>
      )}
    </main>
  );
}
