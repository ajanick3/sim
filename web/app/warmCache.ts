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
