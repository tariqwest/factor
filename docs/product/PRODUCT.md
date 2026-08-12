# Factor — Product Spec

**Status:** Draft v0.1 (planning)  
**Audience:** Maintainers, agents implementing Factor  
**Last updated:** 2026-08-12

## 1. One-liner

Factor is a **local-first agentic terminal**: bash when you want bash, a primary local agent when you want intelligence, fluid movement between the two — with BYOP inference and **no** Warp account platform.

## 2. Goals

1. **Terminal excellence first.** Blocks, panes, tabs, session restoration, shell integration (warpify), SSH (including agent-over-SSH), command history, themes, keybindings, launch configs, local workflows.
2. **Primary local agent embedded in the workflow.** Same pane/block list as shell; not a side-chat product.
3. **Fluid mode transitions.**
   - Type and run normal shell commands.
   - From failed output, selected blocks, or explicit NL prompt: enter agent mode with rich context.
   - Agent runs tools (shell, read, grep, edit, skills, MCP…) on the **same** local or warpified SSH session.
   - Return to bash without losing session state.
4. **Local multi-agent orchestration.** Parent agent may spawn child agent conversations / parallel local work via local orchestration primitives (`start_agent`, and later local `run_agents` if ported). **Not** cloud Oz handoff or hosted runners.
5. **BYOP inference.** User-configured providers and keys (OpenAI, Anthropic, Gemini, DeepSeek, Ollama, OpenAI-compatible gateways, OpenRouter as one option). Keys stay on device. No mandatory routing through Warp servers.
6. **Hard platform amputation.** Accounts, billing, Drive, teams, session sharing, cloud agents, hosted telemetry/crash/autoupdate are **absent**, not merely hidden.
7. **Third-party CLI agents.** Detect and lightly enhance Claude Code, Codex, Gemini CLI, etc. (status, optional rich input, finish notifications) without wrapping them as Factor’s primary agent.
8. **Privacy default.** Offline-capable terminal; network only for user-chosen providers and explicit user actions (e.g. user-run `curl`).

## 3. Non-goals

- Replacing Warp’s commercial product or remaining protocol-compatible with Warp accounts.
- Hosted multi-tenant agent runtime (Zap roadmap phase 2/3 style cloud harness) in v1.
- Full i18n completeness in v1 (English first; Fluent-style i18n may land later from Zap patterns).
- Preserving Warp Drive / team collaboration / shared sessions.
- Computer-use that depends on cloud policy servers (local computer-use only if it works offline with BYOP — deferred decision).

## 4. Keep / strip matrix

### 4.1 Keep (terminal substrate)

| Capability | Rationale |
|------------|-----------|
| Blocks & block list | Core UX identity |
| Local sessions / panes / tabs | Core UX |
| Shell bootstrap + preexec/precmd hooks | Block boundaries, warpify triggers |
| Warpify (local + SSH) | Agent and smart terminal features over remote shells |
| SSH session support | Agent must operate on remote hosts |
| Command history, palette, themes, keybindings | Daily driver |
| Launch configs, local workflows | Power user |
| Inline / next-command suggestions | Fluid assistance (BYOP-backed when model-powered) |
| CLI agent session plugins | Coexist with primary agent |

### 4.2 Keep (local agent / orchestration)

| Capability | Rationale | Explicitly NOT |
|------------|-----------|----------------|
| Agent Mode as primary input mode | Embedded workflow | Side panel only product |
| Blocklist AI controller + tool executors | Real agent loop | Toy command-only AI |
| `start_agent` local child agents | Local orchestration | Cloud task IDs / hosted children |
| Orchestration UI for local parent/child | User visibility | Oz launch / cloud capacity modals |
| Skills, MCP (local), documents tools | Agent power | Cloud-synced MCP gallery if it phones home |
| Conversation persistence **local** | Continuity | Cloud conversation sync |

### 4.3 Strip (Warper-hard)

| Surface | Notes |
|---------|-------|
| Login / signup / SSO / paste token / reauth | No auth product |
| Anonymous Firebase / cloud user | No silent cloud identity |
| Billing / credits / upgrade / PAYG | No entitlements |
| Warp Drive / cloud objects / shared-with-me | Deleted |
| Teams / ACLs / enterprise admin | Deleted |
| Session sharing / RTC / remote control | Deleted |
| Cloud mode / cloud Oz / handoff local↔cloud / cloud runners | Deleted |
| Hosted ambient agent polling / capacity modals | Deleted |
| RudderStack / Segment / Sentry upload / UGC telemetry upload | Local logs only |
| Warp-hosted autoupdate endpoints | Optional later: self-hosted GH releases only |

### 4.4 Replace / integrate

