import type { Meta, StoryObj } from "@storybook/nextjs";
import {
  basicEnergyKinds,
  EnergyIcon,
  energyDefinitions,
  specialEnergyKinds,
  type RegulationMark,
} from "./EnergyIcon";

const meta = {
  title: "board/EnergyIcon",
  component: EnergyIcon,
  args: { kind: "fire", size: 48 },
} satisfies Meta<typeof EnergyIcon>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Single: Story = {};

export const Basics: Story = {
  render: () => (
    <div className="flex flex-wrap gap-3">
      {basicEnergyKinds.map((k) => (
        <div key={k} className="flex w-16 flex-col items-center gap-1 text-[10px] text-dim">
          <EnergyIcon kind={k} size={44} />
          {k}
        </div>
      ))}
    </div>
  ),
};

export const Specials: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      {(["H", "I", "J"] as RegulationMark[]).map((mark) => (
        <div key={mark}>
          <div className="mb-1 text-[11px] uppercase tracking-widest text-dim">
            Regulation {mark}
          </div>
          <div className="flex flex-wrap gap-3">
            {specialEnergyKinds
              .filter((k) => energyDefinitions[k].mark === mark)
              .map((k) => (
                <div key={k} className="flex w-20 flex-col items-center gap-1 text-[10px] text-dim">
                  <EnergyIcon kind={k} size={44} />
                  {energyDefinitions[k].label.replace(" Energy", "")}
                </div>
              ))}
          </div>
        </div>
      ))}
    </div>
  ),
};

export const Sizes: Story = {
  render: () => (
    <div className="flex items-end gap-3">
      {[12, 16, 24, 40, 64].map((s) => (
        <EnergyIcon key={s} kind="water" size={s} />
      ))}
    </div>
  ),
};

export const Flat: Story = { args: { raised: false, size: 44 } };

export const OnACard: Story = {
  render: () => (
    <div className="relative h-[118px] w-[200px] overflow-hidden rounded-card bg-gradient-to-br from-emerald-700 to-emerald-900">
      <span className="absolute inset-x-0 bottom-1 flex justify-center gap-[3px]">
        {(["fire", "fire", "psychic"] as const).map((k, i) => (
          <EnergyIcon key={i} kind={k} size={11} decorative />
        ))}
      </span>
    </div>
  ),
};
