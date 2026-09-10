import { act as domAct, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { encodeRecipe } from "./recipe";

// A stub wasm handle: two legal actions, no moves, a minimal board.
const side = () => ({
  player: 0,
  hand_count: 0,
  library_count: 0,
  prize_count: 0,
  discard: [],
  active: null,
  bench: [],
});
const gameStub = {
  legal_actions: () => JSON.stringify(["Action A", "Action B"]),
  action_meta: () =>
    JSON.stringify([
      { kind: "Other", card: null, target: null },
      { kind: "Other", card: null, target: null },
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
      turn_number: 0,
      phase: "Main",
      your_hand: [],
      stadium: null,
      sides: [side(), side()],
    }),
  free: vi.fn(),
};
const replayStandard = vi.fn(() => gameStub);

vi.mock("./wasm", () => ({
  loadSim: vi.fn(async () => ({
    CardData: { new: vi.fn(() => ({ free: vi.fn() })) },
    Game: { standard: vi.fn(), replay_standard: replayStandard, synthetic: vi.fn() },
  })),
}));

// A controllable stand-in for the app router and the query string.
let mockSearch = "";
let cachedParams: URLSearchParams | null = null;
let cachedFor: string | null = null;
const push = vi.fn((url: string) => setSearch(url.replace(/^\?/, "")));
const replace = vi.fn((url: string) => setSearch(url.replace(/^\?/, "")));
function setSearch(next: string) {
  mockSearch = next;
}
const routerStub = { push, replace };
vi.mock("next/navigation", () => ({
  useRouter: () => routerStub,
  useSearchParams: () => {
    if (cachedFor !== mockSearch) {
      cachedFor = mockSearch;
      cachedParams = new URLSearchParams(mockSearch);
    }
    return cachedParams!;
  },
}));

import GameShell from "./game-shell";

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

describe("<GameShell> recipe wiring", () => {
  it("starts a fresh game and replaces the URL with its recipe", async () => {
    render(<GameShell />);
    await screen.findByText("Copy link");
    await waitFor(() => expect(replace).toHaveBeenCalled());
    expect(replace.mock.calls[0][0]).toMatch(/^\?g=/);
  });

  it("replays the recipe the URL carries", async () => {
    mockSearch = "g=" + encodeRecipe({ v: 1, seed: 99, a: "one", b: "two", moves: [1, 0, 1] });

    render(<GameShell />);
    await screen.findByText("Copy link");

    await waitFor(() => expect(replayStandard).toHaveBeenCalled());
    const call = replayStandard.mock.calls[0] as unknown[];
    expect(call[3]).toBe(99n);
    expect(call[4]).toEqual([1, 0, 1]);
  });

  it("rebuilds the game when Back or Forward changes ?g=", async () => {
    const { rerender } = render(<GameShell />);
    await screen.findByText("Copy link");
    replayStandard.mockClear();

    mockSearch = "g=" + encodeRecipe({ v: 1, seed: 7, a: "one", b: "two", moves: [2, 5] });
    rerender(<GameShell />);

    await waitFor(() => expect(replayStandard).toHaveBeenCalled());
    const call = replayStandard.mock.calls.at(-1) as unknown[];
    expect(call[3]).toBe(7n);
    expect(call[4]).toEqual([2, 5]);
  });

  it("pushes the recipe after a move", async () => {
    render(<GameShell />);
    const button = await screen.findByRole("button", { name: "Action A", hidden: true });
    push.mockClear();

    await domAct(async () => {
      button.click();
    });

    await waitFor(() => expect(push).toHaveBeenCalled());
    expect(push.mock.calls[0][0]).toMatch(/^\?g=/);
    expect(gameStub.apply).toHaveBeenCalledWith(0);
  });
});
