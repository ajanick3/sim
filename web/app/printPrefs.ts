// This player's preferred Print for each card name, kept in
// localStorage so the choice applies everywhere the app shows that
// card's art — a viewer preference, not a Decklist fact. See the
// "Print" glossary entry in docs/architecture/glossary.md.

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
export function savePrintPref(name: string, printId: string): PrintPrefs {
  const next = { ...loadPrintPrefs(), [name]: printId };
  try {
    localStorage.setItem(KEY, JSON.stringify(next));
  } catch {
    // A private window, or storage disabled — preferences are a convenience.
  }
  return next;
}
