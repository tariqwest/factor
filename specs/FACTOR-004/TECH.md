# FACTOR-004 TECH

## Keep

- `app/src/ai/blocklist/**`
- input mode state machine (agent vs shell)
- `cli_agent_sessions`
- predict/suggestions (BYOP-backed when model needed)

## Changes

- Remove login/credit gates on agent entry.
- Ensure context packing includes selected blocks/errors for BYOP user_context.

## Tests

- Mode transitions unit/integration
- Context attachment includes failed block output (truncated rules)
