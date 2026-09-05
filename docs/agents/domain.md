# Domain docs

How the engineering skills consume this repository's domain documentation.

This repository is **single-context**: one `CONTEXT.md` at the repository root and one [ADR directory](../adr/).

## Before exploring, read these

- **`CONTEXT.md`** at the repository root, and the glossaries it points to:
    - [The domain glossary](../architecture/glossary.md) — this repository's domain vocabulary.
    - [The agent-work glossary](glossary.md) — the vocabulary of agent work.
- **The [ADR directory](../adr/)** — the records that touch the area you are about to work in.

The glossaries always exist. The ADR directory may hold only [the template](../adr/template.md); that is not a fault — a record lands when a decision is made.

## Use the glossary's vocabulary

When your output names a domain concept, use the term as the glossaries define it. Do not drift to a synonym a glossary avoids. A concept the glossaries lack is a signal: either you are inventing language the repository does not use, or there is a real gap — note it for `/domain-modeling`.

## Records

[The template](../adr/template.md) fixes a record's form and its lifecycle. It overrides any format or lifecycle guidance a skill carries. A bare number (`0001`) is a record in this repository's [ADR directory](../adr/). If your output contradicts a record, surface the conflict rather than settling it silently.
