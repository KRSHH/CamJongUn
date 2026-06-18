# Implementation Status

## Done

- Rust workspace in `Cargo.toml`.
- Rust SDK crate in `crates/camjongun`.
- Persistent Rust device registry with create/list/get/delete/upsert behavior.
- Rust stream/device functions wired to platform backend traits.
- Rust C ABI crate in `crates/camjongun-ffi`.
- Rust CLI in `crates/camjongunctl`.
- Rust installer helper shell in `crates/camjongun-installer-helper`.
- CLI commands: `list`, `create`, `delete`, `install`, `uninstall`, `doctor`.
- Target-selected Rust platform adapter modules for Windows, macOS, Linux, and
  unsupported platforms.
- Platform artifact reports used by `camjongunctl doctor`.
- Packaging contract for clean artifact layout and OBS conflict avoidance.
- OBS conflict-avoidance identity rules.
- Upstream contract integration tests covering vendored OBS layout, upstream
  marker drift, CamJongUn artifact names, registry compatibility, and platform
  reports.
- Smoke-check scripts in `scripts/check.ps1` and `scripts/check.sh`.
- GitHub Actions workflows for Windows/macOS/Linux Rust contract checks and
  native artifact contract checks.
- GitHub Actions release workflow for Rust SDK/FFI/CLI/helper/header packages.
- GitHub Actions native package jobs for Windows, Linux, and macOS support
  artifacts.
- C ABI header in `include/camjongun/camjongun.h`.
- Local package and GitHub publish helper scripts.

## Not Done Yet

- Windows DirectShow module generation/registration per camera. A standalone
  native build scaffold now exists for CamJongUn-named DirectShow DLLs.
- Windows per-camera shared memory queue wiring.
- macOS Camera Extension/DAL identity templating and activation. The adapter now
  knows the CamJongUn bundle artifact names and reports missing packages cleanly.
- Linux exact `/dev/video*` mapping persistence and safe per-device uninstall.
  Install now attempts `v4l2loopback-ctl add` or privileged `modprobe`.
- Real frame delivery into the OBS-derived platform backends through Rust
  adapters.
- Privilege elevation UX for the installer helper.
- macOS public Camera Extension distribution without an Apple Developer Program
  signing/entitlement setup. Local/ad-hoc support packages are produced; public
  ready-to-go Camera Extension artifacts remain gated by Apple signing.
- `vendor/obs/deps/libdshowcapture/src` is currently empty because the source
  OBS checkout has the `deps/libdshowcapture/src` submodule uninitialized.
  Windows backend builds need that submodule populated before native DirectShow
  work can build.

## Verified

Built and tested with:

```sh
cargo build
cargo test
cargo run -p camjongunctl -- doctor
```

Smoke-tested:

```sh
cargo run -p camjongunctl -- create RustCam
cargo run -p camjongunctl -- list
cargo run -p camjongunctl -- install <device-id>
cargo run -p camjongunctl -- doctor
```

For a quick local guardrail pass, run:

```sh
./scripts/check.sh
```

or on Windows PowerShell:

```powershell
.\scripts\check.ps1
```

The install command currently returns a clean error such as
`CJU_RESULT_BACKEND_ERROR`, `CJU_RESULT_PERMISSION_REQUIRED`, or
`CJU_RESULT_PLATFORM_UNAVAILABLE` depending on platform artifact state. It must
not report success until native camera registration is actually wired.
