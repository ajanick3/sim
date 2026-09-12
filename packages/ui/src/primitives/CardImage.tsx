import Box from "@mui/material/Box";

/** A card's illustration, filling its parent. The one place an `<img>`
 *  tag exists in the library — every other primitive composes this
 *  rather than drawing its own. Renders nothing when there's no art;
 *  the parent `Card` shows its plain-name fallback instead. */
export function CardImage({
  src,
  alt,
  crop = "full",
}: {
  src: string | null;
  alt: string;
  /** "top" shows the name bar and head of the illustration — the
   *  sliver an Active has always shown; "full" the whole card. */
  crop?: "top" | "full";
}) {
  if (!src) return null;
  return (
    <Box
      component="img"
      src={src}
      alt={alt}
      loading="lazy"
      sx={{
        position: "absolute",
        inset: 0,
        width: "100%",
        height: "100%",
        objectFit: "cover",
        objectPosition: crop === "top" ? "top" : "center",
        borderRadius: "inherit",
      }}
    />
  );
}
