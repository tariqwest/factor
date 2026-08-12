# 06 — Warpify, shell hooks, and agent-over-SSH

## Goal

Guarantee Factor’s primary agent can operate **with** Warp’s shell integration (hooks) and **over** warpified SSH sessions — shell tools on the remote, clear semantics for file tools (D-08).

## Components

| Component | Path (post-import) |
|-----------|-------------------|
| Bootstrap hooks | `app/assets/bundled/bootstrap/` |
| Warpify settings | `app/src/terminal/warpify/` |
| SSH | `app/src/terminal/ssh/` |
| ANSI preexec | `app/src/terminal/model/grid/ansi_handler.rs` |
| Shell executor | `app/src/ai/blocklist/action_model/execute/shell_command.rs` |
| LRC | long-running shell command paths (Zap LRC helpers useful) |

## Test matrix

| # | Setup | Action | Expected |
|---|-------|--------|----------|
| H1 | Local bash warpify | run command | Block boundaries via preexec |
| H2 | Local zsh/fish | same | same |
| H3 | Agent local shell tool | run_shell_command | Same PTY/session |
| H4 | SSH warpify | remote `uname` via agent | Remote output |
| H5 | SSH + LRC | long command | Monitor remote, not local |
| H6 | SSH + read_files | per D-08 | Fail closed or remote FS |
| H7 | Un-warpified SSH | agent shell | Documented degraded behavior |

## Implementation notes

1. Thread `ExecutionContext { session_id, is_remote, cwd }` into all executors.  
2. Port Zap LRC tag-in fixes so agent doesn’t open a second local shell for remote TUI.  
3. Do **not** require Zap `warp_ssh_manager` UI for v1 — use Warp SSH session entrypoints.

## Manual smoke

```bash
# script/factor_ssh_agent_smoke (env FACTOR_SSH_TARGET=user@host)
```

## Exit criteria

- [ ] H1–H5 automated or scripted  
- [ ] H6 documented and implemented per DECISIONS  
- [ ] PRODUCT UJ-4 passes  
