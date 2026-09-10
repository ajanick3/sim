import Table from "../table";

// The experimental Pokémon TCG Live-style board. Same game, same engine
// wiring as `/`; only the presentation differs.
export default function LivePage() {
  return <Table variant="live" />;
}
