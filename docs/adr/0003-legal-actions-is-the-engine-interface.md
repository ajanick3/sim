# `legal_actions(state)` is the engine's primary interface

**Status:** Accepted — 2026-09-03

A rules engine can offer either a wide command surface that validates each command as it arrives, or one function that lists what a player may do now. The second was chosen. `legal_actions(state) -> Vec<Action>` makes a bot `(state, legal_actions) -> Action` and makes the text interface the same function with a human behind it, so neither can reach a state the other cannot and neither can act outside the list. `apply` refuses anything the list does not hold.

A mid-effect choice — which Energy to discard, which Pokémon to promote — is an ordinary game state with its own legal actions, not a coroutine that suspends. Generators were the live alternative, and the design that a surveyed project (`ryuu-play`) uses; they were rejected because they fight both Rust and fast headless self-play.

## Consequences

Every choice a card can offer has to be expressible as a phase of the state. The engine must always be able to say who is to act, which is not always the player whose turn it is: a knockout hands the choice to the player who lost the Pokémon.
