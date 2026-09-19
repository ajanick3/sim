"use client";

import type { WireActionMeta, WireCard } from "../view";
import type { Selection } from "../session";
import { PlayingCard } from "./PlayingCard";
import type { Art, DecisionKind } from "./shared";

/** The card picker for a search / discard / choose prompt: a sticky
 *  header with the verb and a DONE button, then a row of card tiles for
 *  the legal picks. Sits in the document flow, never over the board.
 *
 *  During a whole-deck search `deck` holds every card in the deck,
 *  sorted. The bar draws all of them and dims the ones this step cannot
 *  take, so the player sees what the search does not reach.
 *
 *  A slot bound to attach (Crispin's second slot, say) offers the same
 *  card onto more than one Pokémon — one `TakeCardOnto` per legal target.
 *  Tapping such a card cannot act by itself, since which target it means
 *  is still unsettled; it selects the card instead, the same as a hand
 *  card, so the board's own drop-target highlighting (`monHooks` /
 *  `targetsForHandCard`) lets the player tap the Pokémon to attach to. */
export function DecisionBar({
  actions,
  decision,
  busy,
  onAct,
  art,
  meta,
  deck,
  selection,
  onSelect,
}: {
  actions: string[];
  decision: { kind: DecisionKind; verb: string };
  busy: boolean;
  onAct: (index: number) => void;
  art: Art;
  meta: WireActionMeta[];
  deck?: WireCard[] | null;
  /** Current board selection, so a card mid-target-choice reads as selected. */
  selection?: Selection;
  /** Selects a card instead of acting, when its target is still unsettled. */
  onSelect?: (s: Selection) => void;
}) {
  const isFinish = (l: string) => /^(Stop |Finish|Take no more|Move on|Decline)/.test(l);
  const finish = actions.findIndex(isFinish);
  const choices = actions
    .map((label, index) => ({ label, index }))
    .filter(({ label }) => !isFinish(label));

  // Card id -> every action index that takes it. Usually one; a slot bound
  // to attach offers one per legal Pokémon, so this can hold several.
  const takeIndex = new Map<number, number[]>();
  for (const { index } of choices) {
    const id = meta[index]?.card;
    if (id == null) continue;
    const list = takeIndex.get(id);
    if (list) list.push(index);
    else takeIndex.set(id, [index]);
  }
  const showDeck = deck != null && takeIndex.size > 0;
  const awaitingTarget = selection?.kind === "hand" ? selection.card : null;

  // A single legal target acts outright; more than one hands the card to
  // the board's drop-target flow instead of guessing which Pokémon to hit.
  const act = (id: number, indices: number[]) => {
    if (indices.length === 1) {
      onAct(indices[0]);
    } else {
      onSelect?.({ kind: "hand", card: id });
    }
  };

  return (
    <div className="mt-3 overflow-hidden rounded-lg border border-edge bg-bg">
      <div className="sticky top-0 z-10 flex items-center gap-3 bg-accent px-3 py-2 text-black">
        <span className="font-bold">{decision.verb}.</span>
        <span className="text-[11px] opacity-70">
          {awaitingTarget != null
            ? "tap a highlighted Pokémon on the board"
            : showDeck
              ? "dimmed cards cannot be taken"
              : "only playable cards are shown"}
        </span>
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
        {showDeck
          ? [...deck]
              .sort((a, b) => Number(takeIndex.has(b.id)) - Number(takeIndex.has(a.id)))
              .map((card) => {
                const indices = takeIndex.get(card.id);
                const takeable = indices !== undefined;
                return (
                  <PlayingCard
                    key={card.id}
                    size="picker"
                    crop="full"
                    src={art(card.print_id)}
                    name={card.name}
                    energyType={card.energy_type}
                    raised={takeable}
                    dimmed={!takeable}
                    selected={awaitingTarget === card.id}
                    disabled={busy || !takeable}
                    onClick={takeable ? () => act(card.id, indices) : undefined}
                    className={
                      takeable
                        ? "transition-transform hover:-translate-y-1 hover:scale-[1.04] disabled:opacity-50"
                        : "cursor-default"
                    }
                  />
                );
              })
          : choices.map(({ label, index }) => {
              const face = meta[index]?.card_face ?? null;
              const id = meta[index]?.card;
              const indices = id != null ? (takeIndex.get(id) ?? [index]) : [index];
              return (
                <PlayingCard
                  key={index}
                  size="picker"
                  crop="full"
                  src={face ? art(face.print_id) : null}
                  name={face?.name ?? label.replace(/^(Take|Bench) /, "")}
                  energyType={face?.energy_type ?? null}
                  raised
                  selected={id != null && awaitingTarget === id}
                  disabled={busy}
                  onClick={() => (id != null ? act(id, indices) : onAct(index))}
                  className="transition-transform hover:-translate-y-1 hover:scale-[1.04] disabled:opacity-50"
                />
              );
            })}
      </div>
    </div>
  );
}
