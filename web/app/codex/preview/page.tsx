import Link from "next/link";

const ROUTES = [
  { href: "/codex/preview/atoms", label: "Atoms", blurb: "Base card parts and controls." },
  {
    href: "/codex/preview/components",
    label: "Components",
    blurb: "Pokémon cards, hand cards, piles, prizes, damage, targets, and a two-row hand.",
  },
  {
    href: "/codex/preview/board",
    label: "Board",
    blurb: "The full board at mobile, tablet, and desktop widths.",
  },
  {
    href: "/codex/preview/deck-search",
    label: "Deck search",
    blurb: "The responsive deck-search sheet.",
  },
  {
    href: "/codex/preview/actions",
    label: "Actions",
    blurb: "The compact legal-action dialog.",
  },
];

export default function PreviewIndex() {
  return (
    <main className="mx-auto max-w-[720px] px-4 py-8">
      <header className="flex items-baseline justify-between">
        <h1 className="m-0 text-[20px]">codex — preview</h1>
        <Link href="/codex" className="text-[13px] text-dim no-underline hover:text-text">
          Play
        </Link>
      </header>
      <p className="mt-1 text-[13px] text-dim">
        Each route renders the Codex UI's components against fixture data. None loads the game
        engine.
      </p>

      <ul className="mt-6 flex flex-col gap-1">
        {ROUTES.map((r) => (
          <li key={r.href}>
            <Link
              href={r.href}
              className="flex flex-col gap-0.5 rounded-md border border-edge px-3 py-2 no-underline hover:border-accent"
            >
              <span className="text-[13px]">{r.label}</span>
              <span className="text-[12px] text-dim">{r.blurb}</span>
            </Link>
          </li>
        ))}
      </ul>
    </main>
  );
}
