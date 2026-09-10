import { act as domAct, render, screen, waitFor } from "@testing-library/react";
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

import Table, { Mon } from "./table";
import type { WirePokemon } from "./view";

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

  it("rebuilds the game to whatever recipe Back or Forward lands on", async () => {
    render(<Table />);
    await screen.findByText("Copy link");
    replayStandard.mockClear();

    // The browser moved the URL to a two-move recipe; fire popstate.
    window.history.replaceState(
      null,
      "",
      "/?g=" +
        encodeRecipe({
          v: 1,
          seed: 7,
          a: "dragapult",
          b: "alakazam",
          moves: [2, 5],
        }),
    );
    await domAct(async () => {
      window.dispatchEvent(new PopStateEvent("popstate"));
    });

    await waitFor(() => expect(replayStandard).toHaveBeenCalled());
    const call = replayStandard.mock.calls.at(-1) as unknown[];
    expect(call[3]).toBe(7n);
    expect(call[4]).toEqual([2, 5]);
  });

  it("pushes a history entry for each move so Back steps through them", async () => {
    const pushSpy = vi.spyOn(window.history, "pushState");
    render(<Table />);

    // Past the reveal gate first: the action buttons hide behind it.
    const reveal = await screen.findByRole("button", { name: /tap to reveal/ });
    await domAct(async () => {
      reveal.click();
    });

    const button = await screen.findByRole("button", { name: "Action A" });
    pushSpy.mockClear();

    await domAct(async () => {
      button.click();
    });

    await waitFor(() => expect(pushSpy).toHaveBeenCalledTimes(1));
    expect(gameStub.apply).toHaveBeenCalledWith(0);
    pushSpy.mockRestore();
  });
});

describe("<Mon> card shape", () => {
  const bare: WirePokemon = {
    name: "Pikachu",
    hp: 60,
    damage: 0,
    remaining_hp: 60,
    conditions: [],
    attached: [],
  };
  const loaded: WirePokemon = {
    name: "A Very Long Pokemon Name ex",
    hp: 340,
    damage: 120,
    remaining_hp: 220,
    conditions: ["Asleep", "Poisoned"],
    attached: [
      { id: 1, name: "Fire Energy", def: 0, energy_type: "Fire" },
      { id: 2, name: "Water Energy", def: 0, energy_type: "Water" },
      { id: 3, name: "Rescue Board", def: 0, energy_type: null },
    ],
  };

  it("renders every Pokemon at the same width whatever it carries", () => {
    render(
      <>
        <Mon mon={bare} />
        <Mon mon={loaded} />
        <Mon mon={null} />
      </>,
    );
    const [a, b, c] = screen.getAllByTestId("mon-card");
    expect(b.style.width).toBe(a.style.width);
    expect(c.style.width).toBe(a.style.width);
    expect(a.style.width).not.toBe("");
    expect(b.style.minHeight).toBe(a.style.minHeight);
  });

  it("draws a copy badge only when a copy index is given", () => {
    const { rerender } = render(<Mon mon={bare} />);
    expect(screen.queryByTestId("copy-badge")).toBeNull();

    rerender(<Mon mon={bare} copy={2} />);
    const badge = screen.getByTestId("copy-badge");
    expect(badge.style.background).toBe("rgb(90, 167, 228)"); // COPY_COLORS[2], blue
  });
});
