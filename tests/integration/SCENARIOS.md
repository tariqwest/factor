# Integration scenario catalog

Implemented post-import via `crates/integration` and/or dedicated tests.

| ID | Scenario | Milestone |
|----|----------|-----------|
| IT-001 | Launch logged-out; type in shell | M1 |
| IT-002 | Agent mode without provider → CTA | M3 |
| IT-003 | FakeProvider multi-turn tool loop | M3 |
| IT-004 | Failed block → agent context | M4 |
| IT-005 | Local start_agent child conversation | M4/M5 |
| IT-006 | Local warpify block boundaries | M1/M6 |
| IT-007 | CLI agent detection fixture | M4 |
| IT-008 | Denylist: no warp.dev on startup network | M1 |

Each scenario should map to an automated test name once the harness lands.
