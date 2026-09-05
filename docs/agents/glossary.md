# The vocabulary of agent work

The terms the agent docs use. A term that is a property of durable documents — Record — lives in [the domain glossary](../architecture/glossary.md).

**Session mechanics**:
A fact that only constrains how an agent session operates. It lives in the agent-facing files and may die with the harness. "Tracker mechanics" is not a term here; a fact of that kind is session mechanics.

**Register**:
The complete set of rules that bind every session, in the register section of [AGENTS.md](../../AGENTS.md).

**Ephemeral**:
A working document that dies when its effort closes.

**Tracker terms**:
The issue tracker's vocabulary: effort, home, spec, map, frontier, ticket, and research, each defined below.

**Effort**:
One tracked body of work: a spec, a map, and tickets under `.scratch/<effort>/` in one repository.

**Home**:
The repository that holds an effort — the one that owns the effort's destination. An effort that creates its own repository migrates there once that repository exists.

**Spec**:
An effort's decided plan, one file per effort: problem statement, solution, user stories, implementation and testing decisions, and what is out of scope. It synthesizes decisions already made; it never opens new questions.

**Map**:
An effort's working picture: destination, notes, decisions so far, and what is not yet specified. One file per effort, with one child ticket per open question.

**Frontier**:
The next ticket a session may claim: open, unblocked, unclaimed, first by number.

**Ticket**:
One issue in an effort: one file, numbered, carrying its type, its triage status, and — once resolved — its answer.

**Research**:
An investigation of a question against primary sources — official docs, source code, specifications, first-party APIs — never a secondary write-up of them. The findings land in one Markdown file that cites each claim's source. Also a ticket type.
