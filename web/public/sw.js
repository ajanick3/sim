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
//
// v3: the deck list's own data — `/decks/index.json` and each deck's
// `.txt` — used to be cache-first like everything else below, so a
// renumbered deck file (`04-henry-chao.txt` becoming
// `004-henry-chao.txt` once the field passed 99 entries) left any
// browser that had already cached the old index stuck picking a deck
// key whose file no longer existed, forever, with no way to notice
// online. These paths (plus `/cards.json` and `/art-index.json`, the
// artifact's own two other bare, unhashed data files) are now
// network-first below instead. The bump clears every stale copy of
// them still sitting in an old cache.
const CACHE_NAME = "sim-v3";

/** Data the artifact can change without its URL changing — unlike a
 *  Next.js chunk or a piece of card art, neither hashed nor keyed by an
 *  id that pins one immutable value. Cache-first would serve a stale
 *  copy forever once a deploy changes what it says. */
function isMutableData(path) {
  return (
    path === "/cards.json" ||
    path === "/art-index.json" ||
    path === "/decks/index.json" ||
    (path.startsWith("/decks/") && path.endsWith(".txt"))
  );
}

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

  // The artifact's own data (see `isMutableData`): network-first, same
  // shape as a navigation — a deploy that changes it is seen right away
  // while online, and a game already open still replays offline from
  // whatever copy was last cached.
  if (isMutableData(path)) {
    event.respondWith(
      fetch(request)
        .then((response) => {
          const copy = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(request, copy));
          return response;
        })
        .catch(() => caches.match(request)),
    );
    return;
  }

  // Everything else — JS/CSS/wasm chunks and card art from the TCGdex
  // CDN — is cache-first: instant and free once seen, and what makes a
  // card viewed once show up offline later. Safe here because each is
  // either content-hashed (a Next.js chunk) or keyed by an id that pins
  // one immutable value forever (a print's own art).
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
