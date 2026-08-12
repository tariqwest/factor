# 00 — Overview & milestones

## Vision

**Factor** = Warper-level platform strip + Warp local agent/orchestration + Zap BYOP, branded and isolated.

## Milestones

### M0 — Spec & scaffold (current)

- [x] Product / tech / TDD / decisions docs
- [x] Plans directory
- [x] Repo structure stubs
- [ ] Git commits for docs + structure

### M1 — Monorepo import

- Copy Warper (or agreed base) into Factor tree
- Rename branding → Factor
- Green: `cargo check` / offline smoke stub / denylist ported

### M2 — Upstream reconcile

- Add remotes: `upstream` (warpdotdev/warp), optional `warper`, `zap`
- Port terminal + local agent fixes from latest master
- Do **not** reintroduce cloud platform
- Document residual delta

### M3 — BYOP integration

- Port Zap `agent_providers` + settings + secrets
- Seam `generate_multi_agent_output` → BYOP
- FakeProvider tests green; one real provider manual green

### M4 — Local orchestration + workflow polish

- Orchestration flags on for local paths
- start_agent tool in BYOP schema
- Error→agent and mode fluid UX checks

### M5 — Warpify/SSH agent matrix + packaging

- SSH tool context correctness
- macOS/Linux packages
- Release checklist

## Dependency graph

```text
M0 → M1 → M2 → M3 → M4 → M5
              ↘_____↗
         (M3 can start parallel after M1
          if using warper OpenRouter as interim)
```

**Judgment:** M3 may begin after M1 using Warper tree **before** full M2 if upstream reconcile is slow; must re-test after M2.

## Definition of Done (v1)

See PRODUCT success metrics. Short form:

1. Offline terminal works  
2. BYOP multi-turn agent works  
3. Local orchestration child agent works  
4. Warpify SSH agent shell works  
5. Denylist clean  
6. No Warp login required  

## Sibling repos (workspace)

| Path | Role |
|------|------|
| `../warp` | Upstream reference |
| `../warper` | Strip baseline |
| `../zap` | BYOP reference |
| `../warp-refined` | Optional local_openai/Responses ideas |
| `../warp-cn` | Not a base; localization ideas only |

## Effort rough order

| Milestone | Rough effort |
|-----------|--------------|
| M1 | 1–3 days |
| M2 | 1–3 weeks (risk high) |
| M3 | 1–2 weeks |
| M4 | 3–7 days |
| M5 | 1–2 weeks |
