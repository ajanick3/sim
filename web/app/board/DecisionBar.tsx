"use client";

import type { WireActionMeta, WireCard } from "../view";
import { PlayingCard } from "./PlayingCard";
import type { Art, DecisionKind } from "./shared";

/** The card picker for a search / discard / choose prompt: a sticky
 *  header with the verb and a DONE button, then a row of card tiles for
 *  the legal picks. Sits in the document flow, never over the board.
 *
 *  During a whole-library search `library` holds every card in the deck,
 *  sorted. The bar draws all of them and dims the ones this step cannot
 *  take, so the player sees what the search does not reach. */
export function DecisionBar({
  actions,
  decision,
  busy,
  onAct,
  art,
  meta,
  library,
}: {
  actions: string[];
  decision: { kind: DecisionKind; verb: string };
  busy: boolean;
  onAct: (index: number) => void;
  art: Art;
  meta: WireActionMeta[];
  library?: WireCard[] | null;
}) {
  const isFinish = (l: string) => /^(Stop |Finish|Take no more|Move on|Decline)/.test(l);
  const finish = actions.findIndex(isFinish);
  const choices = actions
    .map((label, index) => ({ label, index }))
    .filter(({ label }) => !isFinish(label));

  // Card id -> the action index that takes it, over the takeable choices.
  const takeIndex = new Map<number, number>();
  for (const { index } of choices) {
    const id = meta[index]?.card;
    if (id != null) takeIndex.set(id, index);
  }
  const showLibrary = library != null && takeIndex.size > 0;

  return (
    <div className="mt-3 overflow-hidden rounded-lg border border-edge bg-bg">
      <div className="sticky top-0 z-10 flex items-center gap-3 bg-accent px-3 py-2 text-black">
        <span className="font-bold">{decision.verb}.</span>
        <span className="text-[11px] opacity-70">
          {showLibrary ? "dimmed cards cannot be taken" : "only playable cards are shown"}
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
        {showLibrary
          ? library.map((card) => {
              const index = takeIndex.get(card.id);
              const takeable = index !== undefined;
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
                  disabled={busy || !takeable}
                  onClick={takeable ? () => onAct(index) : undefined}
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
              return (
                <PlayingCard
                  key={index}
                  size="picker"
                  crop="full"
                  src={face ? art(face.print_id) : null}
                  name={face?.name ?? label.replace(/^(Take|Bench) /, "")}
                  energyType={face?.energy_type ?? null}
                  raised
                  disabled={busy}
                  onClick={() => onAct(index)}
                  className="transition-transform hover:-translate-y-1 hover:scale-[1.04] disabled:opacity-50"
                />
              );
            })}
      </div>
    </div>
  );
}
