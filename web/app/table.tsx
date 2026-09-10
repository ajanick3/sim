"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { loadSim, type CardData, type Game } from "./wasm";
import {
  COPY_COLORS,
  copyBadges,
  gateAfterSeat,
  groupActions,
  shouldAutoAdvance,
  sides,
} from "./session";
import { newRecipe, readRecipeParam, writeRecipeParam, type Recipe } from "./recipe";
import type { WireCard, WirePokemon, WireSide, WireView } from "./view";

// The two curated decks, by the key a recipe stores.
const DECK_KEYS = { a: "dragapult", b: "alakazam" };
const deckPath = (key: string) => `/decks/${key}.txt`;
const randomSeed = () => Math.floor(Math.random() * 1_000_000_000);

// Seat 0 is the first player, seat 1 the second — shown as medals.
const SEAT_NAME = ["🥇", "🥈"];

type Status = { kind: "loading" } | { kind: "error"; message: string } | { kind: "playing" };

export default function Table() {
  const [status, setStatus] = useState<Status>({ kind: "loading" });
  const [view, setView] = useState<WireView | null>(null);
  const [actions, setActions] = useState<string[]>([]);
  const [log, setLog] = useState<string[]>([]);
  const [seat, setSeat] = useState<number | undefined>(undefined);
  const [over, setOver] = useState(false);
  const [revealed, setRevealed] = useState(false);
  const [busy, setBusy] = useState(false);

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
        <p style={{ color: "var(--warn)", maxWidth: 480 }}>{status.message}</p>
        <button onClick={() => startGame(newRecipe(randomSeed(), DECK_KEYS.a, DECK_KEYS.b))}>
          Try again
        </button>
      </Centre>
    );
  }

  return (
    <main style={{ maxWidth: 960, margin: "0 auto", padding: "24px 16px" }}>
      <header style={{ display: "flex", alignItems: "baseline", gap: 12 }}>
        <h1 style={{ fontSize: 18, margin: 0 }}>sim</h1>
        <span style={{ color: "var(--dim)" }}>
          Dragapult ex &nbsp;vs&nbsp; Alakazam &nbsp;·&nbsp; turn {view?.turn_number ?? 0}{" "}
          &nbsp;·&nbsp; {view?.phase}
        </span>
        <span style={{ marginLeft: "auto", display: "flex", gap: 8 }}>
          <CopyLinkButton />
          <button
            onClick={() => startGame(newRecipe(randomSeed(), DECK_KEYS.a, DECK_KEYS.b), "push")}
          >
            New game
          </button>
        </span>
      </header>

      {over ? (
        <Banner>{log[log.length - 1] ?? "Game over."}</Banner>
      ) : !revealed && seat !== undefined ? (
        <Centre>
          <p style={{ color: "var(--dim)" }}>Pass the device.</p>
          <button onClick={() => setRevealed(true)}>{SEAT_NAME[seat]} — tap to reveal</button>
        </Centre>
      ) : (
        view && <Board view={view} actions={actions} seat={seat} busy={busy} onAct={act} />
      )}

      <LogPanel lines={log} />
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
  seat,
  busy,
  onAct,
}: {
  view: WireView;
  actions: string[];
  seat: number | undefined;
  busy: boolean;
  onAct: (index: number) => void;
}) {
  const seats = sides(view.you);
  return (
    <div style={{ display: "grid", gap: 16, marginTop: 16 }}>
      <Side side={view.sides[seats.opponent]} label={`${SEAT_NAME[seats.opponent]} Opponent`} />
      <Side side={view.sides[seats.mine]} label={`${SEAT_NAME[seats.mine]} You`} mine />

      <section>
        <h2 style={h2}>Your hand ({view.your_hand.length})</h2>
        <div style={{ color: "var(--dim)" }}>
          {view.your_hand.map((c) => c.name).join(" · ") || "—"}
        </div>
      </section>

      <section>
        <h2 style={h2}>{seat !== undefined ? `${SEAT_NAME[seat]} to act` : "Waiting"}</h2>
        <div style={{ display: "grid", gap: 10 }}>
          {groupActions(actions).map((g) => (
            <div key={g.group}>
              <div style={{ ...h2, margin: "0 0 4px" }}>{g.group}</div>
              <div style={{ display: "flex", flexWrap: "wrap", gap: 8 }}>
                {g.items.map((item) => (
                  <button
                    key={item.index}
                    disabled={busy}
                    onClick={() => onAct(item.index)}
                    style={{ display: "flex", alignItems: "center", gap: 6 }}
                  >
                    {item.copy !== undefined && (
                      <span
                        aria-hidden
                        style={{
                          width: 8,
                          height: 8,
                          borderRadius: "50%",
                          background: COPY_COLORS[item.copy % COPY_COLORS.length],
                        }}
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
    </div>
  );
}

function Side({ side, label, mine = false }: { side: WireSide; label: string; mine?: boolean }) {
  const lineup = [side.active, ...side.bench];
  const badges = copyBadges(lineup.map((m) => m?.name ?? null));
  return (
    <section
      style={{
        background: "var(--panel)",
        border: `1px solid ${mine ? "var(--accent)" : "var(--edge)"}`,
        borderRadius: 8,
        padding: 12,
      }}
    >
      <div style={{ display: "flex", gap: 12, color: "var(--dim)", fontSize: 12 }}>
        <strong style={{ color: "var(--text)" }}>{label}</strong>
        <span>hand {side.hand_count}</span>
        <span>deck {side.library_count}</span>
        <span>prizes {side.prize_count}</span>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          gap: 8,
          marginTop: 8,
        }}
      >
        <Mon mon={side.active} active copy={badges[0]} />
        <div
          style={{
            display: "flex",
            flexWrap: "wrap",
            justifyContent: "center",
            gap: 8,
          }}
        >
          {side.bench.map((m, i) => (
            <Mon key={i} mon={m} copy={badges[i + 1]} />
          ))}
        </div>
      </div>
    </section>
  );
}

export function Mon({
  mon,
  active = false,
  copy,
}: {
  mon: WirePokemon | null;
  active?: boolean;
  /** Index among same-named copies on this side; a colour badge is drawn when set. */
  copy?: number;
}) {
  if (!mon) {
    return (
      <div data-testid="mon-card" style={{ ...monBox, color: "var(--dim)" }}>
        {active ? "no Active" : ""}
      </div>
    );
  }
  return (
    <div
      data-testid="mon-card"
      style={{
        ...monBox,
        borderColor: active ? "var(--accent)" : "var(--edge)",
      }}
    >
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 4,
          fontWeight: 600,
          maxWidth: "100%",
        }}
      >
        {copy !== undefined && (
          <span
            data-testid="copy-badge"
            title={`copy ${copy + 1}`}
            style={{
              flex: "none",
              width: 8,
              height: 8,
              borderRadius: "50%",
              background: COPY_COLORS[copy % COPY_COLORS.length],
            }}
          />
        )}
        <span
          style={{
            overflow: "hidden",
            textOverflow: "ellipsis",
            whiteSpace: "nowrap",
          }}
        >
          {mon.name}
        </span>
      </div>
      <div style={{ color: "var(--dim)", fontSize: 12 }}>
        {mon.remaining_hp}/{mon.hp} HP
      </div>
      {/* Reserve the attachment row's height so a Pokémon with no
          attachments is the same shape as one carrying Energy. */}
      <div style={{ minHeight: 14, display: "flex", alignItems: "center" }}>
        <Attachments cards={mon.attached} />
      </div>
      {mon.conditions.length > 0 && (
        <div style={{ color: "var(--warn)", fontSize: 12 }}>{mon.conditions.join(", ")}</div>
      )}
    </div>
  );
}

const ENERGY_COLOR: Record<string, string> = {
  Grass: "#63B95B",
  Fire: "#E4593E",
  Water: "#5AA7E4",
  Lightning: "#F4D023",
  Psychic: "#A461C2",
  Fighting: "#C4622D",
  Darkness: "#5B5466",
  Metal: "#A8A8B5",
  Fairy: "#E993D0",
  Dragon: "#7B6C4E",
  Colorless: "#C6C0B7",
};

function Attachments({ cards }: { cards: WireCard[] }) {
  if (cards.length === 0) return null;
  const energies = cards.filter((c) => c.energy_type);
  const others = cards.length - energies.length;
  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        flexWrap: "wrap",
        gap: 4,
        marginTop: 4,
      }}
    >
      {energies.map((c) => (
        <span
          key={c.id}
          title={`${c.energy_type} Energy`}
          style={{
            width: 10,
            height: 10,
            borderRadius: "50%",
            background: ENERGY_COLOR[c.energy_type as string] ?? "var(--dim)",
            border: "1px solid rgba(0,0,0,0.35)",
          }}
        />
      ))}
      {others > 0 && (
        <span style={{ color: "var(--dim)", fontSize: 12 }}>
          {`+${others} tool${others > 1 ? "s" : ""}`}
        </span>
      )}
    </div>
  );
}

function LogPanel({ lines }: { lines: string[] }) {
  const ref = useRef<HTMLDivElement | null>(null);
  useEffect(() => {
    ref.current?.scrollTo(0, ref.current.scrollHeight);
  }, [lines]);
  return (
    <section style={{ marginTop: 24 }}>
      <h2 style={h2}>Log</h2>
      <div
        ref={ref}
        style={{
          background: "var(--panel)",
          border: "1px solid var(--edge)",
          borderRadius: 8,
          padding: 12,
          maxHeight: 220,
          overflowY: "auto",
          fontSize: 13,
          color: "var(--dim)",
          whiteSpace: "pre-wrap",
        }}
      >
        {lines.length ? lines.join("\n") : "—"}
      </div>
    </section>
  );
}

function Centre({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        minHeight: "60vh",
        display: "flex",
        flexDirection: "column",
        gap: 12,
        alignItems: "center",
        justifyContent: "center",
        textAlign: "center",
      }}
    >
      {children}
    </div>
  );
}

function Banner({ children }: { children: React.ReactNode }) {
  return (
    <div
      style={{
        marginTop: 16,
        padding: 16,
        background: "var(--panel)",
        border: "1px solid var(--accent)",
        borderRadius: 8,
        fontWeight: 600,
      }}
    >
      {children}
    </div>
  );
}

const h2: React.CSSProperties = {
  fontSize: 13,
  textTransform: "uppercase",
  letterSpacing: 0.5,
  color: "var(--dim)",
  margin: "0 0 6px",
};

// Every Pokémon renders in a card of this fixed shape, so a full Bench
// reads as an even row whatever each Pokémon is carrying.
const monBox: React.CSSProperties = {
  width: 132,
  minHeight: 96,
  boxSizing: "border-box",
  padding: 8,
  border: "1px solid var(--edge)",
  borderRadius: 6,
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  justifyContent: "center",
  gap: 4,
  textAlign: "center",
};
