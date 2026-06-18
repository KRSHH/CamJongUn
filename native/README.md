# Native Backend Work

This directory is for CamJongUn-specific native build glue and thin adapter
shims.

Vendored OBS-derived source stays in `../vendor/obs`. Do not move generated
CamJongUn build glue into `vendor/obs` unless it is intentionally recorded as a
patch in `../patches`.

## Platform Targets

- `windows/`: DirectShow module generation, CLSID templating, registration
  helper integration, and per-camera shared memory queue names.
- `macos/`: Camera Extension/DAL bundle identity templating, signing-ready
  packaging, activation/deactivation helper integration.
- `linux/`: `v4l2loopback` helper integration, device mapping, and per-camera
  label management.

The Rust SDK and tests are already wired to report expected artifact names. The
native work should make those reports go from `missing` to `present`, then wire
streaming.
