// A minimal duplicate of web/app/art.ts's lookup — real card art without
// the installed-app quality distinction, which this demo has no PWA
// shell to make sense of. See ./wasm.ts for why this package can't
// import the original directly.

export type ArtIndex = Record<string, string>;

let cached: Promise<ArtIndex> | null = null;

export function loadArtIndex(): Promise<ArtIndex> {
  if (!cached) {
    cached = fetch("/art-index.json")
      .then((r) => (r.ok ? (r.json() as Promise<ArtIndex>) : {}))
      .catch(() => ({}));
  }
  return cached;
}

export function artUrl(index: ArtIndex, printId: string): string | undefined {
  const base = index[printId];
  return base ? `${base}/low.webp` : undefined;
}
