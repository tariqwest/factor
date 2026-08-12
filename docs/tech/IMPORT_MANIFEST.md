# Import manifest (M1)

| Source | SHA | Date recorded |
|--------|-----|---------------|
| warper (`../warper`) | `1c83af37c3d08d59c163f86683b7b897c8b36f1a` | 2026-08-12 |
| upstream warp (`../warp`) | `a45466be8289c8dd0a73b846e2a161ae2d273e1f` | 2026-08-12 |
| zap (`../zap`) | `5d874456a51cb7c412cfac7c21250f08ba5017f9` | 2026-08-12 |

## Import method

- `rsync` of Warper tree into `factor/` excluding `.git`, `target`, `graphify-out`
- Preserved Factor M0: `docs/`, `.agents/plans/`, `specs/FACTOR-*`, `tests/`, `README.md`, `AGENTS.md`
- Warper README archived at `docs/upstream/WARPER_README.md`
- Warper specs retained under `specs/WARPER-*`

## Branding (FACTOR-002)

| Item | Value |
|------|-------|
| AppId | `dev.factor.Factor` |
| Binary | `factor-oss` |
| Log | `factor.log` |
| Bundle name | Factor |
| URL scheme | `factor` |

## Post-import notes

- Workspace package name remains `warp` (crate name) for upstream compatibility; product identity is Factor.
- OpenRouter remains the interim inference backend until M3 BYOP lands.
