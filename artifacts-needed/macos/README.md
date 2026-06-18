# macOS Artifacts Needed

Build and ship CamJongUn-owned artifacts only:

- `com.camjongun.virtual-camera.systemextension`
- `camjongun-mac-virtualcam.plugin` if legacy DAL support is kept
- `camjongun-installer-helper`
- generated CamJongUn Camera Extension UUIDs per virtual camera

Runtime/install behavior:

- Use CamJongUn bundle IDs and Mach service names, not `com.obsproject.*`.
- Sign with the required entitlements and provisioning profile.
- Bundle the system extension inside the app/package.
- Handle `/Applications` placement requirements where macOS requires them.
- Handle user approval in System Settings.
- Keep legacy DAL install/update/uninstall behavior only for CamJongUn-owned bundles.
- Never remove or update OBS virtual camera bundles.

No-paid-account constraint:

- CamJongUn can ship local/ad-hoc development artifacts without the Apple
  Developer Program.
- A public ready-to-go Camera Extension equivalent to OBS release behavior still
  requires Apple-controlled signing/entitlements. There is no clean technical
  bypass for that in the OBS source tree.

Source references:

- `vendor/obs/platform/macos/obs-plugin/`
- `vendor/obs/platform/macos/dal-plugin/`
- `vendor/obs/platform/macos/camera-extension/`
- `vendor/obs/platform/macos/common/`
- `crates/camjongun/src/platform/macos.rs`
