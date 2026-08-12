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
