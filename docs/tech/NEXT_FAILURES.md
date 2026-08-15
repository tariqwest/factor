# NEXT_FAILURES — full-suite triage (P0 quality, 2026-08-14)

> **Context:** `cargo test -p warp --lib` was 243 FAIL (ulimit 256, default threads) -> 127 -> 97 after `AgentProviderSecrets` fix -> **9 FAIL with `ulimit -n 4096 --test-threads=1`** (3175 pass, 3 ignored) -> **7 FAIL (3177 pass, 3 ignored)** after `ab7d6a0` fixes `test_trims_whitespace` + `test_single_code_file`. With `--test-threads=4` + `ulimit 4096` -> **10 FAIL** (3179 pass) — same 7 + 3 flaky parallel. Filtered `factor_*` 5/5 and `factor_*` + BYOP smokes still green.
> **Update 2026-08-13 (e806e08):** `test_single_code_file` fixed (oss.rs->factor_oss.rs), `test_trims_whitespace` fixed via `build_bootstrap_script` test bypass (BOOTSTRAP_CACHE MemoMap). Verified `ulimit -n 4096 --test-threads=1` = 7 fail (49s).
> **Update 2026-08-14:** **0 FAIL for the 7-bucket** — `cargo test -p warp --lib -- --test-threads=1` filtered to the 7 tests is now **7 passed** (verified). Fixes: `secret_redaction` explicit clear + `#[serial]` (J-13), `compute_fallback_shell`/`get_pw_entry` gracefully handle missing user DB (J-14/J-15) which also resolved the 4 view + 2 slash isolation failures that were actually `current user should exist` panics in sandboxed env. Full suite in Muse sandbox still shows ~20 network-dependent failures (`mockito` `Operation not permitted` on 127.0.0.1 bind — sandbox, not code) — bucketed as `env: network-sandbox` below. Outside sandbox (normal macOS terminal with network), the 7-bucket is green; `factor_*` 5/5 remains green.
> **Update 2026-08-14 (post J-18/J-19):** The remaining order-dependent failures in the full suite were rooted in two forms of shared process state, not missing user DB: `FeatureFlag::*::set_enabled(true)` in unit tests mutates global `FLAG_STATES` (the `cfg!(test)` guard in `warp_features` is inactive when `app` tests link the crate as a lib), and `COMMAND_REGISTRY` was a `LazyLock<Registry>` frozen at first access with the `FeatureFlag` state present at that moment. After converting tests to `override_enabled` RAII guards and making `COMMAND_REGISTRY` resettable per `App::test`, the full unfiltered suite is **0 FAIL** (`3184 passed; 0 failed; 3 ignored`) with `ulimit -n 4096 --test-threads=1`.

