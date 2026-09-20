import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { PokemonBoard } from "../PokemonBoard";
import type { BoardCard } from "../types";
import { adaptView } from "./adapt";
import { loadArtIndex, type ArtIndex } from "./art";
import {
  ACTION_GROUP_ORDER,
  groupActions,
  movesForSelection,
  shouldAutoAdvance,
  targetsForHandCard,
  type Selection,
} from "./session";
import type { WireActionMeta, WireView } from "./view";
import { loadSim, type CardData, type Game } from "./wasm";
import "../PokemonBoard.css";
import "./App.css";

const DEFAULT_DECKS = { a: "003-brent-tonisson", b: "002-diego-cassiraga" };
const params = new URLSearchParams(window.location.search);
const DECK_A = params.get("a") ?? DEFAULT_DECKS.a;
const DECK_B = params.get("b") ?? DEFAULT_DECKS.b;
const SEED = BigInt(params.get("seed") ?? String(Math.floor(Math.random() * 1_000_000_000)));

type Status = { kind: "loading" } | { kind: "error"; message: string } | { kind: "playing" };

/**
 * The whole game, driven by the real engine, drawn with this package's
 * own board — in complete isolation from web/'s Next.js app (its own
 * React, its own MUI, no shared runtime). Compared to web/'s GameShell:
 * no recipe/URL sync or Back/Forward replay, and no pass-the-device
 * privacy gate — this is a single-device A/B comparison demo, not the
 * shipped game. Every legal move stays reachable through the "All legal
 * moves" list below the board even where a card tap doesn't cover it
 * (coin flips, decisions, promoting, …).
 */
