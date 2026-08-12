# Open for your review (decision queue)

Autonomous work log notes **assumptions made** and **items needing your input**.  
Updated: 2026-08-12 (autonomous session).

## Decisions made without you (reasonable defaults)

| ID | Assumption | Why |
|----|------------|-----|
| A-01 | `/model` selection priority is terminal override → last-used → profile → OpenRouter legacy → default | BYOP northstar; previous OpenRouter-first would shadow BYOP picks |
| A-02 | Persist last-used model id in `agents.byop.last_used_model_id` on picker change | Zap parity; new tabs/restarts should stick |
| A-03 | `LocalDockerSandbox` hard-disabled except **test overrides** | Product dropped; parking lot P-01; residual code may still be tested |
| A-04 | `start_agent` always local execution mode | No cloud Oz / remote runners in Factor |
| A-05 | Secrets empty secure-storage is not an error log | Avoids false-positive ERROR spam in tests/fresh install |
| A-06 | Factor CI gate is a **lightweight** workflow (`factor-gates.yml`), not a replacement for full Warp CI | Disk/time; denylist + owned unit tests first |
| A-07 | Docker sandbox **code** not fully deleted yet | Scope/risk; full removal parked in PRODUCT §11.1 |

## Needs your input (non-blocking for continued work)

| ID | Question | Options / recommendation |
|----|----------|---------------------------|
| Q-01 | Unpark full docker-sandbox **code deletion** soon, or leave dead code behind flag forever? | **Rec:** inventory + delete after M5 packaging, not mid-BYOP |
| Q-02 | Keep Warper OpenRouter single-key path as permanent fallback next to multi-provider BYOP? | **Rec:** keep until BYOP onboarding UX is solid; then deprecate UI |
| Q-03 | Persist per-terminal **reasoning effort** (Zap map) in v1? | **Rec:** yes P1 after picker hydrate is trusted |
| Q-04 | `/create-new-project` keep agent-guided scaffold? | SLASH_COMMANDS open Q |
| Q-05 | `/rename-conversation` slash vs UI-only? | SLASH_COMMANDS open Q |
| Q-06 | Title / ActiveAI / next-command: separate model pickers or always = base? | **Rec:** always base for v1 (current) |
| Q-07 | Prefer micro-VM vs Seatbelt vs bubblewrap vs BYO when unparking isolation? | Parking lot; no pick yet |
| Q-08 | Factor settings path `~/.factor` vs share `~/.warp` for import convenience? | PRODUCT branding; **rec:** stay `~/.factor` |
| Q-09 | CI: should `factor-gates` block merge on this fork’s default branch only? | Adjust branches when remote is known |

## Progress this autonomous pass

- [x] Denylist green (startup + all-runtime)
- [x] Fix `/model` active-model priority for BYOP
- [x] Persist + hydrate last-used model id
- [x] Hard-disable Docker sandbox feature flag (P-01 partial)
- [x] `start_agent` local-only unit tests
- [x] Secrets empty-load log noise fix
- [x] `factor-gates.yml` CI sketch
- [ ] Full `factor-oss` GUI binary / Metal (env-limited)
- [x] FakeProvider multi-turn unit/tool-loop test (`api/fake_provider`)
- [ ] Full controller FakeProvider integration (IT-003) — optional next
- [ ] Offline smoke full run (may need long compile)

## Suggested next (when you’re back)

1. Confirm A-01…A-07 + answer Q-01/Q-02 if easy  
2. Unblock Metal/Xcode if you want a runnable GUI tonight  
3. FakeProvider tool-loop test (TDD plan 04)  
4. Optional: begin docker-sandbox file inventory for deletion (no behavior change)
