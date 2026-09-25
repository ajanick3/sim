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
// A second stub: the mulligan bonus-draw phase. The engine always pairs
// "Take a bonus card" with "Decline bonus draws" (src/action.rs), so the
// way out is one of the dialog's own choices, not a separate Cancel —
// End turn is not legal yet, and there must be no Cancel button that
// merely dismisses the dialog without applying either choice.
const bonusDrawStub = {
  ...gameStub,
  legal_actions: () => JSON.stringify(["Take a bonus card", "Decline bonus draws"]),
  action_meta: () =>
    JSON.stringify([
      { kind: "TakeBonusDraw", card: null, target: null, is_fallback: false },
      { kind: "DeclineBonusDraws", card: null, target: null, is_fallback: true },
    ]),
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

// A third stub: an Active with both an Attack and a legal Retreat, and
// a Bench Pokémon to retreat to. Retreat's own action names the Bench
// Pokémon it promotes (src/action.rs), not the Active it retreats, so
// tapping the Active must gather both by kind — reported as no way to
// retreat at all, and as an unconfirmed attack when only Attack turned up.
const activeWithRetreatStub = {
  ...gameStub,
  legal_actions: () =>
    JSON.stringify(["Attack: Teleportation Attack", "Retreat, promoting Bench Buddy", "End turn"]),
  action_meta: () =>
    JSON.stringify([
      { kind: "Attack", card: null, target: null, is_fallback: false },
      { kind: "Retreat", card: null, target: 20, is_fallback: false },
      { kind: "EndTurn", card: null, target: null, is_fallback: true },
    ]),
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
      sides: [
        {
          ...side(),
          active: {
            id: 10,
            name: "Dreepy",
            print_id: "p1",
            hp: 70,
            damage: 0,
            remaining_hp: 70,
            conditions: [],
            attached: [],
          },
          bench: [
            {
              id: 20,
              name: "Bench Buddy",
              print_id: "p2",
              hp: 60,
              damage: 0,
              remaining_hp: 60,
              conditions: [],
              attached: [],
            },
          ],
        },
        side(),
      ],
    }),
};
// A fourth stub: only Retreat is legal off the Active (no Attack) — the
// single-match case that used to auto-apply without any confirmation.
const retreatOnlyStub = {
  ...activeWithRetreatStub,
  legal_actions: () => JSON.stringify(["Retreat, promoting Bench Buddy", "End turn"]),
  action_meta: () =>
    JSON.stringify([
      { kind: "Retreat", card: null, target: 20, is_fallback: false },
      { kind: "EndTurn", card: null, target: null, is_fallback: true },
    ]),
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

  it("offers the phase's own decline choice instead of a Cancel button", async () => {
    replayStandard.mockReturnValueOnce(bonusDrawStub);
    render(<CodexGameShell />);
    await screen.findByRole("button", { name: "Take a bonus card" });
    expect(screen.getByRole("button", { name: "Decline bonus draws" })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Cancel" })).not.toBeInTheDocument();
  });
});

describe("<CodexGameShell> tapping the Active", () => {
  it("offers both Attack and Retreat, and does not apply either on tap", async () => {
    replayStandard.mockReturnValueOnce(activeWithRetreatStub);
    render(<CodexGameShell />);
    const activeButton = await screen.findByRole("button", { name: "Dreepy, 70 HP remaining" });

    await domAct(async () => {
      activeButton.click();
    });

    expect(
      screen.getByRole("button", { name: "Attack: Teleportation Attack" }),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Retreat, promoting Bench Buddy" }),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Cancel" })).toBeInTheDocument();
    expect(activeWithRetreatStub.apply).not.toHaveBeenCalled();
  });

  it("still opens a dialog when Retreat is the only option off the Active", async () => {
    replayStandard.mockReturnValueOnce(retreatOnlyStub);
    render(<CodexGameShell />);
    const activeButton = await screen.findByRole("button", { name: "Dreepy, 70 HP remaining" });

    await domAct(async () => {
      activeButton.click();
    });

    expect(
      screen.getByRole("button", { name: "Retreat, promoting Bench Buddy" }),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Cancel" })).toBeInTheDocument();
    expect(retreatOnlyStub.apply).not.toHaveBeenCalled();
  });

  it("reopens the same choices after Cancel instead of applying one", async () => {
    replayStandard.mockReturnValueOnce(activeWithRetreatStub);
    render(<CodexGameShell />);
    const activeButton = await screen.findByRole("button", { name: "Dreepy, 70 HP remaining" });

    await domAct(async () => {
      activeButton.click();
    });
    await domAct(async () => {
      screen.getByRole("button", { name: "Cancel" }).click();
    });
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();

    await domAct(async () => {
      activeButton.click();
    });

    expect(
      screen.getByRole("button", { name: "Attack: Teleportation Attack" }),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Retreat, promoting Bench Buddy" }),
    ).toBeInTheDocument();
    expect(activeWithRetreatStub.apply).not.toHaveBeenCalled();
  });
});