| Warper default | Factor target |
|----------------|---------------|
| OpenRouter-only multi-agent API | Zap-style multi-provider BYOP (`agent_providers`) |
| OpenRouter as sole LLM source | BYOP + OpenRouter as one provider |
| Dogfood orchestration flags | Orchestration **on** for local paths by default |

## 5. User journeys (acceptance-level)

### UJ-1 Fresh install offline

1. User installs Factor, blocks all outbound network.
2. App opens to a usable terminal; no login wall.
3. User runs shell commands, splits panes, changes theme.
4. Agent features that need a provider show a **local** settings prompt, not a Warp login.

### UJ-2 BYOP agent shell loop

1. User adds OpenAI-compatible (or native) provider + key + model in Settings.
2. User switches to Agent Mode (or NL detection) and asks: “list rust files larger than 1k and summarize the biggest.”
3. Agent uses tools (glob/grep/read/shell) in the **current** session.
4. User presses a clear control to return to bash; CWD and shell state preserved.

### UJ-3 Error → agent → bash

1. User runs a failing command in bash.
2. From the failed block (or selection), user invokes “Ask Factor” / agent with that block as context.
3. Agent proposes and (per permissions) runs a fix command.
4. User continues in bash.

### UJ-4 Agent over SSH (warpify)

1. User SSHs to a host; session is warpified (or Factor prompts to warpify).
2. Agent Mode tools that execute shell run **on the remote** host, not only locally.
3. File tools respect remote context rules defined in TECH.md (flag any gaps).

### UJ-5 Local orchestration

1. User (or parent agent) starts a child agent on a subtask via local orchestration.
2. Child appears as a local conversation/pane; no cloud run id requirement.
3. Parent can observe completion via local events only.

### UJ-6 Third-party CLI agent

1. User types `claude` / `codex` / `gemini` in a shell block.
2. Factor shows CLI-agent status affordances; does not force Factor Agent Mode.
3. Finish notification works without Warp account.

### UJ-7 Denylist / no phone-home

1. With network allowlist empty except user provider, Factor does not contact `*.warp.dev`, Firebase, Rudder, Sentry, or update endpoints during startup or normal shell use.
2. Static denylist CI fails the build if those strings reappear in runtime paths.

## 6. Permissions model (v1 defaults)

| Action class | Default |
|--------------|---------|
| Read-only tools (read/grep/glob/search) | Auto-allow in agent mode |
| Shell commands | Respect execution profile; confirm on high-risk patterns (best-effort classifier; document false positives) |
| File edits / apply diffs | Confirm unless user set auto-approve |
| MCP tools | Per-server trust |
| Start child agent | Allow when orchestration enabled |
| Network from **agent** tools | Only via shell/user config; no silent Factor backend |

**Open:** exact risk classifier source (upstream vs custom). See DECISIONS.md D-07.

## 7. Branding & identity (planned)

| Item | Value | Notes |
|------|-------|-------|
| Product name | Factor | User-facing |
| Binary / default-run | `factor` / `factor-oss` | Align with warper `warp-oss` pattern |
| macOS bundle id | `dev.factor.Factor` | Judgment; confirm before notarization |
| Log file | `factor.log` | |
| Settings path | `~/.factor/settings.toml` (or XDG) | Avoid clobbering `~/.warp` / warper paths |
| Keychain service | `dev.factor.Factor` | Separate from Warp/Warper |
| URL scheme | `factor://` | Optional v1 |

## 8. Platforms (v1)

- **macOS** (Apple Silicon + Intel): primary
- **Linux**: primary
- **Windows**: best-effort after macOS/Linux green (upstream supports it; strip work is higher risk)

## 9. Success metrics (engineering, not growth)

- Offline terminal smoke green
- Denylist green on `all-runtime`
- BYOP multi-turn tool loop green on at least: OpenAI-compat, Anthropic, Ollama
- Warpify SSH agent shell tool green on Linux remote
- Zero required network hosts other than user providers for UJ-2

## 10. Spec slices (FACTOR-*)

| ID | Title | Priority |
|----|-------|----------|
| FACTOR-001 | Hard network / platform amputation | P0 |
| FACTOR-002 | Branding & data isolation | P0 |
| FACTOR-003 | BYOP inference plane | P0 |
| FACTOR-004 | Primary agent terminal workflow | P0 |
| FACTOR-005 | Local orchestration | P1 |
| FACTOR-006 | Warpify + SSH agent tools | P0 |

Detailed PRODUCT/TECH pairs live under `specs/FACTOR-*/`.

## 11. Naming / Trek easter eggs (optional, low priority)

Not product-blocking. Candidates for later UI copy / codenames:

- Settings section “Nacelle” for power/provider config  
- “Deflector” for privacy/denylist  
- “Impulse” for offline-only mode  
- Release trains: “Warp Factor 1…9” versioning joke  

Keep UI professional; easter eggs secondary.
