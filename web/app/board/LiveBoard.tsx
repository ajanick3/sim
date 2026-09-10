"use client";

// The board: a felt mat with prize stacks, deck and discard piles, a
// stadium slot, bench trays and a hand strip, a right-hand rail with the
// prize counts and END TURN, and a decision bar for search / discard
// prompts. `game-shell.tsx` drives it; this file only lays out a view.

import { useEffect, useRef, useState } from "react";
import type { PointerEvent as ReactPointerEvent } from "react";
import { ActionPanel } from "./ActionPanel";
import { movesForSelection, targetsForHandCard, type Selection } from "../session";
import type { WireActionMeta, WireCard, WireView } from "../view";
import { CardFace } from "./CardFace";
import { CoinFlip } from "./CoinFlip";
import { DecisionBar } from "./DecisionBar";
import { HandStrip } from "./HandStrip";
import { LiveMon } from "./LiveMon";
import { PromptBar } from "./PromptBar";
import { SideRail } from "./SideRail";
import { SideRow } from "./SideRow";
import { StadiumSlot } from "./StadiumSlot";
import { asDecision, asPrompt, coinFlipsIn, monHooks, SEAT_NAME, type Art } from "./shared";
import type { CoinResult } from "./shared";

export function LiveBoard({
  view,
  actions,
  meta,
  selection,
  onSelect,
  art,
  seat,
  busy,
  onAct,
  log,
}: {
  view: WireView;
  actions: string[];
  meta: WireActionMeta[];
  selection: Selection;
  onSelect: (s: Selection) => void;
  art: Art;
  seat: number | undefined;
  busy: boolean;
  onAct: (index: number) => void;
  log: string[];
}) {
  const you = view.you;
  const mine = view.sides[you];
  const opp = view.sides[you === 1 ? 0 : 1];
  const endTurn = meta.findIndex((m) => m.kind === "EndTurn");
  // Setup: once the Bench is as the player wants it, this ends placing.
  const finishPlacing = meta.findIndex((m) => m.kind === "FinishPlacing");
  const [showLog, setShowLog] = useState(false);
  const [showRail, setShowRail] = useState(true);
  const [discardView, setDiscardView] = useState<{ label: string; cards: WireCard[] } | null>(null);

  const dropTargets =
    selection?.kind === "hand"
      ? targetsForHandCard(meta, selection.card)
      : new Map<number, number>();
  const onPokemon = (id: number) => {
    const landing = dropTargets.get(id);
    if (landing !== undefined) {
      onAct(landing);
      return;
    }
    // Second tap on an already-selected Pokémon whose only move names it
    // — promoting a Benched Pokémon after a Knockout, above all — plays
    // that move. First tap just selects it.
    if (selection?.kind === "pokemon" && selection.id === id) {
      const mv = movesForSelection(meta, selection);
      if (mv.length === 1) {
        onAct(mv[0]);
        return;
      }
      onSelect(null);
      return;
    }
    onSelect({ kind: "pokemon", id });
  };
  const onHand = (card: number) =>
    onSelect(selection?.kind === "hand" && selection.card === card ? null : { kind: "hand", card });

  // Attacks name no target on the wire — the attacker is always your
  // Active — so pair them with your Active by hand. Tapping the Active
  // card then shows them right on it.
  const attackMoves = meta
    .map((m, i) => ({ m, i }))
    .filter(({ m }) => m.kind === "Attack")
    .map(({ i }) => ({ index: i, label: actions[i].replace(/^Attack:?\s*/, "") }));
  // Retreat names the promoted Bench Pokémon as its target, not the
  // Active — but the Active is what retreats, so put the choice on it.
  const retreatMoves = meta
    .map((m, i) => ({ m, i }))
    .filter(({ m }) => m.kind === "Retreat")
    .map(({ i }) => ({
      index: i,
      label: actions[i].replace(/^Retreat, promoting /, "Retreat ▸ "),
    }));
  const onCardMoves = [...attackMoves, ...retreatMoves];
  const activeSelected =
    selection?.kind === "pokemon" && mine.active != null && selection.id === mine.active.id;

  // A selected Pokémon's usable Abilities — `UseAbility` now names its
  // carrier as the target, so tapping the card brings them up.
  const abilityMoves =
    selection?.kind === "pokemon"
      ? meta
          .map((m, i) => ({ m, i }))
          .filter(({ m }) => m.kind === "UseAbility" && m.target === selection.id)
          .map(({ i }) => ({ index: i, label: actions[i].replace(/^Use .*?'s /, "") }))
      : [];

  const decision = asDecision(actions);
  // A yes/no Ability prompt ("Use Psychic Draw" / "Decline Psychic
  // Draw") — surfaced as its own bar, not left in the All-actions list.
  const prompt = decision ? null : asPrompt(actions);
  // After a Knockout the player must pick a new Active from the Bench.
  const promoting = actions.length > 0 && actions.every((a) => /^Promote /.test(a));
  // The coin-flip winner picks who starts — a full-board modal, not two
  // buttons in a list.
  const firstTurn =
    actions.length > 0 && actions.every((a) => /takes the first turn$/.test(a))
      ? {
          goFirst: actions.findIndex((a) => a.startsWith(["One", "Two"][you])),
          goSecond: actions.findIndex((a) => !a.startsWith(["One", "Two"][you])),
        }
      : null;
  // A tapped hand card (or Pokémon) narrows the action panel to just its
  // moves, so the next step is a short list, not the whole turn.
  const only = selection ? movesForSelection(meta, selection) : undefined;
  // A selected card with one unambiguous move shows a ✅ over itself.
  const confirmIndex =
    selection?.kind === "hand" && only && only.length === 1 ? only[0] : undefined;

  // Coin flips: the engine logs each one. Watch the log grow and play a
  // spin for whatever landed since last render. `null` until first seen,
  // so a replayed game's existing flips are not re-animated.
  const [flipRun, setFlipRun] = useState<{ id: number; results: CoinResult[] } | null>(null);
  const seenLogLen = useRef<number | null>(null);
  useEffect(() => {
    if (seenLogLen.current === null || log.length < seenLogLen.current) {
      seenLogLen.current = log.length;
      return;
    }
    if (log.length > seenLogLen.current) {
      const results = coinFlipsIn(log.slice(seenLogLen.current));
      seenLogLen.current = log.length;
      if (results.length > 0) setFlipRun({ id: Date.now(), results });
    }
  }, [log]);

  // Click anywhere that isn't part of the selection flow to cancel it.
  useEffect(() => {
    if (!selection) return;
    const onDown = (e: Event) => {
      if (!(e.target as HTMLElement).closest("[data-keep-selection]")) onSelect(null);
    };
    document.addEventListener("pointerdown", onDown, true);
    return () => document.removeEventListener("pointerdown", onDown, true);
  }, [selection, onSelect]);

  // A selected hand card that can go to the Bench / the empty Active spot —
  // tapped straight onto an empty slot, no button needed.
  const benchPlace =
    selection?.kind === "hand"
      ? meta.findIndex(
          (m) =>
            m.card === selection.card &&
            m.target === null &&
            (m.kind === "PlayBasic" || m.kind === "PlaceOnBench"),
        )
      : -1;
  const activePlace =
    selection?.kind === "hand"
      ? meta.findIndex(
          (m) => m.card === selection.card && m.target === null && m.kind === "PlaceActive",
        )
      : -1;
  const selectedName =
    selection?.kind === "hand"
      ? view.your_hand.find((c) => c.id === selection.card)?.name
      : selection?.kind === "pokemon"
        ? [mine.active, ...mine.bench, opp.active, ...opp.bench].find((m) => m?.id === selection.id)
            ?.name
        : undefined;

  // --- Drag a hand card onto the board --------------------------------
  // A pointer drag past a small threshold selects the card (so the valid
  // spots light up) and floats a card ghost under the pointer. Releasing
  // over a highlighted Pokémon or an empty slot plays the move; releasing
  // anywhere else just leaves the card selected for a tap.
  const [drag, setDrag] = useState<{
    card: number;
    x: number;
    y: number;
    over: string | null;
  } | null>(null);
  const pending = useRef<{ card: number; x: number; y: number; started: boolean } | null>(null);
  const suppressClick = useRef(false);

  // The window listeners fire long after render, so they read the live
  // board — the drop targets and empty-slot moves — from a ref.
  const board = useRef({ dropTargets, benchPlace, activePlace });
  useEffect(() => {
    board.current = { dropTargets, benchPlace, activePlace };
  });
  const resolveDrop = (dropId: string): number | null => {
    const b = board.current;
    if (dropId.startsWith("mon:")) return b.dropTargets.get(Number(dropId.slice(4))) ?? null;
    if (dropId === "slot:bench") return b.benchPlace >= 0 ? b.benchPlace : null;
    if (dropId === "slot:active") return b.activePlace >= 0 ? b.activePlace : null;
    return null;
  };

  const startDrag = (card: number, e: ReactPointerEvent) => {
    if (busy) return;
    suppressClick.current = false;
    pending.current = { card, x: e.clientX, y: e.clientY, started: false };
  };

  useEffect(() => {
    const dropIdAt = (x: number, y: number) =>
      (document.elementFromPoint(x, y) as HTMLElement | null)
        ?.closest("[data-drop-id]")
        ?.getAttribute("data-drop-id") ?? null;

    const move = (e: PointerEvent) => {
      const p = pending.current;
      if (!p) return;
      if (!p.started) {
        if (Math.hypot(e.clientX - p.x, e.clientY - p.y) < 8) return;
        p.started = true;
        onSelect({ kind: "hand", card: p.card });
      }
      const over = dropIdAt(e.clientX, e.clientY);
      const landable = over ? resolveDrop(over) != null : false;
      setDrag({ card: p.card, x: e.clientX, y: e.clientY, over: landable ? over : null });
    };
    const end = (e: PointerEvent) => {
      const p = pending.current;
      pending.current = null;
      setDrag(null);
      if (!p?.started) return;
      suppressClick.current = true;
      const id = dropIdAt(e.clientX, e.clientY);
      const act = id ? resolveDrop(id) : null;
      if (act != null) onAct(act);
    };
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", end);
    window.addEventListener("pointercancel", end);
    return () => {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", end);
      window.removeEventListener("pointercancel", end);
    };
  }, [onSelect, onAct, busy]);

  const hoverDropId = drag?.over ?? null;
  const dragCard = drag ? view.your_hand.find((c) => c.id === drag.card) : undefined;
  const dragSrc = dragCard ? art(dragCard.print_id) : null;

  return (
    <div className="mt-3">
      <div className="flex gap-2">
        <div className="min-w-0 flex-1 rounded-xl border border-edge bg-felt p-2">
          <div className="flex flex-col gap-1">
            <SideRow
              side={opp}
              label={`${SEAT_NAME[opp.player]} Opponent`}
              art={art}
              meta={meta}
              selection={selection}
              dropTargets={dropTargets}
              onPokemon={onPokemon}
              onViewDiscard={(cards, label) => setDiscardView({ cards, label })}
              hoverDropId={hoverDropId}
            />

            {/* Centre lane: stadium on the left, the two Actives stacked. */}
            <div className="flex items-center justify-center gap-3">
              <StadiumSlot />
              <div className="flex flex-col items-center gap-1">
                <LiveMon
                  mon={opp.active}
                  active
                  art={art}
                  {...monHooks(
                    opp.active,
                    meta,
                    selection,
                    dropTargets,
                    onPokemon,
                    false,
                    hoverDropId,
                  )}
                />
                <div className="h-px w-24 bg-white/15" aria-hidden />
                <div className="relative">
                  <LiveMon
                    mon={mine.active}
                    active
                    art={art}
                    placeHere={activePlace >= 0 ? () => onAct(activePlace) : undefined}
                    {...monHooks(
                      mine.active,
                      meta,
                      selection,
                      dropTargets,
                      onPokemon,
                      onCardMoves.length > 0,
                      hoverDropId,
                    )}
                  />
                  {activeSelected && onCardMoves.length > 0 && (
                    <div
                      className="absolute inset-x-1 bottom-1 z-30 flex flex-col gap-1"
                      data-keep-selection
                    >
                      {attackMoves.map((a) => (
                        <button
                          key={a.index}
                          type="button"
                          data-keep-selection
                          disabled={busy}
                          onClick={() => onAct(a.index)}
                          className="rounded bg-accent px-2 py-1 text-[11px] font-bold text-black shadow-[0_2px_6px_rgba(0,0,0,0.5)] hover:brightness-110 disabled:opacity-50"
                        >
                          {a.label}
                        </button>
                      ))}
                      {retreatMoves.map((a) => (
                        <button
                          key={a.index}
                          type="button"
                          data-keep-selection
                          disabled={busy}
                          onClick={() => onAct(a.index)}
                          className="rounded bg-warn px-2 py-1 text-[11px] font-bold text-black shadow-[0_2px_6px_rgba(0,0,0,0.5)] hover:brightness-110 disabled:opacity-50"
                        >
                          {a.label}
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              </div>
              {finishPlacing >= 0 ? (
                <button
                  type="button"
                  onClick={() => onAct(finishPlacing)}
                  disabled={busy}
                  className="h-[118px] w-[96px] flex-none rounded-lg border-2 border-accent bg-accent/15 px-2 text-sm font-bold leading-tight text-accent hover:bg-accent/25 disabled:opacity-50"
                >
                  ✓ Done placing
                </button>
              ) : (
                <StadiumSlot ghost />
              )}
            </div>

            <SideRow
              side={mine}
              label={`${SEAT_NAME[mine.player]} You`}
              mine
              art={art}
              meta={meta}
              selection={selection}
              dropTargets={dropTargets}
              onPokemon={onPokemon}
              onPlaceBench={benchPlace >= 0 ? () => onAct(benchPlace) : undefined}
              onViewDiscard={(cards, label) => setDiscardView({ cards, label })}
              hoverDropId={hoverDropId}
            />
          </div>

          {prompt && !firstTurn && <PromptBar prompt={prompt} busy={busy} onAct={onAct} />}
          {promoting && (
            <div className="mt-1 rounded-lg border border-warn/60 bg-warn/10 p-2 text-center text-[12px] font-semibold text-warn">
              Choose a new Active — tap a Benched Pokémon, tap again to promote it
            </div>
          )}

          <HandStrip
            hand={view.your_hand}
            meta={meta}
            selection={selection}
            onHand={onHand}
            onConfirm={onAct}
            confirmIndex={confirmIndex}
            art={art}
            onCardPointerDown={startDrag}
            suppressClickRef={suppressClick}
            draggingCard={drag?.card ?? null}
          />
        </div>

        {showRail && (
          <SideRail
            myPrizes={mine.prize_count}
            oppPrizes={opp.prize_count}
            turn={view.turn_number}
            yourTurn={seat === you}
            canEndTurn={endTurn >= 0 && !busy}
            onEndTurn={() => endTurn >= 0 && onAct(endTurn)}
            onLog={() => setShowLog(true)}
            onHide={() => setShowRail(false)}
          />
        )}
      </div>

      {!showRail && (
        <button
          type="button"
          onClick={() => setShowRail(true)}
          aria-label="Show controls"
          className="fixed right-3 top-1/2 z-40 flex -translate-y-1/2 items-center gap-1.5 rounded-l-lg border border-r-0 border-edge bg-panel/95 py-3 pl-2.5 pr-2 text-[11px] font-semibold uppercase tracking-wider text-dim shadow-[0_4px_14px_rgba(0,0,0,0.4)] backdrop-blur transition-colors hover:border-accent hover:text-text"
        >
          <span aria-hidden>‹</span>
          <span className="[writing-mode:vertical-rl]">Controls</span>
        </button>
      )}

      <div>
        {firstTurn ? (
          <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 p-4">
            <div className="w-full max-w-md rounded-xl border border-edge bg-bg p-6 text-center">
              <div className="text-[13px] uppercase tracking-widest text-dim">Coin flip</div>
              <p className="mt-1 text-lg font-semibold">
                {seat === you ? "You choose who goes first" : "Choose who goes first"}
              </p>
              <div className="mt-5 flex justify-center gap-3">
                <button
                  onClick={() => firstTurn.goFirst >= 0 && onAct(firstTurn.goFirst)}
                  disabled={busy || firstTurn.goFirst < 0}
                  className="rounded-lg border-accent bg-accent px-6 py-3 text-base font-bold text-black disabled:opacity-40"
                >
                  Go first
                </button>
                <button
                  onClick={() => firstTurn.goSecond >= 0 && onAct(firstTurn.goSecond)}
                  disabled={busy || firstTurn.goSecond < 0}
                  className="rounded-lg px-6 py-3 text-base font-bold disabled:opacity-40"
                >
                  Go second
                </button>
              </div>
            </div>
          </div>
        ) : decision ? (
          <DecisionBar
            actions={actions}
            decision={decision}
            busy={busy}
            onAct={onAct}
            art={art}
            meta={meta}
          />
        ) : (
          <div className="mt-2" data-keep-selection>
            {selection && (
              <div className="flex items-center gap-2 text-[12px]">
                <span className="rounded bg-accent px-1.5 py-0.5 font-semibold text-black">
                  {selectedName ?? "Selected"}
                </span>
                <span className="text-dim">
                  {activeSelected && onCardMoves.length > 0
                    ? "tap an attack or retreat on your Active"
                    : abilityMoves.length > 0
                      ? "use its Ability, or tap away"
                      : only && only.length === 0
                        ? "no move from here — tap away to cancel"
                        : confirmIndex !== undefined
                          ? "tap ✅ on the card to play it"
                          : "tap a highlighted spot on the board"}
                </span>
              </div>
            )}
            {abilityMoves.length > 0 && (
              <div className="mt-2 flex flex-wrap gap-2">
                {abilityMoves.map((a) => (
                  <button
                    key={a.index}
                    type="button"
                    data-keep-selection
                    disabled={busy}
                    onClick={() => onAct(a.index)}
                    className="rounded-md border-accent bg-accent/15 px-3 py-1.5 text-[12px] font-bold text-accent disabled:opacity-50"
                  >
                    ⚡ {a.label}
                  </button>
                ))}
              </div>
            )}
            {/* The full list stays here as an escape hatch for phases that
              have no on-board affordance yet. */}
            <details className="mt-2 text-[12px] text-dim">
              <summary className="cursor-pointer">All actions</summary>
              <div className="mt-1">
                <ActionPanel
                  actions={actions}
                  only={only}
                  onClearSelection={() => onSelect(null)}
                  seat={seat}
                  busy={busy}
                  onAct={onAct}
                />
              </div>
            </details>
          </div>
        )}
      </div>

      {flipRun && (
        <CoinFlip key={flipRun.id} results={flipRun.results} onDone={() => setFlipRun(null)} />
      )}

      {drag && (
        <div
          className="pointer-events-none fixed z-[60] h-[132px] w-[96px] -translate-x-1/2 -translate-y-1/2 rotate-3 overflow-hidden rounded-card border border-black/10 bg-card shadow-[0_10px_30px_rgba(0,0,0,0.5)]"
          style={{ left: drag.x, top: drag.y }}
        >
          {dragSrc ? (
            // eslint-disable-next-line @next/next/no-img-element
            <img
              src={dragSrc}
              alt=""
              className="absolute inset-0 size-full object-cover object-top"
            />
          ) : dragCard ? (
            <CardFace src={null} name={dragCard.name} energyType={dragCard.energy_type} />
          ) : null}
        </div>
      )}

      {showLog && (
        <div
          className="fixed inset-0 z-50 flex items-end justify-center bg-black/50 p-3 sm:items-center"
          onClick={() => setShowLog(false)}
        >
          <div
            className="flex max-h-[70vh] w-full max-w-lg flex-col overflow-hidden rounded-lg border border-edge bg-bg"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center justify-between border-b border-edge px-3 py-2">
              <span className="text-[12px] uppercase tracking-widest text-dim">Log</span>
              <button className="text-[13px]" onClick={() => setShowLog(false)}>
                Close
              </button>
            </div>
            <div className="overflow-y-auto whitespace-pre-wrap p-3 text-[13px] text-dim">
              {log.length ? [...log].reverse().join("\n") : "—"}
            </div>
          </div>
        </div>
      )}

      {discardView && (
        <div
          className="fixed inset-0 z-50 flex items-end justify-center bg-black/50 p-3 sm:items-center"
          onClick={() => setDiscardView(null)}
        >
          <div
            className="flex max-h-[80vh] w-full max-w-2xl flex-col overflow-hidden rounded-lg border border-edge bg-bg"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center justify-between border-b border-edge px-3 py-2">
              <span className="text-[12px] uppercase tracking-widest text-dim">
                {discardView.label} ({discardView.cards.length})
              </span>
              <button className="text-[13px]" onClick={() => setDiscardView(null)}>
                Close
              </button>
            </div>
            <div className="grid grid-cols-[repeat(auto-fill,minmax(84px,1fr))] gap-2 overflow-y-auto p-3">
              {discardView.cards.length === 0 && (
                <span className="text-[13px] text-dim">empty</span>
              )}
              {discardView.cards.map((c, i) => (
                <span
                  key={i}
                  className="relative block aspect-[5/7] overflow-hidden rounded-card border border-black/10 bg-card shadow-card"
                >
                  <CardFace src={art(c.print_id)} name={c.name} energyType={c.energy_type} />
                </span>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
