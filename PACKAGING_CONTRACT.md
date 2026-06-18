# CamJongUn Packaging Contract

CamJongUn packaging must make virtual cameras work without requiring developers
to manually copy DLLs, bundles, or kernel-module scripts.

This file describes the artifact layout expected by the Rust platform adapters.
Set `CAMJONGUN_ARTIFACT_DIR` to override the default artifact root. If the
variable is not set, adapters look next to the running executable.

## Root Layout

```text
artifact-root/
  camjongunctl(.exe)
  camjongun-installer-helper(.exe)
  windows/
    camjongun-virtualcam-module32.dll
    camjongun-virtualcam-module64.dll
  macos/
    com.camjongun.virtual-camera.systemextension/
    camjongun-mac-virtualcam.plugin/
```

Linux does not ship a universal camera driver artifact from this source tree.
It relies on `v4l2loopback` being installed or installed by package scripts.

## Conflict-Avoidance Requirements

- Never install into OBS plugin directories.
- Never use OBS DLL names, CLSIDs, bundle IDs, Mach services, UUIDs, shared
  memory names, config filenames, or Linux labels.
- Never unregister OBS artifacts.
- Never read or write `obs-virtualcam.txt`.
- Never depend on OBS being installed.

## Adapter Boundary

Rust adapters may:

- validate CamJongUn artifact presence
- call packaged CamJongUn helpers
- generate CamJongUn identities
- pass frames to OBS-derived backends through narrow FFI/IPC boundaries

Rust adapters must not:

- patch vendored OBS source at runtime
- rewrite OBS configuration
- silently claim installation success when OS registration failed
- fall back to OBS-installed components
