# Windows Artifacts Needed

Build and ship CamJongUn-owned artifacts only:

- `camjongun-virtualcam-module32.dll`
- `camjongun-virtualcam-module64.dll`
- `camjongun-virtualcam-module-arm64.dll` when targeting Windows ARM64
- `camjongun-installer-helper.exe`
- generated CamJongUn DirectShow CLSID/header data for the app-owned camera

Runtime/install behavior:

- Developer apps call CamJongUn SDK install/uninstall APIs.
- The SDK launches `camjongun-installer-helper.exe` once with UAC.
- The helper registers/unregisters both DirectShow modules silently; developer
  apps must not spawn PowerShell or call `regsvr32` directly.
- Unregister only CamJongUn modules during uninstall.
- Use a CamJongUn-owned CLSID/name for the app-owned virtual camera.
- Install both 32-bit and 64-bit modules when supporting both 32-bit and 64-bit client applications.
- Never install into OBS plugin directories.
- Never reuse or unregister OBS virtual camera modules.

Runtime/frame behavior:

- Frame producers should use `Runtime::open_camera_stream` and `Stream::push_frame`.
- Frame delivery is owned by the Rust process and the DirectShow shared-memory
  queue, not by a browser tab or other UI timer.
- Windows v1 accepts NV12 frames. Apps that produce other formats should
  convert before calling `Stream::push_frame`.

Source references:

- `vendor/obs/platform/windows/obs-plugin/`
- `vendor/obs/platform/windows/directshow-module/`
- `vendor/obs/platform/windows/install-scripts/`
- `crates/camjongun/src/platform/windows.rs`
