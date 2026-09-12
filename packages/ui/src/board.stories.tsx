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
const oppActive: PlayCard = { ...mon(2, "Pikachu ex", 200, 30), conditions: ["Poisoned"] };
const yourBench: (PlayCard | null)[] = [mon(3, "Pikachu ex", 200, 0), mon(4, "Pikachu ex", 200, 0), null, null, null];
const oppBench: (PlayCard | null)[] = [mon(5, "Pikachu ex", 200, 0), null, null, null, null];
const yourHand: PocketCard[] = [
  { id: 10, name: "Pikachu ex", src: pikachuEx, category: "pokemon" },
  { id: 11, name: "Pikachu ex", src: pikachuEx, category: "pokemon" },
  { id: 12, name: "Pikachu ex", src: pikachuEx, category: "pokemon" },
  { id: 13, name: "Pikachu ex", src: pikachuEx, category: "pokemon" },
];
const discard: PocketCard[] = [{ id: 20, name: "Pikachu ex", src: pikachuEx }];

const divider = (area: string) => (
  <Box
    sx={{
      gridArea: area,
      height: 1,
      my: 0.5,
      borderRadius: 1,
      background: "linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent)",
    }}
  />
);

/** Every Region assembled into the shape a real game takes, laid out
 *  as a real CSS Grid — named areas, not nested flex Stacks guessing
 *  at each other's widths. The first attempt at this used flexbox
 *  throughout; Bench's own hardcoded width didn't fit the row it sat
 *  in, and the whole composite collapsed into an unreadable column.
 *  A fixed 380px court (a phone's rough width), so the grid has to
 *  actually fit, not just fill whatever space it's given. Both sides
 *  mirror around a curved divider; the Stadium and the reimagined
 *  icon-only ActionBar share the lane with the two overlapping
 *  Actives. A composite story, not yet its own component. */
export const FullGame: Story = {
  render: () => (
    <Box
      sx={{
        width: 380,
        mx: "auto",
        p: 1.5,
        borderRadius: 5,
        bgcolor: "#173d2b",
        border: "1px solid rgba(255,255,255,0.12)",
        display: "grid",
        gridTemplateAreas: `
          "opp-deck   opp-bench  opp-bench  opp-prizes"
          "divider1   divider1   divider1   divider1"
          "stadium    actives    actives    action-bar"
          "divider2   divider2   divider2   divider2"
          "you-prizes you-bench  you-bench  you-deck"
          "hand       hand       hand       hand"
        `,
        gridTemplateColumns: "auto 1fr 1fr auto",
        gridTemplateRows: "auto auto auto auto auto auto",
        rowGap: 8,
        columnGap: 8,
        alignItems: "center",
        justifyItems: "center",
      }}
    >
      <Stack sx={{ gridArea: "opp-deck" }} direction="row" spacing={0.5}>
        <DeckRegion count={46} />
        <DiscardRegion cards={[]} />
      </Stack>
      <Box sx={{ gridArea: "opp-bench" }}>
        <BenchRegion mons={oppBench} far maxWidth={180} />
      </Box>
      <Box sx={{ gridArea: "opp-prizes" }}>
        <PrizesRegion remaining={5} />
      </Box>

      {divider("divider1")}

      <Box sx={{ gridArea: "stadium" }}>
        <StadiumRegion card={{ id: 30, name: "Pikachu ex", src: pikachuEx }} />
      </Box>
      {/* The two Actives overlap by a negative margin on the second
          one, not by absolute coordinates on either. */}
      <Box sx={{ gridArea: "actives", width: 150, display: "flex", flexDirection: "column" }}>
        <Box sx={{ ml: "19px" }}>
          <ActiveRegion mon={oppActive} far />
        </Box>
        <Box sx={{ mt: "-40px" }}>
          <ActiveRegion mon={yourActive} />
        </Box>
      </Box>
      <Box sx={{ gridArea: "action-bar" }}>
        <ActionBar
          actions={[
            { id: 1, kind: "attack", label: "Thunderbolt" },
            { id: 2, kind: "retreat", label: "Retreat" },
            { id: 3, kind: "attach", label: "Attach Lightning Energy" },
          ]}
          onAct={() => {}}
        />
      </Box>

      {divider("divider2")}

      <Box sx={{ gridArea: "you-prizes" }}>
        <PrizesRegion remaining={4} />
      </Box>
      <Box sx={{ gridArea: "you-bench" }}>
        <BenchRegion mons={yourBench} maxWidth={220} />
      </Box>
      <Stack sx={{ gridArea: "you-deck" }} direction="row" spacing={0.5}>
        <DeckRegion count={44} />
        <DiscardRegion cards={discard} />
      </Stack>

      <Box sx={{ gridArea: "hand" }}>
        <HandRegion cards={yourHand} />
      </Box>
    </Box>
  ),
};
