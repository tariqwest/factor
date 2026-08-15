# Factor — agent guide

This repository is **Factor**, a local-first agentic terminal (see README).

## Before coding

1. Read `docs/product/PRODUCT.md` and `docs/tech/TECH.md`.
2. Read the active plan under `.agents/plans/` for your milestone.
3. Check `docs/adr/DECISIONS.md` for defaults and open questions.
4. For a feature slice, read `specs/FACTOR-*/PRODUCT.md` + `TECH.md`.

## Rules

- Conventional commits only.
- Factor-owned code: TDD per `docs/tech/TDD.md`.
- Never reintroduce denylisted hosted endpoints without an ADR.
- Cloud Oz handoff is out of scope; local agent + local orchestration are in scope.
- BYOP is the inference plane (Zap-derived); do not hard-depend on Warp servers for agent turns.
- Prefer small PRs aligned to plan waves.

## Monorepo status

Until M1 import, application source lives in sibling trees (`../warper`, `../zap`, `../warp`). Do not assume `app/` and `crates/` are complete yet.

## Useful paths

| Path | Why |
|------|-----|
| `.agents/plans/` | Execution plans |
| `specs/` | Spec-driven slices |
| `tests/` | Test catalogs / future suites |
| `script/` | Gates and smokes (stubs pre-import) |

## Common commands

- `cargo check -p warp` — fast compile check.
- `cargo build -p warp --bin factor-oss` — build the `factor-oss` CLI.
- `cargo test -p warp --lib fake_provider` — run FakeProvider unit tests (good BYOP controller sanity check).
- Local runner: `target/debug/factor-oss agent run --runner local --model byop:<provider>:<model> --prompt "..."`.
- Log: `tail -50 ~/Library/Logs/warp-cli/factor.log`.

## macOS build notes

The active `xcode-select` path is the bare CommandLineTools, so `xcrun metal` is not in the default lookup path. An Xcode beta is installed at `/Applications/Xcode-beta.app` and contains the Metal toolchain.

To compile Factor with Metal shader support (needed for a runnable GUI `.app` bundle), point `xcrun` at the beta Xcode by setting `DEVELOPER_DIR` before any `cargo` command:

```bash
export DEVELOPER_DIR=/Applications/Xcode-beta.app/Contents/Developer
cargo build -p warp --bin factor-oss
```

Without this variable, `crates/warpui/build.rs` prints:

```text
Factor: `xcrun metal` not found; skipping Metal shader compile.
```

The resulting CLI binary still builds, but the GUI will not render correctly because the bundled `.metallib` will be empty.

## Manual BYOP credentials

For live OmniRoute dogfood, retrieve the token from 1Password instead of the macOS keychain:

```bash
op read "op://Private/factor-omniroute-key/credential"
```

Then run `factor-oss` with:

```bash
WARP_API_KEY="$(op read 'op://Private/factor-omniroute-key/credential')" \
    target/debug/factor-oss agent run --runner local --model byop:omniroute:<model> --prompt "..."
```

Do not write the token value to disk or to this file.
