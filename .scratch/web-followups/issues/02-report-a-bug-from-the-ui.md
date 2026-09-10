# Report a bug from inside the UI

Type: task
Status: needs-triage

The operator asked for "a way to call out bugs in the UI and report back to
Claude". Several engine bugs this session were found by playing the web app
and describing what looked wrong (PRs #230, #231, #232, #234, #235). A
report button would carry the game with the words.

## Shape

A control on the page opens a short form — what looked wrong — and submits
it with the context an agent needs to reproduce:

- the game recipe from ticket 01 (seed, decks, action indices), so the
  exact position is replayable;
- the last N log lines and the current `view()` JSON;
- the engine version.

Where the report goes is the triage question. Simplest, given ticket 01
brings in Neon Postgres: a `bug_reports` table
(`id`, `game_id`, `recipe jsonb`, `log jsonb`, `view jsonb`,
`engine_version`, `note text`, `created_at`), written by a Server Action.
An agent reads the table. A GitHub issue or a webhook could forward from
there later.

## Depends on

- 01 (the recipe is the reproduction; build that first).

## Open questions for triage

- Whether the Neon `bug_reports` table is the final destination or a
  staging point that forwards to a GitHub issue.
- Whether reports are rate-limited or need any spam guard, given no login.
- How much of the log to attach by default.

## Acceptance criteria

- [ ] A visible control on the game page opens a report form.
- [ ] A submitted report carries the game recipe, recent log, current
      view, and engine version.
- [ ] The report lands somewhere an agent can pick it up and replay the
      position.
- [ ] Submitting does not interrupt or corrupt the running game.
