// Offline support for the installed app. Nothing here is precached by
// name — Next.js hashes its chunk files per build, so a fixed manifest
// would go stale the moment the app rebuilds. Instead every GET this
// worker sees gets cached the first time it succeeds, so a second visit
// (online or not) is served from the cache it already built.
//
// Bump CACHE_NAME by hand when a change here should force every open
// tab onto a clean cache; a normal app deploy does not need it, since
// asset URLs already change with their content hash.
//
// v2: art.ts now asks for "/high.webp" once the app is installed
// (still "/low.webp" in a plain tab) — a different URL either way for
// anyone who had it installed under v1, so old installs would
// otherwise keep the orphaned low-res bytes forever alongside the new
// ones, since nothing here ever evicts a cached entry on its own. The
// bump drops the old cache outright.
const CACHE_NAME = "sim-v2";

self.addEventListener("install", () => {
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(keys.filter((k) => k !== CACHE_NAME).map((k) => caches.delete(k))),
      )
      .then(() => self.clients.claim()),
  );
});

self.addEventListener("fetch", (event) => {
  const { request } = event;
  if (request.method !== "GET") return;

  // Both Storybooks (/storybook, /sb) are dev exhibits, not part of
  // the offline game — and Storybook's own index.json isn't
  // content-hashed the way a Next.js chunk is, so cache-first below
  // would serve a stale story list forever after the first visit,
  // invisible to a rebuild. Skip this worker for them entirely; the
  // browser's own HTTP cache handles the rest.
  const path = new URL(request.url).pathname;
  if (path.startsWith("/storybook") || path.startsWith("/sb")) return;

  // A page navigation: try the network first, so a deploy is seen right
  // away, but fall back to whatever shell was last cached when offline.
  if (request.mode === "navigate") {
    event.respondWith(
      fetch(request)
        .then((response) => {
          const copy = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(request, copy));
          return response;
        })
        .catch(() => caches.match(request).then((cached) => cached ?? caches.match("/"))),
    );
    return;
  }

  // Everything else — JS/CSS/wasm chunks, the card artifact, deck lists,
  // and card art from the TCGdex CDN — is cache-first: instant and free
  // once seen, and what makes a card viewed once show up offline later.
  event.respondWith(
    caches.match(request).then(
      (cached) =>
        cached ??
        fetch(request).then((response) => {
          // A cross-origin image request (the TCGdex CDN) comes back
          // opaque — status 0, unreadable, but still the real bytes — so
          // it is worth caching on its own terms, not just when `ok`.
          if (response.ok || response.type === "opaque") {
            const copy = response.clone();
            caches.open(CACHE_NAME).then((cache) => cache.put(request, copy));
          }
          return response;
        }),
    ),
  );
});
