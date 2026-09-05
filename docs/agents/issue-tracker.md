# Issue tracker: Local Markdown

Issues live as Markdown files under `.scratch/<effort>/` in this repository.

[The agent-work glossary](glossary.md) defines the tracker terms.

## The `.scratch/` lifecycle

`.scratch/` is tracked and committed, despite the name. Never clear it as scratch space. A closed effort's directory is deleted, and its git history is the record. Every ticket in the effort resolves before the deletion.

An effort that creates its own repository migrates there once that repository exists. The migration is that effort's close, run in the origin repository: the origin resolves the effort before deleting its directory.

## Conventions

- One effort per directory: `.scratch/<effort>/`
- The spec is `.scratch/<effort>/spec.md`
- The map is `.scratch/<effort>/map.md` — the Destination / Notes / Decisions-so-far / Fog body, with one child ticket per open question
- Tickets are one file per issue at `.scratch/<effort>/issues/<NN>-<slug>.md`, numbered from `01` — never one combined tickets file
- A `Type:` line near the top of a ticket records its type: `research`, `prototype`, `grilling`, or `task`
- Triage state is a `Status:` line near the top of each ticket file (see [the triage labels](triage-labels.md) for the role strings); `claimed` and `resolved` are recorded on the same line
- Acceptance criteria close a ticket's body as a checkbox list
- Comments append to the bottom of the file under a `## Comments` heading

## When a skill says "publish to the issue tracker"

Create a new file under `.scratch/<effort>/` (creating the directory if needed). `/to-spec` publishes the spec; `/to-tickets` publishes one file per ticket, blockers first.

## When a skill says "fetch the relevant ticket"

Read the file at the referenced path. The user will normally pass the path or the issue number directly.

## When a skill declares blocking edges or works the frontier

`/to-tickets` writes the edges; `/wayfinder` and `/implement` take work from the frontier.

- **Blocking**: a `Blocked by: NN, NN` line near the top. A ticket is unblocked when every file it lists is `resolved`.
- **Frontier**: the open, unblocked, unclaimed files under `.scratch/<effort>/issues/`; first by number wins.

## When a skill claims or resolves a ticket

`/wayfinder` names these two wayfinding operations; every skill that works a ticket uses the same two.

- **Claim**: set `Status: claimed` and save before any work.
- **Resolve**: append the answer under an `## Answer` heading, set `Status: resolved`, then append the map line to the map's Decisions-so-far in `map.md`.

### The Answer opening

An Answer opens with the resolved date, the branch, and each commit identifier. Report what the remote holds.

### The map line

Fill this form; do not compose a summary:

```
Ticket NN resolved <date>: <one clause>; details under [the ticket's Answer](issues/NN-<slug>.md).
```

The clause says what changed, and the link carries the detail. One clause is the limit; no word count applies. `/code-review` reads the line against this form. Expect it to catch a miss, and read the catch as the mechanism working.
