# Factor execution plans

Ordered plans for building Factor from specs to shippable terminal.

| # | Plan | Status |
|---|------|--------|
| 00 | [Overview & milestones](00-overview.md) | Active |
| 01 | [Bootstrap & monorepo import](01-bootstrap-and-import.md) | Next |
| 02 | [Reconcile Warper ↔ upstream Warp](02-reconcile-warper-upstream.md) | Planned |
| 03 | [Integrate Zap BYOP](03-integrate-zap-byop.md) | Planned |
| 04 | [TDD & quality gates](04-tdd-and-quality-gates.md) | Planned |
| 05 | [Local orchestration enablement](05-local-orchestration.md) | Planned |
| 06 | [Warpify / SSH agent matrix](06-warpify-ssh-agent.md) | Planned |

Related:

- [docs/product/PRODUCT.md](../../docs/product/PRODUCT.md)
- [docs/tech/TECH.md](../../docs/tech/TECH.md)
- [docs/tech/TDD.md](../../docs/tech/TDD.md)
- [docs/adr/DECISIONS.md](../../docs/adr/DECISIONS.md)
- [specs/FACTOR-*](../../specs/)

## Agent operating rules

1. Read PRODUCT + TECH before coding a slice.
2. Prefer TDD for Factor-owned modules (TDD.md).
3. Never reintroduce denylisted endpoints without an ADR.
4. Record judgment calls in DECISIONS.md when PRODUCT is silent.
5. Conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `chore:`, `refactor:`.
