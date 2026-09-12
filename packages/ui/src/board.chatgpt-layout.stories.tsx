import type { Meta, StoryObj } from "@storybook/react";
import Box from "@mui/material/Box";
import { Card } from "./primitives/Card";
import { HealthBar } from "./atoms/HealthBar";
import { DamageCounter } from "./atoms/DamageCounter";
import { PokeBall } from "./atoms/PokeBall";
import { DeckRegion } from "./regions/DeckRegion";
import { DiscardRegion } from "./regions/DiscardRegion";
import { StadiumRegion } from "./regions/StadiumRegion";
import pikachuEx from "./assets/pikachu-ex.png";

const meta = { title: "board/ChatGPTLayout" } satisfies Meta;
export default meta;
type Story = StoryObj<typeof meta>;

const cardSlot = (i: number, opts?: { hp?: number; damage?: number }) => (
  <Card key={i} size="bench" fluid name="Pikachu ex" src={pikachuEx}>
    {opts?.hp && <HealthBar hp={opts.hp} tiny />}
    {opts?.damage ? <DamageCounter damage={opts.damage} top="40%" left="50%" /> : null}
  </Card>
);

const fiveCardRow = (keyPrefix: string) => (
  <Box sx={{ display: "grid", gridTemplateColumns: "repeat(5, minmax(0, 1fr))", gap: 1 }}>
    {Array.from({ length: 5 }, (_, i) => (
      <Box key={`${keyPrefix}-${i}`}>{cardSlot(i)}</Box>
    ))}
  </Box>
);

const prizeGrid = (remaining: number) => (
  <Box sx={{ display: "flex", alignItems: "center" }}>
    <Box sx={{ display: "grid", gridTemplateColumns: "repeat(2, 1fr)", gap: 0.5 }}>
      {Array.from({ length: 6 }, (_, i) => (
        <PokeBall key={i} size={12} taken={i >= remaining} />
      ))}
    </Box>
  </Box>
);

/**
 * A mockup, not a component to build on: the layout the operator
 * pasted (a GameBoard.tsx/CSS pair) rendered against this library's
 * actual Regions, to check the idea against real content instead of
 * empty `<div className="cardSlot">`s.
 *
 * The pasted CSS makes three structural calls this library's existing
 * ActiveRegion/BenchRegion don't:
 *
 * 1. **Perspective lives on the viewport, once** — `perspective` and a
 *    single `rotateX(4deg)` on the whole board, not a per-card tilt.
 *    The "looking across a table" read comes from tilting the table,
 *    not each card on it.
 * 2. **No near/far size difference.** "Do NOT scale opponent cards
 *    separately" is explicit in the source — every card, either side,
 *    is the same `aspect-ratio: 5/7` box at the same width. This
 *    mockup honours that and does NOT use `ActiveRegion`/`BenchRegion`,
 *    since both bake near/far scaling in; it builds each slot from the
 *    bare `Card` primitive instead, uniformly sized.
 * 3. **Golden-ratio vertical split** — the opponent's half and the
 *    player's half aren't equal; the player's is ~1.618× taller, and
 *    each Active is pulled to sit right against the centre line
 *    rather than centred in its own half.
 *
 * Prizes, Deck, Discard, and the Stadium aren't in the pasted code at
 * all (it only covers Bench/Active/Hand) — added here, in the same
 * "one grid, named areas" spirit, to answer what the rest of a real
 * board's Regions want from this layout.
 */