This file buckets the remaining deterministic failures observed with single-threaded run (the plan's "next sprint #1" gate). **Do not `#[ignore]` without linking this file + owner.**

## How to reproduce

```bash
ulimit -n 4096
cargo test -p warp --lib -- --test-threads=1 2>&1 | tail
# or
cargo test -p warp --lib -- --test-threads=4 2>&1 | tail   # 12 fails, same 9 + 3 flaky parallel
cargo test -p warp --lib -- terminal::bootstrap::tests::test_trims_whitespace --nocapture
```

Default OS `ulimit -n 256` reproduces the 81 "Too many open files" bulk (now mitigated by `ulimit -n 4096` in local dev and future CI `script/clean` + runner config).

## Bucket

| # | Test | File:line | Symptom | Origin (assessment) | Proposed fix / owner |
|---|------|-----------|---------|----------------------|----------------------|
| 1 | `ai::blocklist::block::secret_redaction::test::test_detect_secrets_no_regexes_configured` | `app/src/ai/blocklist/block/secret_redaction_test.rs:315` | `left: [SecretRange 4..39] right: []` — expects no regex -> no detection, but finds a range | Factor now registers `AgentProviderSecrets` + default secret regexes via `initialize_settings_for_tests` (previously no secrets). Test was written for no-regex default. | **FIXED 2026-08-14 (J-13):** test now `#[serial]` + explicitly clears `SECRETS_REGEX` via `set_user_and_enterprise_secret_regexes(empty, empty)` before asserting. |
| 2 | `terminal::bootstrap::tests::test_trims_whitespace` | `app/src/terminal/bootstrap_test.rs:48` | `left` = full bootstrap script, `right` = `"asdf\nno whitespace\n …"` | BOOTSTRAP_CACHE MemoMap shadowed TestAssetProvider, so `script_for_shell(Fish)` returned bash script despite `asset_for_shell_config` returning fish fixture. | **FIXED `ab7d6a0`:** extracted `build_bootstrap_script` + `cfg(test)` bypass of global cache. |
| 3 | `terminal::input::slash_command_model::tests::test_docker_sandbox_command_remains_active_when_ai_is_disabled` | `app/src/terminal/input/slash_command_model_tests.rs:188` | expects `["/docker-sandbox"]` active when AI off, got `["/rename-tab","/open-file"]` | Factor gates `/docker-sandbox` behind `LocalDockerSandbox` flag. Test sets the flag, but the `COMMAND_REGISTRY` `LazyLock` had already been built by an earlier test with the flag off, so the slash command was not registered. | **FIXED 2026-08-14 (J-19):** `COMMAND_REGISTRY` is now a resettable `Mutex<Option<Registry>>`; `add_window_with_*` helpers reset it before each `App::test`, so the registry is rebuilt with the test's active feature overrides. |
| 4 | `terminal::input::slash_command_model::tests::test_parse_slash_command_handles_argument_rules` | `.../slash_command_model_tests.rs:29` | `expected at least one slash command without an argument` | Same as #3: `COMMAND_REGISTRY` was frozen without `LocalDockerSandbox` commands by the time this test ran; the filtered active set therefore contained only commands with arguments. | **FIXED 2026-08-14 (J-19):** same `COMMAND_REGISTRY` reset fix. |
| 5 | `terminal::view::open_in_warp::tests::test_single_code_file` | `app/src/terminal/view/open_in_warp_tests.rs:101` | `left: None right: Some(.../app/src/bin/oss.rs)` | Factor renamed `app/src/bin/oss.rs` -> `factor_oss.rs` for branding. Test hard-codes `oss.rs`. | **FIXED `ab7d6a0`:** updated test to assert `factor_oss.rs` + guard oss.rs false. |
| 6-9 | `terminal::view::tests::test_insert` / `test_insert_into_input` / `test_reinput_blocks` / `test_viewport_iter_most_recent_at_bottom` | `app/src/terminal/view_test.rs:487,613,1813,1009` | viewport/insert assertions (`left: "One" right: "None"`, selection leftover, `item 3`) | View tests set `FeatureFlag::AgentView` via `set_enabled(true)`, which leaked into the global `FLAG_STATES`. Later view tests saw `AgentView` enabled and changed `typed_characters_on_terminal` selection-clearing behavior. | **FIXED 2026-08-14 (J-18):** converted all unit-test `FeatureFlag::AgentView.set_enabled(true)` calls (and `ShellSelector.set_enabled`) to `FeatureFlag::AgentView.override_enabled(true)` RAII guards, so overrides are scoped to the test thread and dropped on test exit. |

**Plus flaky with `--test-threads=4` (3 extra, pass individually):**

- `ai::agent_providers::tests::smoke_merge_byop…` (2) — `add_singleton_model twice` before fix, now pass solo but race under parallel threads due to shared `ChannelState::set(Oss)` guard (`OssChannelGuard`). Already fixed with `has_singleton_model` guard, but parallel `ChannelState` still races. Mitigated by single-thread.

**Env: network-sandbox (Muse / sandboxed CI, 2026-08-14)**

In Muse's sandboxed shell, `bind(127.0.0.1:0)` fails with `Operation not permitted` (verified `python3 -c "socket.bind"` -> `PermissionError: [Errno 1]`). This makes `mockito` tests (`ai::agent_providers::tools::web_runtime::*` — ~10 tests) and any test that spawns a `mockito::Server` fail with `ServerFailure` / `closed channel`. These were **not** in the 7-bucket and passed in normal macOS terminal outside sandbox. Bucket as `env: network-sandbox` — not a code defect; run those tests outside sandbox or with `TEST_IGNORE_NETWORK=1`. The `test_get_pw_entry` also gracefully handles missing user DB now (J-15).

## File-descriptor bulk

`81x "Too many open files"` with default `ulimit 256` and `workspace::view` `tokio` runtime creation. Fixed by `ulimit -n 4096`. CI runners should set `ulimit -n 4096` or use `script/clean` + `--test-threads=4`. Documented in `00-overview Ops notes` + `J-07/J-08`.

## Env: missing user DB (sandboxed, 2026-08-14)

When DirectoryService is unavailable (`pwd.getpwuid(501)` -> `KeyError`, `dscl` -> `eServerError`), `nix::unistd::User::from_uid` and `getpwuid_r` return `None`/`null`. Previously `compute_fallback_shell` `expect("current user should exist")` and `get_pw_entry` `panic!("pw not found")` caused **159 extra failures** in full suite in Muse sandbox. **Fixed** via fallback to `HOME`/`USER` env or `/tmp`/`unknown` (J-14/J-15) — see `app/src/terminal/local_tty/{shell.rs,unix.rs}`.

## Test-isolation root causes (2026-08-14)

Two additional global-state leaks were responsible for the remaining order-dependent failures that survived the user-DB and secret-redaction fixes:

1. **Feature-flag global state (J-18).** `FeatureFlag::set_enabled` writes to process-global `FLAG_STATES`. When running `app` unit tests, `warp_features` is compiled as a dependency (not with `cfg(test)`), so the panic guard intended to block `set_enabled` in unit tests is disabled. Tests in `view_test.rs`, `use_agent_footer/mod_test.rs`, `blocks_test.rs`, and `available_shells_test.rs` used `set_enabled(true)` and left flags on for the rest of the suite. Fix: use `FeatureFlag::*::override_enabled(true)` and hold the returned guard for the lifetime of the test.
2. **Slash-command registry freeze (J-19).** `COMMAND_REGISTRY` was a `std::sync::LazyLock<Registry>`. `Registry::new` calls `all_commands()`, which consults `FeatureFlag` at build time. Once initialized, the registry never reflected per-test feature overrides. Tests like `test_docker_sandbox...` set `LocalDockerSandbox` after a previous test had already touched `COMMAND_REGISTRY` without it, so `/docker-sandbox` was absent. Fix: replace `LazyLock` with a `Mutex<Option<Registry>>` wrapper that can be reset, and reset it in `test_util::terminal::add_window_with_id_and_terminal` and `terminal::input_test::add_window_with_bootstrapped_terminal` so each `App::test` builds a fresh registry from its active overrides.

## Next actions (plan 00 #1)

- [x] Fix #5 (oss.rs -> factor_oss.rs) — done `ab7d6a0`
- [x] Fix #2 `test_trims_whitespace` — done `ab7d6a0` (MemoMap bypass)
- [x] Fix #1 `secret_redaction` no-regex — done 2026-08-14 (J-13, `#[serial]` + clear)
- [x] Fix #3/#4 slash matrix — done 2026-08-14 (J-19, `COMMAND_REGISTRY` reset)
- [x] Fix #6-9 view isolation — done 2026-08-14 (J-18, `override_enabled` guards)
- [x] Re-enable `factor-gates.yml` as **non-required** PR check that runs `cargo test -p warp --lib -- --test-threads=1` with `ulimit -n 4096` and publishes `NEXT_FAILURES.md` diff (now 0 fail; full suite 0 fail outside network sandbox)

## Judgments added 2026-08-14

| ID | Decision | Risk | Location |
|---|---|---|---|
| J-13 | `secret_redaction` test clears global `SECRETS_REGEX` via `set_user_and_enterprise_secret_regexes(empty,empty)` and adds `#[serial]` | Minimal — test-local global state, mirrors other `secret_redaction` tests | `app/src/ai/blocklist/block/secret_redaction_test.rs:311` |
| J-14 | `compute_fallback_shell` no longer `expect` on `User::from_uid`; logs warn and falls back to default shell (`/bin/zsh`/`bash`/`fish`) when user DB unavailable | Low — fallback matches existing `ZSH_SHELL_PATH` default path; preserves prior `expect` success path when user exists | `app/src/terminal/local_tty/shell.rs:192` |
| J-15 | `get_pw_entry` returns `Option<Passwd>` with `log::warn` instead of `panic!`; `get_pw_shell` and `build_*_shell_command` fallback to `HOME`/`USER` env or `/tmp`/`unknown` | Low — matches sandboxed CI where DirectoryService unavailable; `test_get_pw_entry` now skips gracefully | `app/src/terminal/local_tty/unix.rs:112,124,240,753` |
| J-18 | Unit tests use `FeatureFlag::*::override_enabled(true)` RAII guards instead of `set_enabled(true)` | Low — preserves test intent while removing global flag leakage; requires the guard to outlive the test body | `app/src/terminal/view_test.rs`, `app/src/terminal/view/use_agent_footer/mod_test.rs`, `app/src/terminal/model/blocks_test.rs`, `app/src/terminal/available_shells_test.rs` |
| J-19 | `COMMAND_REGISTRY` is a resettable `Mutex<Option<Registry>>`; reset before each `App::test` window creation | Low — test-only reset path; product behavior unchanged because `App` initializes flags once | `app/src/search/slash_command_menu/static_commands/commands.rs`, `app/src/test_util/terminal.rs`, `app/src/terminal/input_test.rs` |

## Re-enable gate

During triage, CI should:

```yaml
- run: ulimit -n 4096
- run: cargo test -p warp --lib -- --test-threads=1 2>&1 | tee /tmp/full.log
- run: rg "FAILED" /tmp/full.log | wc -l   # now 0
```

Promote to required when bucket = 0.

## v1 completion sweep (2026-08-15)

- `cargo build -p warp --bin factor-oss` green with `DEVELOPER_DIR=/Applications/Xcode-beta.app/Contents/Developer` (Metal shader compile no longer skipped).
- `script/macos/run --dont-open` green: produced `target/debug/bundle/osx/Factor.app` and ad-hoc signed it.
- `factor_m5_smoke` green (packaging invariants, static denylist, SSH/warpify unit slice, agent session policy, bootstrap assets).
- `cargo test -p warp --lib -- --test-threads=1` green (`3185 passed; 0 failed; 3 ignored`) with `ulimit -n 4096` and `CARGO_INCREMENTAL=0` on a disk-constrained host.
- `cargo deny check` green after dependency refresh and advisory ignores.
- `cargo audit` green with `.cargo/audit.toml` ignore list.
- `ai::blocklist` full unit suite green (236 passed), including `start_agent` and `orchestration` view-model coverage.

### P0 live validation blockers

- **Local BYOP endpoint (partially unblocked 2026-08-15):**
  - Start the server with `fm serve --port 1976` (Apple Foundation Models CLI). `curl http://127.0.0.1:1976/health` and `/v1/models` succeed; `/v1/chat/completions` streams.
  - Factor now uses the short `system/local.j2` prompt for `system`/`pcc` OpenAI-compatible BYOP models and the local CLI runner restricts tools to `run_shell_command`/`read_files`/`start_agent` to fit on-device context windows.
  - The `apply_file_diffs` tool schema was flattened from `oneOf` to a single object with optional fields so `fm serve` no longer rejects the request (it previously returned 400 for `oneOf` in tool JSON Schema).
  - **Remaining limitation:** the on-device `system` model has a small context window and does not reliably emit native `tool_calls` (it often returns pseudo-JSON in plain text). A simple chat turn reaches the model and gets a response, but the multi-turn tool loop (`run_shell_command` → `read_files` → final) is not practical on `fm serve`'s `system` model. Use a larger local OpenAI-compatible server (e.g. `ollama`, `lmstudio`, `jan`) for the full multi-turn dogfood.
  - **PCC** is unavailable outside the Terminal app according to `fm serve`: `pcc` reports `{"available":false, "reason":"Private Cloud Compute is not available in this context. Please use the Terminal app."}`
- **OmniRoute BYOP endpoint:** `op read` cannot access the 1Password item because the 1Password CLI has no accounts configured in this shell. Unblock by configuring `op account add` / biometric integration or passing a short-lived `WARP_API_KEY` directly for the test.
- **Live SSH UJ-4:** `FACTOR_SSH_TARGET` is unset. Set it and re-run `./script/factor_ssh_agent_smoke`.

### P1 blockers / deferred

- **M2 upstream reconcile (`../warp`):** the sibling `../warp` checkout is a shallow single-commit clone (`a45466b` only), so a meaningful cherry-pick/merge review is not possible from that tree. Defer until `../warp` is a full clone or a specific upstream range is supplied.
- **Full GUI parent-child pane E2E:** the existing `start_agent` executor + `orchestration` view-model tests cover local/remote child-agent lifecycle and copy rendering. A real pane/conversation GUI integration test would require the `crates/integration` framework and a runnable macOS GUI build (blocked by missing `xcrun metal`/Xcode CLT on this host).
- **Advisory exceptions:** `deny.toml` and `.cargo/audit.toml` ignore `RUSTSEC-2026-0098/0099/0104` (AWS SDK legacy `rustls-webpki` 0.101.x) and `RUSTSEC-2026-0194/0195` (`quick-xml` 0.37.x via Windows-only `tauri-winrt-notification`). These are documented with reasons; the AWS path is blocked by `aws-smithy-http-client >=1.3.0` requiring Rust 1.94.1+.
- **Disk note:** the debug build/test artifacts consume ~16 GiB in `target/debug/deps`; maintain at least 6 GiB free for full-suite links and use `CARGO_INCREMENTAL=0` on constrained hosts.
