// Duplicated from web/app/wasm.ts — this package builds completely
// isolated from web/ (its own React 18, its own Vite build), so it
// can't import across that boundary. The contract mirrors
// crates/sim-wasm/src/lib.rs; /pkg/sim_wasm.js is served from the same
// origin's public/pkg regardless of which page loads it.

export interface Game {
  legal_actions(): string;
  action_meta(): string;
  apply(index: number): void;
  history(): string;
  log(): string;
  player_to_act(): number | undefined;
  is_over(): boolean;
  view(): string;
  free(): void;
}

export interface CardData {
  free(): void;
}

export interface SimWasm {
  default(path?: string): Promise<unknown>;
  CardData: { new: (cardsJson: string) => CardData };
  Game: {
    synthetic: (seed: bigint) => Game;
    standard: (data: CardData, deckA: string, deckB: string, seed: bigint) => Game;
    replay_standard: (
      data: CardData,
      deckA: string,
      deckB: string,
      seed: bigint,
      indices: Uint32Array | number[],
    ) => Game;
  };
}

let cached: Promise<SimWasm> | null = null;

export function loadSim(): Promise<SimWasm> {
  if (!cached) {
    cached = (async () => {
      const glue = "/pkg/sim_wasm.js";
      const mod = (await import(/* @vite-ignore */ glue)) as unknown as SimWasm;
      await mod.default("/pkg/sim_wasm_bg.wasm");
      return mod;
    })();
  }
  return cached;
}
