// `next-env.d.ts` supplies these same declarations, but Next.js generates
// it at build time and the repository does not commit it. The `web` CI job
// runs `typecheck` before `build`, so this tree's `.module.css` and image
// imports need their own committed declarations to resolve.

declare module "*.module.css" {
  const classes: { readonly [key: string]: string };
  export default classes;
}

declare module "*.png" {
  const content: { src: string; height: number; width: number };
  export default content;
}
