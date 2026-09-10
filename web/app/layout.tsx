import type { ReactNode } from "react";
import "./globals.css";

export const metadata = {
  title: "sim — Pokémon TCG",
  description: "A pure Pokémon TCG rules engine, played in the browser.",
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
