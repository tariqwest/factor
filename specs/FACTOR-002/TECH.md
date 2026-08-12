# FACTOR-002 TECH

## Touch points

- `AppId::new("dev", "factor", "Factor")`
- `ChannelConfig.logfile_name`
- macOS Info.plist / cargo-bundle metadata
- `warp_core` paths / project_dirs
- Secure storage service name

## Tests

- `paths_tests` assert Factor segments, not Warp/Warper.
- Grep gate: no `dev.warper` / `cn.warp` in runtime sources.
