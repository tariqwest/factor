# Autonomous judgment log

Decisions made without waiting for maintainer, for later review.  
Updated: 2026-08-13 — triage `AgentProviderSecrets` + fd limit, 243→9.

| ID | Judgment | Why | Want your input if… |
|----|----------|-----|---------------------|
| J-01 | Commit parallel subagent work even if a few tests are soft | Stabilize tree; disk/CI risk from uncommitted sprawl | You want to review diffs before commit |
| J-02 | Specialized model UI = three text fields, not live dropdowns | Matches existing Free-form model id settings; dropdown needs LLMPreferences+rebuild wiring | You want picker-fed dropdowns in v1 |
| J-03 | Manual BYOP dogfood = scripts + automated unit matrix only when no keys/GUI | Metal missing; no paid keys in env | You provide Ollama running or API keys |
| J-04 | Orchestration “E2E” = unit/integration of tool + executor + SessionContext, not full GUI | Controller UI needs Metal app | You dogfood parent→child in app |
| J-05 | SSH matrix offline-first; skip live when `FACTOR_SSH_TARGET` unset | CI/offline northstar | You set a target for live probe |
| J-06 | M2 = document residual + judgment, **not** full upstream rebase this pass | Rebase is multi-day risk; denylist + Warper base still valid | You want full M2 port campaign next |
| J-07 | Do not install full Xcode/Metal autonomously | Needs user consent / disk (~5–6 Gi free, 98% full) | You free disk + install Xcode CLT Metal |
| J-08 | Prefer prune `target` only if ENOSPC blocks compile | Destructive of build cache | Disk still tight after commits |
| J-09 | OpenRouter stays “legacy” labeled; no code path removal | Q-02 | BYOP onboarding feels solid enough to demote further |
| J-10 | File tools remote = keep D-08 fail-closed unit coverage | Product decision already recorded | You change remote FS semantics |

## Suggested next human actions

1. Free ≥15 Gi disk; install Metal tools if you want GUI  
2. Start Ollama + add provider in Factor when GUI works  
3. Set `FACTOR_SSH_TARGET` for live SSH smoke  
4. Confirm J-02 (text fields vs dropdowns) and J-06 (defer M2 rebase)  
