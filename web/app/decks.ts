// The 2026 Worlds decks, from `public/decks/index.json` (written by
// `scripts/build-assets.mjs`). A deck's `key` is its file stem and what
// a recipe's `a` / `b` carry.

export type DeckEntry = { key: string; player: string; headline: string };

let cached: Promise<DeckEntry[]> | null = null;

export function loadDeckIndex(): Promise<DeckEntry[]> {
  if (!cached) {
    cached = fetch("/decks/index.json")
      .then((r) => (r.ok ? (r.json() as Promise<DeckEntry[]>) : []))
      .catch(() => []);
  }
  return cached;
}

/** The two decks a fresh game opens with when the URL names none. */
export const DEFAULT_DECKS = { a: "03-brent-tonisson", b: "02-diego-cassiraga" };
