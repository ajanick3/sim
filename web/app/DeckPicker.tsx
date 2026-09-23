"use client";

// A searchable stand-in for a native <select> over the 2026 Worlds field —
// over 140 decks is too many to scan by scrolling a plain dropdown. Opens
// as the same bottom-sheet pattern the settings page's print picker
// already uses, so the two searchable lists in this app look and behave
// alike.

import { useEffect, useMemo, useRef, useState } from "react";
import type { DeckEntry } from "./decks";

function matches(deck: DeckEntry, query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  return deck.player.toLowerCase().includes(q) || deck.headline.toLowerCase().includes(q);
}

export function DeckPicker({
  decks,
  value,
  onPick,
}: {
  decks: DeckEntry[];
  value: string;
  onPick: (key: string) => void;
}) {
  const [open, setOpen] = useState(false);
  const selected = decks.find((d) => d.key === value);

  return (
    <div>
      <button
        type="button"
        onClick={() => setOpen(true)}
        className="flex w-full items-center justify-between gap-2 rounded-md border border-edge bg-panel px-3 py-2 text-left text-[13px] hover:border-accent"
      >
        <span className="truncate">
          {selected ? (
            <>
              {selected.player}
              {selected.headline && <span className="text-dim"> — {selected.headline}</span>}
            </>
          ) : (
            <span className="text-dim">Choose a deck…</span>
          )}
        </span>
        <span aria-hidden className="shrink-0 text-dim">
          ⌄
        </span>
      </button>
      {open && (
        <DeckPickerSheet
          decks={decks}
          onPick={(key) => {
            onPick(key);
            setOpen(false);
          }}
          onClose={() => setOpen(false)}
        />
      )}
    </div>
  );
}

function DeckPickerSheet({
  decks,
  onPick,
  onClose,
}: {
  decks: DeckEntry[];
  onPick: (key: string) => void;
  onClose: () => void;
}) {
  const [query, setQuery] = useState("");
  const [highlight, setHighlight] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const filtered = useMemo(() => decks.filter((d) => matches(d, query)), [decks, query]);
  // Clamped at render time, not reset via an effect: a query that
  // shrinks the list can leave a stale index past its new end.
  const activeHighlight = Math.min(highlight, filtered.length - 1);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Escape") {
      onClose();
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      setHighlight(Math.min(activeHighlight + 1, filtered.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setHighlight(Math.max(activeHighlight - 1, 0));
    } else if (e.key === "Enter") {
      e.preventDefault();
      const pick = filtered[activeHighlight];
      if (pick) onPick(pick.key);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-end justify-center bg-black/50 p-3 sm:items-center"
      onClick={onClose}
    >
      <div
        className="flex max-h-[80vh] w-full max-w-md flex-col overflow-hidden rounded-lg border border-edge bg-bg"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="border-b border-edge p-3">
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={onKeyDown}
            placeholder="Search player or Pokémon…"
            className="w-full rounded-md border border-edge bg-transparent px-3 py-1.5 text-[13px] outline-none focus:border-accent"
          />
        </div>
        <ul className="overflow-y-auto p-1" aria-label="Deck results">
          {filtered.length === 0 && (
            <li className="px-3 py-6 text-center text-[13px] text-dim">
              No deck matches “{query}”.
            </li>
          )}
          {filtered.map((d, i) => (
            <li key={d.key}>
              <button
                type="button"
                onClick={() => onPick(d.key)}
                onMouseEnter={() => setHighlight(i)}
                className={`flex w-full items-center justify-between gap-2 rounded-md px-3 py-2 text-left text-[13px] ${
                  i === activeHighlight ? "bg-panel" : "hover:bg-panel"
                }`}
              >
                <span className="truncate">
                  {d.player}
                  {d.headline && <span className="text-dim"> — {d.headline}</span>}
                </span>
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
