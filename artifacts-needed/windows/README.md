# Windows Artifacts Needed

Build and ship CamJongUn-owned artifacts only:

- `camjongun-virtualcam-module32.dll`
- `camjongun-virtualcam-module64.dll`
- `camjongun-virtualcam-module-arm64.dll` when targeting Windows ARM64
- `camjongun-installer-helper.exe`
- generated CamJongUn DirectShow CLSID/header data per virtual camera

Runtime/install behavior:

- Register CamJongUn DirectShow modules with `regsvr32` or equivalent helper code.
- Unregister only CamJongUn modules during uninstall.
- Generate one unique CLSID per CamJongUn virtual camera.
- Install both 32-bit and 64-bit modules when supporting both 32-bit and 64-bit client applications.
- Never install into OBS plugin directories.
- Never reuse or unregister OBS virtual camera modules.

Source references:

- `vendor/obs/platform/windows/obs-plugin/`
- `vendor/obs/platform/windows/directshow-module/`
- `vendor/obs/platform/windows/install-scripts/`
- `crates/camjongun/src/platform/windows.rs`
