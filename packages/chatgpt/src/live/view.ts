// Duplicated from web/app/view.ts — the wire shape `Game.view()` returns.
// See ./wasm.ts for why this package can't import it directly.

export interface WireCard {
  id: number;
  name: string;
  def: number;
  print_id: string;
  energy_type: string | null;
  category: string;
}

export interface WirePokemon {
  id: number;
  name: string;
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
  deck_count: number;
  prize_count: number;
  discard: WireCard[];
  active: WirePokemon | null;
  bench: WirePokemon[];
}

export interface WireActionMeta {
  kind: string;
  card: number | null;
  target: number | null;
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
  stadium: WireCard | null;
  deck_in_search: WireCard[] | null;
  counters_to_place: number | null;
  sides: [WireSide, WireSide];
}
