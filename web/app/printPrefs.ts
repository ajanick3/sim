// This player's preferred Print for each card, kept in localStorage so
// the choice applies everywhere the app shows that card's art — a
// viewer preference, not a Decklist fact. See the "Print" glossary
// entry in docs/architecture/glossary.md.
//
// Keyed by `identityOf`/`CatalogEntry.key`, not bare card name — a
// name alone can name two unrelated cards (see prints.ts), and keying
// by name would let a preference set on one bleed onto the other.

import type { PrintPrefs } from "./prints";

const KEY = "sim.printPrefs";

export function loadPrintPrefs(): PrintPrefs {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return {};
    const prefs = JSON.parse(raw) as PrintPrefs;
    return prefs && typeof prefs === "object" ? prefs : {};
  } catch {
    return {};
  }
}

/** Save one card's preferred print and return the updated map, so the
 *  caller can update its own state without a second read. */
export function savePrintPref(key: string, printId: string): PrintPrefs {
  const next = { ...loadPrintPrefs(), [key]: printId };
  try {
    localStorage.setItem(KEY, JSON.stringify(next));
  } catch {
    // A private window, or storage disabled — preferences are a convenience.
  }
  return next;
}
