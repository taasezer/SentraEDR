# Update Operations Guide

## Agent Update Mechanism
Updates are strictly ATOMIC. The agent downloads the update package, verifies cryptographic signatures, and extracts the payload to a `.staging` directory. The service shuts down, the staging directory atomically swaps with the current directory using `MoveFileEx`, and the service restarts.

## Failure Recovery
If the updated agent panics repeatedly within the first 60 seconds (detected by SCM or Internal Watchdog crash loops), the rollback script triggers, swapping the previous version back into place. The agent is NEVER left in a dead state.
