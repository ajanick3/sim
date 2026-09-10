"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { loadSim, type CardData, type Game } from "./wasm";
import {
  COPY_COLORS,
  copyBadges,
  gateAfterSeat,
  groupActions,
  groupActionsAt,
  movesForSelection,
  shouldAutoAdvance,
  sides,
  targetsForHandCard,
  type Selection,
} from "./session";
import { newRecipe, readRecipeParam, writeRecipeParam, type Recipe } from "./recipe";
import { artUrl, loadArtIndex, type ArtIndex } from "./art";
import { LiveBoard } from "./live-board";
import type { WireActionMeta, WireCard, WirePokemon, WireSide, WireView } from "./view";

// The two curated decks, by the key a recipe stores.
const DECK_KEYS = { a: "dragapult", b: "alakazam" };
const deckPath = (key: string) => `/decks/${key}.txt`;
const randomSeed = () => Math.floor(Math.random() * 1_000_000_000);

// Seat 0 is the first player, seat 1 the second — shown as medals.
const SEAT_NAME = ["🥇", "🥈"];

type Status = { kind: "loading" } | { kind: "error"; message: string } | { kind: "playing" };

export type BoardVariant = "classic" | "live";

export default function Table({ variant = "classic" }: { variant?: BoardVariant }) {
  const [status, setStatus] = useState<Status>({ kind: "loading" });
  const [view, setView] = useState<WireView | null>(null);
  const [actions, setActions] = useState<string[]>([]);
  const [meta, setMeta] = useState<WireActionMeta[]>([]);
  const [selection, setSelection] = useState<Selection>(null);
  const [artIndex, setArtIndex] = useState<ArtIndex>({});
  const [log, setLog] = useState<string[]>([]);
  const [seat, setSeat] = useState<number | undefined>(undefined);
  const [over, setOver] = useState(false);
  const [revealedState, setRevealed] = useState(false);
  const [busy, setBusy] = useState(false);
  // The live board is a solo review surface — no pass-the-device gate.
  const revealed = variant === "live" ? true : revealedState;

  const gameRef = useRef<Game | null>(null);
  const dataRef = useRef<CardData | null>(null);
  const shownSeat = useRef<number | undefined>(undefined);
  const seedRef = useRef(0);

  // The recipe for the game as it stands: seed and decks fixed at the
  // start, moves read live from the handle. This is what a shareable link
  // and, later, the server row carry.
  const currentRecipe = useCallback((): Recipe => {
    const moves = gameRef.current ? (JSON.parse(gameRef.current.history()) as number[]) : [];
    return { v: 1, seed: seedRef.current, a: DECK_KEYS.a, b: DECK_KEYS.b, moves };
  }, []);

  // Write the current recipe to the address bar. "push" adds a history
  // entry — one per move — so the browser's Back and Forward buttons step
  // through the game. "replace" rewrites the current entry (a new game, a
  // shared link on load). "skip" writes nothing (a rebuild that a Back or
  // Forward already moved the URL for).
  const writeRecipe = useCallback(
    (mode: "push" | "replace" | "skip") => {
      if (mode === "skip" || !gameRef.current) return;
      const url = writeRecipeParam(currentRecipe());
      if (mode === "push") window.history.pushState(null, "", url);
      else window.history.replaceState(null, "", url);
    },
    [currentRecipe],
  );

  const refresh = useCallback(() => {
    const game = gameRef.current;
    if (!game) return;
    const next = game.player_to_act();
    setView(JSON.parse(game.view()) as WireView);
    setActions(JSON.parse(game.legal_actions()) as string[]);
    setMeta(JSON.parse(game.action_meta()) as WireActionMeta[]);
    // Indices belong to the list that just changed; drop the selection.
    setSelection(null);
    setLog(JSON.parse(game.log()) as string[]);
    setSeat(next);
    setOver(game.is_over());
    const gate = gateAfterSeat(shownSeat.current, next);
    shownSeat.current = gate.shown;
    if (!gate.reveal) setRevealed(false);
  }, []);

  const startGame = useCallback(
    async (recipe: Recipe, write: "push" | "replace" | "skip" = "replace") => {
      try {
        const sim = await loadSim();
        if (!dataRef.current) {
          const cards = await fetch("/cards.json").then((r) => r.text());
          dataRef.current = sim.CardData.new(cards);
        }
        const [a, b] = await Promise.all([
          fetch(deckPath(recipe.a)).then((r) => r.text()),
          fetch(deckPath(recipe.b)).then((r) => r.text()),
        ]);
        gameRef.current?.free();
        seedRef.current = recipe.seed;
        gameRef.current = sim.Game.replay_standard(
          dataRef.current,
          a,
          b,
          BigInt(recipe.seed),
          recipe.moves,
        );
        shownSeat.current = undefined;
        setStatus({ kind: "playing" });
        refresh();
        writeRecipe(write);
      } catch (err) {
        setStatus({ kind: "error", message: String(err) });
      }
    },
    [refresh, writeRecipe],
  );

  useEffect(() => {
    const shared = readRecipeParam(window.location.search);
    void startGame(shared ?? newRecipe(randomSeed(), DECK_KEYS.a, DECK_KEYS.b));
  }, [startGame]);

  // Card art loads on its own; the board draws its own cards until it lands.
  useEffect(() => {
    void loadArtIndex().then(setArtIndex);
  }, []);

  // Back and Forward change the URL; rebuild the game to whatever recipe
  // the new URL carries. The URL is already where it should be, so the
  // rebuild writes nothing back.
  useEffect(() => {
    const onPop = () => {
      const recipe = readRecipeParam(window.location.search);
      if (recipe) void startGame(recipe, "skip");
    };
    window.addEventListener("popstate", onPop);
    return () => window.removeEventListener("popstate", onPop);
  }, [startGame]);

  const act = useCallback(
    (index: number) => {
      const game = gameRef.current;
      if (!game || busy) return;
      setBusy(true);
      try {
        game.apply(index);
        refresh();
        writeRecipe("push");
      } catch (err) {
        setStatus({ kind: "error", message: String(err) });
      } finally {
        setBusy(false);
      }
    },
    [busy, refresh, writeRecipe],
  );

  // A step with exactly one legal action forces the player's hand — there
  // is no choice to make (a "Finish placing" with no Basics left to bench,
  // promoting an only Pokemon, and so on). Apply it for them. This runs
  // only once the seat is revealed, so it never skips the reveal gate; the
  // counter guards against a pathological forced loop.
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
        revealed,
        over,
        busy,
        steps: autoSteps.current,
      })
    ) {
      autoSteps.current += 1;
      act(0);
    }
  }, [actions, status.kind, revealed, over, busy, act]);

  if (status.kind === "loading") {
    return <Centre>Loading the engine…</Centre>;
  }

  if (status.kind === "error") {
    return (
      <Centre>
        <p className="max-w-[480px] text-warn">{status.message}</p>
        <button onClick={() => startGame(newRecipe(randomSeed(), DECK_KEYS.a, DECK_KEYS.b))}>
          Try again
        </button>
      </Centre>
    );
  }

  const live = variant === "live";

  return (
    <main className="mx-auto max-w-[960px] px-3 py-4 sm:px-4 sm:py-6">
      <header className="flex flex-wrap items-baseline gap-x-3 gap-y-1">
        <h1 className="m-0 text-[18px]">sim</h1>
        <span className="ml-auto flex gap-2">
          <CopyLinkButton />
          <button
            onClick={() => startGame(newRecipe(randomSeed(), DECK_KEYS.a, DECK_KEYS.b), "push")}
          >
            New game
          </button>
        </span>
        <span className="basis-full text-[12px] text-dim sm:text-[13px]">
          Dragapult ex &nbsp;vs&nbsp; Alakazam &nbsp;·&nbsp; turn {view?.turn_number ?? 0}{" "}
          &nbsp;·&nbsp; {view?.phase}
        </span>
      </header>

      <div className="contents">
        {over ? (
          <Banner>{log[log.length - 1] ?? "Game over."}</Banner>
        ) : !revealed && seat !== undefined ? (
          <Centre>
            <p className="text-dim">Pass the device.</p>
            <button onClick={() => setRevealed(true)}>{SEAT_NAME[seat]} — tap to reveal</button>
          </Centre>
        ) : (
          view &&
          (variant === "live" ? (
            <LiveBoard
              view={view}
              actions={actions}
              meta={meta}
              selection={selection}
              onSelect={setSelection}
              art={(printId: string) => artUrl(artIndex, printId)}
              seat={seat}
              busy={busy}
              onAct={act}
              log={log}
            />
          ) : (
            <Board
              view={view}
              actions={actions}
              meta={meta}
              selection={selection}
              onSelect={setSelection}
              art={(printId: string) => artUrl(artIndex, printId)}
              seat={seat}
              busy={busy}
              onAct={act}
            />
          ))
        )}
      </div>

      {!live && <LogPanel lines={log} />}
    </main>
  );
}

