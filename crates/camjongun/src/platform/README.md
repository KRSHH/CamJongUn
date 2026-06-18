# Platform Adapter Boundary

These Rust modules are the only SDK layer allowed to talk to system camera
installation/runtime behavior.

The OBS-derived source in `../../../../vendor/obs` remains vendored upstream
code. CamJongUn-specific identity changes should be generated through build
templates or narrow adapter shims, not by hand-editing upstream files.

## Current State

- Windows adapter validates expected CamJongUn DirectShow artifacts, launches one
  elevated helper for DirectShow install/uninstall, and writes frames to the
  CamJongUn shared-memory queue from Rust.
- macOS adapter validates expected Camera Extension/DAL artifacts and reports
  the approval/helper requirement.
- Linux adapter checks for `v4l2loopback` availability and reports helper
  requirements.

macOS and Linux frame streaming still need native FFI/IPC transport between
these Rust adapters and the OBS-derived camera backends.
