# Prize values

Type: task
Status: ready-for-agent

A knockout always takes 1 Prize. In Standard, 492 cards carry the `ex` rule
and are worth 2, and a Mega ex is worth 3.

The data cannot tell a Mega ex from an ordinary ex except by the `Mega ` at the
start of its name, which is recorded in
[card data findings](../../../docs/architecture/card-data.md). Decide whether
the importer trusts that or refuses the card.

- [ ] A card carries what a knockout of it is worth
- [ ] The count is read at knockout time, not stored as a fixed number
- [ ] A decision on the Mega ex, recorded
