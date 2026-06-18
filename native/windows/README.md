# Windows Native Backend

Goal: build CamJongUn-owned DirectShow camera modules without conflicting with
OBS.

Outputs:

- `camjongun-virtualcam-module32.dll`
- `camjongun-virtualcam-module64.dll`
- `camjongun-virtualcam-module-arm64.dll`
- `camjongun-installer-helper.exe`

Rules:

- Generate one CLSID per CamJongUn virtual camera.
- Use CamJongUn display names and registry entries.
- Do not reuse OBS DLL names or CLSIDs.
- Do not install into OBS plugin directories.
- Use CamJongUn shared memory queue names.
- Hide DirectShow registration behind the SDK/helper flow so the user sees one
  CamJongUn elevation request instead of repeated PowerShell or registry-server
  prompts.

Blocked until `vendor/obs/deps/libdshowcapture/src` is populated from the OBS
submodule.
