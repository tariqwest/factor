# Factor slash-command feature guide & TDD matrix

**Status:** Canonical capability map derived from the user’s keep-list  
**Northstar:** Local-first terminal + embedded primary agent + **local** orchestration + BYOP; strip accounts/Drive/cloud Oz/telemetry.  
**Source of truth in code:** `app/src/search/slash_command_menu/static_commands/commands.rs` (`all_commands()`)

Use this document as:

1. A **product surface checklist** (what “done” for agent UX means)  
2. A **TDD target catalog** (scenario IDs → slash commands)  
3. An early-warning list for **northstar tension** (cloud coupling, architecture forks)

---

## Northstar alignment legend

| Tag | Meaning |
|-----|---------|
| **KEEP** | Aligns with Factor northstar; implement fully |
| **KEEP†** | Keep, but must use **local/BYOP** path (not Warp servers) |
| **WATCH** | Architecturally heavy or easy to re-cloud; design carefully |
| **REWORD** | Product OK; drop Oz/Warp branding, keep behavior |
| **DROP/SCOPE** | Conflicts with strip goals, or not in Factor tree as slash command |

---

## Command matrix

### A. Conversation lifecycle (core agent UX)

| Command | Capability cluster | Northstar | TDD id | Notes |
|---------|-------------------|-----------|--------|-------|
| `/agent`, `/new` | New local conversation / agent view | **KEEP** | `SC-AGENT-NEW` | Alias pair; entry to primary agent |
| `/conversations` | Local conversation history UI | **KEEP** | `SC-CONV-HIST` | Must be local SQLite/history, not cloud list |
| `/rename-conversation` | Rename active conversation | **KEEP** | `SC-CONV-RENAME` | Registered; sets active conversation display title |
| `/fork` | Branch conversation | **KEEP†** | `SC-FORK` | Local fork of message graph |
| `/fork-from` | Fork from specific query | **KEEP†** | `SC-FORK-FROM` | Flag-gated (`ForkFromCommand`) |
| `/fork-and-compact` | Fork + compact child | **KEEP†** | `SC-FORK-COMPACT` | Depends on local compaction |
| `/rewind` | Checkpoint restore | **KEEP†** | `SC-REWIND` | Needs `RevertToCheckpoints` + `RewindSlashCommand` |
| `/export-to-file` | Markdown export | **KEEP** | `SC-EXPORT-FILE` | Pure local I/O |
| `/export-to-clipboard` | Markdown export | **KEEP** | `SC-EXPORT-CLIP` | Pure local I/O |
| `/queue` | Queue prompt after agent turn | **KEEP** | `SC-QUEUE` | Flag `QueueSlashCommand`; terminal/agent coordination |

### B. Model / profile / mode

| Command | Capability cluster | Northstar | TDD id | Notes |
|---------|-------------------|-----------|--------|-------|
| `/model` | Switch base agent model | **KEEP†** | `SC-MODEL` | **Must list BYOP models** (`byop:…`), not only OpenRouter/Warp catalog |
| `/profile` | Execution profile switch | **KEEP** | `SC-PROFILE` | Permissions/autonomy; local settings |
| `/plan` | Plan-mode agent prompt | **KEEP†** | `SC-PLAN` | Local tool gating already exists (plan_mode in BYOP); no cloud plan service required |
| `/orchestrate` | Multi-agent parallel | **KEEP†** | `SC-ORCH` | **Local** `start_agent` only — not cloud Oz. Already default-on in Factor features |

### C. Context management

| Command | Capability cluster | Northstar | TDD id | Notes |
|---------|-------------------|-----------|--------|-------|
| `/compact` | Summarize history | **KEEP†** | `SC-COMPACT` | **WATCH:** must stay on **BYOP compaction** (`byop_compaction`), never Warp multi-agent summarize endpoint |
| `/compact-and` | Compact then queue prompt | **KEEP†** | `SC-COMPACT-AND` | Same as above + queue |

### D. Skills, rules, prompts, MCP

