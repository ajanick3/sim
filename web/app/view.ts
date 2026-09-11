// The shape `Game.view()` returns as JSON, from `WireView` in
// `crates/sim-wasm/src/lib.rs`.

export interface WireCard {
  id: number;
  name: string;
  def: number;
  /** TCGdex print id — the key card art is filed under. */
  print_id: string;
  /** The Energy type this card provides, e.g. "Fire", or null if not an Energy. */
  energy_type: string | null;
  /** Coarse hand-sort bucket: "pokemon" | "supporter" | "item" | "tool" | "stadium" | "special-energy" | "energy". */
  category: string;
}

export interface WirePokemon {
  /** Stable id for this Pokémon in play — matches an action's `target`. */
  id: number;
  name: string;
  /** TCGdex print id of this Pokémon's top card. */
  print_id: string;
  hp: number;
  damage: number;
  remaining_hp: number;
  conditions: string[];
  attached: WireCard[];
}

export interface WireSide {
  player: number;
  hand_count: number;
  library_count: number;
  prize_count: number;
  discard: WireCard[];
  active: WirePokemon | null;
  bench: WirePokemon[];
}

/** One entry of `Game.action_meta()`, index-aligned with `legal_actions()`. */
export interface WireActionMeta {
  /** The `Action` variant's name, e.g. "AttachEnergy", "Attack", "PlayTrainer". */
  kind: string;
  /** The hand/attached card the action names, or null. */
  card: number | null;
  /** The Pokémon in play the action names, or null. */
  target: number | null;
  /** Enough to draw the named card's face, for any zone (e.g. a deck search). */
  card_face?: {
    print_id: string;
    name: string;
    energy_type: string | null;
    category: string;
  } | null;
}

export interface WireView {
  you: number;
  current: number;
  turn_number: number;
  phase: string;
  your_hand: WireCard[];
  /** The Stadium card in play, or null. */
  stadium: WireCard | null;
  /** Your whole library, sorted, while you search the whole of it — null
   *  otherwise. The picker dims the cards the current search cannot take. */
  library_in_search: WireCard[] | null;
  /** How many Phantom Dive-style damage counters are still to place, or
   *  null in every other phase. */
  counters_to_place: number | null;
  sides: [WireSide, WireSide];
}
