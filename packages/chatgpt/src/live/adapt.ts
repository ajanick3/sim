// Maps the engine's wire shape to this package's own, much simpler
// display shape. The two aren't the same model: WireView carries hidden
// information (only your own hand); PokemonBoardState only ever wants
// what a viewer should see.
import type { ArtIndex } from "./art";
import { artUrl } from "./art";
import type { WireCard, WirePokemon, WireSide, WireView } from "./view";
import type { BoardCard, BoardPlayer, PokemonBoardState } from "../types";

function cardOf(art: ArtIndex, card: WireCard): BoardCard {
  return { id: card.id, name: card.name, imageUrl: artUrl(art, card.print_id) };
}

function monOf(art: ArtIndex, mon: WirePokemon): BoardCard {
  return { id: mon.id, name: mon.name, imageUrl: artUrl(art, mon.print_id), damage: mon.damage };
}

function playerOf(art: ArtIndex, side: WireSide, hand: WireCard[] | null): BoardPlayer {
  return {
    name: `Player ${side.player + 1}`,
    active: side.active ? monOf(art, side.active) : null,
    bench: side.bench.map((m) => monOf(art, m)),
    hand: hand ? hand.map((c) => cardOf(art, c)) : undefined,
    deckCount: side.deck_count,
    discardCount: side.discard.length,
    prizesRemaining: side.prize_count,
  };
}

/** `view.you` is this viewer's own seat — the wire never sends the
 *  opponent's hand, so it stays undefined and the board shows a count
 *  of card backs instead. Active/Bench Pokémon are public in the real
 *  game, so both sides' are shown truthfully. */
export function adaptView(view: WireView, art: ArtIndex): PokemonBoardState {
  const mine = view.sides[view.you];
  const theirs = view.sides[view.you === 0 ? 1 : 0];
  return {
    turn: view.turn_number,
    isPlayerTurn: view.current === view.you,
    stadiumName: view.stadium?.name,
    phaseLabel: view.phase,
    player: playerOf(art, mine, view.your_hand),
    opponent: playerOf(art, theirs, null),
  };
}
