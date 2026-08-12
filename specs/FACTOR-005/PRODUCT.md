# FACTOR-005: Local orchestration

## Summary

Parent local agents can spawn child local agents. No cloud Oz handoff.

## Behaviors

1. With orchestration enabled, agent may start child agents for subtasks.
2. Children appear as local conversations/panes.
3. No cloud run id or account required.
4. User can disable orchestration in settings.
5. Cloud Oz UI never appears.

## Validation

- start_agent executor test
- Offline spawn (no warp.dev)
