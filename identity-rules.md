# CamJongUn Identity Rules

CamJongUn must never collide with a user's OBS installation.

## Hard Rules

- Do not install into OBS plugin directories.
- Do not use OBS DLL names for shipped CamJongUn modules.
- Do not use OBS CLSIDs, bundle IDs, Mach services, UUIDs, config filenames, or
  shared memory names.
- Do not read or write `obs-virtualcam.txt`.
- Do not unregister OBS virtual camera artifacts.
- Do not assume OBS is installed.

## Platform Identity Defaults

Windows:

- DirectShow filters use one generated CLSID per CamJongUn camera.
- Display names use `CamJongUn Virtual Camera` or a developer-provided name.
- Shared memory queues must include the CamJongUn device id.

macOS:

- Bundle IDs use a CamJongUn namespace, not `com.obsproject.*`.
- Camera Extension device/source/sink UUIDs are generated per device.
- Mach services use a CamJongUn namespace.

Linux:

- `v4l2loopback` labels use CamJongUn display names.
- Device mappings are recorded in the CamJongUn registry.

## Upstream Update Rule

Apply identity changes through templates, generated files, and adapter code
wherever possible. Avoid editing vendored OBS source directly.
