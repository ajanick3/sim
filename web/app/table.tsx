"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { loadSim, type CardData, type Game } from "./wasm";
import type { WirePokemon, WireSide, WireView } from "./view";

const DECKS = { a: "/decks/dragapult.txt", b: "/decks/alakazam.txt" };
const SEAT_NAME = ["Player 1", "Player 2"];

type Status =
  | { kind: "loading" }
  | { kind: "error"; message: string }
  | { kind: "playing" };

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

  const refresh = useCallback(() => {
    const game = gameRef.current;
    if (!game) return;
    const next = game.player_to_act();
    setView(JSON.parse(game.view()) as WireView);
    setActions(JSON.parse(game.legal_actions()) as string[]);
    setLog(JSON.parse(game.log()) as string[]);
    setSeat(next);
    setOver(game.is_over());
    if (next !== shownSeat.current) {
      shownSeat.current = next;
      setRevealed(false);
    }
  }, []);

  const newGame = useCallback(async (seed: number) => {
    try {
      const sim = await loadSim();
      if (!dataRef.current) {
        const cards = await fetch("/cards.json").then((r) => r.text());
        dataRef.current = sim.CardData.new(cards);
      }
      const [a, b] = await Promise.all([
        fetch(DECKS.a).then((r) => r.text()),
        fetch(DECKS.b).then((r) => r.text()),
      ]);
      gameRef.current?.free();
      gameRef.current = sim.Game.standard(dataRef.current, a, b, BigInt(seed));
      shownSeat.current = undefined;
      setStatus({ kind: "playing" });
      refresh();
    } catch (err) {
      setStatus({ kind: "error", message: String(err) });
    }
  }, [refresh]);

  useEffect(() => {
    void newGame(Math.floor(Math.random() * 1_000_000_000));
  }, [newGame]);

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

  if (status.kind === "loading") {
    return <Centre>Loading the engine…</Centre>;
  }

  if (status.kind === "error") {
    return (
      <Centre>
        <p style={{ color: "var(--warn)", maxWidth: 480 }}>{status.message}</p>
        <button onClick={() => newGame(Math.floor(Math.random() * 1e9))}>
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
          Dragapult ex &nbsp;vs&nbsp; Alakazam &nbsp;·&nbsp; turn{" "}
          {view?.turn_number ?? 0} &nbsp;·&nbsp; {view?.phase}
        </span>
        <button
          style={{ marginLeft: "auto" }}
          onClick={() => newGame(Math.floor(Math.random() * 1e9))}
        >
          New game
        </button>
      </header>

      {over ? (
        <Banner>{log[log.length - 1] ?? "Game over."}</Banner>
      ) : !revealed && seat !== undefined ? (
        <Centre>
          <p style={{ color: "var(--dim)" }}>Pass the device.</p>
          <button onClick={() => setRevealed(true)}>
            {SEAT_NAME[seat]} — reveal
          </button>
        </Centre>
      ) : (
        view && (
          <Board
            view={view}
            actions={actions}
            seat={seat}
            busy={busy}
            onAct={act}
          />
        )
      )}

      <LogPanel lines={log} />
    </main>
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
  return (
    <div style={{ display: "grid", gap: 16, marginTop: 16 }}>
      <Side side={view.sides[view.you === 1 ? 0 : 1]} label="Opponent" />
      <Side side={view.sides[view.you]} label="You" mine />

      <section>
        <h2 style={h2}>Your hand ({view.your_hand.length})</h2>
        <div style={{ color: "var(--dim)" }}>
          {view.your_hand.map((c) => c.name).join(" · ") || "—"}
        </div>
      </section>

      <section>
        <h2 style={h2}>
          {seat !== undefined ? `${SEAT_NAME[seat]} to act` : "Waiting"}
        </h2>
        <div style={{ display: "flex", flexWrap: "wrap", gap: 8 }}>
          {actions.map((label, i) => (
            <button key={i} disabled={busy} onClick={() => onAct(i)}>
              {label}
            </button>
          ))}
        </div>
      </section>
    </div>
  );
}

function Side({
  side,
  label,
  mine = false,
}: {
  side: WireSide;
  label: string;
  mine?: boolean;
}) {
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
        <Mon mon={side.active} active />
        <div
          style={{
            display: "flex",
            flexWrap: "wrap",
            justifyContent: "center",
            gap: 8,
          }}
        >
          {side.bench.map((m, i) => (
            <Mon key={i} mon={m} />
          ))}
        </div>
      </div>
    </section>
  );
}

function Mon({
  mon,
  active = false,
}: {
  mon: WirePokemon | null;
  active?: boolean;
}) {
  if (!mon) {
    return (
      <div style={{ ...monBox, color: "var(--dim)" }}>
        {active ? "no Active" : ""}
      </div>
    );
  }
  return (
    <div
      style={{
        ...monBox,
        borderColor: active ? "var(--accent)" : "var(--edge)",
      }}
    >
      <div style={{ fontWeight: 600 }}>{mon.name}</div>
      <div style={{ color: "var(--dim)", fontSize: 12 }}>
        {mon.remaining_hp}/{mon.hp} HP
        {mon.attached.length > 0 && ` · ${mon.attached.length} attached`}
      </div>
      {mon.conditions.length > 0 && (
        <div style={{ color: "var(--warn)", fontSize: 12 }}>
          {mon.conditions.join(", ")}
        </div>
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

const monBox: React.CSSProperties = {
  minWidth: 120,
  padding: 8,
  border: "1px solid var(--edge)",
  borderRadius: 6,
};
