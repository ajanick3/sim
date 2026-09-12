// Build public/art-index.json: a map from every TCGdex print id in the
// card artifact to the base URL its images live at. The app appends a
// quality suffix (e.g. "/high.webp") at render time and falls back to a
// drawn card when an id is missing or its image 404s.
//
// Run by hand — it hits the network — and commit the result:
//   pnpm run art-index

import { readFile, writeFile } from "node:fs/promises";

const ARTIFACT = new URL("../../data/cards.json", import.meta.url);
const OUT = new URL("../public/art-index.json", import.meta.url);
const BULK = "https://api.tcgdex.net/v2/en/cards";

const artifact = JSON.parse(await readFile(ARTIFACT, "utf8"));
const wanted = new Set(artifact.cards.map((c) => c.id));
console.log(`${wanted.size} print ids in the artifact`);

const all = await fetch(BULK).then((r) => {
  if (!r.ok) throw new Error(`bulk list: ${r.status}`);
  return r.json();
});

const index = {};
let missing = 0;
for (const card of all) {
  if (!wanted.has(card.id)) continue;
  if (card.image) index[card.id] = card.image;
  else missing++;
}

const found = Object.keys(index).length;
await writeFile(OUT, JSON.stringify(index, null, 0) + "\n");
console.log(`wrote ${found} image bases (${missing} artifact cards have no art)`);
