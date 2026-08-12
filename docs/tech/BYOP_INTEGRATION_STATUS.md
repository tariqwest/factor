# BYOP integration status (M3+)

## Progress (B/C/A order)

### B — Settings UI (landed)

- [x] `settings_view/agent_providers_widget.rs` (from Zap)
- [x] `SettingsSection::AgentProviders` under Agents umbrella ("Providers")
- [x] `AISubpage::Providers` + `build_page` wiring
- [x] Full provider/models.dev `AISettingsPageAction` surface + handlers
- [x] `ModelCapabilityKind` for model detail chips

### C — Local orchestration tool (landed)

- [x] `agent_providers/tools/start_agent.rs` → `Tool::StartAgentV2` (local mode only)
- [x] Registered in `tools::REGISTRY`
- [x] Gated by `RequestParams.orchestration_enabled`
- [x] Default cargo features include `orchestration` + `orchestration_v2`

### A — Compile-fix (blocked in this environment)

- [x] Soft-skip Metal shader compile when `xcrun metal` missing (`crates/warpui/build.rs`)
- [x] Installed `protoc` via Homebrew
- [ ] Full `cargo check -p warp --bin factor-oss` — **ENOSPC** (disk full) after dep compile; no Factor source errors observed before failure

## Next machine steps

```bash
# free multi-GB first
cd factor
rm -rf target
export PATH="/opt/homebrew/bin:$PATH"  # protoc
cargo check -p warp --bin factor-oss
./script/check_factor_static_denylist all-runtime
```

Install full Xcode (or Metal tools) for a runnable macOS GUI binary.
