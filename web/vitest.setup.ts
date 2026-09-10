import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

// jsdom implements neither of these; the app calls them for cosmetics only.
if (!Element.prototype.scrollTo) {
  Element.prototype.scrollTo = () => {};
}

// Unmount rendered trees between tests (no `globals: true`, so RTL's own
// auto-cleanup hook is not registered).
afterEach(cleanup);
