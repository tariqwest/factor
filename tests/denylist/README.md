# Denylist tests

Runtime enforcement lives in `script/check_factor_static_denylist`.

This directory holds:

- Pattern documentation
- Optional fixture files for unit-testing the script later

Banned host families (FACTOR-001): Warp app APIs, Firebase identity, Rudder/Segment, Sentry, Warp releases CDN.
