"use client";

import { useEffect, useState } from "react";
import type { CoinResult } from "./shared";

/** A short overlay for one run of coin flips: the coins spin in a row,
 *  land on their faces, and the whole thing fades after a beat. Render
 *  it with a fresh `key` per run so it replays. */
export function CoinFlip({
  results,
  onDone,
  persist = false,
}: {
  results: CoinResult[];
  onDone: () => void;
  /** Skip the auto-dismiss — for Storybook. */
  persist?: boolean;
}) {
  const [gone, setGone] = useState(false);

  useEffect(() => {
    if (persist) return;
    const t = setTimeout(() => setGone(true), 1100 + results.length * 90);
    const d = setTimeout(onDone, 1500 + results.length * 90);
    return () => {
      clearTimeout(t);
      clearTimeout(d);
    };
  }, [results.length, onDone, persist]);

  if (results.length === 0) return null;
  const heads = results.filter((r) => r === "heads").length;

  return (
    <div
      className={`pointer-events-none fixed inset-0 z-[70] grid place-items-center transition-opacity duration-300 ${
        gone ? "opacity-0" : "opacity-100"
      }`}
      aria-live="polite"
    >
      <div className="flex flex-col items-center gap-2 rounded-2xl bg-black/45 px-6 py-4 backdrop-blur-sm">
        <div className="flex gap-2">
          {results.map((r, i) => (
            <span
              key={i}
              className="coin-spin grid size-12 place-items-center rounded-full border-2 border-yellow-200/70 text-lg font-black text-yellow-950 shadow-[0_4px_12px_rgba(0,0,0,0.5)]"
              style={{
                animationDelay: `${i * 90}ms`,
                background:
                  r === "heads"
                    ? "radial-gradient(circle at 35% 30%, #fde68a, #d4a017 70%)"
                    : "radial-gradient(circle at 35% 30%, #e5e7eb, #9aa6b2 70%)",
              }}
            >
              {r === "heads" ? "H" : "T"}
            </span>
          ))}
        </div>
        <span className="text-[13px] font-bold uppercase tracking-widest text-text">
          {results.length === 1
            ? results[0]
            : `${heads} ${heads === 1 ? "head" : "heads"}, ${results.length - heads} tails`}
        </span>
      </div>
    </div>
  );
}
