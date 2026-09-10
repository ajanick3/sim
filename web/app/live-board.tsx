"use client";

// An experimental board that arranges the game the way Pokémon TCG Live
// does: a tilted felt mat with prize stacks, deck and discard piles, a
// stadium slot, bench trays, and a hand strip, plus a right-hand rail
// with prize counts and an END TURN button, and a blue decision bar for
// search / discard prompts. Reached at /live. Shares the engine wiring
// with the classic board; only the presentation differs.

import { ActionPanel, CardArt, ENERGY_COLOR } from "./table";
import { COPY_COLORS, copyBadges, targetsForHandCard, type Selection } from "./session";
import type { WireActionMeta, WireCard, WirePokemon, WireSide, WireView } from "./view";

const SEAT_NAME = ["🥇", "🥈"];

type Art = (printId: string) => string | null;

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

  return (
    <div className="mt-3">
      <div className="relative flex gap-2">
        {/* The mat, under a mild tilt like the real client. */}
        <div className="min-w-0 flex-1 [perspective:1400px]">
          <div className="origin-top rounded-2xl border border-edge bg-felt p-2 [transform:rotateX(6deg)] sm:p-3">
            <SideRow side={opp} label={`${SEAT_NAME[opp.player]} Opponent`} art={art} />

            <div className="my-2 flex items-stretch justify-center gap-3">
              <StadiumSlot />
              <div className="flex flex-col items-center gap-2">
                <LiveMon
                  mon={opp.active}
                  active
                  art={art}
                  {...monHooks(opp.active, meta, selection, dropTargets, onPokemon)}
                />
                <div className="text-[10px] uppercase tracking-widest text-dim">active</div>
                <LiveMon
                  mon={mine.active}
                  active
                  art={art}
                  {...monHooks(mine.active, meta, selection, dropTargets, onPokemon)}
                />
              </div>
              <div className="w-[92px]" aria-hidden />
            </div>

            <SideRow side={mine} label={`${SEAT_NAME[mine.player]} You`} mine art={art} />
          </div>

          {/* The hand sits flat, off the tilted mat. */}
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

      {decision ? (
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
          <ActionPanel
            actions={actions}
            only={undefined}
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
}: {
  side: WireSide;
  label: string;
  mine?: boolean;
  art: Art;
}) {
  const badges = copyBadges(side.bench.map((m) => m?.name ?? null));
  return (
    <div className={`flex items-start gap-2 ${mine ? "" : "flex-row-reverse"}`}>
      <PrizeStack count={side.prize_count} />
      <div
        className={`flex-1 rounded-lg border-2 p-1.5 ${
          mine ? "border-accent/60" : "border-warn/50"
        }`}
      >
        <div className="mb-1 flex items-center justify-between text-[10px] uppercase tracking-widest text-dim">
          <span>{label}</span>
          <span>
            bench {side.bench.length}/5 · hand {side.hand_count}
          </span>
        </div>
        <div className="flex gap-1.5 overflow-x-auto">
          {side.bench.length === 0 ? (
            <span className="px-2 py-6 text-[11px] text-dim">bench empty</span>
          ) : (
            side.bench.map((m, i) => <LiveMon key={i} mon={m} art={art} copy={badges[i]} small />)
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

function StadiumSlot() {
  return (
    <div className="flex h-full w-[92px] flex-col items-center justify-center rounded border border-dashed border-white/20 text-center text-[9px] text-dim">
      stadium
      <span className="opacity-60">(not in view yet)</span>
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
}) {
  const size = small ? "w-[62px] min-h-[86px]" : "w-[112px] min-h-[156px]";
  if (!mon) {
    return (
      <div
        className={`${size} flex items-center justify-center rounded-md border border-dashed border-white/20 text-[10px] text-dim`}
      >
        {active ? "no Active" : ""}
      </div>
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
      {src && <CardArt src={src} alt={mon.name} />}
      {!src && (
        <span className="relative z-10 p-1 text-[9px] font-semibold leading-tight">{mon.name}</span>
      )}

      {/* HP pill, top-right. */}
      <span className="absolute right-0.5 top-0.5 z-10 rounded bg-black/70 px-1 text-[9px] font-bold">
        {mon.hp}
      </span>
      {/* Damage counter, orange, when hurt. */}
      {mon.damage > 0 && (
        <span className="absolute right-0.5 top-4 z-10 rounded-full bg-orange-500 px-1 text-[9px] font-bold text-black">
          {mon.damage}
        </span>
      )}
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
  return (
    <div className="mt-2 rounded-lg border-2 border-cyan-400/60 p-1.5">
      <div className="mb-1 text-[10px] uppercase tracking-widest text-dim">
        Hand ({hand.length})
      </div>
      <div className="flex gap-1.5 overflow-x-auto">
        {hand.length === 0 && <span className="px-2 py-8 text-[11px] text-dim">empty</span>}
        {hand.map((c) => {
          const playable = meta.some((x) => x.card === c.id);
          const selected = selection?.kind === "hand" && selection.card === c.id;
          const src = art(c.print_id);
          return (
            <button
              key={c.id}
              type="button"
              disabled={!playable}
              onClick={() => onHand(c.id)}
              className={`relative h-[132px] w-[94px] flex-none overflow-hidden rounded-md border bg-panel transition-transform hover:-translate-y-1 disabled:translate-y-0 disabled:opacity-50 ${
                selected ? "border-accent ring-2 ring-accent" : "border-edge"
              }`}
            >
              {src ? (
                <CardArt src={src} alt={c.name} />
              ) : (
                <span className="p-1 text-left text-[10px] font-semibold leading-tight">
                  {c.name}
                </span>
              )}
            </button>
          );
        })}
      </div>
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
  const artFor = (index: number) => {
    const c = meta[index]?.card;
    if (c === null || c === undefined) return null;
    const card = cardById.get(c);
    return card ? art(card.print_id) : null;
  };

  return (
    <div className="mt-3">
      <div className="flex items-center gap-3 rounded-md bg-accent px-3 py-2 text-black">
        <span className="font-bold">{decision.verb}.</span>
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
      <div className="mt-2 flex gap-2 overflow-x-auto pb-1">
        {choices.map(({ label, index }) => {
          const src = artFor(index);
          return (
            <button
              key={index}
              type="button"
              disabled={busy}
              onClick={() => onAct(index)}
              className="relative h-[150px] w-[108px] flex-none overflow-hidden rounded-md border border-edge bg-panel transition-transform hover:-translate-y-1 disabled:opacity-50"
            >
              {src ? (
                <CardArt src={src} alt={label} />
              ) : (
                <span className="block p-1 text-left text-[11px] font-semibold leading-tight">
                  {label}
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
