"use client";

// An experimental board that arranges the game the way Pokémon TCG Live
// does: a tilted felt mat with prize stacks, deck and discard piles, a
// stadium slot, bench trays, and a hand strip, plus a right-hand rail
// with prize counts and an END TURN button, and a blue decision bar for
// search / discard prompts. Reached at /live. Shares the engine wiring
// with the classic board; only the presentation differs.

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
    const colour = ENERGY_COLOR[energyType] ?? "var(--color-dim)";
    return (
      <span
        className="absolute inset-0 flex flex-col items-center justify-center gap-1"
        style={{ background: `radial-gradient(circle at 50% 40%, ${colour}33, transparent 70%)` }}
      >
        <span
          className="size-8 rounded-full border-2 border-black/40"
          style={{ background: colour }}
        />
        <span className="text-[8px] uppercase tracking-widest text-dim">{energyType}</span>
      </span>
    );
  }
  return (
    <span className="absolute inset-0 p-1 text-left text-[10px] font-semibold leading-tight">
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

  const decision = asDecision(actions);
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

          {/* Centre lane: stadium, then the two Actives nose to nose. */}
          <div className="my-1.5 flex items-center justify-center gap-2">
            <StadiumSlot />
            <LiveMon
              mon={opp.active}
              active
              art={art}
              {...monHooks(opp.active, meta, selection, dropTargets, onPokemon)}
            />
            <div className="h-px w-6 bg-white/15" aria-hidden />
            <LiveMon
              mon={mine.active}
              active
              art={art}
              placeHere={activePlace >= 0 ? () => onAct(activePlace) : undefined}
              {...monHooks(mine.active, meta, selection, dropTargets, onPokemon)}
            />
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
            art={art}
          />
        </div>

        <SideRail
          myPrizes={mine.prize_count}
          oppPrizes={opp.prize_count}
          turn={view.turn_number}
          yourTurn={seat === you}
          canEndTurn={endTurn >= 0 && !busy}
          onEndTurn={() => endTurn >= 0 && onAct(endTurn)}
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
          hand={view.your_hand}
          meta={meta}
        />
      ) : (
        <div className="mt-3">
          {selection && (
            <div className="mb-1.5 flex items-center gap-2 text-[12px]">
              <span className="rounded bg-accent px-1.5 py-0.5 font-semibold text-black">
                {selectedName ?? "Selected"}
              </span>
              <span className="text-dim">
                {only && only.length === 0
                  ? "no move from here"
                  : "choose an action, or tap again to cancel"}
              </span>
            </div>
          )}
          <ActionPanel
            actions={actions}
            only={only}
            onClearSelection={() => onSelect(null)}
            seat={seat}
            busy={busy}
            onAct={onAct}
          />
        </div>
      )}

      <details className="mt-3 text-[12px] text-dim">
        <summary className="cursor-pointer">Log</summary>
        <div className="mt-1 max-h-40 overflow-y-auto whitespace-pre-wrap rounded border border-edge bg-panel p-2">
          {log.length ? log.join("\n") : "—"}
        </div>
      </details>
    </div>
  );
}