// The address bar already holds the recipe (`syncUrl`), so a share link is
// just the current URL.
function CopyLinkButton() {
  const [copied, setCopied] = useState(false);
  return (
    <button
      onClick={() => {
        navigator.clipboard?.writeText(window.location.href).then(
          () => {
            setCopied(true);
            setTimeout(() => setCopied(false), 1500);
          },
          () => {},
        );
      }}
    >
      {copied ? "Copied" : "Copy link"}
    </button>
  );
}

function Board({
  view,
  actions,
  meta,
  selection,
  onSelect,
  art,
  seat,
  busy,
  onAct,
}: {
  view: WireView;
  actions: string[];
  meta: WireActionMeta[];
  selection: Selection;
  onSelect: (s: Selection) => void;
  art: (printId: string) => string | null;
  seat: number | undefined;
  busy: boolean;
  onAct: (index: number) => void;
}) {
  const seats = sides(view.you);
  const dropTargets =
    selection?.kind === "hand"
      ? targetsForHandCard(meta, selection.card)
      : new Map<number, number>();

  // Clicking a Pokémon: if a hand card is waiting for a target and this is a
  // valid one, land it; otherwise select the Pokémon.
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

  const shared = { meta, selection, dropTargets, onPokemon, art };
  return (
    <div className="mt-4 space-y-3">
      <div className="overflow-hidden rounded-xl border border-edge bg-felt p-2 sm:p-3">
        <SideBoard
          {...shared}
          side={view.sides[seats.opponent]}
          label={`${SEAT_NAME[seats.opponent]} Opponent`}
        />
        <div className="my-3 border-t border-white/10" />
        <SideBoard
          {...shared}
          side={view.sides[seats.mine]}
          label={`${SEAT_NAME[seats.mine]} You`}
          mine
          hand={view.your_hand}
          onHand={onHand}
        />
      </div>

      <ActionPanel
        actions={actions}
        only={selection ? movesForSelection(meta, selection) : undefined}
        onClearSelection={() => onSelect(null)}
        seat={seat}
        busy={busy}
        onAct={onAct}
      />
    </div>
  );
}

