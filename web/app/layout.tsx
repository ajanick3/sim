import type { ReactNode } from "react";
import "./globals.css";

export const metadata = {
  title: "sim — Pokémon TCG",
  description: "A pure Pokémon TCG rules engine, played in the browser.",
};

export const viewport = {
  width: "device-width",
  initialScale: 1,
  // A hotseat game is passed between two players on one phone held
  // upright; let them pinch-zoom a crowded board without locking it.
  maximumScale: 5,
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
