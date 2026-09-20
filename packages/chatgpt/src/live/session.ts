// A minimal duplicate of web/app/session.ts's selection primitives —
// only what this isolated demo needs to drive the board from taps. See
// ./wasm.ts for why this package can't import it directly.
import type { WireActionMeta } from "./view";

export type Selection = { kind: "pokemon"; id: number } | { kind: "hand"; card: number } | null;

export function movesForSelection(meta: WireActionMeta[], sel: Selection): number[] {
  if (!sel) return [];
  return meta
    .map((m, i) => ({ m, i }))
    .filter(({ m }) => (sel.kind === "pokemon" ? m.target === sel.id : m.card === sel.card))
    .map(({ i }) => i);
}

export function targetsForHandCard(meta: WireActionMeta[], card: number): Map<number, number> {
  const out = new Map<number, number>();
  meta.forEach((m, i) => {
    if (m.card === card && m.target !== null && !out.has(m.target)) out.set(m.target, i);
  });
  return out;
}

const ACTION_GROUPS: { group: string; test: RegExp }[] = [
  { group: "Attack", test: /^Attack/ },
  { group: "Attach Energy", test: /^Attach .* Energy /i },
  { group: "Attach", test: /^Attach / },
  { group: "Evolve", test: /^Evolve / },
  { group: "Bench", test: /^Bench / },
  { group: "Play", test: /^Play / },
  { group: "Retreat", test: /^Retreat/ },
  { group: "Promote", test: /^Promote / },
  { group: "Move Energy", test: /^(Move .* to|Stop moving Energy)/ },
  { group: "Heal", test: /^Heal / },
  { group: "Take cards", test: /^(Take |Stop taking)/ },
  { group: "Discard", test: /^Discard /i },
  { group: "Choose", test: /^Choose /i },
];
const FINISH_GROUP = "Finish";
const OTHER_GROUP = "Other";

export const ACTION_GROUP_ORDER = [...ACTION_GROUPS.map((g) => g.group), OTHER_GROUP, FINISH_GROUP];

export interface GroupedAction {
  index: number;
  label: string;
  copy?: number;
}

export interface ActionGroup {
  group: string;
  items: GroupedAction[];
}

function groupOf(label: string): string {
  if (label === "End turn" || label === "End your turn") return FINISH_GROUP;
  for (const { group, test } of ACTION_GROUPS) {
    if (test.test(label)) return group;
  }
  return OTHER_GROUP;
}

export function groupActions(labels: string[]): ActionGroup[] {
  const total = new Map<string, number>();
  for (const l of labels) total.set(l, (total.get(l) ?? 0) + 1);
  const seen = new Map<string, number>();

  const byGroup = new Map<string, GroupedAction[]>();
  labels.forEach((label, index) => {
    const g = groupOf(label);
    const item: GroupedAction = { index, label };
    if ((total.get(label) ?? 0) > 1) {
      const rank = seen.get(label) ?? 0;
      seen.set(label, rank + 1);
      item.copy = rank;
    }
    const bucket = byGroup.get(g) ?? [];
    bucket.push(item);
    byGroup.set(g, bucket);
  });

  return ACTION_GROUP_ORDER.filter((g) => byGroup.has(g)).map((group) => ({
    group,
    items: byGroup.get(group)!,
  }));
}

export const AUTO_ADVANCE_CAP = 100;

export interface AutoAdvanceInput {
  actionCount: number;
  playing: boolean;
  over: boolean;
  busy: boolean;
  steps: number;
  phase?: string;
}

const NEVER_AUTO_ADVANCE = new Set(["PlacingActive"]);

export function shouldAutoAdvance(i: AutoAdvanceInput): boolean {
  return (
    i.actionCount === 1 &&
    i.playing &&
    !i.over &&
    !i.busy &&
    i.steps < AUTO_ADVANCE_CAP &&
    !(i.phase != null && NEVER_AUTO_ADVANCE.has(i.phase))
  );
}
