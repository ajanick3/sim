import type { Meta, StoryObj } from "@storybook/nextjs";
import { CardArt } from "./CardArt";
import { CardFrame } from "./CardFrame";
import { DREEPY_ART, CRUSHING_HAMMER_ART } from "../../_fixtures/cardArt";
import theme from "../../_styles/theme.module.css";

const meta = {
  title: "Codex/Atoms/CardArt",
  component: CardArt,
  args: { src: DREEPY_ART },

  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <CardFrame label="Card preview">
            <Story />
          </CardFrame>
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof CardArt>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Pokemon: Story = {};
export const Trainer: Story = { args: { src: CRUSHING_HAMMER_ART } };
export const Missing: Story = { args: { src: null } };
export const Failed: Story = { args: { src: "/invalid-card-image.png" } };
