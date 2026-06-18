# Linux Artifacts Needed

Build and ship:

- `camjongun-installer-helper`
- application/library binary using the Rust CamJongUn SDK

Runtime/install behavior:

- Require or install `v4l2loopback` for the target distro.
- Ensure a `/dev/video*` output device exists for each CamJongUn-managed camera.
- Use CamJongUn camera labels and registry metadata.
- Load/configure `v4l2loopback` through the helper or package scripts, never by asking SDK users to run commands manually.
- Do not reuse OBS labels or config paths.

Source references:

- `vendor/obs/platform/linux/obs-plugin/`
- `crates/camjongun/src/platform/linux.rs`
