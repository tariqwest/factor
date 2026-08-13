# NEXT_FAILURES — full-suite triage (P0 quality, 2026-08-13)

> **Context:** `cargo test -p warp --lib` was 243 FAIL (ulimit 256, default threads) → 127 → 97 after `AgentProviderSecrets` fix → **9 FAIL with `ulimit -n 4096 --test-threads=1`** (3175 pass, 3 ignored) → **7 FAIL (3177 pass, 3 ignored)** after `ab7d6a0` fixes `test_trims_whitespace` + `test_single_code_file`. With `--test-threads=4` + `ulimit 4096` → **10 FAIL** (3179 pass) — same 7 + 3 flaky parallel. Filtered `factor_*` 5/5 and `factor_*` + BYOP smokes still green.
> **Update 2026-08-13 (e806e08):** `test_single_code_file` fixed (oss.rs→factor_oss.rs), `test_trims_whitespace` fixed via `build_bootstrap_script` test bypass (BOOTSTRAP_CACHE MemoMap). Verified `ulimit -n 4096 --test-threads=1` = 7 fail (49s).

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
| 1 | `ai::blocklist::block::secret_redaction::test::test_detect_secrets_no_regexes_configured` | `app/src/ai/blocklist/block/secret_redaction_test.rs:315` | `left: [SecretRange 4..39] right: []` — expects no regex → no detection, but finds a range | Factor now registers `AgentProviderSecrets` + default secret regexes via `initialize_settings_for_tests` (previously no secrets). Test was written for no-regex default. | Make test explicitly clear regexes or assert `has_singleton_model` guard doesn't auto-seed. Owner: BYOP. |
| 2 | `terminal::bootstrap::tests::test_trims_whitespace` | `app/src/terminal/bootstrap_test.rs:48` | `left` = full bootstrap script, `right` = `"asdf\nno whitespace\n …"` | BOOTSTRAP_CACHE MemoMap shadowed TestAssetProvider, so `script_for_shell(Fish)` returned bash script despite `asset_for_shell_config` returning fish fixture. | **FIXED `ab7d6a0`:** extracted `build_bootstrap_script` + `cfg(test)` bypass of global cache. |
| 3 | `terminal::input::slash_command_model::tests::test_docker_sandbox_command_remains_active_when_ai_is_disabled` | `app/src/terminal/input/slash_command_model_tests.rs:188` | expects `["/docker-sandbox"]` active when AI off, got `["/rename-tab","/open-file"]` | Factor now gates `/docker-sandbox` behind `LocalDockerSandbox` flag (Warper parity, dogfood). Test was written when warper had it always active. | Align test to Factor policy: gate on `LocalDockerSandbox`, or mark `#[ignore]` with link to P-01/Warper parity. |
| 4 | `terminal::input::slash_command_model::tests::test_parse_slash_command_handles_argument_rules` | `.../slash_command_model_tests.rs:29` | `expected at least one slash command without an argument` | Test asserts some command has no arg rule; Factor added `/rename-conversation` (with arg) and changed matrix, leaving no zero-arg commands in filtered set. | Add a zero-arg slash (e.g. keep `/agent`) or update test to check Factor's `factor_*` matrix. |
| 5 | `terminal::view::open_in_warp::tests::test_single_code_file` | `app/src/terminal/view/open_in_warp_tests.rs:101` | `left: None right: Some(.../app/src/bin/oss.rs)` | Factor renamed `app/src/bin/oss.rs` → `factor_oss.rs` for branding. Test hard-codes `oss.rs`. | **FIXED `ab7d6a0`:** updated test to assert `factor_oss.rs` + guard oss.rs false. |
| 6-9 | `terminal::view::tests::test_insert` / `test_insert_into_input` / `test_reinput_blocks` / `test_viewport_iter_most_recent_at_bottom` | `app/src/terminal/view_test.rs:487,613,1813,1009` | viewport/insert assertions (`left: "One" right: "None"`, selection leftover, `item 3`) | View tests depend on exact block/model state; Factor's `AgentProviderSecrets` + `LLMPreferences` + `ChannelState::Oss` setup may pollute `App` global (seen: passes individually, fails in full suite). Order-dependent, not deterministic code bug. | Run with `--test-threads=1` + `ulimit 4096` still flaky; add `ChannelState::init()` guard in `Drop` or sequentialise. Bucket as `blocked: test-isolation` — not P0 for BYOP. |

**Plus flaky with `--test-threads=4` (3 extra, pass individually):**

- `ai::agent_providers::tests::smoke_merge_byop…` (2) — `add_singleton_model twice` before fix, now pass solo but race under parallel threads due to shared `ChannelState::set(Oss)` guard (`OssChannelGuard`). Already fixed with `has_singleton_model` guard, but parallel `ChannelState` still races. Mitigated by single-thread.

## File-descriptor bulk

`81× "Too many open files"` with default `ulimit 256` and `workspace::view` `tokio` runtime creation. Fixed by `ulimit -n 4096`. CI runners should set `ulimit -n 4096` or use `script/clean` + `--test-threads=4`. Documented in `00-overview Ops notes` + `J-07/J-08`.

## Next actions (plan 00 #1)

- [x] Fix #5 (oss.rs → factor_oss.rs) — done `ab7d6a0`
- [x] Fix #2 `test_trims_whitespace` — done `ab7d6a0` (MemoMap bypass)
- [ ] Fix #3/#4 slash matrix tests to match Factor's Warper-parity + `factor_*` matrix
- [ ] For #1, decide: keep default regex empty in `initialize_settings_for_tests` vs update test
- [ ] For #6-9, triage as `test-isolation` — run full suite with `--test-threads=1` in CI during bucket, not default parallel
- [ ] Re-enable `factor-gates.yml` as **non-required** PR check that runs `cargo test -p warp --lib -- --test-threads=1` with `ulimit -n 4096` and publishes `NEXT_FAILURES.md` diff (now 7 fail)

## Re-enable gate

During triage, CI should:

```yaml
- run: ulimit -n 4096
- run: cargo test -p warp --lib -- --test-threads=4 2>&1 | tee /tmp/full.log
- run: rg "FAILED" /tmp/full.log | wc -l   # drive to 9 → 0
```

Promote to required when bucket = 0.
