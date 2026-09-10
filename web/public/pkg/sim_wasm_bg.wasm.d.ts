/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const __wbg_carddata_free: (a: number, b: number) => void;
export const __wbg_game_free: (a: number, b: number) => void;
export const carddata_new: (a: number, b: number) => [number, number, number];
export const game_apply: (a: number, b: number) => [number, number];
export const game_history: (a: number) => [number, number];
export const game_is_over: (a: number) => number;
export const game_legal_actions: (a: number) => [number, number];
export const game_log: (a: number) => [number, number];
export const game_player_to_act: (a: number) => number;
export const game_replay_standard: (a: number, b: number, c: number, d: number, e: number, f: bigint, g: number, h: number) => [number, number, number];
export const game_standard: (a: number, b: number, c: number, d: number, e: number, f: bigint) => [number, number, number];
export const game_synthetic: (a: bigint) => number;
export const game_view: (a: number) => [number, number];
export const __wbindgen_externrefs: WebAssembly.Table;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
