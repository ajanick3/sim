"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { loadRecent } from "../recent";

export default function SettingsPage() {
  const [recentCount, setRecentCount] = useState(0);

  useEffect(() => {
    setRecentCount(loadRecent().length);
  }, []);

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
    </main>
  );
}
