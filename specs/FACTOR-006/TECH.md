# FACTOR-006 TECH

## Components

- bootstrap scripts (hooks)
- warpify + ssh modules
- shell_command executor context
- Zap LRC helpers for remote binding

## ExecutionContext

```text
session_id, pane_id, is_remote, remote_cwd?, transport
```

Passed into tool executors from blocklist controller.

## Tests

See `.agents/plans/06-warpify-ssh-agent.md`.
