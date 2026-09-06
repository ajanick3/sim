# Undo one action

Type: task
Status: ready-for-agent
Blocked by: 01

Take back the last action.

The fog names the choice: replay the log from the start without its last
entry, or reverse the action in place. A shuffle cannot be reversed without
recording what it did, which points at the first, and the first is only as
slow as a replay.

- [ ] The last action can be taken back
- [ ] The state after an undo equals the state before the action
- [ ] A decision on which of the two ways, and why
