import { Card } from "../primitives/Card";
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
        <>
          <HealthBar hp={mon.hp} tiny={far} />
          {mon.damage > 0 && <DamageCounter damage={mon.damage} top="35%" left="60%" large={!far} />}
          {tool && <ToolBadge name={tool.name} />}
          {energies.length > 0 && (
            <div style={{ position: "absolute", bottom: 2, right: 2, display: "flex", gap: 2 }}>
              {energies.map((e) => (
                <EnergyChip key={e.id} kind={e.energyType!} />
              ))}
            </div>
          )}
          {mon.conditions && <SpecialConditions conditions={mon.conditions} />}
        </>
      )}
    </Card>
  );
  return far ? card : <ActiveIndicator>{card}</ActiveIndicator>;
}
