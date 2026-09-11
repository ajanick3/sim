// Card art. `public/art-index.json` maps a TCGdex print id to the base URL
// its images sit at; `build-art-index.mjs` regenerates it. The app appends
// a quality suffix here and falls back to a drawn card when a print id is
// absent or its image fails to load.
//
// `sw.js` caches every image it sees forever (cache-first, never
// revalidated), so an installed app — the same tab, reopened, over and
// over — pays a bigger download once per print and keeps it. A plain
// browser tab, more likely a one-off visit that never comes back to spend
// that cache, defaults to the smaller tier instead.

export type ArtIndex = Record<string, string>;
export type ArtQuality = "low" | "high";

let cached: Promise<ArtIndex> | null = null;

/** Load the print-id → image-base map once; later calls share it. */
export function loadArtIndex(): Promise<ArtIndex> {
  if (!cached) {
    cached = fetch("/art-index.json")
      .then((r) => (r.ok ? (r.json() as Promise<ArtIndex>) : {}))
      .catch(() => ({}));
  }
  return cached;
}

/** Whether the app is running installed — a standalone PWA window, not
 *  a browser tab. `false` on the server and whenever the platform gives
 *  no way to tell. */
export function isInstalled(): boolean {
  if (typeof window === "undefined") return false;
  try {
    if (window.matchMedia?.("(display-mode: standalone)").matches) return true;
  } catch {
    // matchMedia unsupported or unimplemented — fall through to the
    // iOS-only flag below.
  }
  return (window.navigator as Navigator & { standalone?: boolean }).standalone === true;
}

/** The image URL for a print id at the given quality, or null when
 *  there is no art to show. Defaults to "low" — a caller that cares
 *  about the installed-app distinction passes "high" itself. */
export function artUrl(
  index: ArtIndex,
  printId: string,
  quality: ArtQuality = "low",
): string | null {
  const base = index[printId];
  return base ? `${base}/${quality}.webp` : null;
}
