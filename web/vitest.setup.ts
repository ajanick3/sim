import "@testing-library/jest-dom/vitest";

// jsdom implements neither of these; the app calls them for cosmetics only.
if (!Element.prototype.scrollTo) {
  Element.prototype.scrollTo = () => {};
}
