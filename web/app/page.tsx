"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { DEFAULT_DECKS, loadDeckIndex, type DeckEntry } from "./decks";
import { loadRecent, type RecentGame } from "./recent";

export default function Page() {
  const router = useRouter();
  const [decks, setDecks] = useState<DeckEntry[]>([]);
  const [a, setA] = useState(DEFAULT_DECKS.a);
  const [b, setB] = useState(DEFAULT_DECKS.b);
  const [recent, setRecent] = useState<RecentGame[]>([]);

  useEffect(() => {
    void loadDeckIndex().then(setDecks);
    setRecent(loadRecent());
  }, []);

  const play = () => router.push(`/play?a=${a}&b=${b}`);
  const shuffle = () => {
    if (decks.length < 2) return;
    const i = Math.floor(Math.random() * decks.length);
    let j = Math.floor(Math.random() * decks.length);
    if (j === i) j = (j + 1) % decks.length;
    router.push(`/play?a=${decks[i].key}&b=${decks[j].key}`);
  };

  const pick = (side: "a" | "b", value: string) => (side === "a" ? setA(value) : setB(value));

  return (
    <main className="mx-auto max-w-[720px] px-4 py-8">
      <header className="flex items-baseline justify-between">
        <h1 className="m-0 text-[20px]">sim</h1>
        <Link href="/settings" className="text-[13px] text-dim no-underline hover:text-text">
          Settings
        </Link>
      </header>
      <p className="mt-1 text-[13px] text-dim">
        Pick a deck for each side from the 2026 Worlds field.
      </p>

      <div className="mt-6 grid gap-4 sm:grid-cols-2">
        <DeckColumn label="You" side="a" value={a} decks={decks} onPick={pick} />
        <DeckColumn label="Opponent" side="b" value={b} decks={decks} onPick={pick} />
      </div>

      <div className="mt-5 flex gap-2">
        <button
          onClick={play}
          className="rounded-md border-accent bg-accent px-5 py-2 font-bold text-black"
        >
          Play
        </button>
        <button
          onClick={shuffle}
          className="rounded-md border border-edge px-4 py-2 text-[13px] hover:border-accent"
        >
          Random matchup
        </button>
      </div>

      {recent.length > 0 && (
        <section className="mt-9">
          <h2 className="m-0 text-[13px] uppercase tracking-widest text-dim">Recent games</h2>
          <ul className="mt-2 flex flex-col gap-1">
            {recent.map((r) => (
              <li key={r.g}>
                <Link
                  href={`/play?g=${r.g}`}
                  className="flex items-center justify-between rounded-md border border-edge px-3 py-2 text-[13px] no-underline hover:border-accent"
                >
                  <span>
                    {name(r.a)} <span className="text-dim">vs</span> {name(r.b)}
                  </span>
                  <span className="text-[12px] text-dim">
                    turn {r.turn} · {ago(r.at)}
                  </span>
                </Link>
              </li>
            ))}
          </ul>
        </section>
      )}
    </main>
  );
}

function DeckColumn({
  label,
  side,
  value,
  decks,
  onPick,
}: {
  label: string;
  side: "a" | "b";
  value: string;
  decks: DeckEntry[];
  onPick: (side: "a" | "b", value: string) => void;
}) {
  return (
    <label className="flex flex-col gap-1">
      <span className="text-[12px] uppercase tracking-widest text-dim">{label}</span>
      <select
        value={value}
        onChange={(e) => onPick(side, e.target.value)}
        className="rounded-md border border-edge bg-panel px-2 py-2 text-[13px]"
      >
        {decks.map((d) => (
          <option key={d.key} value={d.key}>
            {d.player}
            {d.headline ? ` — ${d.headline}` : ""}
          </option>
        ))}
      </select>
    </label>
  );
}

const name = (key: string) =>
  key
    .replace(/^\d+-/, "")
    .replace(/-/g, " ")
    .replace(/\b\w/g, (c) => c.toUpperCase());

function ago(at: number): string {
  const s = Math.max(0, Math.round((Date.now() - at) / 1000));
  if (s < 60) return "just now";
  const m = Math.round(s / 60);
  if (m < 60) return `${m}m ago`;
  const h = Math.round(m / 60);
  if (h < 24) return `${h}h ago`;
  return `${Math.round(h / 24)}d ago`;
}
