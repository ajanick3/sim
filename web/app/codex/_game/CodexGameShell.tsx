"use client";

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { loadSim, type CardData, type Game } from "../../wasm";
import { shouldAutoAdvance } from "../../session";
import { decodeRecipe, encodeRecipe, newRecipe, type Recipe } from "../../recipe";
import { artUrl, isInstalled, loadArtIndex, type ArtIndex, type ArtQuality } from "../../art";
import { DEFAULT_DECKS, isPastedDeck, pastedDeckText } from "../../decks";
import { loadPrintPrefs } from "../../printPrefs";
import { buildPrintIndex, resolveCardPrint, type CatalogCard, type PrintIndex } from "../../prints";
import type { WireActionMeta, WireView } from "../../view";
import { Board } from "../_components/regions/Board";
import { DeckSearchDialog } from "../_components/dialogs/DeckSearchDialog";
import { ActionDialog } from "../_components/dialogs/ActionDialog";
import {
  actionChoices,
  actionIndexForCard,
  boardFromView,
  deckAssetPath,
  searchCardsFromView,
} from "./adapter";
import theme from "../_styles/theme.module.css";
import styles from "./game.module.css";

type Status = { kind: "loading" } | { kind: "error"; message: string } | { kind: "playing" };
const randomSeed = () => Math.floor(Math.random() * 1_000_000_000);

