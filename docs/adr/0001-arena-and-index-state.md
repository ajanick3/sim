# The game state holds every object in an arena and names it by index

**Status:** Accepted — 2026-09-03

A card game's state is a graph: a Pokémon points at the cards attached to it, a zone points at the cards it holds, and an effect points at both. Rust owns a mutable graph badly through references, and the reflex answer, `Rc<RefCell<T>>`, was the live alternative. It was rejected: it moves every borrow error to run time, it teaches a habit that Rust game code does not use, and this project exists to learn Rust. So every object lives for the life of the game in one `Vec` — cards, Pokémon in play — and everything else names it by a typed index such as `CardId` or `PokemonId`. An index is `Copy`, so passing one never borrows the arena that holds the object, and a zone moves an id rather than a card.

## Consequences

A knocked-out Pokémon stays in its arena, marked, rather than being freed; the arena is history as well as state. An index carries no lifetime, so nothing stops a stale one being used — the types keep the kinds apart, and the engine keeps them valid.