| Command | Capability cluster | Northstar | TDD id | Notes |
|---------|-------------------|-----------|--------|-------|
| `/skills` | Invoke skill | **KEEP** | `SC-SKILLS` | Local skill files + bundled skills |
| `/open-skill` | Edit skill markdown | **KEEP** | `SC-OPEN-SKILL` | Built-in editor |
| `/add-rule` | Global agent rules | **KEEP** | `SC-ADD-RULE` | Local object store / settings |
| `/open-rules` | View global+project rules | **KEEP** | `SC-OPEN-RULES` | |
| `/open-project-rules` | Open `AGENTS.md` | **KEEP** | `SC-OPEN-AGENTS-MD` | Repo file |
| `/add-prompt` | Saved prompts | **KEEP** | `SC-ADD-PROMPT` | Local |
| `/prompts` | Search saved prompts | **KEEP** | `SC-PROMPTS` | Local |
| `/add-mcp` | MCP settings entry | **KEEP** | `SC-ADD-MCP` | Local MCP config; no cloud gallery |
| `/open-mcp-servers` | MCP settings page | **KEEP** | `SC-OPEN-MCP` | |

### E. Codebase indexing / projects

| Command | Capability cluster | Northstar | TDD id | Notes |
|---------|-------------------|-----------|--------|-------|
| `/index` | Index codebase | **KEEP†** | `SC-INDEX` | **WATCH:** indexing must be **on-device** (local embeddings/FS index). Reject any path that uploads repo to Warp |
| `/init` | Index + generate `AGENTS.md` | **KEEP†** | `SC-INIT` | Same + local file write |
| `/open-repo` | Switch indexed repo | **KEEP†** | `SC-OPEN-REPO` | Local repo metadata / multi-root |
| `/create-new-project` | Guided project creation | **REWORD** | `SC-CREATE-PROJ` | Description in upstream: “Oz walkthrough”. Behavior = agent-led scaffold via **local BYOP agent**. Drop Oz branding. Flag `CreateProjectFlow` |
| `/open-code-review` | Code review panel | **KEEP** | `SC-CODE-REVIEW` | Local panel; AI content via BYOP if needed |
| `/open-file` | Open in built-in editor | **KEEP** | `SC-OPEN-FILE` | Terminal/editor substrate |

### F. Terminal chrome / settings

| Command | Capability cluster | Northstar | TDD id | Notes |
|---------|-------------------|-----------|--------|-------|
| `/rename-tab` | Tab rename | **KEEP** | `SC-RENAME-TAB` | Present in tree |
| `/set-tab-color` | Tab color | **DROP** | — | Explicitly out of Factor product surface |
| `/open-settings-file` | Open `settings.toml` | **KEEP** | `SC-SETTINGS-TOML` | Factor path (`~/.factor` / project dirs), not Warp |

---

## Architectural choices that matter (northstar tension)

These are the ones that can **pull Factor off-course** if implemented the “Warp hosted” way:

### 1. `/compact`, `/compact-and`, `/fork-and-compact` — **WATCH (critical)**

- **Right path:** Factor `byop_compaction` + BYOP model summarize (already ported modules).  
- **Wrong path:** Warp multi-agent “summarize conversation” server RPC / credits.  
- **Decision:** Compact is a **local agent turn** with a summarization system prompt, not a hosted service.

### 2. `/orchestrate` — **WATCH (critical, but desired)**

- **Right path:** Local parent/child via `start_agent` + local execution mode (Factor already registers BYOP `start_agent`).  
- **Wrong path:** Cloud Oz, remote environment IDs, capacity modals, hosted child runs.  
- **Decision:** Orchestration UI + local spawn only; never `StartAgentExecutionMode::Remote` against Warp cloud.

### 3. `/model` — **WATCH (product-critical for BYOP)**

- **Right path:** Picker includes `byop:provider:model` from Providers settings.  
- **Wrong path:** Only OpenRouter defaults or Warp server model catalog.  
- **Decision:** Model menu synthesis must call BYOP builder; OpenRouter remains one provider among many.

### 4. `/index`, `/init`, `/open-repo` — **WATCH (privacy)**

- **Right path:** Local codebase index (on-disk DB / embeddings).  
- **Wrong path:** Upload corpus to Warp for remote search (`search_codebase` hosted).  
- **Decision:** Index/search tools must resolve against local index only in Factor.

### 5. `/create-new-project` — **REWORD, not drop**

- User-facing “Oz” is branding, not a separate architecture.  
- Implement as agent workflow with `CreateNewProject` system query if present, driven by BYOP.  
- Ensure no Oz cloud handoff.

### 6. Not on your list but in tree — decide explicitly

| Command | Recommendation |
|---------|----------------|
| `/docker-sandbox` | **KEEP (flagged)** — Warper: dogfood flag; slash when on |
| `/pr-comments` | KEEP if pure GitHub CLI/local git; WATCH if Warp PR cloud |

