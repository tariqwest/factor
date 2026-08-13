# 06 — Warpify, shell hooks, and agent-over-SSH

## Status (2026-08-12)

- **Matrix doc:** [`docs/tech/SSH_AGENT_MATRIX.md`](../../docs/tech/SSH_AGENT_MATRIX.md) (H1–H7 + UJ-4, status unit/smoke/manual/blocked)
- **Smoke:** `script/factor_ssh_agent_smoke` — offline CI safe when `FACTOR_SSH_TARGET` unset; denylist + pure unit filters
- **Unit landed:** SSH parse/login/sftp/one-line helpers; `SessionContext` remote/legacy flags; D-08 fail-closed message
- **Still open:** H4 full agent-over-SSH (manual), H5 LRC remote binding (blocked), H1–H2 full preexec integration

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

Canonical table + statuses: **`docs/tech/SSH_AGENT_MATRIX.md`**.

| # | Setup | Action | Expected | Status |
|---|-------|--------|----------|--------|
| H1 | Local bash warpify | run command | Block boundaries via preexec | manual / partial unit |
| H2 | Local zsh/fish | same | same | manual |
| H3 | Agent local shell tool | run_shell_command | Same PTY/session | manual / existing units |
| H4 | SSH warpify | remote `uname` via agent | Remote output | manual (+ smoke probe) |
| H5 | SSH + LRC | long command | Monitor remote, not local | blocked |
| H6 | SSH + read_files | per D-08 | Fail closed or remote FS | unit (message + flags) |
| H7 | Un-warpified SSH | agent shell | Documented degraded behavior | unit (flags) / manual UX |

## Implementation notes

1. Thread `ExecutionContext { session_id, is_remote, cwd }` into all executors.  
2. Port Zap LRC tag-in fixes so agent doesn’t open a second local shell for remote TUI.  
3. Do **not** require Zap `warp_ssh_manager` UI for v1 — use Warp SSH session entrypoints.

## Manual smoke

```bash
# Offline (CI):
./script/factor_ssh_agent_smoke

# Optional live probe:
FACTOR_SSH_TARGET=user@host ./script/factor_ssh_agent_smoke
```

## Exit criteria

- [x] Matrix documented with unit/smoke/manual/blocked  
- [x] Offline smoke script (skip live when no target)  
- [x] H6 fail-closed path present + unit coverage for message / remote flags  
- [x] Agent session policy pure units (shell target / D-08 / LRC attach)  
- [x] Bootstrap asset markers unit (H1–H2 offline slice)  
- [x] `script/factor_m5_smoke` packaging + offline SSH gate  
- [ ] H1–H5 fully automated (H1–H4 partial; H5 still blocked for remote bind)  
- [ ] PRODUCT UJ-4 passes on real Linux remote
- [ ] **Assessment addition:** `script/clean` + perf budget (cold start) documented; `cargo deny` passes for SSH deps

## Assessment additions — H5 unblock recipe (close TECH §6.2)

TECH §6.2 says “must target remote PTY” but leaves binding implicit. To unblock H5:

```rust
// app/src/ai/blocklist/agent_session_policy.rs already has lrc_attached().
// Next: in shell_command.rs, when SessionContext::is_remote() && lrc_attached()
// use remote command id (not local) for tag-in; add regression test
// `lrc_attached` true → must_not_nest_ssh true (already unit-covered).
```

> 2026-08-13: policy + doc landed, executor threading still open (see SSH_AGENT_MATRIX). `factor_ssh_agent_smoke --unit-only` already covers `lrc_attached` detection.

Add to `SSH_AGENT_MATRIX.md` Gap #2: exact file + test name. Measure cold-start ≤ warper+100ms in `factor_offline_local_smoke` appendix (TECH §10).  