function monHooks(
  m: WirePokemon | null,
  meta: WireActionMeta[],
  selection: Selection,
  dropTargets: Map<number, number>,
  onPokemon: (id: number) => void,
) {
  if (!m) return {};
  return {
    onSelect: () => onPokemon(m.id),
    selectable: meta.some((x) => x.target === m.id) || dropTargets.has(m.id),
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
        <div className="flex gap-1.5 overflow-x-auto">
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
      <DeckPile deck={side.library_count} discard={side.discard} art={art} />
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

function DeckPile({ deck, discard, art }: { deck: number; discard: WireCard[]; art: Art }) {
  const top = discard.at(-1);
  return (
    <div className="flex flex-col items-center gap-1">
      <div className="relative h-[64px] w-[46px] rounded border border-white/20 bg-white/5 shadow-[2px_2px_0_rgba(255,255,255,0.06)]">
        <span className="absolute inset-x-0 bottom-0.5 text-center text-[10px] text-dim">
          {deck}
        </span>
      </div>
      <div className="relative h-[64px] w-[46px] overflow-hidden rounded border border-white/15 bg-panel">
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
    ? "w-[156px] h-[116px]"
    : small
      ? "w-[64px] min-h-[90px]"
      : "w-[96px] min-h-[134px]";
  if (!mon) {
    return (
      <button
        type="button"
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
    ? "ring-2 ring-accent border-accent"
    : dropTarget
      ? "ring-2 ring-warn border-warn"
      : active
        ? "border-accent"
        : "border-edge";
  const energies = mon.attached.filter((c) => c.energy_type);
  return (
    <button
      type="button"
      disabled={!interactive}
      onClick={onSelect}
      className={`deal-in ${size} relative flex flex-none flex-col overflow-hidden rounded-md border bg-panel transition-colors disabled:cursor-default disabled:opacity-100 ${ring} ${
        interactive ? "hover:border-accent" : ""
      }`}
    >
      {/* Active: just the illustration, cropped from the top of the card.
          Bench: the whole small card behind a scrim. */}
      {src ? (
        active ? (
          // eslint-disable-next-line @next/next/no-img-element
          <img
            src={src}
            alt={mon.name}
            loading="lazy"
            className="absolute inset-0 size-full object-cover object-[50%_16%]"
          />
        ) : (
          <CardArt src={src} alt={mon.name} />
        )
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

function HandStrip({
  hand,
  meta,
  selection,
  onHand,
  art,
}: {
  hand: WireCard[];
  meta: WireActionMeta[];
  selection: Selection;
  onHand: (card: number) => void;
  art: Art;
}) {
  // One row per category, in play order. Rows after the first slide up so
  // the front row covers the text band of the row behind it but not its
  // illustration; a later row sits above an earlier one.
  const rows = HAND_ORDER.map((cat) => hand.filter((c) => c.category === cat)).filter(
    (r) => r.length > 0,
  );

  return (
    <div className="mt-2 rounded-lg border-2 border-cyan-400/60 p-1.5">
      <div className="mb-1 text-[10px] uppercase tracking-widest text-dim">
        Hand ({hand.length})
      </div>
      {hand.length === 0 ? (
        <span className="px-2 py-8 text-[11px] text-dim">empty</span>
      ) : (
        <div className="flex flex-col">
          {rows.map((row, ri) => (
            <div
              key={ri}
              className="flex flex-wrap justify-center"
              style={{ marginTop: ri === 0 ? 0 : -84, zIndex: ri + 1 }}
            >
              {row.map((c) => {
                const playable = meta.some((x) => x.card === c.id);
                const selected = selection?.kind === "hand" && selection.card === c.id;
                const src = art(c.print_id);
                return (
                  <button
                    key={c.id}
                    type="button"
                    disabled={!playable}
                    onClick={() => onHand(c.id)}
                    className={`relative -ml-3 h-[150px] w-[104px] flex-none overflow-hidden rounded-md border bg-panel transition-transform first:ml-0 hover:z-20 hover:-translate-y-6 disabled:translate-y-0 disabled:opacity-50 ${
                      selected
                        ? "z-20 -translate-y-6 border-accent ring-2 ring-accent"
                        : "border-edge"
                    }`}
                  >
                    <CardFace src={src} name={c.name} energyType={c.energy_type} />
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
}: {
  myPrizes: number;
  oppPrizes: number;
  turn: number;
  yourTurn: boolean;
  canEndTurn: boolean;
  onEndTurn: () => void;
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

function DecisionBar({
  actions,
  decision,
  busy,
  onAct,
  art,
  hand,
  meta,
}: {
  actions: string[];
  decision: { kind: DecisionKind; verb: string };
  busy: boolean;
  onAct: (index: number) => void;
  art: Art;
  hand: WireCard[];
  meta: WireActionMeta[];
}) {
  const isFinish = (l: string) => /^(Stop |Finish|Take no more|Move on|Decline)/.test(l);
  const finish = actions.findIndex(isFinish);
  const choices = actions
    .map((label, index) => ({ label, index }))
    .filter(({ label }) => !isFinish(label));

  const cardById = new Map(hand.map((c) => [c.id, c] as const));
  const cardFor = (index: number) => {
    const c = meta[index]?.card;
    return c === null || c === undefined ? undefined : cardById.get(c);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/75 p-4">
      <div className="flex max-h-full w-full max-w-3xl flex-col overflow-hidden rounded-lg border border-edge bg-bg">
        <div className="flex items-center gap-3 bg-accent px-3 py-2 text-black">
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
        <div className="flex flex-wrap justify-center gap-2 overflow-y-auto p-3">
          {choices.map(({ label, index }) => {
            const card = cardFor(index);
            const src = card ? art(card.print_id) : null;
            return (
              <button
                key={index}
                type="button"
                disabled={busy}
                onClick={() => onAct(index)}
                className="relative h-[168px] w-[120px] flex-none overflow-hidden rounded-md border border-edge bg-panel transition-transform hover:-translate-y-1 hover:border-accent disabled:opacity-50"
              >
                <CardFace
                  src={src}
                  name={card?.name ?? label}
                  energyType={card?.energy_type ?? null}
                />
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}

export { asDecision };
