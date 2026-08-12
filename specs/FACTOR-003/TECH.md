# FACTOR-003 TECH

## Port from Zap

- `app/src/ai/agent_providers/**`
- `byop_readiness`, `byop_compaction` (as needed)
- Settings types + AI settings page sections

## Seam

`generate_multi_agent_output` → BYOP chat_stream for `byop:*` model ids.

## Tests

- llm_id roundtrip
- tool adapters
- FakeProvider multi-turn
- denylist unchanged
