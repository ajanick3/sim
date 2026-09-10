// A recipe is everything needed to rebuild one game from its start: the
// seed, the two deck keys, and the action indices taken so far. The
// engine replays it (`Game.replay_standard`). See
// `.scratch/web-followups/issues/01`.

export interface Recipe {
  v: 1;
  seed: number;
  a: string;
  b: string;
  moves: number[];
}

const PARAM = "g";

export function newRecipe(seed: number, a: string, b: string): Recipe {
  return { v: 1, seed, a, b, moves: [] };
}

export function encodeRecipe(recipe: Recipe): string {
  return toBase64Url(JSON.stringify(recipe));
}

export function decodeRecipe(encoded: string): Recipe | null {
  let json: string;
  try {
    json = fromBase64Url(encoded);
  } catch {
    return null;
  }
  let value: unknown;
  try {
    value = JSON.parse(json);
  } catch {
    return null;
  }
  return isRecipe(value) ? value : null;
}

/** Read a recipe out of a `?g=...` query string, or null if absent/bad. */
export function readRecipeParam(search: string): Recipe | null {
  const encoded = new URLSearchParams(search).get(PARAM);
  return encoded ? decodeRecipe(encoded) : null;
}

/** The `?g=...` query string that carries `recipe`. */
export function writeRecipeParam(recipe: Recipe): string {
  const params = new URLSearchParams();
  params.set(PARAM, encodeRecipe(recipe));
  return `?${params.toString()}`;
}

function isRecipe(value: unknown): value is Recipe {
  if (typeof value !== "object" || value === null) return false;
  const r = value as Record<string, unknown>;
  return (
    r.v === 1 &&
    typeof r.seed === "number" &&
    typeof r.a === "string" &&
    typeof r.b === "string" &&
    Array.isArray(r.moves) &&
    r.moves.every((m) => typeof m === "number")
  );
}

function toBase64Url(s: string): string {
  return btoa(unescape(encodeURIComponent(s)))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
}

function fromBase64Url(s: string): string {
  const b64 = s.replace(/-/g, "+").replace(/_/g, "/");
  return decodeURIComponent(escape(atob(b64)));
}
