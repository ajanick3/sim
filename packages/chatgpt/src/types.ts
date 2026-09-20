export type BoardCard = {
  id: number;
  name: string;
  imageUrl?: string;
  damage?: number;
};

export type BoardPlayer = {
  name: string;
  active: BoardCard | null;
  bench: Array<BoardCard | null>;
  hand?: BoardCard[];
  deckCount: number;
  discardCount: number;
  prizesRemaining: number;
};

export type PokemonBoardState = {
  turn: number;
  isPlayerTurn: boolean;
  stadiumName?: string;
  phaseLabel?: string;
  player: BoardPlayer;
  opponent: BoardPlayer;
  log?: string[];
};

export type PokemonBoardProps = {
  state: PokemonBoardState;
  selectedCardId?: number | null;
  onCardSelect?: (card: BoardCard) => void;
  onEndTurn?: () => void;
  className?: string;
};

export type BoardViewport = "mobile" | "tablet" | "medium-desktop";
