// Card art. `public/art-index.json` maps a TCGdex print id to the base URL
// its images sit at; `build-art-index.mjs` regenerates it. The app appends
// a quality suffix here and falls back to a drawn card when a print id is
// absent or its image fails to load.

export type ArtIndex = Record<string, string>;

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

/** The image URL for a print id, or null when there is no art to show. */
export function artUrl(index: ArtIndex, printId: string): string | null {
  const base = index[printId];
  return base ? `${base}/low.webp` : null;
}
