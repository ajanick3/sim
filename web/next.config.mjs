/** @type {import('next').NextConfig} */
const nextConfig = {
  // The wasm glue is loaded at runtime with a native `import()` of a file in
  // `public/pkg`, so the bundler needs no WebAssembly settings.

  // The board used to live at `/live` while it was the experimental variant.
  // A config redirect forwards the `?g=` recipe; a redirecting page would
  // drop it, so Back / Forward could not step through a shared game.
  async redirects() {
    return [
      { source: "/live", destination: "/", permanent: true },
      // Storybook is built into `public/storybook` by `prebuild`. Its
      // index uses relative asset paths, so land on the file itself.
      { source: "/storybook", destination: "/storybook/index.html", permanent: false },
      // packages/ui's own, separate Storybook — a different component
      // library (MUI, not this app's Tailwind), built into `public/sb`
      // by the same `prebuild` step. Short name is deliberate — this
      // one's typed often enough to earn it.
      { source: "/sb", destination: "/sb/index.html", permanent: false },
      // packages/chatgpt's own Storybook — the ChatGPT-reviewed board
      // layouts, on React 18 like packages/ui, built into
      // `public/chatgpt` by the same `prebuild` step.
      { source: "/chatgpt", destination: "/chatgpt/index.html", permanent: false },
    ];
  },
};

export default nextConfig;
