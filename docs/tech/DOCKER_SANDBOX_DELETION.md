# Docker Sandbox Deletion Inventory (P-01 / Q-01)

**Status:** Product surface dropped; residual implementation partially stubbed/disabled.  
**Date:** 2026-08-12  
**Scope:** Factor fork — local Docker sandbox (`sbx` / `LocalDockerSandbox`), not generic Dockerfile language support or `docker/` build images.

## Product decision

- Factor v1 **does not ship** Docker sandbox as a user-facing session type.
- `/docker-sandbox` is **never registered** (even if `LocalDockerSandbox` is forced on in tests).
- `FeatureFlag::LocalDockerSandbox` is **hard-disabled** in production; test overrides only.
- Do **not** reintroduce cloud/hosted sandbox flows.

## Phases

| Phase | Goal | Status |
|-------|------|--------|
| **0** | Product surface drop (slash unregistered, flag hard-off, dogfood off) | **Done** |
| **1** | Unconditionally disable remaining UI/settings/slash entrypoints (not only flag-gated) | **Done** |
| **2** | Delete view-layer `terminal/view/docker_sandbox/` and make create paths no-ops | **Done** |
| **3** | Remove `ShellStarter` / `ShellLaunchData` / `AvailableShell` Docker variants + `local_tty` spawn | Residual |
| **4** | Remove `DefaultSessionMode::DockerSandbox`, `WorkspaceAction::AddDockerSandboxTab`, event variants | Residual (kept for settings/action stability) |
| **5** | Remove `FeatureFlag::LocalDockerSandbox` + warp_features hard-disable tests | Residual (keep until phase 3) |
| **6** | isolation_platform docker_sandbox, SessionPlatform::DockerSandbox | Residual (platform identity, not product UI) |

## Touchpoints

### A. User-facing entrypoints (Phase 1 — disabled unconditionally)

| Location | What | Action |
|----------|------|--------|
| `app/src/workspace/view.rs` | New-session menu "Local Docker Sandbox" | **Removed** (not flag-gated) |
| `app/src/workspace/view.rs` | `add_docker_sandbox_tab` | **No-op → Terminal** |
| `app/src/workspace/view.rs` | `DefaultSessionMode::DockerSandbox` handler | **Falls back to Terminal** |
| `app/src/workspace/view.rs` | `AddDockerSandboxTab` action | **Falls back to Terminal** |
| `app/src/workspace/action.rs` | `WorkspaceAction::AddDockerSandboxTab` | Kept (handler falls back) |
| `app/src/settings/ai.rs` | `DefaultSessionMode::DockerSandbox` | Enum kept; **always** resolves to Terminal |
| `app/src/settings_view/features_page.rs` | Default session mode dropdown | **Always** hides DockerSandbox |
| `app/src/terminal/view.rs` | `InputEvent::CreateDockerSandbox` | **Unconditional no-op** |
| `app/src/terminal/input.rs` | `Event::CreateDockerSandbox` | Kept; handler no-ops |
| `app/src/terminal/input/slash_commands/mod.rs` | `/docker-sandbox` execute arm | **No-op** |
| `app/src/search/.../commands.rs` | `CREATE_DOCKER_SANDBOX` const | Dead const; not in registry |
| `app/src/search/.../commands.rs` | Registry + Factor DROP tests | Kept |

### B. View / creation modules (Phase 2)

| Location | What | Action |
|----------|------|--------|
| `app/src/terminal/view/docker_sandbox/` | create/push sandbox pane, init env | **Deleted** |
| `app/src/terminal/view.rs` | `mod docker_sandbox` | **Removed** |

### C. Shell / PTY infrastructure (Phase 3 — residual)

