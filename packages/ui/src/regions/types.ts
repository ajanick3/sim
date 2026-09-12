// The shapes every Region composes from, kept deliberately close to
// the engine's own wire format (`crates/sim-wasm/src/lib.rs`'s
// `WireCard` / `WirePokemon`) even though this library never imports
// Rust — a card here should carry the same facts a real one does, not
// a convenient subset invented for the mockups. Field names match the
// wire format's; `src` is this library's own addition (the wire
// format sends `print_id`, and resolving that to art is a `web/`
// concern this library doesn't have).

/** The ten Energy types the engine actually has — `Type` in
 *  `src/card.rs`. No "Fairy": this game has none. */
export type EnergyType =
  | "Grass"
  | "Fire"
  | "Water"
  | "Lightning"
  | "Psychic"
  | "Fighting"
  | "Darkness"
  | "Metal"
  | "Dragon"
  | "Colorless";

/** The hand-sort bucket the wire format sends on every `WireCard` —
 *  `card_category` in `crates/sim-wasm/src/lib.rs`. */
export type Category = "pokemon" | "special-energy" | "supporter" | "item" | "tool" | "stadium" | "energy";

/** Mirrors `WireCard`: any card outside play — in a hand, a deck
 *  search, a discard pile, the Stadium slot. */
export type PocketCard = {
  id: number;
  name: string;
  src?: string | null;
  energyType?: EnergyType | null;
  category?: Category;
};

/** Mirrors `WirePokemon`: one Pokémon in play. `hp` is the printed
 *  max — a physical card's HP box never changes — `remainingHp` is
 *  `hp` less `damage`, the engine's own knockout-facing number, kept
 *  here even where no atom shows it yet. `attached` is the one list
 *  the engine carries, exactly like `WirePokemon.attached`: Energy and
 *  a Tool both live in it, told apart by `category`, not two separate
 *  fields — a Region derives "energies" and "the Tool" from it the
 *  same way `LiveMon.tsx` does. */
export type PlayCard = {
  id: number;
  name: string;
  src?: string | null;
  hp: number;
  damage: number;
  remainingHp?: number;
  conditions?: string[];
  attached?: PocketCard[];
};

/** The two facts `attached` actually carries, split the way a Region
 *  needs them to draw `EnergyChip`s and a `ToolBadge`. */
export function attachedParts(mon: PlayCard): { energies: PocketCard[]; tool: PocketCard | null } {
  const attached = mon.attached ?? [];
  return {
    energies: attached.filter((c) => c.energyType != null),
    tool: attached.find((c) => c.category === "tool") ?? null,
  };
}
