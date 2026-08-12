# FACTOR-006: Warpify + SSH agent tools

## Summary

Shell integration hooks and warpified SSH sessions work; the primary agent’s shell tools execute in the correct session context.

## Behaviors

1. Local warpify creates proper block boundaries via preexec hooks.
2. SSH sessions can be warpified.
3. Agent `run_shell_command` on warpified SSH runs remotely.
4. Long-running remote commands are monitored on the remote session.
5. File tools follow D-08 (fail closed or true remote FS — see DECISIONS).
6. Degraded behavior when not warpified is documented in UI/docs.

## Validation

- Plan 06 matrix H1–H7
- Manual SSH smoke
