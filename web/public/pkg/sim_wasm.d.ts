/* tslint:disable */
/* eslint-disable */

/**
 * The card artifact, read once. Parsing and interning the 2 MB of card data
 * is done here, so a page that starts many games pays it a single time.
 */
export class CardData {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Read `data/cards.json`, passed in as a string.
     */
    static new(cards_json: string): CardData;
}

/**
 * One game, held open across calls.
 */
export class Game {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Apply the action at `index` in the current `legal_actions` list.
     */
    apply(index: number): void;
    /**
     * Whether the game has finished.
     */
    is_over(): boolean;
    /**
     * The legal actions right now, each as the text `describe` prints.
     * The index into this list is the argument `apply` takes.
     */
    legal_actions(): string;
    /**
     * The whole log, oldest line first.
     */
    log(): string;
    /**
     * Which seat must act next, `0` or `1`, or nothing once the game is over.
     */
    player_to_act(): number | undefined;
    /**
     * A game of the Standard set. Each decklist is the text of a `.txt`
     * deck file. Both must be fully playable.
     */
    static standard(data: CardData, deck_a: string, deck_b: string, seed: bigint): Game;
    /**
     * A game of the synthetic set, both seats dealt the starter decklist.
     * This needs no card artifact, so it is the path a test and a demo take.
     */
    static synthetic(seed: bigint): Game;
    /**
     * The board as the seat to act sees it, masked. JSON of [`WireView`].
     */
    view(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_carddata_free: (a: number, b: number) => void;
    readonly __wbg_game_free: (a: number, b: number) => void;
    readonly carddata_new: (a: number, b: number) => [number, number, number];
    readonly game_apply: (a: number, b: number) => [number, number];
    readonly game_is_over: (a: number) => number;
    readonly game_legal_actions: (a: number) => [number, number];
    readonly game_log: (a: number) => [number, number];
    readonly game_player_to_act: (a: number) => number;
    readonly game_standard: (a: number, b: number, c: number, d: number, e: number, f: bigint) => [number, number, number];
    readonly game_synthetic: (a: bigint) => number;
    readonly game_view: (a: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
