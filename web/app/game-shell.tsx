"use client";

// The shell around the board: it loads the wasm engine, replays the
// recipe the URL carries, applies moves, and keeps the address bar in
// step so Back and Forward step through the game. The board itself is
// `app/board`.

import { useCallback, useEffect, useRef, useState } from "react";
import { loadSim, type CardData, type Game } from "./wasm";
import { shouldAutoAdvance, type Selection } from "./session";
import { newRecipe, readRecipeParam, writeRecipeParam, type Recipe } from "./recipe";
import { artUrl, loadArtIndex, type ArtIndex } from "./art";
import { LiveBoard } from "./board/LiveBoard";
import type { WireActionMeta, WireView } from "./view";

// The two curated decks, by the key a recipe stores.
const DECK_KEYS = { a: "dragapult", b: "alakazam" };
const deckPath = (key: string) => `/decks/${key}.txt`;
const randomSeed = () => Math.floor(Math.random() * 1_000_000_000);

type Status = { kind: "loading" } | { kind: "error"; message: string } | { kind: "playing" };

export default function GameShell() {
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

  const gameRef = useRef<Game | null>(null);
  const dataRef = useRef<CardData | null>(null);
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
    setView(JSON.parse(game.view()) as WireView);
    setActions(JSON.parse(game.legal_actions()) as string[]);
    setMeta(JSON.parse(game.action_meta()) as WireActionMeta[]);
    // Indices belong to the list that just changed; drop the selection.
    setSelection(null);
    setLog(JSON.parse(game.log()) as string[]);
    setSeat(game.player_to_act());
    setOver(game.is_over());
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
  // promoting an only Pokemon, and so on). Apply it for them; the counter
  // guards against a pathological forced loop.
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
            art={(printId: string) => artUrl(artIndex, printId)}
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

// The address bar already holds the recipe, so a share link is just the
// current URL.
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
