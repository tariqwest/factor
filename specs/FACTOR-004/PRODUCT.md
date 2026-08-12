# FACTOR-004: Primary agent terminal workflow

## Summary

Agent Mode is a first-class mode in the same terminal block list as bash. Users move fluidly between shell and agent with block context.

## Behaviors

1. Toggle or NL detection enters Agent Mode in-pane.
2. Failed command / selection can seed agent context (“Ask Factor”).
3. Agent tools run against current session (local or remote per FACTOR-006).
4. Exit agent returns to bash without destroying session.
5. Suggestions/next-command remain available when configured.
6. 3p CLI agents coexist without replacing primary agent.

## Validation

- Integration: mode switch + context attachment
- Manual UJ-2, UJ-3
