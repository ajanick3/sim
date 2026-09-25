import { Suspense } from "react";
import { CodexGameShell } from "../_game/CodexGameShell";

export default function CodexPlayPage() {
  return (
    <Suspense fallback={null}>
      <CodexGameShell />
    </Suspense>
  );
}
