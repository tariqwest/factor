# FACTOR-002: Branding & data isolation

## Summary

Factor identifies as Factor everywhere user-visible and on disk; it never reads Warp/Warper containers or keychain items at launch.

## Behaviors

1. App name Factor; About copyleft/AGPL attribution to Warp OSS as appropriate.
2. Bundle id `dev.factor.Factor`; log `factor.log`.
3. Settings/state under Factor project dirs only.
4. Keychain service Factor-only.
5. Launch does not prompt for other apps’ data due to cross-container access.
6. Optional future import from Warper/Zap is explicit user action.

## Validation

- Unit tests on AppId/path helpers.
- Manual first-launch on clean macOS user.
