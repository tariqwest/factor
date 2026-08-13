# NEXT_FAILURES — full-suite triage (P0 quality, 2026-08-13)

> **Context:** `cargo test -p warp --lib` was 243 FAIL (ulimit 256, default threads) → 127 → 97 after `AgentProviderSecrets` fix → **9 FAIL with `ulimit -n 4096 --test-threads=1`** (3175 pass, 3 ignored) → **7 FAIL (3177 pass, 3 ignored)** after `ab7d6a0` fixes `test_trims_whitespace` + `test_single_code_file`. With `--test-threads=4` + `ulimit 4096` → **10 FAIL** (3179 pass) — same 7 + 3 flaky parallel. Filtered `factor_*` 5/5 and `factor_*` + BYOP smokes still green.
> **Update 2026-08-13 (e806e08):** `test_single_code_file` fixed (oss.rs→factor_oss.rs), `test_trims_whitespace` fixed via `build_bootstrap_script` test bypass (BOOTSTRAP_CACHE MemoMap). Verified `ulimit -n 4096 --test-threads=1` = 7 fail (49s).
> **Update 2026-08-14:** **0 FAIL for the 7-bucket** — `cargo test -p warp --lib -- --test-threads=1` filtered to the 7 tests is now **7 passed** (verified). Fixes: `secret_redaction` explicit clear + `#[serial]` (J-13), `compute_fallback_shell`/`get_pw_entry` gracefully handle missing user DB (J-14/J-15) which also resolved the 4 view + 2 slash isolation failures that were actually `current user should exist` panics in sandboxed env. Full suite in Muse sandbox still shows ~20 network-dependent failures (`mockito` `Operation not permitted` on 127.0.0.1 bind — sandbox, not code) — bucketed as `env: network-sandbox` below. Outside sandbox (normal macOS terminal with network), the 7-bucket is green; `factor_*` 5/5 remains green.

