# Unit tests

Post-import, unit tests live next to Rust sources (`*_tests.rs`, `#[cfg(test)]`).

Factor-owned priority modules:

- BYOP id / provider settings
- Tool adapters
- Denylist helpers
- AppId / path isolation

This folder is reserved for non-Rust unit test helpers if needed.