### 7. Missing from static registry vs your list

| Your list | Factor static today | Action |
|-----------|---------------------|--------|
| `/rename-conversation` | **Added** | KEEP; renames active conversation display title |
| `/set-tab-color` | **Dropped** | Not offered as slash or context menu |

---

## Capability clusters (architecture map)

Your slash list collapses into these **subsystems** Factor must keep healthy:

```text
Agent conversation core     /agent /new /conversations /fork* /rewind /export* /queue /rename-conversation
Context economy             /compact /compact-and /fork-and-compact
Inference selection         /model  (+ Providers settings)
Execution policy            /profile
Planning & multi-agent      /plan /orchestrate
Skills & rules & prompts    /skills /open-skill /add-rule /open-rules /open-project-rules /add-prompt /prompts
MCP                         /add-mcp /open-mcp-servers
Local codebase intelligence /index /init /open-repo
Editor & review             /open-file /open-code-review
Terminal chrome             /rename-tab
Config                      /open-settings-file
Project bootstrap           /create-new-project (local agent)
```

Shell integration (warpify, blocks, SSH agent tools) is **not** slash-driven but is implied by agent tools these commands enable.

---

## TDD strategy for slash commands

### Layer 1 — Registry tests (unit, fast)

File: `app/src/search/slash_command_menu/static_commands/commands.rs` (`mod tests`, `factor_*` cases)

- Every unflagged **KEEP** command is registered (`factor_unflagged_keep_commands_are_registered`).
- Flagged KEEP commands register when required flags are forced on (`factor_flagged_keep_commands_register_when_enabled`).
- Explicit drops never register (`factor_dropped_commands_are_not_registered`, `factor_docker_sandbox_stays_off_even_if_flag_forced`).
- Names unique; `/new` aliases `/agent` (`factor_agent_and_new_are_aliases`).

### Layer 2 — Dispatch tests (unit/integration)

| TDD id | Assert |
|--------|--------|
| `SC-AGENT-NEW` | Creates local conversation id; no network |
| `SC-MODEL` | Selecting BYOP id updates LLM preferences; agent request uses that id |
| `SC-COMPACT` | Issues `SummarizeConversation` / BYOP compaction path; no `app.warp.dev` |
| `SC-ORCH` | Emits local `start_agent` tool availability when orchestration on |
| `SC-QUEUE` | Queued prompt fires after agent Done |
| `SC-FORK*` | History fork preserves local messages; BYOP repair sidecars handled |
| `SC-INDEX` / `SC-INIT` | Index writes local artifacts; `AGENTS.md` created for init |
| `SC-SETTINGS-TOML` | Opens Factor settings path, not Warp |

### Layer 3 — Denylist + offline smoke

- Startup + slash palette open + run `/agent` with network blocked.  
- `/compact` with FakeProvider only.

### Layer 4 — Manual release checklist

Extend `tests/e2e/RELEASE_CHECKLIST.md` with the full KEEP list once binary runs.

---

## Recommended Factor policy (judgment)

1. **Treat this slash list as the v1 agent product surface** (plus Providers settings + terminal substrate).  
2. **Do not** add slash commands that only exist for cloud Drive/teams/sharing.  
3. Prioritize engineering in this order (matches northstar risk):  
   1. `/model` + BYOP picker  
   2. `/compact*` local path verified  
   3. `/orchestrate` + `start_agent` E2E  
   4. `/index` `/init` local-only audit  
   5. Rest of conversation/skills/MCP/chrome  
4. `/rename-conversation` landed; `/set-tab-color` stays dropped.

---

## Open questions for you

1. **`/create-new-project`:** keep agent-guided scaffold (recommended) or drop until later?  
2. ~~`/rename-conversation`~~ **Slash added.** (`/set-tab-color` **dropped**)  
3. `/docker-sandbox`: **Warper approach** — keep code; dogfood flag.


## Explicitly dropped (Factor)

| Feature | Status |
|---------|--------|
| `/set-tab-color` + tab color context menu | Context-menu color picker returns empty; `directory_tab_colors` not in default features |

## Flag-gated (Warper parity)

| Feature | Status |
|---------|--------|
| `/docker-sandbox` + Docker session | Code retained; `LocalDockerSandbox` in DOGFOOD_FLAGS only |
