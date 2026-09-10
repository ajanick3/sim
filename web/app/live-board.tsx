"use client";

// An experimental board that arranges the game the way Pokémon TCG Live
// does: a tilted felt mat with prize stacks, deck and discard piles, a
// stadium slot, bench trays, and a hand strip, plus a right-hand rail
// with prize counts and an END TURN button, and a blue decision bar for
// search / discard prompts. Reached at /live. Shares the engine wiring
// with the classic board; only the presentation differs.

import { useEffect, useRef, useState } from "react";
import type { PointerEvent as ReactPointerEvent, RefObject } from "react";
import { ActionPanel, CardArt, ENERGY_COLOR } from "./table";
import {
  COPY_COLORS,
  copyBadges,
  movesForSelection,
  targetsForHandCard,
  type Selection,
} from "./session";
import type { WireActionMeta, WireCard, WirePokemon, WireSide, WireView } from "./view";

const SEAT_NAME = ["🥇", "🥈"];

type Art = (printId: string) => string | null;

/** The face of a card in a tile: real art when there is any, a drawn
 *  Energy card for Basic Energy (TCGdex has no art for those), else the
 *  name. */
function CardFace({
  src,
  name,
  energyType,
}: {
  src: string | null;
  name: string;
  energyType: string | null;
}) {
  if (src) return <CardArt src={src} alt={name} />;
  if (energyType) {
    // TCGdex has no art for Basic Energy, so draw the card: the type
    // colour edge to edge with the big centre disc a real one carries.
    const colour = ENERGY_COLOR[energyType] ?? "var(--color-dim)";
    return (
      <span
        className="absolute inset-0 flex items-center justify-center"
        style={{ background: `linear-gradient(155deg, ${colour}, ${colour}bb 55%, ${colour}77)` }}
      >
        <span
          className="grid size-[46%] place-items-center rounded-full border-[3px] border-white/80"
          style={{ background: `radial-gradient(circle at 38% 32%, #ffffffd0, ${colour} 72%)` }}
        >
          <span className="text-[10px] font-black uppercase text-black/55">
            {energyType.slice(0, 2)}
          </span>
        </span>
      </span>
    );
  }
  return (
    <span className="absolute inset-0 bg-white p-1 text-left text-[10px] font-semibold leading-tight text-neutral-800">
      {name}
    </span>
  );
}

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
  const [showLog, setShowLog] = useState(false);

  const dropTargets =
    selection?.kind === "hand"
      ? targetsForHandCard(meta, selection.card)
      : new Map<number, number>();
  const onPokemon = (id: number) => {
    const landing = dropTargets.get(id);
    if (landing !== undefined) onAct(landing);
    else
      onSelect(
        selection?.kind === "pokemon" && selection.id === id ? null : { kind: "pokemon", id },
      );
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
  const activeSelected =
    selection?.kind === "pokemon" && mine.active != null && selection.id === mine.active.id;

  const decision = asDecision(actions);
  // A yes/no Ability prompt ("Use Psychic Draw" / "Decline Psychic
  // Draw") — surfaced as its own bar, not left in the All-actions list.
  const prompt = decision ? null : asPrompt(actions);
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
  const [drag, setDrag] = useState<{ card: number; x: number; y: number } | null>(null);
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
      setDrag({ card: p.card, x: e.clientX, y: e.clientY });
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

  const dragCard = drag ? view.your_hand.find((c) => c.id === drag.card) : undefined;
  const dragSrc = dragCard ? art(dragCard.print_id) : null;

  return (
    <div className="mt-3">
      <div className="flex gap-2">
        <div className="min-w-0 flex-1 rounded-xl border border-edge bg-felt p-2">
          <SideRow
            side={opp}
            label={`${SEAT_NAME[opp.player]} Opponent`}
            art={art}
            meta={meta}
            selection={selection}
            dropTargets={dropTargets}
            onPokemon={onPokemon}
          />

          {/* Centre lane: stadium on the left, the two Actives stacked. */}
          <div className="my-1.5 flex items-center justify-center gap-3">
            <StadiumSlot />
            <div className="flex flex-col items-center gap-1">
              <LiveMon
                mon={opp.active}
                active
                art={art}
                {...monHooks(opp.active, meta, selection, dropTargets, onPokemon)}
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
                    attackMoves.length > 0,
                  )}
                />
                {activeSelected && attackMoves.length > 0 && (
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
                  </div>
                )}
              </div>
            </div>
            <StadiumSlot ghost />
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
          />

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

        <SideRail
          myPrizes={mine.prize_count}
          oppPrizes={opp.prize_count}
          turn={view.turn_number}
          yourTurn={seat === you}
          canEndTurn={endTurn >= 0 && !busy}
          onEndTurn={() => endTurn >= 0 && onAct(endTurn)}
          onLog={() => setShowLog(true)}
        />
      </div>

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
      ) : prompt ? (
        <PromptBar prompt={prompt} busy={busy} onAct={onAct} />
      ) : (
        <div className="mt-2" data-keep-selection>
          {selection && (
            <div className="flex items-center gap-2 text-[12px]">
              <span className="rounded bg-accent px-1.5 py-0.5 font-semibold text-black">
                {selectedName ?? "Selected"}
              </span>
              <span className="text-dim">
                {activeSelected && attackMoves.length > 0
                  ? "tap an attack on your Active"
                  : only && only.length === 0
                    ? "no move from here — tap away to cancel"
                    : confirmIndex !== undefined
                      ? "tap ✅ on the card to play it"
                      : "tap a highlighted spot on the board"}
              </span>
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

      {drag && (
        <div
          className="pointer-events-none fixed z-[60] h-[132px] w-[96px] -translate-x-1/2 -translate-y-1/2 rotate-3 overflow-hidden rounded-[7px] border border-black/10 bg-white shadow-[0_10px_30px_rgba(0,0,0,0.5)]"
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
              {log.length ? log.join("\n") : "—"}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function monHooks(
  m: WirePokemon | null,
  meta: WireActionMeta[],
  selection: Selection,
  dropTargets: Map<number, number>,
  onPokemon: (id: number) => void,
  extraSelectable = false,
) {
  if (!m) return {};
  return {
    onSelect: () => onPokemon(m.id),
    selectable: extraSelectable || meta.some((x) => x.target === m.id) || dropTargets.has(m.id),
    selected: selection?.kind === "pokemon" && selection.id === m.id,
    dropTarget: dropTargets.has(m.id),
  };
}

function SideRow({
  side,
  label,
  mine = false,
  art,
  meta,
  selection,
  dropTargets,
  onPokemon,
  onPlaceBench,
}: {
  side: WireSide;
  label: string;
  mine?: boolean;
  art: Art;
  meta: WireActionMeta[];
  selection: Selection;
  dropTargets: Map<number, number>;
  onPokemon: (id: number) => void;
  onPlaceBench?: () => void;
}) {
  const badges = copyBadges(side.bench.map((m) => m?.name ?? null));
  // Five slots: the Pokémon on the Bench, then empty pads to fill.
  const slots = [...side.bench, ...Array(Math.max(0, 5 - side.bench.length)).fill(null)];
  return (
    <div className={`flex items-start gap-2 ${mine ? "" : "flex-row-reverse"}`}>
      <PrizeStack count={side.prize_count} />
      <div
        className={`min-w-0 flex-1 rounded-lg border-2 p-1.5 ${
          mine ? "border-accent/60" : "border-warn/50"
        }`}
      >
        <div className="mb-1 flex items-center justify-between text-[10px] uppercase tracking-widest text-dim">
          <span>{label}</span>
          <span>hand {side.hand_count}</span>
        </div>
        <div className="flex flex-wrap gap-1.5">
          {slots.map((m, i) =>
            m ? (
              <LiveMon
                key={i}
                mon={m}
                art={art}
                small
                copy={badges[i]}
                {...monHooks(m, meta, selection, dropTargets, onPokemon)}
              />
            ) : (
              <LiveMon
                key={i}
                mon={null}
                art={art}
                small
                placeHere={mine ? onPlaceBench : undefined}
              />
            ),
          )}
        </div>
      </div>
      <DeckPile deck={side.library_count} discard={side.discard} art={art} mine={mine} />
    </div>
  );
}

function PrizeStack({ count }: { count: number }) {
  return (
    <div className="flex flex-col items-center">
      <div className="grid grid-cols-2 gap-0.5">
        {Array.from({ length: 6 }, (_, i) => (
          <span
            key={i}
            className={`h-6 w-4 rounded-[2px] border ${
              i < count ? "border-rose-300/60 bg-rose-400/25" : "border-white/10 bg-transparent"
            }`}
          />
        ))}
      </div>
      <span className="mt-0.5 text-[10px] text-dim">{count}</span>
    </div>
  );
}

function DeckPile({
  deck,
  discard,
  art,
  mine = false,
}: {
  deck: number;
  discard: WireCard[];
  art: Art;
  mine?: boolean;
}) {
  const top = discard.at(-1);
  return (
    <div className="flex flex-col items-center gap-1">
      <div className="relative h-[64px] w-[46px] rounded border border-white/20 bg-white/5 shadow-[2px_2px_0_rgba(255,255,255,0.06)]">
        <span className="absolute inset-x-0 bottom-0.5 text-center text-[10px] text-dim">
          {deck}
        </span>
      </div>
      <div
        data-toss-target={mine ? "discard" : undefined}
        className="relative h-[64px] w-[46px] overflow-hidden rounded border border-white/15 bg-panel"
      >
        {top && art(top.print_id) ? (
          <CardArt src={art(top.print_id)!} alt={top.name} />
        ) : (
          <span className="absolute inset-0 flex items-center justify-center px-0.5 text-center text-[8px] leading-tight text-dim">
            {top?.name ?? "discard"}
          </span>
        )}
        <span className="absolute inset-x-0 bottom-0 bg-black/60 text-center text-[9px]">
          {discard.length}
        </span>
      </div>
    </div>
  );
}

function StadiumSlot({ ghost = false }: { ghost?: boolean }) {
  return (
    <div
      className={`flex h-[90px] w-[56px] flex-col items-center justify-center rounded border border-dashed border-white/15 text-center text-[8px] text-dim ${
        ghost ? "invisible" : ""
      }`}
    >
      stadium
    </div>
  );
}

function LiveMon({
  mon,
  active = false,
  small = false,
  copy,
  art,
  onSelect,
  selectable = false,
  selected = false,
  dropTarget = false,
  placeHere,
}: {
  mon: WirePokemon | null;
  active?: boolean;
  small?: boolean;
  copy?: number;
  art: Art;
  onSelect?: () => void;
  selectable?: boolean;
  selected?: boolean;
  dropTarget?: boolean;
  /** Empty slot: a selected hand card can be placed here. */
  placeHere?: () => void;
}) {
  const size = active
    ? "w-[228px] h-[140px]"
    : small
      ? "w-[64px] min-h-[90px]"
      : "w-[96px] min-h-[134px]";
  if (!mon) {
    return (
      <button
        type="button"
        data-keep-selection
        data-drop-id={placeHere ? (active ? "slot:active" : "slot:bench") : undefined}
        disabled={!placeHere}
        onClick={placeHere}
        className={`${size} flex flex-none items-center justify-center rounded-md border border-dashed text-[9px] disabled:cursor-default ${
          placeHere
            ? "border-accent bg-accent/10 text-accent animate-pulse"
            : "border-white/15 text-dim"
        }`}
      >
        {placeHere ? "place here" : active ? "no Active" : ""}
      </button>
    );
  }
  const src = art(mon.print_id);
  const interactive = selectable && !!onSelect;
  const ring = selected
    ? "z-20 ring-2 ring-accent border-accent"
    : dropTarget
      ? "z-20 border-white ring-2 ring-white shadow-[0_0_0_2px_#fff,0_0_18px_5px_rgba(255,255,255,0.7)]"
      : active
        ? "border-accent"
        : "border-edge";
  const energies = mon.attached.filter((c) => c.energy_type);
  return (
    <button
      type="button"
      data-keep-selection
      data-drop-id={`mon:${mon.id}`}
      disabled={!interactive}
      onClick={onSelect}
      className={`deal-in ${size} relative flex flex-none flex-col overflow-hidden rounded-md border bg-panel transition-colors disabled:cursor-default disabled:opacity-100 ${ring} ${
        interactive ? "hover:border-accent" : ""
      }`}
    >
      {/* Every card on the board shows the top slice of the print — the
          name bar and the head of the illustration — cropped from the
          top edge. A scrim at the foot keeps the overlays readable. */}
      {src ? (
        <>
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img
            src={src}
            alt={mon.name}
            loading="lazy"
            className="absolute inset-0 size-full object-cover object-top"
          />
          <div className="absolute inset-x-0 bottom-0 h-1/2 bg-gradient-to-t from-black/80 to-transparent" />
        </>
      ) : (
        <span className="relative z-10 p-1 text-[9px] font-semibold leading-tight">{mon.name}</span>
      )}

      {/* HP pill — top-left on the Active so it clears the damage coin. */}
      <span
        className={`absolute top-0.5 z-10 rounded bg-black/75 px-1 text-[9px] font-bold ${
          active ? "left-0.5" : "right-0.5"
        }`}
      >
        {mon.hp}
      </span>
      {/* Damage: a coin at the top-right of the image on the Active,
          a small chip on a Bench card. */}
      {mon.damage > 0 &&
        (active ? (
          <span className="absolute right-0.5 top-0.5 z-10 grid size-7 place-items-center rounded-full border-2 border-black/40 bg-orange-500 text-[11px] font-black text-black shadow-md">
            {mon.damage}
          </span>
        ) : (
          <span className="absolute right-0.5 top-4 z-10 rounded-full bg-orange-500 px-1 text-[9px] font-bold text-black">
            {mon.damage}
          </span>
        ))}
      {copy !== undefined && (
        <span
          className="absolute left-0.5 top-0.5 z-10 size-2 rounded-full"
          style={{ background: COPY_COLORS[copy % COPY_COLORS.length] }}
        />
      )}

      {/* Energy row, bottom-centre. */}
      <span className="absolute inset-x-0 bottom-0.5 z-10 flex justify-center gap-0.5">
        {energies.map((c) => (
          <span
            key={c.id}
            title={`${c.energy_type} Energy`}
            className="size-2 rounded-full border border-black/40"
            style={{ background: ENERGY_COLOR[c.energy_type as string] ?? "var(--color-dim)" }}
          />
        ))}
      </span>

      {mon.conditions.length > 0 && (
        <span className="absolute inset-x-0 top-1/2 z-10 bg-black/60 text-center text-[8px] text-warn">
          {mon.conditions.join(", ")}
        </span>
      )}
    </button>
  );
}

const HAND_ORDER = [
  "pokemon",
  "supporter",
  "item",
  "tool",
  "stadium",
  "special-energy",
  "energy",
] as const;

function splitHandRows(hand: WireCard[]): WireCard[][] {
  const order = HAND_ORDER as readonly string[];
  const rank = (c: string) => {
    const i = order.indexOf(c);
    return i < 0 ? order.length : i;
  };
  const sorted = [...hand].sort((a, b) => rank(a.category) - rank(b.category));
  const n = sorted.length;
  if (n <= 4) return [sorted];
  const target = Math.ceil(n / 2);
  const boundaries: number[] = [];
  for (let i = 1; i < n; i++) {
    if (sorted[i].category !== sorted[i - 1].category) boundaries.push(i);
  }
  let split = target;
  if (boundaries.length) {
    const best = boundaries.reduce((p, c) => (Math.abs(c - target) < Math.abs(p - target) ? c : p));
    if (Math.abs(best - target) <= 2) split = best;
  }
  return [sorted.slice(0, split), sorted.slice(split)];
}

function HandStrip({
  hand,
  meta,
  selection,
  onHand,
  onConfirm,
  confirmIndex,
  art,
  onCardPointerDown,
  suppressClickRef,
  draggingCard,
}: {
  hand: WireCard[];
  meta: WireActionMeta[];
  selection: Selection;
  onHand: (card: number) => void;
  onConfirm: (index: number) => void;
  /** The one move a selected card resolves to, if it is unambiguous. */
  confirmIndex?: number;
  art: Art;
  /** Begin a possible drag from this card. */
  onCardPointerDown: (card: number, e: ReactPointerEvent) => void;
  /** Set true by a finished drag; the click it spawns is then skipped. */
  suppressClickRef: RefObject<boolean>;
  /** The card currently being dragged, dimmed in its slot. */
  draggingCard: number | null;
}) {
  // Sort by category, then break into at most two rows at the category
  // boundary nearest the midpoint — an even split is nice but keeping a
  // category whole is nicer, so allow the split to drift a card or two.
  const rows = splitHandRows(hand);
  const [tossing, setTossing] = useState<number | null>(null);
  const reduce =
    typeof window !== "undefined" && window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  const onCardClick = (c: WireCard, selected: boolean, el: HTMLElement) => {
    if (selected && confirmIndex !== undefined) {
      if (reduce) {
        onConfirm(confirmIndex);
        return;
      }
      // Fly the card to where the move actually sends it — the target
      // Pokémon, the empty slot it fills, or the discard pile.
      const m = meta[confirmIndex];
      const sel =
        m?.target != null
          ? `[data-drop-id="mon:${m.target}"]`
          : m?.kind === "PlaceActive"
            ? '[data-drop-id="slot:active"]'
            : m?.kind === "PlayBasic" || m?.kind === "PlaceOnBench"
              ? '[data-drop-id="slot:bench"]'
              : '[data-toss-target="discard"]';
      const dest = document.querySelector(sel)?.getBoundingClientRect();
      if (dest) {
        const from = el.getBoundingClientRect();
        el.style.setProperty(
          "--toss-x",
          `${dest.left + dest.width / 2 - (from.left + from.width / 2)}px`,
        );
        el.style.setProperty(
          "--toss-y",
          `${dest.top + dest.height / 2 - (from.top + from.height / 2)}px`,
        );
      }
      setTossing(c.id);
      return;
    }
    onHand(c.id);
  };

  return (
    <div className="mt-2 rounded-lg border-2 border-cyan-400/60 p-1.5">
      <div className="mb-1 text-[10px] uppercase tracking-widest text-dim">
        Hand ({hand.length})
      </div>
      {hand.length === 0 ? (
        <span className="px-2 py-8 text-[11px] text-dim">empty</span>
      ) : (
        <div className="flex flex-col gap-2">
          {rows.map((row, ri) => (
            <div key={ri} className="flex flex-wrap justify-center gap-2">
              {row.map((c) => {
                const playable = meta.some((x) => x.card === c.id);
                const selected = selection?.kind === "hand" && selection.card === c.id;
                const src = art(c.print_id);
                return (
                  <button
                    key={c.id}
                    type="button"
                    data-keep-selection
                    disabled={!playable}
                    onPointerDown={playable ? (e) => onCardPointerDown(c.id, e) : undefined}
                    onClick={(e) => {
                      if (suppressClickRef.current) {
                        suppressClickRef.current = false;
                        return;
                      }
                      onCardClick(c, selected, e.currentTarget);
                    }}
                    onAnimationEnd={(e) => {
                      if (
                        e.animationName === "card-toss" &&
                        tossing === c.id &&
                        confirmIndex !== undefined
                      )
                        onConfirm(confirmIndex);
                    }}
                    className={`group relative h-[96px] w-[132px] flex-none touch-none overflow-hidden rounded-[7px] bg-white transition-transform duration-150 will-change-transform hover:z-20 hover:-translate-y-2 hover:scale-[1.05] disabled:opacity-50 ${
                      draggingCard === c.id ? "opacity-30" : ""
                    } ${selected ? "ring-2 ring-accent" : ""} ${
                      tossing === c.id
                        ? "card-toss z-40 shadow-[0_1px_2px_rgba(28,16,8,0.55),0_4px_8px_rgba(28,16,8,0.35)]"
                        : selected
                          ? "card-tap z-30 -translate-y-3 scale-[1.06] shadow-[0_2px_4px_rgba(28,16,8,0.4),0_16px_32px_rgba(28,16,8,0.45)]"
                          : "shadow-[0_1px_2px_rgba(28,16,8,0.55),0_4px_8px_rgba(28,16,8,0.35)] group-hover:shadow-[0_2px_4px_rgba(28,16,8,0.4),0_14px_28px_rgba(28,16,8,0.45)]"
                    }`}
                  >
                    {/* Only the top slice of the print: the name bar and
                        the head of the illustration. */}
                    {src ? (
                      // eslint-disable-next-line @next/next/no-img-element
                      <img
                        src={src}
                        alt={c.name}
                        loading="lazy"
                        className="absolute inset-0 size-full object-cover object-top"
                      />
                    ) : (
                      <CardFace src={null} name={c.name} energyType={c.energy_type} />
                    )}
                  </button>
                );
              })}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function SideRail({
  myPrizes,
  oppPrizes,
  turn,
  yourTurn,
  canEndTurn,
  onEndTurn,
  onLog,
}: {
  myPrizes: number;
  oppPrizes: number;
  turn: number;
  yourTurn: boolean;
  canEndTurn: boolean;
  onEndTurn: () => void;
  onLog: () => void;
}) {
  return (
    <div className="flex w-[72px] flex-none flex-col items-center gap-2 pt-6">
      <div className="text-[10px] text-dim">turn {turn}</div>
      <div className="grid h-9 w-9 place-items-center rounded bg-rose-500 text-lg font-bold">
        {oppPrizes}
      </div>
      <button
        onClick={onEndTurn}
        disabled={!canEndTurn}
        className="w-full rounded-md border-warn bg-warn/20 px-1 py-2 text-[11px] font-bold leading-tight text-warn disabled:opacity-40"
      >
        END
        <br />
        TURN
      </button>
      <div className="grid h-9 w-9 place-items-center rounded bg-accent text-lg font-bold text-black">
        {myPrizes}
      </div>
      <div className="text-[10px] text-dim">{yourTurn ? "your move" : "waiting"}</div>
      <button
        onClick={onLog}
        aria-label="Log"
        className="mt-1 grid size-9 place-items-center rounded-full border-edge text-base"
      >
        📜
      </button>
    </div>
  );
}

// --- Decision bar -----------------------------------------------------------

type DecisionKind = "take" | "discard" | "choose" | "pay";

/** When the whole legal-action set is a search / discard / pick prompt,
 *  describe it; otherwise null and the normal panel shows. */
function asDecision(actions: string[]): { kind: DecisionKind; verb: string } | null {
  if (actions.length === 0) return null;
  const isFinish = (l: string) => /^(Stop |Finish|Take no more|Move on|Decline)/.test(l);
  const body = actions.filter((l) => !isFinish(l));
  if (body.length === 0) return null;
  const test = (re: RegExp) => body.every((l) => re.test(l));
  if (test(/^Take /)) return { kind: "take", verb: "Choose cards to take" };
  if (test(/^Discard /)) return { kind: "discard", verb: "Choose cards to discard" };
  if (test(/^Choose /)) return { kind: "choose", verb: "Make a choice" };
  if (test(/^Discard .* to pay/)) return { kind: "pay", verb: "Discard to pay the cost" };
  return null;
}

type Prompt = { verb: string; accepts: { label: string; index: number }[]; decline: number };

/** A "may" Ability the engine is waiting on — the action set is one or
 *  more ways to use it plus a single "Decline …". Returns null when
 *  there is no "Decline …" line, so an ordinary turn is never caught. */
function asPrompt(actions: string[]): Prompt | null {
  const decline = actions.findIndex((a) => /^Decline /.test(a));
  if (decline < 0) return null;
  const accepts = actions
    .map((label, index) => ({ label, index }))
    .filter(({ index }) => index !== decline);
  if (accepts.length === 0) return null;
  const name = actions[decline].replace(/^Decline (the )?/, "");
  return { verb: `Use ${name}?`, accepts, decline };
}

function PromptBar({
  prompt,
  busy,
  onAct,
}: {
  prompt: Prompt;
  busy: boolean;
  onAct: (index: number) => void;
}) {
  return (
    <div className="mt-3 flex flex-wrap items-center gap-2 rounded-lg border border-edge bg-bg p-3">
      <span className="mr-1 font-bold">{prompt.verb}</span>
      {prompt.accepts.map(({ label, index }) => (
        <button
          key={index}
          type="button"
          disabled={busy}
          onClick={() => onAct(index)}
          className="rounded-md border-accent bg-accent px-4 py-1.5 font-bold text-black disabled:opacity-50"
        >
          {label}
        </button>
      ))}
      <button
        type="button"
        disabled={busy}
        onClick={() => onAct(prompt.decline)}
        className="rounded-md px-4 py-1.5 text-dim disabled:opacity-50"
      >
        Decline
      </button>
    </div>
  );
}

function DecisionBar({
  actions,
  decision,
  busy,
  onAct,
  art,
  meta,
}: {
  actions: string[];
  decision: { kind: DecisionKind; verb: string };
  busy: boolean;
  onAct: (index: number) => void;
  art: Art;
  meta: WireActionMeta[];
}) {
  const isFinish = (l: string) => /^(Stop |Finish|Take no more|Move on|Decline)/.test(l);
  const finish = actions.findIndex(isFinish);
  const choices = actions
    .map((label, index) => ({ label, index }))
    .filter(({ label }) => !isFinish(label));

  // In the document flow, below the board — the picker never covers the
  // table.
  return (
    <div className="mt-3 overflow-hidden rounded-lg border border-edge bg-bg">
      <div className="sticky top-0 z-10 flex items-center gap-3 bg-accent px-3 py-2 text-black">
        <span className="font-bold">{decision.verb}.</span>
        <span className="text-[11px] opacity-70">only playable cards are shown</span>
        {finish >= 0 && (
          <button
            onClick={() => onAct(finish)}
            disabled={busy}
            className="ml-auto rounded border-black/30 bg-warn px-4 py-1 font-bold text-black disabled:opacity-50"
          >
            DONE
          </button>
        )}
      </div>
      <div className="flex flex-wrap justify-center gap-3 p-3">
        {choices.map(({ label, index }) => {
          const face = meta[index]?.card_face ?? null;
          const src = face ? art(face.print_id) : null;
          return (
            <button
              key={index}
              type="button"
              disabled={busy}
              onClick={() => onAct(index)}
              className="relative h-[184px] w-auto flex-none transition-transform hover:-translate-y-1 hover:scale-[1.04] disabled:opacity-50"
            >
              {src ? (
                // eslint-disable-next-line @next/next/no-img-element
                <img
                  src={src}
                  alt={face?.name ?? label}
                  className="block h-full w-auto rounded-[8px] bg-white object-cover shadow-[0_1px_2px_rgba(28,16,8,0.55),0_5px_10px_rgba(28,16,8,0.4),0_16px_30px_-4px_rgba(28,16,8,0.4)]"
                />
              ) : (
                <span className="relative block h-full w-[132px] overflow-hidden rounded-[8px] border border-black/10 bg-white text-neutral-800 shadow-[0_1px_2px_rgba(28,16,8,0.55),0_5px_10px_rgba(28,16,8,0.4),0_16px_30px_-4px_rgba(28,16,8,0.4)]">
                  <CardFace
                    src={null}
                    name={face?.name ?? label.replace(/^Take /, "")}
                    energyType={face?.energy_type ?? null}
                  />
                </span>
              )}
            </button>
          );
        })}
      </div>
    </div>
  );
}

export { asDecision };
