import type { ReactNode } from "react";
import "./globals.css";
import { RegisterServiceWorker } from "./RegisterServiceWorker";

export const metadata = {
  title: "sim — Pokémon TCG",
  description: "A pure Pokémon TCG rules engine, played in the browser.",
  manifest: "/manifest.webmanifest",
  icons: {
    icon: [{ url: "/favicon-32.png", sizes: "32x32", type: "image/png" }],
    apple: [{ url: "/apple-touch-icon.png", sizes: "180x180", type: "image/png" }],
  },
  appleWebApp: {
    // Standalone home-screen launch on iOS; the engine and every asset it
    // needs are local, so this reads the same offline as it does online.
    capable: true,
    statusBarStyle: "black-translucent",
    title: "sim",
  },
};

export const viewport = {
  width: "device-width",
  initialScale: 1,
  // A hotseat game is passed between two players on one phone held
  // upright; let them pinch-zoom a crowded board without locking it.
  maximumScale: 5,
  themeColor: "#17251c",
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>
        {children}
        <RegisterServiceWorker />
      </body>
    </html>
  );
}
