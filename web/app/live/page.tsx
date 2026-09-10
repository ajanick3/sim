import { redirect } from "next/navigation";

// The board used to live here while it was the experimental variant.
// It is the only board now, at `/`.
export default function LivePage() {
  redirect("/");
}