export function ActionPanel({
  actions,
  only,
  onClearSelection,
  seat,
  busy,
  onAct,
}: {
  actions: string[];
  /** When set, show only these action indices — the current selection's moves. */
  only?: number[];
  onClearSelection: () => void;
  seat: number | undefined;
  busy: boolean;
  onAct: (index: number) => void;
}) {
  const groups = only ? groupActionsAt(actions, only) : groupActions(actions);
  return (
    <section>
      <div className="mb-1.5 flex items-center gap-2">
        <SectionHeading>
          {seat !== undefined ? `${SEAT_NAME[seat]} to act` : "Waiting"}
        </SectionHeading>
        {only && (
          <button className="text-[11px]" onClick={onClearSelection}>
            Clear selection
          </button>
        )}
      </div>
      <div className="grid gap-[10px]">
        {only && groups.length === 0 && (
          <p className="text-[12px] text-dim">No move here. Pick something else.</p>
        )}
        {groups.map((g) => (
          <div key={g.group}>
            <SectionHeading className="mb-1">{g.group}</SectionHeading>
            <div className="flex flex-wrap gap-2">
              {g.items.map((item) => (
                <button
                  key={item.index}
                  disabled={busy}
                  onClick={() => onAct(item.index)}
                  className="flex min-h-[34px] items-center gap-1.5"
                >
                  {item.copy !== undefined && (
                    <span
                      aria-hidden
                      className="size-2 rounded-full"
                      style={{ background: COPY_COLORS[item.copy % COPY_COLORS.length] }}
                    />
                  )}
                  {item.label}
                </button>
              ))}
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}

function SideBoard({
  side,
  label,
  mine = false,
  hand,
  meta,
  selection,
  dropTargets,
  onPokemon,
  onHand,
  art,
}: {
  side: WireSide;
  label: string;
  mine?: boolean;
  hand?: WireCard[];
  meta: WireActionMeta[];
  selection: Selection;
  dropTargets: Map<number, number>;
  onPokemon: (id: number) => void;
  onHand?: (card: number) => void;
  art: (printId: string) => string | null;
}) {
  const lineup = [side.active, ...side.bench];
  const badges = copyBadges(lineup.map((m) => m?.name ?? null));

  const monProps = (m: WirePokemon | null) => {
    if (!m) return {};
    return {
      onSelect: () => onPokemon(m.id),
      selectable: meta.some((x) => x.target === m.id) || dropTargets.has(m.id),
      selected: selection?.kind === "pokemon" && selection.id === m.id,
      dropTarget: dropTargets.has(m.id),
      art: art(m.print_id),
    };
  };

  // The player's own Active sits nearest the centre line; the opponent's
  // does too, so their rows read top-down: prizes, bench, Active.
  return (
    <div className={`flex flex-col gap-2 ${mine ? "" : "flex-col-reverse"}`}>
      <div className="flex flex-wrap items-center gap-3 text-[12px] text-dim">
        <strong className="text-text">{label}</strong>
        <Pile label="deck" count={side.library_count} />
        <Pile label="discard" count={side.discard.length} top={side.discard.at(-1)?.name} />
        <span data-testid="prizes" className="flex items-center gap-1">
          prizes
          <span className="flex gap-0.5">
            {Array.from({ length: 6 }, (_, i) => (
              <span
                key={i}
                className={`h-4 w-3 rounded-[2px] border border-white/20 ${
                  i < side.prize_count ? "bg-accent/30" : "bg-transparent"
                }`}
              />
            ))}
          </span>
        </span>
      </div>

      <div className="flex gap-2 overflow-x-auto pb-1 sm:flex-wrap sm:justify-center sm:overflow-visible">
        {side.bench.length === 0 ? (
          <span className="self-center text-[12px] text-dim">bench empty</span>
        ) : (
          side.bench.map((m, i) => <Mon key={i} mon={m} copy={badges[i + 1]} {...monProps(m)} />)
        )}
      </div>

      <div className="flex justify-center">
        <Mon mon={side.active} active copy={badges[0]} {...monProps(side.active)} />
      </div>

      {mine && hand && (
        <div>
          <SectionHeading className="mb-1">Your hand ({hand.length})</SectionHeading>
          <div data-testid="hand" className="flex gap-2 overflow-x-auto pb-1">
            {hand.length === 0 ? (
              <span className="text-[12px] text-dim">empty</span>
            ) : (
              hand.map((c) => (
                <HandCard
                  key={c.id}
                  card={c}
                  art={art(c.print_id)}
                  playable={meta.some((x) => x.card === c.id)}
                  selected={selection?.kind === "hand" && selection.card === c.id}
                  onSelect={onHand ? () => onHand(c.id) : undefined}
                />
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
}

function Pile({ label, count, top }: { label: string; count: number; top?: string }) {
  return (
    <span className="flex items-center gap-1" title={top ? `top: ${top}` : undefined}>
      {label}
      <span className="inline-flex h-6 min-w-6 items-center justify-center rounded border border-white/20 bg-white/5 px-1 text-text">
        {count}
      </span>
    </span>
  );
}

export const CARD_SIZE = "w-[88px] min-h-[116px] sm:w-[104px] sm:min-h-[132px]";

/** The card's TCGdex art, filling the card, with a scrim so overlaid text
 *  stays readable. Falls away (returns null) the moment the image 404s. */
export function CardArt({ src, alt }: { src: string; alt: string }) {
  const [broken, setBroken] = useState(false);
  if (broken) return null;
  return (
    <>
      <img
        src={src}
        alt={alt}
        loading="lazy"
        onError={() => setBroken(true)}
        className="absolute inset-0 size-full rounded-md object-cover"
      />
      <div className="absolute inset-x-0 bottom-0 h-2/3 rounded-b-md bg-gradient-to-t from-black/85 to-transparent" />
    </>
  );
}

function HandCard({
  card,
  art,
  playable = false,
  selected = false,
  onSelect,
}: {
  card: WireCard;
  art?: string | null;
  playable?: boolean;
  selected?: boolean;
  onSelect?: () => void;
}) {
  const interactive = playable && !!onSelect;
  return (
    <button
      type="button"
      data-testid="hand-card"
      disabled={!interactive}
      onClick={onSelect}
      className={`deal-in ${CARD_SIZE} relative flex flex-none flex-col items-start overflow-hidden rounded-md border bg-panel p-1.5 text-left transition-colors disabled:cursor-default disabled:opacity-100 ${
        selected
          ? "border-accent ring-2 ring-accent"
          : interactive
            ? "border-edge hover:border-accent"
            : "border-edge opacity-60"
      }`}
    >
      {art && <CardArt src={art} alt={card.name} />}
      <span className="relative z-10 mt-auto text-[11px] font-semibold leading-tight [text-shadow:0_1px_2px_rgba(0,0,0,0.9)]">
        {card.name}
      </span>
      {card.energy_type && (
        <span
          className="relative z-10 mt-1 size-2.5 rounded-full border border-black/35"
          style={{ background: ENERGY_COLOR[card.energy_type] ?? "var(--color-dim)" }}
        />
      )}
    </button>
  );
}

export function Mon({
  mon,
  active = false,
  copy,
  selectable = false,
  selected = false,
  dropTarget = false,
  onSelect,
  art,
}: {
  mon: WirePokemon | null;
  active?: boolean;
  /** Index among same-named copies on this side; a colour badge is drawn when set. */
  copy?: number;
  /** This Pokémon is named by at least one legal move right now. */
  selectable?: boolean;
  selected?: boolean;
  /** A selected hand card can land here — show it as a drop target. */
  dropTarget?: boolean;
  onSelect?: () => void;
  /** The card art URL, or null to draw the card. */
  art?: string | null;
}) {
  if (!mon) {
    return (
      <div
        data-testid="mon-card"
        className={`${CARD_SIZE} flex flex-col items-center justify-center rounded-md border border-dashed border-edge text-[12px] text-dim`}
      >
        {active ? "no Active" : ""}
      </div>
    );
  }
  const pct = mon.hp > 0 ? Math.max(0, Math.min(100, (mon.remaining_hp / mon.hp) * 100)) : 0;
  const interactive = selectable && !!onSelect;
  const ring = selected
    ? "ring-2 ring-accent border-accent"
    : dropTarget
      ? "ring-2 ring-warn border-warn"
      : active
        ? "border-accent shadow-[0_0_0_1px_var(--color-accent)]"
        : "border-edge";
  return (
    <button
      type="button"
      data-testid="mon-card"
      disabled={!interactive}
      onClick={onSelect}
      className={`deal-in ${CARD_SIZE} relative flex flex-none flex-col gap-1 overflow-hidden rounded-md border bg-panel p-1.5 text-center transition-colors disabled:cursor-default disabled:opacity-100 ${ring} ${
        interactive ? "hover:border-accent" : ""
      }`}
    >
      {art && <CardArt src={art} alt={mon.name} />}
      <div className="relative z-10 flex max-w-full items-center gap-1 text-[11px] font-semibold leading-tight">
        {copy !== undefined && (
          <span
            data-testid="copy-badge"
            title={`copy ${copy + 1}`}
            className="size-2 flex-none rounded-full"
            style={{ background: COPY_COLORS[copy % COPY_COLORS.length] }}
          />
        )}
        {!art && (
          <span className="overflow-hidden text-ellipsis whitespace-nowrap">{mon.name}</span>
        )}
      </div>
      <div className="relative z-10 mt-auto h-1 overflow-hidden rounded-full bg-white/10">
        <div
          className="h-full rounded-full bg-accent transition-[width] duration-500 ease-out"
          style={{ width: `${pct}%` }}
        />
      </div>
      <div className="relative z-10 text-[11px] text-dim [text-shadow:0_1px_2px_rgba(0,0,0,0.9)]">
        {mon.remaining_hp}/{mon.hp}
      </div>
      {/* Reserve the attachment row so a Pokémon carrying nothing keeps
          the same shape as one holding Energy. */}
      <div className="relative z-10 flex min-h-[14px] items-center justify-center">
        <Attachments cards={mon.attached} />
      </div>
      {mon.conditions.length > 0 && (
        <div className="relative z-10 text-[10px] text-warn [text-shadow:0_1px_2px_rgba(0,0,0,0.9)]">
          {mon.conditions.join(", ")}
        </div>
      )}
    </button>
  );
}

// Kept in step with the --color-energy-* tokens in globals.css; matched
// to the printed basic Energy cards of the Mega Evolution era.
export const ENERGY_COLOR: Record<string, string> = {
  Grass: "#4CA858",
  Fire: "#E84B32",
  Water: "#3F9BE0",
  Lightning: "#F7CE14",
  Psychic: "#9B4FB5",
  Fighting: "#C05A28",
  Darkness: "#3B4A5A",
  Metal: "#9AA6B2",
  Fairy: "#E96BAE",
  Dragon: "#C6A63B",
  Colorless: "#D8D3C7",
};

export function Attachments({ cards }: { cards: WireCard[] }) {
  if (cards.length === 0) return null;
  const energies = cards.filter((c) => c.energy_type);
  const others = cards.length - energies.length;
  return (
    <div className="mt-1 flex flex-wrap items-center gap-1">
      {energies.map((c) => (
        <span
          key={c.id}
          title={`${c.energy_type} Energy`}
          className="size-2.5 rounded-full border border-black/35"
          style={{ background: ENERGY_COLOR[c.energy_type as string] ?? "var(--color-dim)" }}
        />
      ))}
      {others > 0 && (
        <span className="text-[12px] text-dim">{`+${others} tool${others > 1 ? "s" : ""}`}</span>
      )}
    </div>
  );
}

export function LogPanel({ lines }: { lines: string[] }) {
  const ref = useRef<HTMLDivElement | null>(null);
  useEffect(() => {
    ref.current?.scrollTo(0, ref.current.scrollHeight);
  }, [lines]);
  const last = lines.length - 1;
  return (
    <section className="mt-6">
      <SectionHeading className="mb-1.5">Log</SectionHeading>
      <div
        ref={ref}
        className="max-h-[220px] overflow-y-auto rounded-lg border border-edge bg-panel p-3 text-[13px]"
      >
        {lines.length === 0 ? (
          <span className="text-dim">—</span>
        ) : (
          lines.map((line, i) => (
            <div
              key={i}
              className={
                i === last
                  ? "line-in whitespace-pre-wrap rounded px-1 text-text"
                  : "whitespace-pre-wrap px-1 text-dim"
              }
            >
              {line}
            </div>
          ))
        )}
      </div>
    </section>
  );
}

function Centre({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex min-h-[60vh] flex-col items-center justify-center gap-3 text-center">
      {children}
    </div>
  );
}

function Banner({ children }: { children: React.ReactNode }) {
  return (
    <div className="mt-4 rounded-lg border border-accent bg-panel p-4 font-semibold">
      {children}
    </div>
  );
}

export function SectionHeading({
  children,
  className = "",
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <h2 className={`m-0 text-[13px] uppercase tracking-[0.5px] text-dim ${className}`}>
      {children}
    </h2>
  );
}
