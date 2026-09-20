import Button from "@mui/material/Button";
import useMediaQuery from "@mui/material/useMediaQuery";
import { CardSurface, EmptyCardSlot } from "./CardSurface";
import type { BoardCard, BoardPlayer, BoardViewport, PokemonBoardProps } from "./types";
import "./PokemonBoard.css";

const BENCH_SIZE = 5;
const MOBILE_QUERY = "(max-width: 599px)";
const MEDIUM_DESKTOP_QUERY = "(min-width: 900px)";

function benchSlots(cards: Array<BoardCard | null>) {
  return [...cards.slice(0, BENCH_SIZE), ...Array<BoardCard | null>(BENCH_SIZE).fill(null)].slice(0, BENCH_SIZE);
}

function findSelectedCard(props: PokemonBoardProps): BoardCard | null {
  const { state, selectedCardId } = props;
  if (selectedCardId == null) return null;
  const cards = [
    state.player.active,
    ...state.player.bench,
    ...(state.player.hand ?? []),
    state.opponent.active,
    ...state.opponent.bench,
  ];
  return cards.find((card) => card?.id === selectedCardId) ?? null;
}

function Bench({ player, props, label }: { player: BoardPlayer; props: PokemonBoardProps; label: string }) {
  return (
    <div className="chatgpt-bench" aria-label={label}>
      {benchSlots(player.bench).map((card, index) =>
        card ? (
          <CardSurface
            key={card.id}
            card={card}
            selected={props.selectedCardId === card.id}
            onSelect={props.onCardSelect}
          />
        ) : (
          <EmptyCardSlot key={`empty-${index}`} label={`${label} empty slot ${index + 1}`} />
        ),
      )}
    </div>
  );
}

function Active({ player, props, label }: { player: BoardPlayer; props: PokemonBoardProps; label: string }) {
  return (
    <div className="chatgpt-active" aria-label={label}>
      {player.active ? (
        <CardSurface
          card={player.active}
          selected={props.selectedCardId === player.active.id}
          onSelect={props.onCardSelect}
        />
      ) : (
        <EmptyCardSlot label={`${label} empty slot`} />
      )}
    </div>
  );
}

function Prizes({ player, owner }: { player: BoardPlayer; owner: string }) {
  return (
    <div className="chatgpt-prizes" aria-label={`${owner} prizes: ${player.prizesRemaining} remaining`}>
      {Array.from({ length: 6 }, (_, index) =>
        index < player.prizesRemaining ? (
          <CardSurface
            key={index}
            card={{ id: -(index + 1), name: `${owner} prize ${index + 1}` }}
            className="chatgpt-prize-card"
          />
        ) : (
          <EmptyCardSlot key={index} label={`${owner} taken prize ${index + 1}`} />
        ),
      )}
    </div>
  );
}

function Piles({ player, owner }: { player: BoardPlayer; owner: string }) {
  return (
    <div className="chatgpt-piles">
      <div className="chatgpt-pile">
        <CardSurface card={{ id: -101, name: `${owner} deck` }} className="chatgpt-pile-card" />
        <span>{player.deckCount} deck</span>
      </div>
      <div className="chatgpt-pile">
        <CardSurface card={{ id: -102, name: `${owner} discard` }} className="chatgpt-pile-card" />
        <span>{player.discardCount} discard</span>
      </div>
    </div>
  );
}

function PlayerSide({ side, player, props }: { side: "opponent" | "player"; player: BoardPlayer; props: PokemonBoardProps }) {
  const owner = side === "player" ? "Your" : "Opponent";
  return (
    <section className={`chatgpt-side chatgpt-side--${side}`} aria-label={`${owner} field`}>
      <div className="chatgpt-side__piles">
        <Piles player={player} owner={owner} />
      </div>
      <div className="chatgpt-side__bench">
        <Bench player={player} props={props} label={`${owner} bench`} />
      </div>
      <div className="chatgpt-side__active">
        <Active player={player} props={props} label={`${owner} active`} />
      </div>
      <div className="chatgpt-side__prizes">
        <Prizes player={player} owner={owner} />
      </div>
    </section>
  );
}

function Court({ props }: { props: PokemonBoardProps }) {
  const { state } = props;
  return (
    <div className="chatgpt-court-viewport">
      <main className="chatgpt-court-plane">
        <PlayerSide side="opponent" player={state.opponent} props={props} />
        <div className="chatgpt-center-line" aria-label={state.stadiumName ? `Stadium: ${state.stadiumName}` : "No stadium"}>
          {state.stadiumName && <span>{state.stadiumName}</span>}
        </div>
        <PlayerSide side="player" player={state.player} props={props} />
      </main>
    </div>
  );
}