| Location | What | Status |
|----------|------|--------|
| `app/src/terminal/local_tty/docker_sandbox.rs` | `DockerSandboxShellStarter`, `sbx` resolve | Residual module |
| `app/src/terminal/local_tty/mod.rs` | `pub mod docker_sandbox` | Residual |
| `app/src/terminal/local_tty/shell.rs` | `ShellStarter::DockerSandbox` | Residual match arms |
| `app/src/terminal/local_tty/unix.rs` | `spawn_docker_sandbox`, prepare/build | Residual (unreachable from UI) |
| `app/src/terminal/local_tty/windows/mod.rs` | DockerSandbox arm | Residual |
| `app/src/terminal/local_tty/terminal_manager.rs` | DockerSandbox arms | Residual |
| `app/src/terminal/local_shell/mod.rs` | DockerSandbox arm | Residual |
| `app/src/terminal/available_shells.rs` | `Config::DockerSandbox`, constructors | Residual |
| `crates/warp_terminal/src/shell/mod.rs` | `ShellLaunchData::DockerSandbox` | Residual |
| `app/src/shell_indicator.rs` | DockerSandbox → Linux | Residual |
| `app/src/terminal/writeable_pty/pty_controller.rs` | DockerSandbox match | Residual |
| `app/src/terminal/model/session/command_executor.rs` | DockerSandbox matches | Residual |
| `app/src/ai/.../agent_input_footer/mod.rs` | DockerSandbox shell checks | Residual |
| `app/src/integration_testing/terminal/util.rs` | DockerSandbox arm | Residual |
| `app/src/terminal/bootstrap.rs` | Comment re prepare_docker_sandbox | Residual |

### D. Platform / isolation (Phase 6 — residual)

| Location | What | Status |
|----------|------|--------|
| `crates/warp_core/src/platform.rs` | `SessionPlatform::DockerSandbox` | Residual |
| `crates/isolation_platform/` | `docker_sandbox` module, type detection | Residual (runtime identity when *inside* a container) |
| `crates/warp_features` | `FeatureFlag::LocalDockerSandbox` hard-disable | **Keep** until full delete |

### E. Assets / unrelated (out of scope)

| Location | Why out of scope |
|----------|------------------|
| `app/assets/bundled/svg/docker.svg` | Icon still used by context chips / generic UI |
| `crates/languages/grammars/dockerfile` | Language highlighting, not sandbox product |
| `docker/agent-dev`, `docker/linux-dev` | Dev container images for building Factor |
| `app/tests/ssh/Dockerfile` | SSH integration test fixture |
| `app/src/uri/docker.rs` | URI scheme handling — not session product |

### F. Tests

| Location | Action |
|----------|--------|
| `crates/warp_features/src/features_test.rs` | **Keep** hard-disable + override + absent-from-lists |
| `app/src/search/.../commands.rs` | **Keep** `factor_docker_sandbox_stays_off_*`, FACTOR_DROPPED |
| `app/src/terminal/input_test.rs` | Create-slash happy-path test **removed** |
| `app/src/terminal/input/slash_command_model_tests.rs` | Asserts command stays off even if flag forced |

## Safety constraints

1. Do not break non-Docker shell spawn (`Direct`, `WSL`, `MSYS2`).
2. Do not reintroduce cloud/hosted environment init.
3. Settings that still deserialize `default_session_mode = "docker_sandbox"` must **not** wedge the user — always fall back to Terminal.
4. `cargo test -p warp --lib factor_` and `cargo test -p warp_features --features test-util --lib factor_local_docker` must pass.

## Next deletion steps

1. Remove `ShellLaunchData::DockerSandbox` + `ShellStarter::DockerSandbox` + match arms.
2. Delete `app/src/terminal/local_tty/docker_sandbox.rs` and unix spawn helpers.
3. Remove `AvailableShell` Docker config + `new_docker_sandbox_shell`.
4. Drop settings enum variant after migration note; drop action/event variants.
5. Delete `FeatureFlag::LocalDockerSandbox` and its tests last.
6. Revisit `isolation_platform` / `SessionPlatform` only if no agent-in-container identity is needed.
