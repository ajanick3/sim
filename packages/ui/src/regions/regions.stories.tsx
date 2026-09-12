import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import Stack from "@mui/material/Stack";
import Divider from "@mui/material/Divider";
import Typography from "@mui/material/Typography";
import { ActiveRegion } from "./ActiveRegion";
import { BenchRegion } from "./BenchRegion";
import { HandRegion } from "./HandRegion";
import { PrizesRegion } from "./PrizesRegion";
import { DeckRegion } from "./DeckRegion";
import { DiscardRegion } from "./DiscardRegion";
import { DiscardViewDialog } from "./DiscardViewDialog";
import { StadiumRegion } from "./StadiumRegion";
import { SearchRegion } from "./SearchRegion";
import type { PlayCard, PocketCard } from "./types";

const meta = { title: "regions/overview" } satisfies Meta;
export default meta;
type Story = StoryObj<typeof meta>;

const munkidori: PlayCard = {
  id: 1,
  name: "Munkidori",
  hp: 110,
  damage: 30,
  attached: [{ id: 101, name: "Psychic Energy", energyType: "Psychic", category: "special-energy" }],
};
const dragapult: PlayCard = {
  id: 2,
  name: "Dragapult ex",
  hp: 330,
  damage: 90,
  attached: [{ id: 102, name: "Rescue Board", category: "tool" }],
};
const bench: (PlayCard | null)[] = [
  { id: 3, name: "Dreepy", hp: 70, damage: 0 },
  { id: 4, name: "Toxel", hp: 70, damage: 0 },
  null,
  null,
  null,
];
const hand: PocketCard[] = [
  { id: 10, name: "Boss's Orders" },
  { id: 11, name: "Ultra Ball" },
  { id: 12, name: "Rare Candy" },
  { id: 13, name: "Night Stretcher" },
  { id: 14, name: "Basic Fire Energy" },
];
const discardCards: PocketCard[] = [{ id: 20, name: "Poke Pad" }];

export const Active: Story = { render: () => <ActiveRegion mon={munkidori} /> };
export const ActiveFarSide: Story = { render: () => <ActiveRegion mon={dragapult} far /> };
export const EmptyActive: Story = { render: () => <ActiveRegion mon={null} /> };
export const Bench5: Story = { render: () => <BenchRegion mons={bench} /> };
export const Bench6Squishes: Story = {
  render: () => (
    <BenchRegion mons={[...bench.filter(Boolean), { id: 5, name: "Bidoof", hp: 60, damage: 0 }] as PlayCard[]} />
  ),
};
export const Hand5: Story = { render: () => <HandRegion cards={hand} /> };
export const OpponentHandCount: Story = { render: () => <HandRegion count={7} /> };
export const Prizes: Story = { render: () => <PrizesRegion remaining={4} /> };
export const Deck: Story = { render: () => <DeckRegion count={46} /> };
export const DiscardEmpty: Story = { render: () => <DiscardRegion cards={[]} /> };
export const DiscardWithCards: Story = { render: () => <DiscardRegion cards={discardCards} /> };
export const StadiumEmpty: Story = { render: () => <StadiumRegion card={null} /> };
export const StadiumFilled: Story = {
  render: () => <StadiumRegion card={{ id: 30, name: "Risky Ruins" }} />,
};
export const Search: Story = {
  render: () => (
    <SearchRegion cards={hand} takeable={new Set([10, 12])} onTake={() => {}} />
  ),
};

/** The Discard dialog, openable — proves the real interactive piece,
 *  not just a static picture. */
export const DiscardDialog: Story = {
  render: function Render() {
    const [open, setOpen] = useState(false);
    return (
      <>
        <DiscardRegion cards={discardCards} onOpen={() => setOpen(true)} />
        <DiscardViewDialog open={open} label="You — discard" cards={discardCards} onClose={() => setOpen(false)} />
      </>
    );
  },
};

/** One full side, top to bottom — proves the Regions compose into the
 *  shape a real player's half of the board takes. */
export const OneSide: Story = {
  render: () => (
    <Stack spacing={1.5}>
      <Typography variant="overline" color="text.secondary">
        You
      </Typography>
      <Stack direction="row" spacing={2} alignItems="flex-start">
        <PrizesRegion remaining={4} />
        <BenchRegion mons={bench} />
        <Stack direction="row" spacing={1}>
          <DeckRegion count={46} />
          <DiscardRegion cards={discardCards} />
        </Stack>
      </Stack>
      <Divider flexItem />
      <ActiveRegion mon={dragapult} />
      <Divider flexItem />
      <HandRegion cards={hand} />
    </Stack>
  ),
};
