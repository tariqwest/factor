# FACTOR-001 TECH

## Implementation

1. Import Warper amputation + denylist; rename to Factor.
2. `ChannelConfig` nulls telemetry/crash/autoupdate.
3. Startup graph: no `ServerApiProvider` / AuthManager cloud init.
4. Script: `script/check_factor_static_denylist` with excludes for `docs/`, `specs/`, `.agents/`.

## Patterns to ban (non-exhaustive)

```
app.warp.dev
rtc.app.warp.dev
firebase
rudderstack
sentry.io
releases.warp.dev
identitytoolkit.googleapis.com
```

## Tests

- Denylist unit: fixture file with banned string fails; clean tree passes.
- Offline smoke script.
