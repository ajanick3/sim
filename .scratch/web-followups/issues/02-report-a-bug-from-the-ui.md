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

Where the report goes is the triage question. Options: a GitHub issue via a
small serverless route, an entry in the same store ticket 01 uses, or a
plain webhook. It must land somewhere an agent reads, not only a human.

## Depends on

- 01 (the recipe is the reproduction; build that first).

## Open questions for triage

- Destination: GitHub issue, a store the agent polls, or a chat webhook.
- Whether reports are rate-limited or need any spam guard, given no login.
- How much of the log to attach by default.

## Acceptance criteria

- [ ] A visible control on the game page opens a report form.
- [ ] A submitted report carries the game recipe, recent log, current
      view, and engine version.
- [ ] The report lands somewhere an agent can pick it up and replay the
      position.
- [ ] Submitting does not interrupt or corrupt the running game.
