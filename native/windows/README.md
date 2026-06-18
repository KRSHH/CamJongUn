# Windows Native Backend

Goal: build CamJongUn-owned DirectShow camera modules without conflicting with
OBS.

Future outputs:

- `camjongun-virtualcam-module32.dll`
- `camjongun-virtualcam-module64.dll`
- `camjongun-virtualcam-module-arm64.dll`

Rules:

- Generate one CLSID per CamJongUn virtual camera.
- Use CamJongUn display names and registry entries.
- Do not reuse OBS DLL names or CLSIDs.
- Do not install into OBS plugin directories.
- Use per-camera CamJongUn shared memory queue names.

Blocked until `vendor/obs/deps/libdshowcapture/src` is populated from the OBS
submodule.
