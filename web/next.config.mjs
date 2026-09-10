/** @type {import('next').NextConfig} */
const nextConfig = {
  // The wasm glue is loaded at runtime with a native `import()` of a file in
  // `public/pkg`, so the bundler needs no WebAssembly settings.

  // The board used to live at `/live` while it was the experimental variant.
  // A config redirect forwards the `?g=` recipe; a redirecting page would
  // drop it, so Back / Forward could not step through a shared game.
  async redirects() {
    return [{ source: "/live", destination: "/", permanent: true }];
  },
};

export default nextConfig;
