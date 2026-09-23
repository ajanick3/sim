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

// A Pokémon's evolution stage, by name, read once from the same artifact
// `cards.json` this script already copies. Every print of a name shares
// one stage, so the name alone is enough — no print id needed.
const STAGE_RANK = { Basic: 0, Stage1: 1, Stage2: 2 };
const stageOf = (() => {
  const cardsJson = JSON.parse(
    readFileSync(new URL("../../data/cards.json", import.meta.url).pathname, "utf8"),
  );
  const cards = Array.isArray(cardsJson) ? cardsJson : cardsJson.cards;
  const byName = new Map();
  for (const c of cards) {
    if (c.category === "Pokemon" && c.stage in STAGE_RANK) byName.set(c.name, c.stage);
  }
  return (name) => byName.get(name);
})();

/**
 * A name's place in its own evolution line, highest first. A Mega
 * Pokémon ex (e.g. `Mega Kangaskhan ex`) is the finished form of its
 * line same as any Stage 2 — it just Mega Evolves from a Basic already
 * in play instead of evolving by a card in the deck, so the artifact
 * still tags it `Basic`. Read the name, not the tag, for this one case.
 */
function evolutionRank(name) {
  if (name.startsWith("Mega ")) return 3;
  return STAGE_RANK[stageOf(name)] ?? -1;
}

/**
 * The line "N <Pokémon> …" naming this deck's highest-evolution Pokémon —
 * the finished attacker (Dragapult ex), not the Basic that starts its
 * line (Dreepy) just because more copies of it are run. Ties on rank
 * (e.g. two different Stage 2 lines) fall back to the higher count.
 */
function headline(text) {
  const lines = text.split("\n");
  const start = lines.findIndex((l) => /^Pok[ée]mon:/i.test(l));
  const end = lines.findIndex((l, i) => i > start && /^\s*$/.test(l));
  let best = "";
  let bestRank = -1;
  let bestN = 0;
  for (const line of lines.slice(start + 1, end === -1 ? undefined : end)) {
    const m = /^(\d+)\s+(.+?)\s+[A-Z0-9]{2,4}\s+\d+\s*$/.exec(line.trim());
    if (!m) continue;
    const name = m[2];
    const rank = evolutionRank(name);
    const count = Number(m[1]);
    if (rank > bestRank || (rank === bestRank && count > bestN)) {
      bestRank = rank;
      bestN = count;
      best = name;
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
