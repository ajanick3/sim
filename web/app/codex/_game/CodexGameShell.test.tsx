import { act as domAct, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

// A stub wasm handle: one generic action (no card, no target) alongside
// End turn, so the auto-shown ActionDialog has an alternative and can be
// dismissed — the shape reported stuck on a live game (a lone attack
// choice next to the End turn button).
const side = () => ({
  player: 0,
  hand_count: 0,
  deck_count: 0,
  prize_count: 0,
  discard: [],
  active: null,
  bench: [],
});
const gameStub = {
  legal_actions: () => JSON.stringify(["Attack: Teleportation Attack", "End turn"]),
  action_meta: () =>
    JSON.stringify([
      { kind: "Attack", card: null, target: null, is_fallback: false },
      { kind: "EndTurn", card: null, target: null, is_fallback: true },
    ]),
  apply: vi.fn(),
  history: vi.fn(() => JSON.stringify([])),
  log: () => JSON.stringify([]),
  player_to_act: () => 0,
  is_over: () => false,
  view: () =>
    JSON.stringify({
      you: 0,
      current: 0,
      turn_number: 1,
      phase: "Main",
      your_hand: [],
      stadium: null,
      deck_in_search: null,
      counters_to_place: null,
      sides: [side(), side()],
    }),
  free: vi.fn(),
};
// A second stub: the mulligan bonus-draw phase, where "Take a bonus card"
// is the only legal action and End turn is not legal yet. Nothing here
// is a safe fallback, so this dialog must not offer Cancel at all —
// dismissing it would leave the player with no way to proceed.
const bonusDrawStub = {
  ...gameStub,
  legal_actions: () => JSON.stringify(["Take a bonus card"]),
  action_meta: () =>
    JSON.stringify([{ kind: "TakeBonusDraw", card: null, target: null, is_fallback: false }]),
  view: () =>
    JSON.stringify({
      you: 0,
      current: 0,
      turn_number: 0,
      phase: "TakingBonusDraws",
      your_hand: [],
      stadium: null,
      deck_in_search: null,
      counters_to_place: null,
      sides: [side(), side()],
    }),
};

const replayStandard = vi.fn(() => gameStub);

vi.mock("../../wasm", () => ({
  loadSim: vi.fn(async () => ({
    CardData: { new: vi.fn(() => ({ free: vi.fn() })) },
    Game: { standard: vi.fn(), replay_standard: replayStandard, synthetic: vi.fn() },
  })),
}));

let mockSearch = "";
let cachedParams: URLSearchParams | null = null;
let cachedFor: string | null = null;
const push = vi.fn((url: string) => (mockSearch = url.replace(/^\?/, "")));
const replace = vi.fn((url: string) => (mockSearch = url.replace(/^\?/, "")));
vi.mock("next/navigation", () => ({
  useRouter: () => ({ push, replace }),
  useSearchParams: () => {
    if (cachedFor !== mockSearch) {
      cachedFor = mockSearch;
      cachedParams = new URLSearchParams(mockSearch);
    }
    return cachedParams!;
  },
}));

import { CodexGameShell } from "./CodexGameShell";

beforeEach(() => {
  vi.clearAllMocks();
  mockSearch = "";
  cachedParams = null;
  cachedFor = null;
  global.fetch = vi.fn(async () => ({
    text: async () => "",
    ok: true,
    json: async () => [],
  })) as never;
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe("<CodexGameShell> action dialog", () => {
  it("offers Cancel when an alternative action (End turn) exists", async () => {
    render(<CodexGameShell />);
    await screen.findByRole("button", { name: "Attack: Teleportation Attack" });
    expect(screen.getByRole("button", { name: "Cancel" })).toBeInTheDocument();
  });

  it("stays dismissed after Cancel instead of reopening itself", async () => {
    render(<CodexGameShell />);
    await screen.findByRole("button", { name: "Attack: Teleportation Attack" });

    await domAct(async () => {
      screen.getByRole("button", { name: "Cancel" }).click();
    });

    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    // The board underneath, with its own End turn button, stays reachable.
    await waitFor(() => expect(screen.getByRole("button", { name: "End turn" })).toBeVisible());
  });

  it("offers no Cancel for a mandatory choice with no legal fallback", async () => {
    replayStandard.mockReturnValueOnce(bonusDrawStub);
    render(<CodexGameShell />);
    await screen.findByRole("button", { name: "Take a bonus card" });
    expect(screen.queryByRole("button", { name: "Cancel" })).not.toBeInTheDocument();
  });
});
