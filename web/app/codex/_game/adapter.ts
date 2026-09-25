import type { WireActionMeta, WireCard, WirePokemon, WireView } from "../../view";
import type { BoardProps } from "../_components/regions/Board";
import type { SearchCard } from "../_components/dialogs/DeckSearchDialog";
import type { GameAction } from "../_components/dialogs/ActionDialog";

export type ArtResolver = (printId: string) => string | null;

export const deckAssetPath = (key: string) => `/decks/${key}.txt`;

const pokemon = (card: WirePokemon, art: ArtResolver) => ({
  id: card.id,
  name: card.name,
  imageUrl: art(card.print_id),
  hp: card.remaining_hp,
  damage: card.damage,
});

const topCard = (cards: WireCard[]) => cards.at(-1) ?? null;

export function boardFromView(view: WireView, art: ArtResolver): BoardProps {
  const player = view.sides[view.you];
  const opponent = view.sides[view.you === 0 ? 1 : 0];
  const playerDiscard = topCard(player.discard);
  const opponentDiscard = topCard(opponent.discard);
  return {
    active: player.active ? pokemon(player.active, art) : null,
    opponentActive: opponent.active ? pokemon(opponent.active, art) : null,
    bench: player.bench.map((card) => pokemon(card, art)),
    opponentBench: opponent.bench.map((card) => pokemon(card, art)),
    hand: view.your_hand.map((card) => ({
      id: card.id,
      name: card.name,
      imageUrl: art(card.print_id),
    })),
    deckCount: player.deck_count,
    opponentDeckCount: opponent.deck_count,
    discardCount: player.discard.length,
    opponentDiscardCount: opponent.discard.length,
    discardImageUrl: playerDiscard ? art(playerDiscard.print_id) : null,
    opponentDiscardImageUrl: opponentDiscard ? art(opponentDiscard.print_id) : null,
    prizesRemaining: player.prize_count,
    opponentPrizesRemaining: opponent.prize_count,
  };
}

export function searchCardsFromView(
  view: WireView,
  meta: WireActionMeta[],
  art: ArtResolver,
): SearchCard[] {
  const eligibleIds = new Set(meta.flatMap((action) => (action.card == null ? [] : [action.card])));
  return (view.deck_in_search ?? []).map((card) => ({
    id: card.id,
    name: card.name,
    imageUrl: art(card.print_id),
    eligible: eligibleIds.has(card.id),
  }));
}

export function actionIndexForCard(meta: WireActionMeta[], cardId: number): number[] {
  return meta.flatMap((action, index) => (action.card === cardId ? [index] : []));
}

export function actionChoices(actions: string[], indices: number[]): GameAction[] {
  return indices
    .filter((index) => !/^End turn$/i.test(actions[index] ?? ""))
    .map((index) => ({ id: index, label: actions[index] ?? `Action ${index + 1}` }));
}
