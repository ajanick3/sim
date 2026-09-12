// Copy the card artifact and every 2026 Worlds decklist into public/, and
// write an index the deck selector reads. Run by `pnpm run assets` before
// dev and build.

import { mkdirSync, readdirSync, readFileSync, writeFileSync, copyFileSync } from "node:fs";
import { join } from "node:path";

const DECK_SRC = new URL("../../decks/2026-worlds/", import.meta.url);
const OUT = new URL("../public/decks/", import.meta.url);
const outDir = OUT.pathname;

mkdirSync(outDir, { recursive: true });
copyFileSync(
  new URL("../../data/cards.json", import.meta.url).pathname,
  join(outDir, "..", "cards.json"),
);

const titleCase = (s) => s.replace(/\b\w/g, (c) => c.toUpperCase());

/** The line "N <Pokémon> …" with the highest N — a rough archetype label. */
function headline(text) {
  const lines = text.split("\n");
  const start = lines.findIndex((l) => /^Pok[ée]mon:/i.test(l));
  const end = lines.findIndex((l, i) => i > start && /^\s*$/.test(l));
  let best = "";
  let bestN = 0;
  for (const line of lines.slice(start + 1, end === -1 ? undefined : end)) {
    const m = /^(\d+)\s+(.+?)\s+[A-Z0-9]{2,4}\s+\d+\s*$/.exec(line.trim());
    if (m && Number(m[1]) >= bestN) {
      bestN = Number(m[1]);
      best = m[2];
    }
  }
  return best;
}

const files = readdirSync(DECK_SRC.pathname)
  .filter((f) => f.endsWith(".txt"))
  .sort();

const index = files.map((file) => {
  const key = file.replace(/\.txt$/, "");
  const text = readFileSync(join(DECK_SRC.pathname, file), "utf8");
  writeFileSync(join(outDir, `${key}.txt`), text);
  const player = titleCase(key.replace(/^\d+-/, "").replace(/-/g, " "));
  return { key, player, headline: headline(text) };
});

writeFileSync(join(outDir, "index.json"), JSON.stringify(index, null, 0) + "\n");
console.log(`assets: ${index.length} decks + cards.json`);
