// The card catalog Settings browses to pick a preferred Print. A Print
// is one release of a Card definition, identified by its TCGdex print
// id — see the "Print" glossary entry in docs/architecture/glossary.md.
// TCGdex relates Prints of one card only by a shared `name`, so that is
// the grouping key here too.

/** The shape read from `public/cards.json`, trimmed to what the
 *  catalog needs. */
export type CatalogCard = {
  id: string;
  name: string;
  /** TCGdex's own category, as imported: "Pokemon" | "Trainer" | "Energy". */
  category: string;
  /** Set only when `category` is "Trainer": "Supporter" | "Item" | "Tool" | "Stadium". */
  trainerType?: string;
};

/** The catalog's own category order — its own symbol, not a reuse of
 *  `HAND_ORDER` (`web/app/board/shared.ts`). The two start identical
 *  but are allowed to drift: a hand's sorting needs and a catalog
 *  browse's needs are not guaranteed to stay the same thing. */
export const CATALOG_ORDER = [
  "pokemon",
  "supporter",
  "item",
  "tool",
  "stadium",
  "special-energy",
  "energy",
] as const;

/** The coarse bucket one raw card falls into, matching the wire's
 *  `WireCard.category` buckets. `data/cards.json` holds only Special
 *  Energy (Basic Energy carries no regulation mark and is never
 *  imported), so an "Energy" card always reads as special-energy here. */
export function bucketOf(card: CatalogCard): string {
  switch (card.category) {
    case "Pokemon":
      return "pokemon";
    case "Energy":
      return "special-energy";
    case "Trainer":
      return (card.trainerType ?? "").toLowerCase();
    default:
      return "energy";
  }
}

/** Every print id sharing one card name, sorted. */
export function printsFor(cards: CatalogCard[], name: string): string[] {
  return cards
    .filter((c) => c.name === name)
    .map((c) => c.id)
    .sort();
}

export type CatalogEntry = {
  name: string;
  bucket: string;
  /** Every print id sharing this name, sorted. */
  prints: string[];
};

/** One entry per distinct card name, in `CATALOG_ORDER` then
 *  alphabetical order — the catalog grid's own display order. */
export function catalogEntries(cards: CatalogCard[]): CatalogEntry[] {
  const byName = new Map<string, CatalogEntry>();
  for (const c of cards) {
    const entry = byName.get(c.name);
    if (entry) entry.prints.push(c.id);
    else byName.set(c.name, { name: c.name, bucket: bucketOf(c), prints: [c.id] });
  }
  const order = CATALOG_ORDER as readonly string[];
  const rank = (b: string) => {
    const i = order.indexOf(b);
    return i < 0 ? order.length : i;
  };
  return [...byName.values()]
    .map((e) => ({ ...e, prints: [...e.prints].sort() }))
    .sort((a, b) => rank(a.bucket) - rank(b.bucket) || a.name.localeCompare(b.name));
}

/** The deterministic fallback print for a name with no stored
 *  preference and no live context to defer to: the first print id,
 *  sorted. Used for the Settings grid's own tile art, where there is
 *  no Decklist or game to resolve a print from. */
export function defaultPrint(prints: string[]): string {
  return [...prints].sort()[0];
}
