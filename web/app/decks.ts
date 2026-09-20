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
export const DEFAULT_DECKS = { a: "003-brent-tonisson", b: "002-diego-cassiraga" };

// A pasted decklist has no file key, so it travels as the recipe's `a` /
// `b` itself, marked with this prefix. `Game.replay_standard` already
// takes raw decklist text, so the game shell only needs to tell a pasted
// list apart from a key to fetch.
const PASTE_PREFIX = "paste:";

export const isPastedDeck = (key: string): boolean => key.startsWith(PASTE_PREFIX);
export const encodePastedDeck = (text: string): string => PASTE_PREFIX + text;
export const pastedDeckText = (key: string): string => key.slice(PASTE_PREFIX.length);
