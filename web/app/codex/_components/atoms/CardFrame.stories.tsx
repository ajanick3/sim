import type { Meta, StoryObj } from "@storybook/nextjs";
import { CardFrame } from "./CardFrame";
import { CardArt } from "./CardArt";
import { DREEPY_ART } from "../../_fixtures/cardArt";
import type { CardState } from "./CardFrame";
import theme from "../../_styles/theme.module.css";

function FrameStory({
  state = "resting",
  disabled = false,
}: {
  state?: CardState;
  disabled?: boolean;
}) {
  return (
    <CardFrame label="Dreepy" state={state} disabled={disabled} interactive>
      <CardArt src={DREEPY_ART} />
    </CardFrame>
  );
}

const meta = {
  title: "Codex/Atoms/CardFrame",
  component: FrameStory,
  decorators: [
    (Story) => (
      <div className={theme.theme} style={{ padding: 24 }}>
        <div style={{ width: 110 }}>
          <Story />
        </div>
      </div>
    ),
  ],
} satisfies Meta<typeof FrameStory>;
export default meta;
type Story = StoryObj<typeof meta>;
export const Resting: Story = {};
export const Selected: Story = { args: { state: "selected" } };
export const Target: Story = { args: { state: "target" } };
export const Unavailable: Story = { args: { state: "unavailable" } };
export const Disabled: Story = { args: { disabled: true } };
