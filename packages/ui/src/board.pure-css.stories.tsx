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
 * The same board `board.chatgpt-layout.stories.tsx` mocks up, rebuilt
 * with the layout itself as a real `.css` file (`board.pure-css.css`)
 * and plain `<div className>` elements for every grid area — no MUI
 * `Box`, no `sx` prop anywhere in the scaffold. The Card/atom content
 * inside each slot is unchanged (those are this library's own
 * components, built on MUI internally) — only the board's own
 * layout stops being CSS-in-JS.
 *
 * Same three structural calls as the ChatGPT-layout mockup:
 * perspective on the viewport once (not per card), no near/far size
 * difference between the two sides, and a golden-ratio vertical split
 * with each Active pulled to the centre line. Prizes/Deck/Discard
 * again added in the same "one grid, named areas" spirit — the
 * pasted code only covered Bench/Active/Hand.
 */
export const PureCSS: Story = {
  render: () => (
    <div className="boardViewport">
      <main className="board">
        <section className="opponent">
          <div className="opponentAside">
            <DeckRegion count={46} />
            <DiscardRegion cards={[]} />
          </div>
          <div className="opponentBench">
            <FiveCardRow keyPrefix="opp-bench" />
          </div>
          <div className="opponentActive">
            <CardSlot hp={200} damage={30} />
          </div>
          <div className="opponentPrizes">
            <PrizeGrid remaining={5} />
          </div>
        </section>

        <div className="centerLine" />

        <section className="player">
          <div className="playerPrizes">
            <PrizeGrid remaining={4} />
          </div>
          <div className="playerActive">
            <CardSlot hp={200} damage={60} />
          </div>
          <div className="playerAside">
            <DeckRegion count={44} />
            <DiscardRegion cards={[{ id: 1, name: "Pikachu ex", src: pikachuEx }]} />
          </div>
          <div className="playerBench">
            <FiveCardRow keyPrefix="you-bench" />
          </div>
          <div className="playerHand">
            {/* 10 cards, five columns — two rows without any extra logic. */}
            <div className="playerHandRow">
              <FiveCardRow keyPrefix="hand-1" />
            </div>
            <div className="playerHandRow">
              <FiveCardRow keyPrefix="hand-2" />
            </div>
          </div>
        </section>
      </main>
    </div>
  ),
};
