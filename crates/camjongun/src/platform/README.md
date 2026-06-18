# Platform Adapter Boundary

These Rust modules are the only SDK layer allowed to talk to system camera
installation/runtime behavior.

The OBS-derived source in `../../../../vendor/obs` remains vendored upstream
code. CamJongUn-specific identity changes should be generated through build
templates or narrow adapter shims, not by hand-editing upstream files.

## Current State

- Windows adapter validates expected CamJongUn DirectShow artifacts and reports
  the helper requirement.
- macOS adapter validates expected Camera Extension/DAL artifacts and reports
  the approval/helper requirement.
- Linux adapter checks for `v4l2loopback` availability and reports helper
  requirements.

Frame streaming is still intentionally not wired. The next step is adding the
native FFI/IPC transport between these Rust adapters and the OBS-derived camera
backends.
