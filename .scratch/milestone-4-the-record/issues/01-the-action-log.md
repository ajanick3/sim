# The action log

Type: task
Status: ready-for-agent

Record every action the engine applies, so a game can be replayed from its
seed and its log.

ADR 0002 (now 0007) promised that a replay and a test are the same thing. The
engine is pure enough for it and keeps no actions, so the promise has never
been collectable.

- [ ] The state holds the actions applied to it, in order
- [ ] A game replays from its seed and its log to the same state
- [ ] A refused action is not recorded, since it changed nothing
- [ ] A recorded decision on what the log is for, and what it is not