export function CodexGameShell() {
  const router = useRouter();
  const params = useSearchParams();
  const [status, setStatus] = useState<Status>({ kind: "loading" });
  const [view, setView] = useState<WireView | null>(null);
  const [actions, setActions] = useState<string[]>([]);
  const [meta, setMeta] = useState<WireActionMeta[]>([]);
  const [busy, setBusy] = useState(false);
  const [over, setOver] = useState(false);
  const [selectedHandId, setSelectedHandId] = useState<number | null>(null);
  const [selectedPokemonId, setSelectedPokemonId] = useState<number | null>(null);
  const [dialogIndices, setDialogIndices] = useState<number[]>([]);
  // Dismissing the auto-shown (not card-selected) action dialog must not
  // just clear dialogIndices — it's empty already, so the same "generic"
  // list would reopen it on the very next render. This flag suppresses
  // that list until the next legal-action list arrives from a real move.
  const [genericDismissed, setGenericDismissed] = useState(false);
  const [artIndex, setArtIndex] = useState<ArtIndex>({});
  const [artQuality] = useState<ArtQuality>(() => (isInstalled() ? "high" : "low"));
  const [printPrefs] = useState<Record<string, string>>(() => loadPrintPrefs());
  const [printIndex, setPrintIndex] = useState<PrintIndex>({ byId: new Map() });
  const gameRef = useRef<Game | null>(null);
  const dataRef = useRef<CardData | null>(null);
  const recipeRef = useRef<Recipe | null>(null);
  const lastWritten = useRef<string | null>(null);

  const currentRecipe = useCallback(() => {
    const base = recipeRef.current ?? newRecipe(0, DEFAULT_DECKS.a, DEFAULT_DECKS.b);
    const moves = gameRef.current ? (JSON.parse(gameRef.current.history()) as number[]) : [];
    return encodeRecipe({ ...base, moves });
  }, []);

  const refresh = useCallback(() => {
    if (!gameRef.current) return null;
    const nextView = JSON.parse(gameRef.current.view()) as WireView;
    setView(nextView);
    setActions(JSON.parse(gameRef.current.legal_actions()) as string[]);
    setMeta(JSON.parse(gameRef.current.action_meta()) as WireActionMeta[]);
    setOver(gameRef.current.is_over());
    setSelectedHandId(null);
    setSelectedPokemonId(null);
    setDialogIndices([]);
    setGenericDismissed(false);
    return nextView;
  }, []);

  const startGame = useCallback(
    async (recipe: Recipe, write: boolean) => {
      try {
        const sim = await loadSim();
        if (!dataRef.current) {
          const cards = await fetch("/cards.json").then((response) => response.text());
          dataRef.current = sim.CardData.new(cards);
          const catalog = JSON.parse(cards) as { cards: CatalogCard[] };
          setPrintIndex(buildPrintIndex(catalog.cards ?? []));
        }
        const readDeck = (key: string) =>
          isPastedDeck(key)
            ? Promise.resolve(pastedDeckText(key))
            : fetch(deckAssetPath(key)).then((response) => response.text());
        const [a, b] = await Promise.all([readDeck(recipe.a), readDeck(recipe.b)]);
        gameRef.current?.free();
        recipeRef.current = recipe;
        gameRef.current = sim.Game.replay_standard(
          dataRef.current,
          a,
          b,
          BigInt(recipe.seed),
          recipe.moves,
        );
        refresh();
        setStatus({ kind: "playing" });
        if (write) {
          const encoded = currentRecipe();
          lastWritten.current = encoded;
          router.replace(`?g=${encoded}`, { scroll: false });
        }
      } catch (error) {
        setStatus({ kind: "error", message: String(error) });
      }
    },
    [currentRecipe, refresh, router],
  );

  useEffect(() => {
    void loadArtIndex().then(setArtIndex);
  }, []);

  useEffect(() => {
    const encoded = params.get("g");
    if (encoded) {
      if (encoded === lastWritten.current) return;
      const recipe = decodeRecipe(encoded);
      // oxlint-disable-next-line react/set-state-in-effect -- URL recipes initialize external WASM state.
      if (recipe) void startGame(recipe, false);
      return;
    }
    void startGame(
      newRecipe(
        randomSeed(),
        params.get("a") ?? DEFAULT_DECKS.a,
        params.get("b") ?? DEFAULT_DECKS.b,
      ),
      true,
    );
  }, [params, startGame]);

  const act = useCallback(
    (index: number) => {
      if (!gameRef.current || busy) return;
      setBusy(true);
      try {
        gameRef.current.apply(index);
        refresh();
        const encoded = currentRecipe();
        lastWritten.current = encoded;
        router.push(`?g=${encoded}`, { scroll: false });
      } catch (error) {
        setStatus({ kind: "error", message: String(error) });
      } finally {
        setBusy(false);
      }
    },
    [busy, currentRecipe, refresh, router],
  );

  const autoSteps = useRef(0);
  useEffect(() => {
    if (actions.length !== 1) {
      autoSteps.current = 0;
      return;
    }
    if (
      shouldAutoAdvance({
        actionCount: 1,
        playing: status.kind === "playing",
        revealed: true,
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

  const art = useMemo(
    () => (printId: string) =>
      artUrl(artIndex, resolveCardPrint(printIndex, printId, printPrefs), artQuality),
    [artIndex, artQuality, printIndex, printPrefs],
  );
  const encodedParam = params.get("g");
  const invalidLink = encodedParam !== null && decodeRecipe(encodedParam) === null;
  const chooseCard = (id: number, kind: "hand" | "pokemon") => {
    // Retreat's own action names the Bench Pokémon it promotes, not the
    // Active it retreats — so a tap on the Active itself only turns up
    // Retreat here, alongside Attack, by asking for both by kind rather
    // than by target.
    const isActiveTap = kind === "pokemon" && view?.sides[view.you].active?.id === id;
    const indices =
      kind === "hand"
        ? actionIndexForCard(meta, id)
        : meta.flatMap((action, index) =>
            action.target === id ||
            (isActiveTap && (action.kind === "Attack" || action.kind === "Retreat"))
              ? [index]
              : [],
          );
    if (kind === "hand") setSelectedHandId(id);
    else setSelectedPokemonId(id);
    // A tap on the Active can mean either "attack" or "retreat" even
    // when only one option is currently legal, so it always opens the
    // dialog rather than committing to that option on the spot — unlike
    // a hand card or a Bench tap, where a single match is unambiguous.
    if (indices.length === 1 && !isActiveTap) act(indices[0]);
    else setDialogIndices(indices);
  };

  if (invalidLink || status.kind !== "playing" || !view) {
    return (
      <main className={`${theme.theme} ${styles.status}`}>
        {invalidLink
          ? "The saved game link is invalid."
          : status.kind === "error"
            ? status.message
            : "Loading the engine…"}
      </main>
    );
  }

  const board = boardFromView(view, art);
  const searchCards = searchCardsFromView(view, meta, art);
  const endTurn = actions.findIndex((label) => /^End turn$/i.test(label));
  const finishSearch =
    searchCards.length > 0
      ? meta.findIndex((action) => action.is_fallback === true && action.kind !== "EndTurn")
      : -1;
  // Any other fallback action — Finish placing, Decline bonus draws, and
  // the rest (WireActionMeta.is_fallback, ADR 0107) — gets the same
  // persistent button treatment as End turn, below, instead of the
  // auto-shown dialog: it's always legal on its own, so popping a modal
  // for it blocks placing a second Pokémon (or any other real, optional
  // move) behind an unprompted, undismissable "Choose an action" — the
  // dialog is for a genuine choice among several actions, not a single
  // "you may stop now" the engine already guarantees is safe.
  const otherFallback = meta.findIndex(
    (action, index) => action.is_fallback && index !== endTurn && index !== finishSearch,
  );
  // The deck-search drawer offers `finishSearch` as its own header
  // button, and otherFallback gets its own persistent button (below),
  // so neither belongs in the auto-shown dialog's choices either.
  const generic = meta.flatMap((action, index) =>
    action.card == null &&
    action.target == null &&
    index !== endTurn &&
    index !== finishSearch &&
    index !== otherFallback
      ? [index]
      : [],
  );
  const visibleDialogIndices = dialogIndices.length
    ? dialogIndices
    : generic.length > 0 && !genericDismissed
      ? generic
      : [];

  return (
    <div className={`${theme.theme} ${styles.game}`}>
      <Board
        {...board}
        selectedHandId={selectedHandId}
        selectedPokemonId={selectedPokemonId}
        onHandSelect={(id) => chooseCard(id, "hand")}
        onPokemonSelect={(id) => chooseCard(id, "pokemon")}
      />
      <div className={styles.turnStatus}>
        Turn {view.turn_number} · {view.phase}
      </div>
      {endTurn >= 0 && (
        <button className={styles.endTurn} disabled={busy} onClick={() => act(endTurn)}>
          End turn
        </button>
      )}
      {/* Each phase pushes at most one fallback other than End turn (its
          own function returns before End turn's push), so this can never
          collide with the button above. */}
      {otherFallback >= 0 && (
        <button className={styles.endTurn} disabled={busy} onClick={() => act(otherFallback)}>
          {actions[otherFallback]}
        </button>
      )}
      {over && <div className={styles.gameOver}>Game over</div>}
      {searchCards.length > 0 && (
        <DeckSearchDialog
          cards={searchCards}
          onDone={finishSearch >= 0 ? () => act(finishSearch) : undefined}
          doneLabel={finishSearch >= 0 ? actions[finishSearch] : undefined}
          onConfirm={([id]) => {
            const index = actionIndexForCard(meta, id)[0];
            if (index !== undefined) act(index);
          }}
        />
      )}
      {visibleDialogIndices.length > 0 && (
        <ActionDialog
          title="Choose an action"
          actions={actionChoices(actions, visibleDialogIndices, view.phase)}
          onChoose={act}
          // Canceling a card-tapped dialog is always safe — it only clears
          // the player's own selection, no legal action goes unresolved.
          // An auto-shown (generic) dialog is only cancelable when End
          // turn is itself legal, a real fallback the engine allows; a
          // phase like TakingBonusDraws offers no such fallback; without
          // one, "actions.length > shown" is true (some *other* generic
          // action exists) but none of them are an escape, so canceling
          // would strand the player with nothing left to do.
          showCancel={dialogIndices.length > 0 || endTurn >= 0}
          onCancel={() => {
            setSelectedHandId(null);
            setSelectedPokemonId(null);
            setDialogIndices([]);
            setGenericDismissed(true);
          }}
        />
      )}
    </div>
  );
}
