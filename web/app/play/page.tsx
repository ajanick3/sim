import { Suspense } from "react";
import GameShell from "../game-shell";

// The game. Reads the matchup or a saved recipe from the query string:
// `/play?a=<deck>&b=<deck>` starts a fresh game, `/play?g=<recipe>`
// reopens one. `useSearchParams` in the shell needs a Suspense boundary.
export default function PlayPage() {
  return (
    <Suspense fallback={null}>
      <GameShell />
    </Suspense>
  );
}
