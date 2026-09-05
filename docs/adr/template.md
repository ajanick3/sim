# The ADR template

An architecture decision record captures one decision — a choice between live alternatives — and keeps its date and its reasoning after the people who made it have moved on. Copy the skeleton, replace every `<placeholder>`, and commit it as `docs/adr/<NNNN>-<slug>.md`: `<NNNN>` is the next unused number, zero-padded to four digits, and never a number already spent; `<slug>` is the title in lowercase words joined by dashes.

```text
# <The decision, stated as a sentence>

**Status:** Accepted — <YYYY-MM-DD>

<One paragraph: what made the decision necessary, the alternatives that were live, what was decided, and why it won. Written for a reader who was not there.>
```

Those lines are the whole mandatory form. Add a heading only where its section earns the place — `## Context`, `## Decision`, and `## Consequences` are the usual three. Length is earned by the decision, not by the ceremony.

Four rules the form carries:

- **The date is the decision's, not the document's.** A record written after the fact carries the date the decision was made.
- **Supersede rather than rewrite.** A record that stopped holding keeps its file and its number; its status line becomes `**Status:** Superseded by [<NNNN>](<NNNN>-<slug>.md) — <YYYY-MM-DD>`, dated to the supersession.
- **Correct a misstated fact with a dated erratum**, under a final `## Errata` heading, which a record grows only when one is owed.
- **Maintenance that preserves the recorded meaning needs no erratum** — a link repair, a typo, formatting. The decision, its date, and its reasoning are the immutable substance; a citation's plumbing is not.