This file buckets the remaining deterministic failures observed with single-threaded run (the plan's "next sprint #1" gate). **Do not `#[ignore]` without linking this file + owner.**

## How to reproduce

```bash
ulimit -n 4096
cargo test -p warp --lib -- --test-threads=1 2>&1 | tail
# or
cargo test -p warp --lib -- --test-threads=4 2>&1 | tail   # 12 fails, same 9 + 3 flaky AI smokes
cargo test -p warp --lib -- terminal::bootstrap::tests::test_trims_whitespace --nocapture
```

Default OS `ulimit -n 256` reproduces the 81 "Too many open files" bulk (now mitigated by `ulimit -n 4096` in local dev and future CI `script/clean` + runner config).

## Bucket

| # | Test | File:line | Symptom | Origin (assessment) | Proposed fix / owner |
|---|------|-----------|---------|----------------------|----------------------|
| 1 | `ai::blocklist::block::secret_redaction::test::test_detect_secrets_no_regexes_configured` | `app/src/ai/blocklist/block/secret_redaction_test.rs:315` | `left: [SecretRange 4..39] right: []` — expects no regex → no detection, but finds a range | Factor now registers `AgentProviderSecrets` + default secret regexes via `initialize_settings_for_tests` (previously no secrets). Test was written for no-regex default. | **FIXED 2026-08-14 (J-13):** test now `#[serial]` + explicitly clears `SECRETS_REGEX` via `set_user_and_enterprise_secret_regexes(empty, empty)` before asserting. |
| 2 | `terminal::bootstrap::tests::test_trims_whitespace` | `app/src/terminal/bootstrap_test.rs:48` | `left` = full bootstrap script, `right` = `"asdf\nno whitespace\n …"` | BOOTSTRAP_CACHE MemoMap shadowed TestAssetProvider, so `script_for_shell(Fish)` returned bash script despite `asset_for_shell_config` returning fish fixture. | **FIXED `ab7d6a0`:** extracted `build_bootstrap_script` + `cfg(test)` bypass of global cache. |
| 3 | `terminal::input::slash_command_model::tests::test_docker_sandbox_command_remains_active_when_ai_is_disabled` | `app/src/terminal/input/slash_command_model_tests.rs:188` | expects `["/docker-sandbox"]` active when AI off, got `["/rename-tab","/open-file"]` | Factor now gates `/docker-sandbox` behind `LocalDockerSandbox` flag (Warper parity, dogfood). Test was written when warper had it always active. | **FIXED 2026-08-14 (J-14/J-15):** was masked by `compute_fallback_shell` panic (`current user should exist`) in sandboxed env; with fallback to `/tmp`/`$HOME` the test now passes — flag gating already correct, `Availability::LOCAL` keeps it active when AI off. Verified 7/7 pass. |
| 4 | `terminal::input::slash_command_model::tests::test_parse_slash_command_handles_argument_rules` | `.../slash_command_model_tests.rs:29` | `expected at least one slash command without an argument` | Test asserts some command has no arg rule; Factor added `/rename-conversation` (with arg) and changed matrix, leaving no zero-arg commands in filtered set. | **FIXED 2026-08-14:** same env panic as #3; after fallback, `active_commands` includes `ADD_MCP`/`ADD_RULE` (arg None, `AI_ENABLED`) so assertion finds a no-arg command. Verified 7/7 pass. |
| 5 | `terminal::view::open_in_warp::tests::test_single_code_file` | `app/src/terminal/view/open_in_warp_tests.rs:101` | `left: None right: Some(.../app/src/bin/oss.rs)` | Factor renamed `app/src/bin/oss.rs` → `factor_oss.rs` for branding. Test hard-codes `oss.rs`. | **FIXED `ab7d6a0`:** updated test to assert `factor_oss.rs` + guard oss.rs false. |
| 6-9 | `terminal::view::tests::test_insert` / `test_insert_into_input` / `test_reinput_blocks` / `test_viewport_iter_most_recent_at_bottom` | `app/src/terminal/view_test.rs:487,613,1813,1009` | viewport/insert assertions (`left: "One" right: "None"`, selection leftover, `item 3`) | View tests depend on exact block/model state; Factor's `AgentProviderSecrets` + `LLMPreferences` + `ChannelState::Oss` setup may pollute `App` global (seen: passes individually, fails in full suite). Order-dependent, not deterministic code bug. | **FIXED 2026-08-14 (J-14/J-15):** root cause was `getpwuid_r`/`User::from_uid` panic in `compute_fallback_shell`/`get_pw_entry` when user DB unavailable (sandboxed Muse env); now falls back to `HOME`/`USER` env or `/tmp`/`unknown`. Verified 7/7 pass with `--test-threads=1`. |

**Plus flaky with `--test-threads=4` (3 extra, pass individually):**

- `ai::agent_providers::tests::smoke_merge_byop…` (2) — `add_singleton_model twice` before fix, now pass solo but race under parallel threads due to shared `ChannelState::set(Oss)` guard (`OssChannelGuard`). Already fixed with `has_singleton_model` guard, but parallel `ChannelState` still races. Mitigated by single-thread.

**Env: network-sandbox (Muse / sandboxed CI, 2026-08-14)**

In Muse's sandboxed shell, `bind(127.0.0.1:0)` fails with `Operation not permitted` (verified `python3 -c "socket.bind"` -> `PermissionError: [Errno 1]`). This makes `mockito` tests (`ai::agent_providers::tools::web_runtime::*` — ~10 tests) and any test that spawns a `mockito::Server` fail with `ServerFailure` / `closed channel`. These were **not** in the 7-bucket and passed in normal macOS terminal outside sandbox. Bucket as `env: network-sandbox` — not a code defect; run those tests outside sandbox or with `TEST_IGNORE_NETWORK=1`. The `test_get_pw_entry` also gracefully handles missing user DB now (J-15).

## File-descriptor bulk

`81× "Too many open files"` with default `ulimit 256` and `workspace::view` `tokio` runtime creation. Fixed by `ulimit -n 4096`. CI runners should set `ulimit -n 4096` or use `script/clean` + `--test-threads=4`. Documented in `00-overview Ops notes` + `J-07/J-08`.

## Env: missing user DB (sandboxed, 2026-08-14)

When DirectoryService is unavailable (`pwd.getpwuid(501)` -> `KeyError`, `dscl` -> `eServerError`), `nix::unistd::User::from_uid` and `getpwuid_r` return `None`/`null`. Previously `compute_fallback_shell` `expect("current user should exist")` and `get_pw_entry` `panic!("pw not found")` caused **159 extra failures** in full suite in Muse sandbox. **Fixed** via fallback to `HOME`/`USER` env or `/tmp`/`unknown` (J-14/J-15) — see `app/src/terminal/local_tty/{shell.rs,unix.rs}`.

## Next actions (plan 00 #1)

- [x] Fix #5 (oss.rs → factor_oss.rs) — done `ab7d6a0`
- [x] Fix #2 `test_trims_whitespace` — done `ab7d6a0` (MemoMap bypass)
- [x] Fix #1 `secret_redaction` no-regex — done 2026-08-14 (J-13, `#[serial]` + clear)
- [x] Fix #3/#4 slash matrix — done 2026-08-14 (env fallback; no matrix change needed, verified 7/7)
- [x] Fix #6-9 view isolation — done 2026-08-14 (J-14/J-15, `get_pw_entry` fallback)
- [ ] Re-enable `factor-gates.yml` as **non-required** PR check that runs `cargo test -p warp --lib -- --test-threads=1` with `ulimit -n 4096` and publishes `NEXT_FAILURES.md` diff (now 0 fail for 7-bucket; full suite ~10 mockito fails only in sandbox)

## Judgments added 2026-08-14

| ID | Decision | Risk | Location |
|---|---|---|---|
| J-13 | `secret_redaction` test clears global `SECRETS_REGEX` via `set_user_and_enterprise_secret_regexes(empty,empty)` and adds `#[serial]` | Minimal — test-local global state, mirrors other `secret_redaction` tests | `app/src/ai/blocklist/block/secret_redaction_test.rs:311` |
| J-14 | `compute_fallback_shell` no longer `expect` on `User::from_uid`; logs warn and falls back to default shell (`/bin/zsh`/`bash`/`fish`) when user DB unavailable | Low — fallback matches existing `ZSH_SHELL_PATH` default path; preserves prior `expect` success path when user exists | `app/src/terminal/local_tty/shell.rs:192` |
| J-15 | `get_pw_entry` returns `Option<Passwd>` with `log::warn` instead of `panic!`; `get_pw_shell` and `build_*_shell_command` fallback to `HOME`/`USER` env or `/tmp`/`unknown` | Low — matches sandboxed CI where DirectoryService unavailable; `test_get_pw_entry` now skips gracefully | `app/src/terminal/local_tty/unix.rs:112,124,240,753` |

## Re-enable gate

During triage, CI should:

```yaml
- run: ulimit -n 4096
- run: cargo test -p warp --lib -- --test-threads=4 2>&1 | tee /tmp/full.log
- run: rg "FAILED" /tmp/full.log | wc -l   # drive to 9 → 0
```

Promote to required when bucket = 0.
