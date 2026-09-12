import type { Meta, StoryObj } from "@storybook/react";
import Box from "@mui/material/Box";
import Stack from "@mui/material/Stack";
import { ActiveRegion } from "./regions/ActiveRegion";
import { BenchRegion } from "./regions/BenchRegion";
import { HandRegion } from "./regions/HandRegion";
import { PrizesRegion } from "./regions/PrizesRegion";
import { DeckRegion } from "./regions/DeckRegion";
import { DiscardRegion } from "./regions/DiscardRegion";
import { StadiumRegion } from "./regions/StadiumRegion";
import { ActionBar } from "./regions/ActionBar";
import type { PlayCard, PocketCard } from "./regions/types";
import pikachuEx from "./assets/pikachu-ex.png";

const meta = { title: "board/FullGame" } satisfies Meta;
export default meta;
type Story = StoryObj<typeof meta>;

const mon = (id: number, name: string, hp: number, damage: number): PlayCard => ({
  id,
  name,
  src: pikachuEx,
  hp,
  damage,
});

const yourActive: PlayCard = {
  ...mon(1, "Pikachu ex", 200, 60),
  attached: [{ id: 201, name: "Lightning Energy", energyType: "Lightning", category: "special-energy" }],
};
const oppActive: PlayCard = {
  ...mon(2, "Pikachu ex", 200, 30),
  conditions: ["Poisoned"],
};
const yourBench: (PlayCard | null)[] = [mon(3, "Pikachu ex", 200, 0), mon(4, "Pikachu ex", 200, 0), null, null, null];
const oppBench: (PlayCard | null)[] = [mon(5, "Pikachu ex", 200, 0), null, null, null, null];
const yourHand: PocketCard[] = [
  { id: 10, name: "Pikachu ex", src: pikachuEx, category: "pokemon" },
  { id: 11, name: "Pikachu ex", src: pikachuEx, category: "pokemon" },
  { id: 12, name: "Pikachu ex", src: pikachuEx, category: "pokemon" },
  { id: 13, name: "Pikachu ex", src: pikachuEx, category: "pokemon" },
];
const discard: PocketCard[] = [{ id: 20, name: "Pikachu ex", src: pikachuEx }];

/** Every Region assembled into the shape a real game takes — the felt
 *  court from the reference layout, both sides mirrored around a
 *  curved centre divider, the Stadium and the reimagined icon-only
 *  ActionBar sharing the lane between the two Actives. A composite,
 *  not yet its own component: proves the Regions actually fit
 *  together before a real `Board` gets extracted from this shape. */
export const FullGame: Story = {
  render: () => (
    <Box
      sx={{
        maxWidth: 420,
        mx: "auto",
        p: 2,
        borderRadius: 6,
        bgcolor: "#173d2b",
        border: "1px solid",
        borderColor: "rgba(255,255,255,0.12)",
      }}
    >
      {/* Opponent — reversed reading order, farther away, smaller. */}
      <Stack direction="row" spacing={1.5} alignItems="flex-start" justifyContent="space-between">
        <Stack direction="row" spacing={1}>
          <DeckRegion count={46} />
          <DiscardRegion cards={[]} />
        </Stack>
        <BenchRegion mons={oppBench} far />
        <PrizesRegion remaining={5} />
      </Stack>
      <Box sx={{ display: "flex", justifyContent: "center", my: 1 }}>
        <HandRegion count={6} />
      </Box>

      {/* The curved divider the reference layout draws between the two
          halves of the court. */}
      <Box
        sx={{
          height: 2,
          my: 1.5,
          borderRadius: 1,
          background: "linear-gradient(90deg, transparent, rgba(255,255,255,0.35), transparent)",
        }}
      />

      {/* Centre lane: Stadium, both Actives stacked, the ActionBar. */}
      <Stack direction="row" spacing={2} alignItems="center" justifyContent="center">
        <StadiumRegion card={{ id: 30, name: "Pikachu ex", src: pikachuEx }} />
        <Stack alignItems="center" spacing={1}>
          <ActiveRegion mon={oppActive} far />
          <ActiveRegion mon={yourActive} />
        </Stack>
        <ActionBar
          actions={[
            { id: 1, kind: "attack", label: "Thunderbolt" },
            { id: 2, kind: "retreat", label: "Retreat" },
            { id: 3, kind: "attach", label: "Attach Lightning Energy" },
          ]}
          onAct={() => {}}
        />
      </Stack>

      <Box
        sx={{
          height: 2,
          my: 1.5,
          borderRadius: 1,
          background: "linear-gradient(90deg, transparent, rgba(255,255,255,0.35), transparent)",
        }}
      />

      {/* You — closest, largest, reads first. */}
      <Stack direction="row" spacing={1.5} alignItems="flex-start" justifyContent="space-between">
        <PrizesRegion remaining={4} />
        <BenchRegion mons={yourBench} />
        <Stack direction="row" spacing={1}>
          <DeckRegion count={44} />
          <DiscardRegion cards={discard} />
        </Stack>
      </Stack>
      <Box sx={{ display: "flex", justifyContent: "center", mt: 1.5 }}>
        <HandRegion cards={yourHand} />
      </Box>
    </Box>
  ),
};
