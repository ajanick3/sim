/** @type {import('next').NextConfig} */
const nextConfig = {
  // The wasm glue is loaded at runtime with a native `import()` of a file in
  // `public/pkg`, so the bundler needs no WebAssembly settings.
};

export default nextConfig;