function Hand({ props, viewport }: { props: PokemonBoardProps; viewport: BoardViewport }) {
  const cards = props.state.player.hand ?? [];
  return (
    <section className="chatgpt-hand" aria-label={`Your hand: ${cards.length} cards`}>
      <div className="chatgpt-hand__heading">
        <strong>Your hand · {cards.length}</strong>
        <span>{viewport === "mobile" ? "Tap a card for actions" : "Select a card to inspect"}</span>
      </div>
      <div
        className="chatgpt-hand__grid"
        data-columns={viewport === "mobile" ? 5 : 7}
      >
        {cards.map((card) => (
          <CardSurface
            key={card.id}
            card={card}
            selected={props.selectedCardId === card.id}
            onSelect={props.onCardSelect}
          />
        ))}
      </div>
    </section>
  );
}

function Header({ props }: { props: PokemonBoardProps }) {
  const { state } = props;
  return (
    <header className="chatgpt-board-header">
      <div className="chatgpt-player-name">
        <span className="chatgpt-avatar" aria-hidden="true">NA</span>
        <span>{state.player.name} vs {state.opponent.name}</span>
      </div>
      <span className="chatgpt-turn-label">Turn {state.turn} · {state.isPlayerTurn ? "Your move" : "Waiting"}</span>
    </header>
  );
}

function ActionPanel({ props, compact = false }: { props: PokemonBoardProps; compact?: boolean }) {
  const selectedCard = findSelectedCard(props);
  return (
    <aside className={`chatgpt-actions${compact ? " chatgpt-actions--compact" : ""}`} aria-label="Card actions">
      {!compact && (
        <div className="chatgpt-actions__selected">
          {selectedCard ? <CardSurface card={selectedCard} className="chatgpt-inspector-card" /> : null}
          <div>
            <strong>{selectedCard?.name ?? "No card selected"}</strong>
            <span>{selectedCard ? "Item · Trainer" : "Select a card on the board"}</span>
          </div>
        </div>
      )}
      <div className="chatgpt-actions__buttons">
        <Button variant="contained" size="small" disabled={!selectedCard}>Play card</Button>
        <Button variant="outlined" size="small" disabled={!selectedCard}>View larger</Button>
        <Button variant="outlined" size="small" onClick={props.onEndTurn}>End turn</Button>
      </div>
      {!compact && props.state.log && props.state.log.length > 0 && (
        <div className="chatgpt-game-log">
          <strong>Game log</strong>
          {props.state.log.slice(0, 3).map((entry, index) => <span key={`${entry}-${index}`}>{entry}</span>)}
        </div>
      )}
    </aside>
  );
}

function PokemonBoardLayout({ viewport, ...props }: PokemonBoardProps & { viewport: BoardViewport }) {
  return (
    <div
      className={`chatgpt-board chatgpt-board--${viewport}${props.className ? ` ${props.className}` : ""}`}
      data-layout={viewport}
    >
      <Header props={props} />
      <div className="chatgpt-board__body">
        <div className="chatgpt-board__main">
          <Court props={props} />
          <Hand props={props} viewport={viewport} />
        </div>
        {viewport === "tablet" && <ActionPanel props={props} compact />}
        {viewport === "medium-desktop" && <ActionPanel props={props} />}
      </div>
      {viewport === "mobile" && (
        <div className="chatgpt-mobile-status">
          <span>{props.state.phaseLabel ?? "Main phase"}</span>
          <Button variant="contained" size="small" onClick={props.onEndTurn}>End turn</Button>
        </div>
      )}
    </div>
  );
}

export function MobilePokemonBoard(props: PokemonBoardProps) {
  return <PokemonBoardLayout {...props} viewport="mobile" />;
}

export function TabletPokemonBoard(props: PokemonBoardProps) {
  return <PokemonBoardLayout {...props} viewport="tablet" />;
}

export function MediumDesktopPokemonBoard(props: PokemonBoardProps) {
  return <PokemonBoardLayout {...props} viewport="medium-desktop" />;
}

export function PokemonBoard(props: PokemonBoardProps) {
  const isMobile = useMediaQuery(MOBILE_QUERY, { noSsr: true });
  const isMediumDesktop = useMediaQuery(MEDIUM_DESKTOP_QUERY, { noSsr: true });
  if (isMobile) return <MobilePokemonBoard {...props} />;
  if (isMediumDesktop) return <MediumDesktopPokemonBoard {...props} />;
  return <TabletPokemonBoard {...props} />;
}
