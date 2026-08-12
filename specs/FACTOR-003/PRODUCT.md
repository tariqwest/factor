# FACTOR-003: BYOP inference plane

## Summary

Users configure one or more AI providers (native or OpenAI-compatible). Factor Agent uses those providers directly; keys stay local; Warp servers are not required.

## Behaviors

1. Settings → Providers: add/edit/delete provider (name, api_type, base_url, models).
2. API keys stored in OS keychain; never in plain settings.toml.
3. Model picker lists configured models; incomplete providers omitted.
4. Agent multi-turn tool use works for OpenAI-compat, Anthropic, Ollama (minimum).
5. Empty config: clear CTA, no login wall.
6. OpenRouter can be added as a provider; not mandatory.

## Validation

See TDD.md BYOP checklist + FakeProvider integration test.
