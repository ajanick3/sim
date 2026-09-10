"use client";

import { useEffect } from "react";

/** Registers `/sw.js` once the page has loaded. A tiny client island so
 *  the (server) root layout can stay a plain component. */
export function RegisterServiceWorker() {
  useEffect(() => {
    if (!("serviceWorker" in navigator)) return;
    window.addEventListener("load", () => {
      navigator.serviceWorker.register("/sw.js").catch(() => {
        // Offline install is a bonus, not a requirement — a failed
        // registration (an unsupported browser, a blocked script) just
        // leaves the app working online, same as before this existed.
      });
    });
  }, []);
  return null;
}
