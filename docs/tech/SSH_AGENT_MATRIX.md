# SSH / Warpify agent matrix (M5 / FACTOR-006)

**Status:** Scaffold 2026-08-12  
**Product:** [PRODUCT.md UJ-4](../product/PRODUCT.md) · [FACTOR-006](../../specs/FACTOR-006/PRODUCT.md)  
**Plan:** [`.agents/plans/06-warpify-ssh-agent.md`](../../.agents/plans/06-warpify-ssh-agent.md)  
**Decision:** [D-08 remote file tools](../adr/DECISIONS.md) — shell tools remote; file tools **fail closed** unless remote FS already exists in session.

## How to run (offline CI safe)

| Command | Needs SSH host? | What it covers |
|---------|-----------------|----------------|
| `./script/factor_ssh_agent_smoke` | **No** (default) | Denylist + pure unit filters for SSH/warpify/session context |
| `FACTOR_SSH_TARGET=user@host ./script/factor_ssh_agent_smoke` | Yes | Above + BatchMode connectivity probe; prints remaining manual UJ-4 steps |
| `cargo test -p warp --lib ssh_` | No | Interactive SSH parse, login-state helpers, sftp command builder |
| `cargo test -p warp --lib session_context_remote` | No | `SessionContext` remote / host_id / legacy-SSH flags |
| `cargo test -p warp --lib remote_file_operations_fail_closed` | No | D-08 fail-closed conversation message |
| `cargo test -p ai --lib paths` | No | Host/shell native path join/normalize |
| Docker SSH fixture (`app/tests/ssh/`) | Local Docker | Manual warpify against container (see fixture README) |

When `FACTOR_SSH_TARGET` is **unset**, the smoke script **skips** live SSH and exits 0 after offline checks. Do not require a remote host in CI.

## Status legend

| Status | Meaning |
|--------|---------|
| **unit** | Pure logic or in-process unit tests; PR-safe |
| **smoke** | Scripted harness (offline and/or optional live) |
| **manual** | Human / dogfood with real shell or SSH |
| **blocked** | Needs missing feature, fixture, or decision validation |

## Matrix (H1–H7 + related)

| ID | Scenario | Expected (UJ-4 / plan 06) | Coverage | Status | Notes |
|----|----------|---------------------------|----------|--------|-------|
| H1 | Local bash warpify → run command | Block boundaries via preexec | Bootstrap assets + terminal hooks; IT-006 planned | **manual** / partial **unit** (script size) | Full block boundary needs PTY / integration harness |
| H2 | Local zsh/fish warpify | Same block lifecycle | Same as H1 for multi-shell assets | **manual** | Fish/zsh bootstrap scripts exist under `app/assets/bundled/bootstrap/` |
| H3 | Agent local shell tool | `run_shell_command` on same PTY/session | Shell executor unit surface elsewhere; no remote | **manual** / existing agent tool units | Not SSH-specific; keep as local baseline |
| H4 | SSH warpify + agent shell | Remote `uname` (or equivalent) via agent on remote session | Needs warpified remote + agent tool loop | **manual** / **smoke** (probe only) | Live step when `FACTOR_SSH_TARGET` set; full agent loop still manual |
| H5 | SSH + LRC | Long-running command monitored on **remote**, not a second local shell | Zap LRC binding reference; Factor path TBD | **blocked** | Port LRC session binding; no CI network test until pure helpers exist |
| H6 | SSH + `read_files` / apply diffs | Per D-08: **fail closed** with clear error (no silent local FS) | `read_files` executor + `DiffApplicationError::RemoteFileOperationsUnsupported` | **unit** (message + `SessionContext::is_remote`) | Remote FS path only if session already has remote client; v1 default fail closed |
| H7 | Un-warpified / legacy SSH | Documented degraded behavior | `SessionContext::is_legacy_ssh` + `ssh_connection_info` | **unit** (flags) / **manual** (UX copy) | Shell may still work via legacy executor; file tools must not assume local paths |
| UJ-4 | End-to-end agent over warpified SSH | Shell tools remote; file tools respect D-08 | Matrix H4–H7 | **manual** | Product success metric: green on Linux remote |
| P-parse | Interactive SSH command parse | Detect `ssh` / gcloud / eb / doctl interactive forms | `terminal::ssh::util` tests | **unit** | Offline |
| P-login | SSH login grid state | Last login / password / prompt heuristics | `check_ssh_login_state` | **unit** | Offline |
| P-script | One-line SSH script pack | Newlines → safe one-line for macOS SSH server | `convert_script_to_one_line` | **unit** | Offline |
| P-sftp | SFTP put command builder | Builds `sftp … <<< "put …"` string | `transfer_file_sftp_command` | **unit** | Offline; does not transfer |
| P-path | Path semantics for tools | Host-native absolute paths from cwd | `crates/ai` `paths` tests | **unit** | Offline |
| Denylist | No hosted Warp phone-home in runtime | `check_factor_static_denylist` | Script + offline smoke | **smoke** | Always run in `factor_ssh_agent_smoke` |

## Component map

| Area | Path |
|------|------|
| Bootstrap hooks | `app/assets/bundled/bootstrap/` |
| Warpify settings / UI | `app/src/terminal/warpify/` |
| SSH detect / warpify / util | `app/src/terminal/ssh/` |
| Session types | `app/src/terminal/model/session.rs` (`SessionType::Local` / `WarpifiedRemote`) |
| Agent session context | `app/src/ai/blocklist/controller.rs` (`SessionContext`) |
| Shell tool executor | `app/src/ai/blocklist/action_model/execute/shell_command.rs` |
| File read fail-closed | `app/src/ai/blocklist/action_model/execute/read_files.rs` |
| Apply-diff fail-closed | `…/request_file_edits/apply_diff_model.rs` + `diff_application.rs` |
| Docker SSH fixture | `app/tests/ssh/` |

## ExecutionContext (target shape)

From plan 06 / FACTOR-006 TECH — thread into executors:

```text
session_id, pane_id?, is_remote, host_id?, cwd, is_legacy_ssh?, transport?
```

Today `SessionContext` exposes `session_type`, `host_id()`, `is_remote()`, `current_working_directory()`, `is_legacy_ssh()`, `ssh_connection_info()`. Expand only when a tool needs more fields; keep flags pure and unit-tested.

## Manual UJ-4 checklist (when you have a host)

1. SSH to host (`ssh $FACTOR_SSH_TARGET` or Factor session entry).
2. Warpify session (or accept warpify prompt).
3. Open Agent Mode; ask for `uname -a` (or `hostname`) via shell tool → output must be **remote**.
4. Start a long command; LRC UI must track the **remote** block (H5 — may still be incomplete).
5. Ask agent to `read_files` a path that exists only remotely → clear fail-closed error (H6), not local contents.
6. Repeat without warpify (legacy SSH) and confirm degraded behavior is understandable (H7).

## Gaps / next code work

1. Wire/verify shell executor always uses active remote session PTY (H4).
2. Port Zap LRC remote binding so agent does not open a second local shell (H5).
3. Integration scenario IT-006 (local warpify blocks) when harness is cheap enough for CI.
4. Optional recorded/mock SSH for H4 without live host (not required for v1 gate if manual matrix + units pass).
