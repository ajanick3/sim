import type { Meta, StoryObj } from "@storybook/react";
import { Card } from "./primitives/Card";
import { HealthBar } from "./atoms/HealthBar";
import { DamageCounter } from "./atoms/DamageCounter";
import { PokeBall } from "./atoms/PokeBall";
import { DeckRegion } from "./regions/DeckRegion";
import { DiscardRegion } from "./regions/DiscardRegion";
import pikachuEx from "./assets/pikachu-ex.png";
import "./board.pure-css.css";

const meta = { title: "board/PureCSS" } satisfies Meta;
export default meta;
type Story = StoryObj<typeof meta>;

function CardSlot({ hp, damage }: { hp?: number; damage?: number }) {
  return (
    <div className="cardSlot">
      <Card size="bench" fluid name="Pikachu ex" src={pikachuEx}>
        {hp && <HealthBar hp={hp} tiny />}
        {damage ? <DamageCounter damage={damage} top="40%" left="50%" /> : null}
      </Card>
    </div>
  );
}

function FiveCardRow({ keyPrefix }: { keyPrefix: string }) {
  return (
    <>
      {Array.from({ length: 5 }, (_, i) => (
        <CardSlot key={`${keyPrefix}-${i}`} />
      ))}
    </>
  );
}

function PrizeGrid({ remaining }: { remaining: number }) {
  return (
    <div className="prizeGrid">
      {Array.from({ length: 6 }, (_, i) => (
        <PokeBall key={i} size={12} taken={i >= remaining} />
      ))}
    </div>
  );
}

/**
 * The same board `board.chatgpt-layout.stories.tsx` mocks up, with
 * `board.pure-css.css` reworked so nothing needs a named grid area:
 * every section is a plain 3-column grid, and every child gets only
 * a `grid-column` (plus a row span for the two side columns) — never
 * an area name. Two children sharing a column with no span between
 * them (Bench and Active, here) stack in the order they appear below;
 * reorder the JSX and they reorder on the board, no CSS edit needed.
 * Compare against `Reordered`, which is this story with exactly two
 * lines swapped.
 *
 * Same three structural calls as the ChatGPT-layout mockup:
 * perspective on the viewport once (not per card), no near/far size
 * difference between the two sides, and a golden-ratio vertical split
 * with each Active pulled to the centre line.
 */
export const PureCSS: Story = {
  render: () => (
    <div className="boardViewport">
      <main className="board">
        <section className="side">
          <div className="sideLeft">
            <DeckRegion count={46} />
            <DiscardRegion cards={[]} />
          </div>
          {/* Bench above Active — swapping this order is the whole
              "Reordered" story. */}
          <div className="bench">
            <FiveCardRow keyPrefix="opp-bench" />
          </div>
          <div className="active activePullDown">
            <CardSlot hp={200} damage={30} />
          </div>
          <div className="sideRight">
            <PrizeGrid remaining={5} />
          </div>
        </section>

        <div className="centerLine" />

        <section className="side">
          {/* Active above Bench — the near side reads top-to-bottom
              the opposite of the far side, both leaning toward the
              centre line between them. */}
          <div className="active activePullUp">
            <CardSlot hp={200} damage={60} />
          </div>
          <div className="bench">
            <FiveCardRow keyPrefix="you-bench" />
          </div>
          <div className="sideLeft">
            <PrizeGrid remaining={4} />
          </div>
          <div className="sideRight">
            <DeckRegion count={44} />
            <DiscardRegion cards={[{ id: 1, name: "Pikachu ex", src: pikachuEx }]} />
          </div>
          <div className="hand">
            {/* 10 cards, five columns — two rows without any extra logic. */}
            <div className="handRow">
              <FiveCardRow keyPrefix="hand-1" />
            </div>
            <div className="handRow">
              <FiveCardRow keyPrefix="hand-2" />
            </div>
          </div>
        </section>
      </main>
    </div>
  ),
};

/** `PureCSS` with the near side's Bench and Active swapped — nothing
 *  in `board.pure-css.css` changed to get this; only the order of two
 *  `<div>`s in the story below did. */
export const Reordered: Story = {
  render: () => (
    <div className="boardViewport">
      <main className="board">
        <section className="side">
          <div className="sideLeft">
            <DeckRegion count={46} />
            <DiscardRegion cards={[]} />
          </div>
          <div className="bench">
            <FiveCardRow keyPrefix="opp-bench" />
          </div>
          <div className="active activePullDown">
            <CardSlot hp={200} damage={30} />
          </div>
          <div className="sideRight">
            <PrizeGrid remaining={5} />
          </div>
        </section>

        <div className="centerLine" />

        <section className="side">
          {/* Swapped: Bench now listed before Active, so Bench renders
              on top instead. */}
          <div className="bench">
            <FiveCardRow keyPrefix="you-bench" />
          </div>
          <div className="active activePullUp">
            <CardSlot hp={200} damage={60} />
          </div>
          <div className="sideLeft">
            <PrizeGrid remaining={4} />
          </div>
          <div className="sideRight">
            <DeckRegion count={44} />
            <DiscardRegion cards={[{ id: 1, name: "Pikachu ex", src: pikachuEx }]} />
          </div>
          <div className="hand">
            <div className="handRow">
              <FiveCardRow keyPrefix="hand-1" />
            </div>
            <div className="handRow">
              <FiveCardRow keyPrefix="hand-2" />
            </div>
          </div>
        </section>
      </main>
    </div>
  ),
};
