"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import {
  DEFAULT_DECKS,
  encodePastedDeck,
  isPastedDeck,
  loadDeckIndex,
  type DeckEntry,
} from "./decks";
import { DeckPicker } from "./DeckPicker";
import { loadRecent, type RecentGame } from "./recent";

type SideMode = "field" | "paste";

export default function Page() {
  const router = useRouter();
  const [decks, setDecks] = useState<DeckEntry[]>([]);
  const [a, setA] = useState(DEFAULT_DECKS.a);
  const [b, setB] = useState(DEFAULT_DECKS.b);
  const [aMode, setAMode] = useState<SideMode>("field");
  const [bMode, setBMode] = useState<SideMode>("field");
  const [aPaste, setAPaste] = useState("");
  const [bPaste, setBPaste] = useState("");
  const [recent, setRecent] = useState<RecentGame[]>([]);

  useEffect(() => {
    void loadDeckIndex().then(setDecks);
    setRecent(loadRecent());
  }, []);

  const effective = (mode: SideMode, key: string, paste: string) =>
    mode === "paste" ? encodePastedDeck(paste) : key;
  const canPlay =
    (aMode === "field" || aPaste.trim() !== "") && (bMode === "field" || bPaste.trim() !== "");
  const play = () => {
    if (!canPlay) return;
    const ea = encodeURIComponent(effective(aMode, a, aPaste));
    const eb = encodeURIComponent(effective(bMode, b, bPaste));
    router.push(`/play?a=${ea}&b=${eb}`);
  };
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
        <DeckColumn
          label="You"
          side="a"
          value={a}
          decks={decks}
          onPick={pick}
          mode={aMode}
          onModeChange={setAMode}
          pasteValue={aPaste}
          onPasteChange={setAPaste}
        />
        <DeckColumn
          label="Opponent"
          side="b"
          value={b}
          decks={decks}
          onPick={pick}
          mode={bMode}
          onModeChange={setBMode}
          pasteValue={bPaste}
          onPasteChange={setBPaste}
        />
      </div>

      <div className="mt-5 flex gap-2">
        <button
          onClick={play}
          disabled={!canPlay}
          className="rounded-md border-accent bg-accent px-5 py-2 font-bold text-black disabled:opacity-50"
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
  mode,
  onModeChange,
  pasteValue,
  onPasteChange,
}: {
  label: string;
  side: "a" | "b";
  value: string;
  decks: DeckEntry[];
  onPick: (side: "a" | "b", value: string) => void;
  mode: SideMode;
  onModeChange: (mode: SideMode) => void;
  pasteValue: string;
  onPasteChange: (text: string) => void;
}) {
  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-baseline justify-between">
        <span className="text-[12px] uppercase tracking-widest text-dim">{label}</span>
        <button
          type="button"
          onClick={() => onModeChange(mode === "field" ? "paste" : "field")}
          className="text-[11px] text-dim underline hover:text-text"
        >
          {mode === "field" ? "Paste a decklist instead" : "Pick from the field instead"}
        </button>
      </div>
      {mode === "field" ? (
        <DeckPicker decks={decks} value={value} onPick={(key) => onPick(side, key)} />
      ) : (
        <textarea
          value={pasteValue}
          onChange={(e) => onPasteChange(e.target.value)}
          placeholder={"Pokémon: 18\n4 Dreepy TWM 128\n…"}
          rows={6}
          className="rounded-md border border-edge bg-panel px-2 py-2 font-mono text-[12px]"
        />
      )}
    </div>
  );
}

const name = (key: string) =>
  isPastedDeck(key)
    ? "Pasted deck"
    : key
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
