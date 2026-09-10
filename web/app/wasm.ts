// The wasm-pack glue in `public/pkg` is loaded at runtime with a native
// `import()`, so the bundler never sees it. These types describe the part of
// its surface this app uses; they mirror `crates/sim-wasm/src/lib.rs`.

export interface Game {
  legal_actions(): string;
  apply(index: number): void;
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
  };
}

let cached: Promise<SimWasm> | null = null;

/** Load and initialise the engine once; later calls share the instance. */
export function loadSim(): Promise<SimWasm> {
  if (!cached) {
    cached = (async () => {
      // A non-literal specifier keeps the bundler and the type checker from
      // trying to resolve this path; the file is served from `public/pkg`.
      const glue = "/pkg/sim_wasm.js";
      const mod = (await import(/* webpackIgnore: true */ glue)) as unknown as SimWasm;
      await mod.default("/pkg/sim_wasm_bg.wasm");
      return mod;
    })();
  }
  return cached;
}
