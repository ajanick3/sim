// Copy the card artifact and every tournament's decklist into public/, and
// write an index the deck selector reads. Run by `pnpm run assets` before
// dev and build.
//
// Every direct subdirectory of decks/ is one tournament's field — the same
// shape tools/fetch_worlds_decklists.py already writes, one file per player
// named `<placement>-<player-slug>.txt`. A player can place in more than
// one tournament (Andrew Hedrick reached Day 2 at both Worlds and
// Baltimore), so a deck's key carries its tournament folder too:
// `<tournament-slug>/<placement>-<player-slug>` — the same path
// `deckPath` in game-shell.tsx already builds a URL from.

import {
  mkdirSync,
  readdirSync,
  readFileSync,
  writeFileSync,
  copyFileSync,
  statSync,
} from "node:fs";
import { join } from "node:path";

const DECKS_ROOT = new URL("../../decks/", import.meta.url);
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

const tournamentSlugs = readdirSync(DECKS_ROOT.pathname)
  .filter((f) => statSync(join(DECKS_ROOT.pathname, f)).isDirectory())
  .sort();

const index = [];
for (const tournamentSlug of tournamentSlugs) {
  const tournament = titleCase(tournamentSlug.replace(/-/g, " "));
  const srcDir = join(DECKS_ROOT.pathname, tournamentSlug);
  const outSubdir = join(outDir, tournamentSlug);
  mkdirSync(outSubdir, { recursive: true });

  const files = readdirSync(srcDir)
    .filter((f) => f.endsWith(".txt"))
    .sort();

  for (const file of files) {
    const slug = file.replace(/\.txt$/, "");
    const key = `${tournamentSlug}/${slug}`;
    const text = readFileSync(join(srcDir, file), "utf8");
    writeFileSync(join(outSubdir, file), text);
    const player = titleCase(slug.replace(/^\d+-/, "").replace(/-/g, " "));
    index.push({ key, player, tournament, headline: headline(text) });
  }
}

writeFileSync(join(outDir, "index.json"), JSON.stringify(index, null, 0) + "\n");
console.log(
  `assets: ${index.length} decks across ${tournamentSlugs.length} tournaments + cards.json`,
);
