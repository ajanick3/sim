"use client";

// The shell around the board: it loads the wasm engine, replays the
// recipe the URL carries, applies moves, and keeps the address bar in
// step. Back and Forward move through the game because the recipe lives
// in the query string and `useSearchParams` reacts to it.

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { loadSim, type CardData, type Game } from "./wasm";
import { shouldAutoAdvance, type Selection } from "./session";
import { decodeRecipe, encodeRecipe, newRecipe, type Recipe } from "./recipe";
import { artUrl, loadArtIndex, type ArtIndex } from "./art";
import { DEFAULT_DECKS } from "./decks";
import { noteRecent } from "./recent";
import { LiveBoard } from "./board/LiveBoard";
import type { WireActionMeta, WireView } from "./view";

const deckPath = (key: string) => `/decks/${key}.txt`;
const randomSeed = () => Math.floor(Math.random() * 1_000_000_000);

type Status = { kind: "loading" } | { kind: "error"; message: string } | { kind: "playing" };

export default function GameShell() {
  const router = useRouter();
  const params = useSearchParams();

  const [status, setStatus] = useState<Status>({ kind: "loading" });
  const [view, setView] = useState<WireView | null>(null);
  const [actions, setActions] = useState<string[]>([]);
  const [meta, setMeta] = useState<WireActionMeta[]>([]);
  const [selection, setSelection] = useState<Selection>(null);
  const [artIndex, setArtIndex] = useState<ArtIndex>({});
  const [log, setLog] = useState<string[]>([]);
  const [seat, setSeat] = useState<number | undefined>(undefined);
  const [over, setOver] = useState(false);
  const [busy, setBusy] = useState(false);
  const [matchup, setMatchup] = useState<{ a: string; b: string } | null>(null);

  const gameRef = useRef<Game | null>(null);
  const dataRef = useRef<CardData | null>(null);
  const recipeRef = useRef<Recipe | null>(null);
  // The `?g=` we last wrote, so reacting to our own URL change is a no-op.
  const lastWritten = useRef<string | null>(null);

  const currentG = useCallback((): string => {
    const base = recipeRef.current ?? newRecipe(0, DEFAULT_DECKS.a, DEFAULT_DECKS.b);
    const moves = gameRef.current ? (JSON.parse(gameRef.current.history()) as number[]) : [];
    return encodeRecipe({ ...base, moves });
  }, []);

  const refresh = useCallback(() => {
    const game = gameRef.current;
    if (!game) return;
    const wire = JSON.parse(game.view()) as WireView;
    setView(wire);
    setActions(JSON.parse(game.legal_actions()) as string[]);
    setMeta(JSON.parse(game.action_meta()) as WireActionMeta[]);
    setSelection(null);
    setLog(JSON.parse(game.log()) as string[]);
    setSeat(game.player_to_act());
    setOver(game.is_over());
    return wire;
  }, []);

  const remember = useCallback((g: string, recipe: Recipe, turn: number) => {
    noteRecent({ g, seed: recipe.seed, a: recipe.a, b: recipe.b, turn, at: Date.now() });
  }, []);

  const startGame = useCallback(
    async (recipe: Recipe, write: "push" | "replace" | "none") => {
      try {
        setStatus((s) => (s.kind === "playing" ? s : { kind: "loading" }));
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
        recipeRef.current = recipe;
        setMatchup({ a: recipe.a, b: recipe.b });
        gameRef.current = sim.Game.replay_standard(
          dataRef.current,
          a,
          b,
          BigInt(recipe.seed),
          recipe.moves,
        );
        setStatus({ kind: "playing" });
        const wire = refresh();
        const g = currentG();
        lastWritten.current = g;
        if (write === "replace") router.replace(`?g=${g}`, { scroll: false });
        else if (write === "push") router.push(`?g=${g}`, { scroll: false });
        remember(g, recipe, wire?.turn_number ?? 0);
      } catch (err) {
        setStatus({ kind: "error", message: String(err) });
      }
    },
    [refresh, currentG, remember, router],
  );

  useEffect(() => {
    void loadArtIndex().then(setArtIndex);
  }, []);

  // The single source of what game is on screen: the query string.
  // Runs on load and on every Back / Forward.
  useEffect(() => {
    const g = params.get("g");
    if (g) {
      if (g === lastWritten.current) return;
      const recipe = decodeRecipe(g);
      if (recipe) void startGame(recipe, "none");
      return;
    }
    const a = params.get("a") ?? DEFAULT_DECKS.a;
    const b = params.get("b") ?? DEFAULT_DECKS.b;
    void startGame(newRecipe(randomSeed(), a, b), "replace");
  }, [params, startGame]);

  const act = useCallback(
    (index: number) => {
      const game = gameRef.current;
      if (!game || busy) return;
      setBusy(true);
      try {
        game.apply(index);
        const wire = refresh();
        const g = currentG();
        lastWritten.current = g;
        router.push(`?g=${g}`, { scroll: false });
        if (recipeRef.current) remember(g, recipeRef.current, wire?.turn_number ?? 0);
      } catch (err) {
        setStatus({ kind: "error", message: String(err) });
      } finally {
        setBusy(false);
      }
    },
    [busy, refresh, currentG, remember, router],
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
        revealed: true,
        over,
        busy,
        steps: autoSteps.current,
      })
    ) {
      autoSteps.current += 1;
      act(0);
    }
  }, [actions, status.kind, over, busy, act]);

  const art = useMemo(() => (printId: string) => artUrl(artIndex, printId), [artIndex]);

  if (status.kind === "loading") {
    return <Centre>Loading the engine…</Centre>;
  }
  if (status.kind === "error") {
    return (
      <Centre>
        <p className="max-w-[480px] text-warn">{status.message}</p>
        <Link href="/" className="underline">
          Back to the decks
        </Link>
      </Centre>
    );
  }

  return (
    <main className="mx-auto max-w-[960px] px-3 py-4 sm:px-4 sm:py-6">
      <header className="flex flex-wrap items-baseline gap-x-3 gap-y-1">
        <Link href="/" className="m-0 text-[18px] no-underline">
          sim
        </Link>
        <span className="ml-auto flex gap-2">
          <CopyLinkButton />
          <Link
            href="/"
            className="rounded-md border border-edge px-2.5 py-1 text-[13px] no-underline hover:border-accent"
          >
            New game
          </Link>
          <Link
            href="/settings"
            className="rounded-md border border-edge px-2.5 py-1 text-[13px] no-underline hover:border-accent"
          >
            Settings
          </Link>
        </span>
        <span className="basis-full text-[12px] text-dim sm:text-[13px]">
          {matchup ? `${deckLabel(matchup.a)} vs ${deckLabel(matchup.b)}` : ""} &nbsp;·&nbsp; turn{" "}
          {view?.turn_number ?? 0} &nbsp;·&nbsp; {view?.phase}
        </span>
      </header>

      {over ? (
        <Banner>{log[log.length - 1] ?? "Game over."}</Banner>
      ) : (
        view && (
          <LiveBoard
            view={view}
            actions={actions}
            meta={meta}
            selection={selection}
            onSelect={setSelection}
            art={art}
            seat={seat}
            busy={busy}
            onAct={act}
            log={log}
          />
        )
      )}
    </main>
  );
}

const deckLabel = (key: string) =>
  key
    .replace(/^\d+-/, "")
    .replace(/-/g, " ")
    .replace(/\b\w/g, (c) => c.toUpperCase());

function CopyLinkButton() {
  const [copied, setCopied] = useState(false);
  return (
    <button
      className="rounded-md border border-edge px-2.5 py-1 text-[13px] hover:border-accent"
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
