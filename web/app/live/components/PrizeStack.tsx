"use client";

/** Six prize markers in a 2×3 grid, the taken ones dimmed. */
export function PrizeStack({ count }: { count: number }) {
  return (
    <div className="flex flex-col items-center">
      <div className="grid grid-cols-2 gap-0.5">
        {Array.from({ length: 6 }, (_, i) => (
          <span
            key={i}
            className={`h-6 w-4 rounded-[2px] border ${
              i < count ? "border-rose-300/60 bg-rose-400/25" : "border-white/10 bg-transparent"
            }`}
          />
        ))}
      </div>
      <span className="mt-0.5 text-[10px] text-dim">{count}</span>
    </div>
  );
}
