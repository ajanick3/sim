// Pre-warming the service worker's cache: fetching every known card's art
// once, up front, through the app's own normal per-image request path
// (`sw.js` caches whatever it sees fetched, forever) — the same thing
// that happens organically as cards come up in play, just driven in one
// pass instead of waiting on it.

/** Fetch every URL, `concurrency` at a time, reporting how many are done
 *  so far after each one. One URL failing (offline, a 404, a transient
 *  CDN error) never stops the rest — this is a best-effort warm, not a
 *  transaction. */
export async function warmCache(
  urls: string[],
  fetchOne: (url: string) => Promise<unknown>,
  onProgress: (done: number, total: number) => void,
  concurrency = 6,
): Promise<void> {
  const total = urls.length;
  let next = 0;
  let done = 0;
  onProgress(0, total);

  async function worker() {
    for (;;) {
      const index = next++;
      if (index >= urls.length) return;
      try {
        await fetchOne(urls[index]);
      } catch {
        // Best-effort — see the module comment.
      }
      done++;
      onProgress(done, total);
    }
  }

  await Promise.all(Array.from({ length: Math.min(concurrency, total) }, worker));
}

// Kept in step with `CACHE_NAME` in `public/sw.js` by hand — bump one,
// bump the other.
const CACHE_NAME = "sim-v4";

/** How many of these URLs the service worker has already cached, or
 *  null if the Cache API isn't reachable here (no service worker, an
 *  insecure context, a browser that blocks it) — the caller should
 *  treat null as "unknown", not "none cached". */
export async function countCached(urls: string[]): Promise<number | null> {
  if (typeof caches === "undefined") return null;
  try {
    const cache = await caches.open(CACHE_NAME);
    const hits = await Promise.all(urls.map((url) => cache.match(url)));
    return hits.filter((hit) => hit != null).length;
  } catch {
    return null;
  }
}
