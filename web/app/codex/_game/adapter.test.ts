import { describe, expect, it } from "vitest";
import type { WireActionMeta, WireCard, WirePokemon, WireView } from "../../view";
import {
  actionChoices,
  actionIndexForCard,
  battlefieldFromView,
  deckAssetPath,
  searchCardsFromView,
} from "./adapter";

const card = (id: number, name = "Dreepy"): WireCard => ({
  id,
  name,
  def: id,
  print_id: `print-${id}`,
  energy_type: null,
  category: "pokemon",
});
const pokemon = (id: number): WirePokemon => ({
  id,
  name: "Dreepy",
  print_id: `print-${id}`,
  hp: 70,
  damage: 20,
  remaining_hp: 50,
  conditions: [],
  attached: [],
});
const view: WireView = {
  you: 1,
  current: 1,
  turn_number: 3,
  phase: "Main",
  your_hand: [card(1)],
  stadium: null,
  deck_in_search: [card(2), card(3)],
  counters_to_place: null,
  sides: [
    {
      player: 0,
      hand_count: 2,
      deck_count: 30,
      prize_count: 5,
      discard: [card(4, "Hammer")],
      active: pokemon(5),
      bench: [],
    },
    {
      player: 1,
      hand_count: 1,
      deck_count: 24,
      prize_count: 4,
      discard: [card(6, "Hammer")],
      active: pokemon(7),
      bench: [pokemon(8)],
    },
  ],
};
const meta: WireActionMeta[] = [
  { kind: "TakeCard", card: 3, target: null },
  { kind: "PlayCard", card: 1, target: null },
];

describe("codex game adapter", () => {
  it("maps the player-relative board and remaining health", () => {
    const board = battlefieldFromView(view, (id) => `/art/${id}`);
    expect(board.deckCount).toBe(24);
    expect(board.opponentDeckCount).toBe(30);
    expect(board.active).toMatchObject({ id: 7, hp: 50, damage: 20 });
    expect(board.hand[0]).toMatchObject({ id: 1, imageUrl: "/art/print-1" });
  });

  it("marks only searchable cards eligible", () => {
    expect(searchCardsFromView(view, meta, () => null).map((item) => item.eligible)).toEqual([
      false,
      true,
    ]);
  });

  it("preserves engine action indexes and excludes End turn from choices", () => {
    expect(actionIndexForCard(meta, 1)).toEqual([1]);
    expect(actionChoices(["Bite", "End turn"], [0, 1])).toEqual([{ id: 0, label: "Bite" }]);
  });

  it("builds a deck asset path from a recipe key, same as game-shell.tsx's deckPath", () => {
    expect(deckAssetPath("2026-worlds/003-brent-tonisson")).toBe(
      "/decks/2026-worlds/003-brent-tonisson.txt",
    );
  });
});