function ChatGPTLayoutBoard() {
  return (
    <Box
      sx={{
        // The viewport: perspective lives here, once — not per card.
        width: 380,
        height: 620,
        mx: "auto",
        overflow: "hidden",
        perspective: "1200px",
        perspectiveOrigin: "50% 100%",
        bgcolor: "#221f22",
        borderRadius: 3,
      }}
    >
      <Box
        sx={{
          width: "100%",
          height: "100%",
          display: "grid",
          gridTemplateAreas: `"opponent" "center" "player"`,
          // Golden ratio: opponent 1, player 1.618.
          gridTemplateRows: "1fr 1px 1.618fr",
          transform: "rotateX(4deg)",
          transformOrigin: "50% 100%",
          bgcolor: "#2d2a2e",
          p: 1,
        }}
      >
        {/* Opponent half */}
        <Box
          sx={{
            gridArea: "opponent",
            minHeight: 0,
            display: "grid",
            gridTemplateAreas: `"opp-aside opp-bench opp-prizes" "opp-aside opp-active opp-prizes"`,
            gridTemplateColumns: "auto 1fr auto",
            gridTemplateRows: "auto minmax(0, 1fr)",
            columnGap: 1,
          }}
        >
          <Box sx={{ gridArea: "opp-aside", display: "flex", flexDirection: "column", gap: 0.5 }}>
            <DeckRegion count={46} />
            <DiscardRegion cards={[]} />
          </Box>
          <Box sx={{ gridArea: "opp-bench", py: 0.5 }}>{fiveCardRow("opp-bench")}</Box>
          <Box sx={{ gridArea: "opp-active", display: "grid", placeItems: "end center", pb: 1.5 }}>
            <Box sx={{ width: "20%", minWidth: 60 }}>{cardSlot(0, { hp: 200, damage: 30 })}</Box>
          </Box>
          <Box sx={{ gridArea: "opp-prizes" }}>{prizeGrid(5)}</Box>
        </Box>

        {/* Centre line */}
        <Box sx={{ gridArea: "center", bgcolor: "rgba(252,252,250,0.25)" }} />

        {/* Player half */}
        <Box
          sx={{
            gridArea: "player",
            minHeight: 0,
            display: "grid",
            gridTemplateAreas: `
              "you-prizes you-active you-aside"
              "you-prizes you-bench  you-aside"
              "you-hand   you-hand   you-hand"
            `,
            gridTemplateColumns: "auto 1fr auto",
            gridTemplateRows: "minmax(0, 1fr) auto auto",
            columnGap: 1,
          }}
        >
          <Box sx={{ gridArea: "you-prizes" }}>{prizeGrid(4)}</Box>
          <Box sx={{ gridArea: "you-active", display: "grid", placeItems: "start center", pt: 1.5 }}>
            <Box sx={{ width: "20%", minWidth: 60 }}>{cardSlot(1, { hp: 200, damage: 60 })}</Box>
          </Box>
          <Box sx={{ gridArea: "you-aside", display: "flex", flexDirection: "column", gap: 0.5 }}>
            <DeckRegion count={44} />
            <DiscardRegion cards={[{ id: 1, name: "Pikachu ex", src: pikachuEx }]} />
          </Box>
          <Box sx={{ gridArea: "you-bench", py: 0.5 }}>{fiveCardRow("you-bench")}</Box>
          <Box sx={{ gridArea: "you-hand", display: "flex", flexDirection: "column", gap: 1, py: 0.5 }}>
            {/* 10 cards, five columns — two rows without any extra logic. */}
            {fiveCardRow("hand-1")}
            {fiveCardRow("hand-2")}
          </Box>
        </Box>
      </Box>
    </Box>
  );
}

export const ChatGPTLayout: Story = { render: () => <ChatGPTLayoutBoard /> };

/** The Stadium, on the centre line itself — the one Region the pasted
 *  layout has no opinion on at all, so this is purely this library's
 *  own call: it belongs neither to the opponent's half nor the
 *  player's, so it sits on the line that separates them. */
export const ChatGPTLayoutWithStadium: Story = {
  render: () => (
    <Box sx={{ position: "relative", width: 380, mx: "auto" }}>
      <ChatGPTLayoutBoard />
      <Box sx={{ position: "absolute", top: "38%", left: "50%", transform: "translate(-50%, -50%)" }}>
        <StadiumRegion card={{ id: 30, name: "Pikachu ex", src: pikachuEx }} />
      </Box>
    </Box>
  ),
};
