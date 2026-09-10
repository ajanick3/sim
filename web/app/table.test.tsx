import { render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { encodeRecipe, readRecipeParam } from "./recipe";

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
      sides: [side(), side()],
    }),
  free: vi.fn(),
};
const replayStandard = vi.fn(() => gameStub);
const standard = vi.fn(() => gameStub);

vi.mock("./wasm", () => ({
  loadSim: vi.fn(async () => ({
    CardData: { new: vi.fn(() => ({ free: vi.fn() })) },
    Game: { standard, replay_standard: replayStandard, synthetic: vi.fn() },
  })),
}));

import Table from "./table";

beforeEach(() => {
  vi.clearAllMocks();
  window.history.replaceState(null, "", "/");
  global.fetch = vi.fn(async () => ({ text: async () => "" })) as never;
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe("<Table> recipe wiring", () => {
  it("starts a fresh game and writes a recipe to the address bar", async () => {
    render(<Table />);
    await screen.findByText("Copy link");

    await waitFor(() => {
      expect(window.location.search).toMatch(/^\?g=/);
    });
    const recipe = readRecipeParam(window.location.search);
    expect(recipe).toMatchObject({ a: "dragapult", b: "alakazam", moves: [] });
  });

  it("replays a game handed to it in the URL", async () => {
    window.history.replaceState(
      null,
      "",
      "/?g=" +
        encodeRecipe({
          v: 1,
          seed: 99,
          a: "dragapult",
          b: "alakazam",
          moves: [1, 0, 1],
        }),
    );

    render(<Table />);
    await screen.findByText("Copy link");

    await waitFor(() => expect(replayStandard).toHaveBeenCalled());
    const call = replayStandard.mock.calls[0] as unknown[];
    expect(call[3]).toBe(99n); // seed, as bigint
    expect(call[4]).toEqual([1, 0, 1]); // the moves to replay
    expect(standard).not.toHaveBeenCalled();
  });
});
