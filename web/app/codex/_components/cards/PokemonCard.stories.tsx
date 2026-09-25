import type { Meta, StoryObj } from "@storybook/nextjs";
import { PokemonCard } from "./PokemonCard";
import { DREEPY_ART } from "../../_fixtures/cardArt";
import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Cards/PokemonCard",
  component: PokemonCard,
  args: { name: "Dreepy", imageUrl: DREEPY_ART, hp: 70 },
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ width: 110, padding: 20 }}>
        <Story />
      </div>
    ),
  ],
} satisfies Meta<typeof PokemonCard>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Active: Story = {};
export const Opponent: Story = { args: { opponent: true } };
export const Damaged: Story = { args: { damage: 30 } };
export const Target: Story = { args: { state: "target", onSelect: () => {} } };
