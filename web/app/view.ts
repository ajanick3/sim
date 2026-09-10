// The shape `Game.view()` returns as JSON, from `WireView` in
// `crates/sim-wasm/src/lib.rs`.

export interface WireCard {
  id: number;
  name: string;
  def: number;
}

export interface WirePokemon {
  name: string;
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

export interface WireView {
  you: number;
  current: number;
  turn_number: number;
  phase: string;
  your_hand: WireCard[];
  sides: [WireSide, WireSide];
}
