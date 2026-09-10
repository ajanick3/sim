"use client";

import type { Prompt } from "./shared";

/** A yes / no bar for a "may" Ability or the setup bonus draw: the
 *  positive choices as accent buttons, the decline as a quiet one. */
export function PromptBar({
  prompt,
  busy,
  onAct,
}: {
  prompt: Prompt;
  busy: boolean;
  onAct: (index: number) => void;
}) {
  return (
    <div className="mt-1 flex flex-wrap items-center justify-center gap-2 rounded-lg border border-accent/60 bg-accent/10 p-2">
      <span className="mr-1 font-bold">{prompt.verb}</span>
      {prompt.accepts.map(({ label, index }) => (
        <button
          key={index}
          type="button"
          disabled={busy}
          onClick={() => onAct(index)}
          className="rounded-md border-accent bg-accent px-4 py-1.5 font-bold text-black disabled:opacity-50"
        >
          {label}
        </button>
      ))}
      <button
        type="button"
        disabled={busy}
        onClick={() => onAct(prompt.decline)}
        className="rounded-md px-4 py-1.5 text-dim disabled:opacity-50"
      >
        {prompt.declineLabel ?? "Decline"}
      </button>
    </div>
  );
}
