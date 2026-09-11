"use client";

import { useEffect, useMemo, useState } from "react";
import Link from "next/link";
import { loadRecent } from "../recent";
import { artUrl, loadArtIndex, type ArtIndex } from "../art";
import { catalogEntries, defaultPrint, type CatalogCard } from "../prints";
import { PlayingCard } from "../board/PlayingCard";

export default function SettingsPage() {
  const [recentCount, setRecentCount] = useState(0);
  const [cards, setCards] = useState<CatalogCard[]>([]);
  const [artIndex, setArtIndex] = useState<ArtIndex>({});
  const [query, setQuery] = useState("");

  useEffect(() => {
    setRecentCount(loadRecent().length);
    void fetch("/cards.json")
      .then((r) => (r.ok ? r.json() : { cards: [] }))
      .then((data: { cards: CatalogCard[] }) => setCards(data.cards ?? []))
      .catch(() => setCards([]));
    void loadArtIndex().then(setArtIndex);
  }, []);

  const entries = useMemo(() => catalogEntries(cards), [cards]);
  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    return q ? entries.filter((e) => e.name.toLowerCase().includes(q)) : entries;
  }, [entries, query]);

  const clearRecent = () => {
    try {
      localStorage.removeItem("sim.recent");
    } catch {
      // storage disabled — nothing to clear
    }
    setRecentCount(0);
  };

  return (
    <main className="mx-auto max-w-[560px] px-4 py-8">
      <header className="flex items-baseline justify-between">
        <h1 className="m-0 text-[20px]">Settings</h1>
        <Link href="/" className="text-[13px] text-dim no-underline hover:text-text">
          Decks
        </Link>
      </header>

      <section className="mt-6 flex flex-col gap-2">
        <h2 className="m-0 text-[13px] uppercase tracking-widest text-dim">Recent games</h2>
        <p className="m-0 text-[13px] text-dim">
          {recentCount} saved in this browser. They are keyed by seed and decks, and hold the latest
          position of each.
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

      <section className="mt-8 flex flex-col gap-2">
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
        <div className="mt-1 flex flex-wrap gap-3">
          {filtered.map((entry) => {
            const single = entry.prints.length === 1;
            const printId = defaultPrint(entry.prints);
            return (
              <PlayingCard
                key={entry.name}
                size="picker"
                crop="full"
                src={artUrl(artIndex, printId)}
                name={entry.name}
                dimmed={single}
                disabled={single}
                className={single ? "cursor-default" : "cursor-pointer"}
              />
            );
          })}
        </div>
      </section>
    </main>
  );
}