export default function App() {
  const [status, setStatus] = useState<Status>({ kind: "loading" });
  const [view, setView] = useState<WireView | null>(null);
  const [actions, setActions] = useState<string[]>([]);
  const [meta, setMeta] = useState<WireActionMeta[]>([]);
  const [selection, setSelection] = useState<Selection>(null);
  const [art, setArt] = useState<ArtIndex>({});
  const [log, setLog] = useState<string[]>([]);
  const [over, setOver] = useState(false);
  const [busy, setBusy] = useState(false);

  const gameRef = useRef<Game | null>(null);

  const refresh = useCallback(() => {
    const game = gameRef.current;
    if (!game) return;
    setView(JSON.parse(game.view()) as WireView);
    setActions(JSON.parse(game.legal_actions()) as string[]);
    setMeta(JSON.parse(game.action_meta()) as WireActionMeta[]);
    setSelection(null);
    setLog(JSON.parse(game.log()) as string[]);
    setOver(game.is_over());
  }, []);

  useEffect(() => {
    void loadArtIndex().then(setArt);
    let cancelled = false;
    (async () => {
      try {
        const sim = await loadSim();
        const cardsJson = await fetch("/cards.json").then((r) => r.text());
        const data: CardData = sim.CardData.new(cardsJson);
        const [a, b] = await Promise.all([
          fetch(`/decks/${DECK_A}.txt`).then((r) => r.text()),
          fetch(`/decks/${DECK_B}.txt`).then((r) => r.text()),
        ]);
        if (cancelled) return;
        gameRef.current = sim.Game.standard(data, a, b, SEED);
        setStatus({ kind: "playing" });
        refresh();
      } catch (err) {
        if (!cancelled) setStatus({ kind: "error", message: String(err) });
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [refresh]);

  const act = useCallback(
    (index: number) => {
      const game = gameRef.current;
      if (!game || busy) return;
      setBusy(true);
      try {
        game.apply(index);
        refresh();
      } catch (err) {
        setStatus({ kind: "error", message: String(err) });
      } finally {
        setBusy(false);
      }
    },
    [busy, refresh],
  );

  const autoSteps = useRef(0);
  useEffect(() => {
    if (actions.length !== 1) {
      autoSteps.current = 0;
      return;
    }
    if (
      shouldAutoAdvance({
        actionCount: actions.length,
        playing: status.kind === "playing",
        over,
        busy,
        steps: autoSteps.current,
        phase: view?.phase,
      })
    ) {
      autoSteps.current += 1;
      act(0);
    }
  }, [actions, status.kind, over, busy, act, view]);

  const monIds = useMemo(() => {
    const ids = new Set<number>();
    if (!view) return ids;
    for (const side of view.sides) {
      if (side.active) ids.add(side.active.id);
      for (const m of side.bench) ids.add(m.id);
    }
    return ids;
  }, [view]);

  const handIds = useMemo(
    () => new Set(view ? view.your_hand.map((c) => c.id) : []),
    [view],
  );

  const handleSelect = useCallback(
    (card: BoardCard) => {
      if (busy) return;
      const id = card.id;

      if (selection?.kind === "hand") {
        const landing = targetsForHandCard(meta, selection.card).get(id);
        if (landing !== undefined) {
          act(landing);
          return;
        }
      }

      if (handIds.has(id)) {
        if (selection?.kind === "hand" && selection.card === id) {
          const mv = movesForSelection(meta, selection);
          if (mv.length === 1) act(mv[0]);
          else setSelection(null);
          return;
        }
        setSelection({ kind: "hand", card: id });
        return;
      }

      if (monIds.has(id)) {
        if (selection?.kind === "pokemon" && selection.id === id) {
          const mv = movesForSelection(meta, selection);
          if (mv.length === 1) act(mv[0]);
          else setSelection(null);
          return;
        }
        setSelection({ kind: "pokemon", id });
        return;
      }
      // Deck / discard / prize dummy cards (negative ids) carry no move.
    },
    [act, busy, handIds, meta, monIds, selection],
  );

  const handleEndTurn = useCallback(() => {
    const index = actions.findIndex((a) => a === "End turn" || a === "End your turn");
    if (index >= 0) act(index);
  }, [act, actions]);

  if (status.kind === "loading") {
    return (
      <div className="live-app">
        <div className="live-app__centre">Loading the engine…</div>
      </div>
    );
  }
  if (status.kind === "error") {
    return (
      <div className="live-app">
        <div className="live-app__centre">{status.message}</div>
      </div>
    );
  }
  if (!view) return null;

  const state = { ...adaptView(view, art), log };
  const selectedCardId =
    selection?.kind === "hand" ? selection.card : selection?.kind === "pokemon" ? selection.id : null;
  const groups = groupActions(actions);

  return (
    <div className="live-app">
      <div className="live-app__header">
        <span className="live-app__badge">chatgpt board · A/B · isolated build</span>
        <span>
          Turn {view.turn_number} · {view.phase}
        </span>
      </div>

      {over ? (
        <div className="live-app__centre">{log[log.length - 1] ?? "Game over."}</div>
      ) : (
        <>
          <PokemonBoard
            state={state}
            selectedCardId={selectedCardId}
            onCardSelect={handleSelect}
            onEndTurn={handleEndTurn}
          />
          {groups.length > 0 && (
            <div className="live-app__moves">
              <div className="live-app__moves-heading">All legal moves</div>
              {ACTION_GROUP_ORDER.filter((g) => groups.some((group) => group.group === g)).map((g) => {
                const group = groups.find((x) => x.group === g)!;
                return (
                  <div key={g} className="live-app__move-group">
                    <span className="live-app__move-group-label">{g}:</span>
                    {group.items.map((item) => (
                      <button
                        key={item.index}
                        disabled={busy}
                        onClick={() => act(item.index)}
                        className="live-app__move-button"
                      >
                        {item.label}
                        {item.copy !== undefined ? ` (${item.copy + 1})` : ""}
                      </button>
                    ))}
                  </div>
                );
              })}
            </div>
          )}
        </>
      )}
    </div>
  );
}
