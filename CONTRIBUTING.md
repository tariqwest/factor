# Contributing to Factor

1. Read `AGENTS.md`, `docs/product/PRODUCT.md`, and the relevant plan under `.agents/plans/`.
2. Prefer spec-driven work: add/update `specs/FACTOR-*` when behavior changes.
3. Use conventional commits.
4. Run `./script/presubmit` before pushing (after monorepo import).
5. Do not add Warp-hosted phone-home paths; denylist must stay green.

## Local sibling trees

Development often requires:

- `../warp` — upstream reference  
- `../warper` — strip baseline  
- `../zap` — BYOP reference  
