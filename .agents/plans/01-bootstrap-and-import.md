# 01 — Bootstrap & monorepo import

## Goal

Turn this docs-first repo into a buildable Factor monorepo based on Warper, with Factor branding and working denylist stubs replaced by real checks.

## Preconditions

- [ ] PRODUCT/TECH reviewed (or proceed per DECISIONS defaults)
- [ ] `../warper` checked out and buildable on maintainer machine (optional but ideal)
- [ ] Q-01 remote still optional

## Steps

### 1. Snapshot Warper

```bash
# from warp-oss/
# ensure warper clean
cd warper && git status && git rev-parse HEAD
```

Record Warper SHA in `docs/tech/IMPORT_MANIFEST.md`.

### 2. Import strategy (D-12)

**Preferred for clarity:**

```bash
# From factor/ after M0 commits
rsync -a --delete \
  --exclude '.git' \
  --exclude 'graphify-out' \
  --exclude 'target' \
  ../warper/ ./
# restore factor docs/plans that rsync might not delete if we use careful excludes
```

Better approach — **layered import**:

1. Keep Factor `docs/`, `.agents/`, `specs/`, `tests/` planning tree.
2. rsync warper **source** (`app/`, `crates/`, `script/`, `Cargo.*`, licenses, etc.) into factor.
3. Resolve collisions (README: keep Factor README; move warper README to `docs/upstream/WARPER_README.md`).

### 3. Branding pass (FACTOR-002)

Search-replace carefully (not blind):

| Warper | Factor |
|--------|--------|
| `Warper` | `Factor` |
| `warper` binary names | `factor` |
| `dev.warper.Warper` | `dev.factor.Factor` |
| `warper.log` | `factor.log` |
| `~/.warp-oss` if used | `~/.factor` |

Files of interest:

- `app/src/bin/oss.rs` → `factor.rs` / `factor_oss.rs`
- `app/Cargo.toml` package names / bundle metadata
- `script/macos/*`, `script/windows/*`
- About strings, menu labels

### 4. Port denylist script

- Copy `script/check_warper_static_denylist` → `script/check_factor_static_denylist`
- Rename WARPER-001 → FACTOR-001 messages
- Extend patterns if Factor docs mention warp.dev in non-runtime paths (exclude `docs/`, `specs/`, `.agents/`)

### 5. First build

```bash
./script/bootstrap
./script/run
./script/check_factor_static_denylist startup
```

### 6. Commit series (conventional)

```text
chore: import warper monorepo as factor baseline
feat: rebrand warper identity to factor
test: port static denylist as factor gate
docs: record import manifest and warper SHA
```

## Exit criteria

- [ ] `cargo check -p warp` (or factor package name) succeeds on macOS or Linux
- [ ] App launches with Factor name
- [ ] Denylist startup scope green
- [ ] Factor docs/plans still present and accurate

## Risks

- Blind replace breaks upstream test strings that intentionally say “Warp”
- Asset/LFS files — use `GIT_LFS_SKIP_SMUDGE=1` patterns as needed
- Bundle signing identities still Warper — fix before release
