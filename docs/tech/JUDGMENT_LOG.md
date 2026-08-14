# Autonomous judgment log

Decisions made without waiting for maintainer, for later review.  
Updated: 2026-08-14 — 7-bucket 0, H5 minimal threading, gates re-enabled, test-isolation root causes corrected.

| ID | Judgment | Why | Want your input if… |
|----|----------|-----|---------------------|
| J-01 | Commit parallel subagent work even if a few tests are soft | Stabilize tree; disk/CI risk from uncommitted sprawl | You want to review diffs before commit |
| J-02 | Specialized model UI = three text fields, not live dropdowns | Matches existing Free-form model id settings; dropdown needs LLMPreferences+rebuild wiring | You want picker-fed dropdowns in v1 |
| J-03 | Manual BYOP dogfood = scripts + automated unit matrix only when no keys/GUI | Metal missing; no paid keys in env | You provide Ollama running or API keys |
| J-04 | Orchestration "E2E" = unit/integration of tool + executor + SessionContext, not full GUI | Controller UI needs Metal app | You dogfood parent→child in app |
| J-05 | SSH matrix offline-first; skip live when `FACTOR_SSH_TARGET` unset | CI/offline northstar | You set a target for live probe |
| J-06 | M2 = document residual + judgment, **not** full upstream rebase this pass | Rebase is multi-day risk; denylist + Warper base still valid | You want full M2 port campaign next |
| J-07 | Do not install full Xcode/Metal autonomously | Needs user consent / disk (~5–6 Gi free, 98% full) | You free disk + install Xcode CLT Metal |
| J-08 | Prefer prune `target` only if ENOSPC blocks compile | Destructive of build cache | Disk still tight after commits |
| J-09 | OpenRouter stays "legacy" labeled; no code path removal | Q-02 | BYOP onboarding feels solid enough to demote further |
| J-10 | File tools remote = keep D-08 fail-closed unit coverage | Product decision already recorded | You change remote FS semantics |
| J-11 | Bootstrap global MemoMap bypass in `cfg(test)` via `build_bootstrap_script` | BOOTSTRAP_CACHE shadowed TestAssetProvider, breaking `test_trims_whitespace` deterministically | You want caching preserved in tests (trade determinism) |
| J-12 | H5 LRC remote bind = policy landed, executor threading deferred (now J-16) | ShellCommandExecutor lacked SessionContext; needs is_remote+lrc_attached threading | You fund executor SessionContext plumbing now |
| J-13 | `secret_redaction` test clears global `SECRETS_REGEX` via `set_user_and_enterprise_secret_regexes(empty,empty)` + `#[serial]` | Factor seeds defaults via `initialize_settings_for_tests`; test expects no regex | You want default regex empty in test harness |
| J-14 | `compute_fallback_shell` fallback when `User::from_uid` None/Err (DirectoryService unavailable) | `pwd.getpwuid` fails in sandboxed Muse env (`eServerError`) → 159 extra panics; fallback to default shell preserves success path | You want hard panic on missing user |
| J-15 | `get_pw_entry` returns `Option<Passwd>` + log warn, `build_*_shell_command` fallback to HOME/USER env | Same sandboxed missing user DB; `test_get_pw_entry` now skips gracefully | You want panic on pw not found |
| J-16 | H5 minimal threading: `ShellCommandExecutor::execute` threads `SessionContext` + `shell_execution_target` + remote-LRC guard (log + CancelledBeforeExecution) — full `lrc_attached(params)` routing still TODO | Prevents second local shell that would miss remote TUI without risky full RequestParams plumbing; low-risk log/guard | You fund full `lrc_attached` + is_remote → LRC-tool routing now |
| J-17 | Re-enable `factor-gates.yml` (was `.skip`) as non-required PR check with `ulimit -n 4096 --test-threads=1` + NEXT_FAILURES publish | 7-bucket now 0; full suite ~10 mockito fails only in network-sandbox (Operation not permitted) bucketed separately | You want it required or full-suite green outside sandbox |
| J-18 | Replace `FeatureFlag::*::set_enabled(true)` in unit tests with `FeatureFlag::*::override_enabled(true)` RAII guards | `set_enabled` mutates process-global `FLAG_STATES`; because `warp_features` is compiled as a lib when running `app` tests, the `cfg!(test)` panic guard is inactive and flag state leaks across tests | You want a different per-test feature-flag cleanup strategy |
| J-19 | Make `COMMAND_REGISTRY` a lazily-initialized, test-resettable `Mutex<Option<Registry>>` wrapper; reset it in `add_window_with_*` helpers | `COMMAND_REGISTRY` was a `std::sync::LazyLock<Registry>` built once from `all_commands()`, which depends on `FeatureFlag` state at first access; slash tests with feature overrides were order-dependent | You want `COMMAND_REGISTRY` to reflect runtime feature changes without test-only reset |

## Suggested next human actions

1. Free ≥15 Gi disk; install Metal tools if you want GUI
2. Start Ollama + add provider in Factor when GUI works
3. Set `FACTOR_SSH_TARGET` for live SSH smoke
4. Confirm J-02 (text fields vs dropdowns) and J-06 (defer M2 rebase)
