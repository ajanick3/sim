import type { Meta, StoryObj } from "@storybook/nextjs";
import { CardPile } from "./CardPile";
import { CRUSHING_HAMMER_ART } from "../../_fixtures/cardArt";
import theme from "../../_styles/theme.module.css";

function PileStory({ kind = "deck", count = 24 }: { kind?: "deck" | "discard"; count?: number }) {
  return kind === "deck" ? (
    <CardPile kind="deck" count={count} />
  ) : (
    <CardPile
      kind="discard"
      count={count}
      topCardName={count ? "Crushing Hammer" : undefined}
      topCardImageUrl={count ? CRUSHING_HAMMER_ART : undefined}
    />
  );
}

const meta = {
  title: "Codex/Cards/CardPile",
  component: PileStory,
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ width: 78, padding: 20 }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof PileStory>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Deck: Story = {};
export const Discard: Story = { args: { kind: "discard", count: 6 } };
export const EmptyDiscard: Story = { args: { kind: "discard", count: 0 } };
