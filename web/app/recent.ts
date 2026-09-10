// Recently opened games, kept in localStorage so the selector can offer
// "pick up where you left off". Keyed by seed + decks so the list holds
// distinct games, each at its latest position.

const KEY = "sim.recent";
const CAP = 24;

export type RecentGame = {
  /** The `?g=` value — everything needed to reopen the game. */
  g: string;
  seed: number;
  a: string;
  b: string;
  turn: number;
  /** Epoch millis of the last move. */
  at: number;
};

export function loadRecent(): RecentGame[] {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return [];
    const list = JSON.parse(raw) as RecentGame[];
    return Array.isArray(list) ? list : [];
  } catch {
    return [];
  }
}

export function noteRecent(entry: RecentGame): void {
  try {
    const same = (r: RecentGame) => r.seed === entry.seed && r.a === entry.a && r.b === entry.b;
    const next = [entry, ...loadRecent().filter((r) => !same(r))].slice(0, CAP);
    localStorage.setItem(KEY, JSON.stringify(next));
  } catch {
    // A private window, or storage disabled — recents are a convenience.
  }
}
