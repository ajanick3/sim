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

// A third stub: PlacingBench, with two Basics still placeable and Finish
// placing always legal alongside them (src/action.rs). Finish placing is
// a fallback (is_fallback: true) with no card/target, the same shape as
// the lone generic action the auto-shown dialog exists for — reported as
// an unprompted, undismissable "Choose an action" popup blocking the
// board as soon as the first Bench Pokémon went down.
const placingBenchStub = {
  ...gameStub,
  legal_actions: () =>
    JSON.stringify(["Place on Bench: Dreepy", "Place on Bench: Bench Buddy", "Finish placing"]),
  action_meta: () =>
    JSON.stringify([
      { kind: "PlaceOnBench", card: 1, target: null, is_fallback: false },
      { kind: "PlaceOnBench", card: 2, target: null, is_fallback: false },
      { kind: "FinishPlacing", card: null, target: null, is_fallback: true },
    ]),
  view: () =>
    JSON.stringify({
      you: 0,
      current: 0,
      turn_number: 0,
      phase: "PlacingBench",
      your_hand: [
        { id: 1, name: "Dreepy", def: 1, print_id: "p1", energy_type: null, category: "pokemon" },
        {
          id: 2,
          name: "Bench Buddy",
          def: 2,
          print_id: "p2",
          energy_type: null,
          category: "pokemon",
        },
      ],
      stadium: null,
      deck_in_search: null,
      counters_to_place: null,
      sides: [side(), side()],
    }),
};

// A fourth stub: an Active with both an Attack and a legal Retreat, and
// a Bench Pokémon to retreat to. Retreat's own action names the Bench
// Pokémon it promotes (src/action.rs), not the Active it retreats, so
// tapping the Active must gather both by kind — reported as no way to
// retreat at all, and as an unconfirmed attack when only Attack turned up.
const activeWithRetreatStub = {
  ...gameStub,
  legal_actions: () =>
    JSON.stringify(["Attack: Teleportation Attack", "Retreat, promoting Bench Buddy", "End turn"]),
  action_meta: () =>
    // Attack names the Active (id 10) as its own target, same as the real
    // engine post-fix — see crates/sim-wasm/src/lib.rs's action_handles —
    // so it does not also read as a card/target-free generic action here.
    JSON.stringify([
      { kind: "Attack", card: null, target: 10, is_fallback: false },
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
// A fifth stub: only Retreat is legal off the Active (no Attack) — the
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

  it("offers the phase's own decline choice as a persistent button, not a Cancel", async () => {
    replayStandard.mockReturnValueOnce(bonusDrawStub);
    render(<CodexGameShell />);
    await screen.findByRole("button", { name: "Take a bonus card" });
    const decline = screen.getByRole("button", { name: "Decline bonus draws" });
    expect(decline).toBeInTheDocument();
    // A fallback like this is never a dialog choice — see the
    // "does not block a second placement" test below for why.
    expect(decline.closest('[role="dialog"]')).toBeNull();
    expect(screen.queryByRole("button", { name: "Cancel" })).not.toBeInTheDocument();
  });
});

describe("<CodexGameShell> a fallback action off the dialog", () => {
  it("does not block placing a second Pokémon behind an unprompted popup", async () => {
    replayStandard.mockReturnValueOnce(placingBenchStub);
    render(<CodexGameShell />);

    // Finish placing is always legal here, but it must render as a
    // persistent button (like End turn), never as an auto-shown,
    // undismissable "Choose an action" dialog sitting over the board.
    await screen.findByRole("button", { name: "Finish placing" });
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();

    // The second Bench Pokémon is still directly tappable underneath.
    expect(screen.getByRole("button", { name: "Bench Buddy" })).toBeInTheDocument();
  });
});

describe("<CodexGameShell> tapping the Active", () => {
  it("offers both Attack and a single Retreat choice, and does not apply either on tap", async () => {
    replayStandard.mockReturnValueOnce(activeWithRetreatStub);
    render(<CodexGameShell />);
    const activeButton = await screen.findByRole("button", { name: "Dreepy, 70 HP remaining" });

    await domAct(async () => {
      activeButton.click();
    });

    expect(
      screen.getByRole("button", { name: "Attack: Teleportation Attack" }),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Retreat" })).toBeInTheDocument();
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

    expect(screen.getByRole("button", { name: "Retreat" })).toBeInTheDocument();
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
    expect(screen.getByRole("button", { name: "Retreat" })).toBeInTheDocument();
    expect(activeWithRetreatStub.apply).not.toHaveBeenCalled();
  });
});

describe("<CodexGameShell> retreating", () => {
  it("does not retreat on a direct tap of the Bench, with no Active tap first", async () => {
    replayStandard.mockReturnValueOnce(activeWithRetreatStub);
    render(<CodexGameShell />);
    const benchButton = await screen.findByRole("button", { name: "Bench Buddy, 60 HP remaining" });

    await domAct(async () => {
      benchButton.click();
    });

    expect(activeWithRetreatStub.apply).not.toHaveBeenCalled();
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
  });

  it("retreats onto the Bench Pokémon tapped after choosing Retreat off the Active", async () => {
    replayStandard.mockReturnValueOnce(activeWithRetreatStub);
    render(<CodexGameShell />);
    const activeButton = await screen.findByRole("button", { name: "Dreepy, 70 HP remaining" });

    await domAct(async () => {
      activeButton.click();
    });
    await domAct(async () => {
      screen.getByRole("button", { name: "Retreat" }).click();
    });
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    expect(activeWithRetreatStub.apply).not.toHaveBeenCalled();

    const benchButton = await screen.findByRole("button", { name: "Bench Buddy, 60 HP remaining" });
    await domAct(async () => {
      benchButton.click();
    });

    // Retreat is index 1 in activeWithRetreatStub's action_meta/legal_actions.
    expect(activeWithRetreatStub.apply).toHaveBeenCalledWith(1);
  });

  it("cancels the armed retreat on a tap elsewhere without applying anything", async () => {
    replayStandard.mockReturnValueOnce(activeWithRetreatStub);
    render(<CodexGameShell />);
    const activeButton = await screen.findByRole("button", { name: "Dreepy, 70 HP remaining" });

    await domAct(async () => {
      activeButton.click();
    });
    await domAct(async () => {
      screen.getByRole("button", { name: "Retreat" }).click();
    });

    await domAct(async () => {
      activeButton.click();
    });

    // Back to the Active's own dialog, not an applied retreat.
    expect(
      screen.getByRole("button", { name: "Attack: Teleportation Attack" }),
    ).toBeInTheDocument();
    expect(activeWithRetreatStub.apply).not.toHaveBeenCalled();
  });
});
