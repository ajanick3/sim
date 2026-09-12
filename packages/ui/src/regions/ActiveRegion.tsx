import { Card } from "../primitives/Card";
import { CardOverlay } from "../atoms/CardOverlay";
import { HealthBar } from "../atoms/HealthBar";
import { DamageCounter } from "../atoms/DamageCounter";
import { EnergyChip } from "../atoms/EnergyChip";
import { ToolBadge } from "../atoms/ToolBadge";
import { SpecialConditions } from "../atoms/SpecialConditions";
import { ActiveIndicator } from "../atoms/ActiveIndicator";
import { attachedParts, type PlayCard } from "./types";

/** The Active spot — one Pokémon, ringed to mark it as the Active, or
 *  an empty dashed slot. `far` renders the opponent's Active: smaller,
 *  no ring (only your own Active carries the ring — the opponent's is
 *  just as much "their Active" but this board never asks you to place
 *  onto it, so it earns no indicator). */
export function ActiveRegion({
  mon,
  far = false,
  onClick,
}: {
  mon: PlayCard | null;
  far?: boolean;
  onClick?: () => void;
}) {
  const { energies, tool } = mon ? attachedParts(mon) : { energies: [], tool: null };
  const card = (
    <Card size={far ? "activeFar" : "active"} src={mon?.src} name={mon?.name ?? "empty"} onClick={onClick}>
      {mon && (
        <CardOverlay
          top={<HealthBar hp={mon.hp} tiny={far} />}
          middle={
            <>
              {mon.damage > 0 && <DamageCounter damage={mon.damage} large={!far} />}
              {mon.conditions && <SpecialConditions conditions={mon.conditions} />}
            </>
          }
          bottomStart={tool && <ToolBadge name={tool.name} />}
          bottomEnd={energies.map((e) => (
            <EnergyChip key={e.id} kind={e.energyType!} />
          ))}
        />
      )}
    </Card>
  );
  return far ? card : <ActiveIndicator>{card}</ActiveIndicator>;
}
