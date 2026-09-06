# Keep the record of every card

Type: task
Status: ready-for-agent

`load` drops the printed fields of the 2777 cards the engine cannot run. Keep
them, so a caller can read any card in the artifact.

The shape is the decision. A raw `serde_json::Value` per card costs nothing to
write and pushes the parsing onto every reader. A typed record parses once and
refuses malformed data at load, but it has to model fields the engine has no
use for, such as a Trainer's text and an Energy's kind.

Weigh the memory too: the artifact is 2.0 MB on disk, so keeping every record
roughly doubles what a load holds.

Whatever the shape, a refused card must stay refused. The engine plays a card
only when it can run all of it, which [ADR 0008](../../../docs/adr/0008-the-engine-refuses-a-card-it-cannot-run.md)
decided and this ticket does not revisit.

- [ ] A recorded decision on the kept shape, raw or typed, and why
- [ ] Any card in the artifact can be looked up by its id and its printed
      fields read
- [ ] The admitted cards still play, and the coverage count does not move
- [ ] The cost in memory, measured rather than guessed
