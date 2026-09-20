import { defineConfig } from "vite";

// The isolated live-demo app (src/live/) — a plain Vite build, output
// into web/'s public dir by web/scripts/build-chatgpt-live.sh, same as
// the Storybook build already is. Kept separate from vitest.config.ts,
// which only ever needed jsdom + the test setup file.
export default defineConfig({
  // Relative asset paths — this build lands at web/'s /chatgpt-live/,
  // not the domain root, and Vite's default "/" base would send the
  // browser looking for its JS at the wrong path (matching how
  // Storybook's own build already does this by default).
  base: "./",
  build: {
    outDir: "dist",
  },
});
