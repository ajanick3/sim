# AGENTS.md

## The register

The rules that bind every session in this repository.

- **Write all documentation toward ASD-STE100 Part 1 (Simplified Technical English); do not claim conformance with it.** This includes every Markdown file, comment, and commit message. Write with verbs; break a long nominal sentence into short ones. Delete a line that changes nothing. Give one fact one home.
- **Branch off `main` before committing; stop at a local commit and report the branch name.** `push`, `pull`, `fetch`, and the pull request belong to the operator; a `git` command that reaches the remote fails by design.
- **Write the commit header to the Conventional Commits standard, within 50 characters.** Wrap the body at 72 characters. Close the message with a trailer block: a blank line, a `---` line, a blank line, then the `Co-Authored-By:` trailer that names the agent used.
- **An architecture decision between live alternatives gets a record in the ADR directory when it is made.** [The ADR template](docs/adr/template.md) fixes the form and the lifecycle.
- **Session mechanics live only in `AGENTS.md`, `CONTEXT.md`, `docs/agents/`, and `.scratch/`.** Every other document must read unchanged after these files are deleted: do not link them, name them, or defer a fact to them — absorb the fact into the document you write. Records and commit messages keep old names as history.
- **Raise a conflict between a document and a convention the code already runs on.** Never settle it silently in either direction. Evidence read from the live resources outranks both documents.

## What this repository is

[TBD]

## Agent skills

- [The issue tracker](docs/agents/issue-tracker.md) — how an effort, a ticket, and a resolution work.
- [The triage labels](docs/agents/triage-labels.md) — the role strings a `Status:` line takes.
- [The domain docs](docs/agents/domain.md) — where a domain fact homes.
- [The agent-work glossary](docs/agents/glossary.md) — the vocabulary the agent docs use.
