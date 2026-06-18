# Implementation Status

## Done

- Rust workspace, SDK crate, C ABI crate, CLI, and installer helper.
- Single app-owned virtual camera registry.
- Camera naming/editing, install/uninstall, delete, and stream entry points.
- Windows DirectShow helper flow: one elevated CamJongUn helper registers both
  64-bit and 32-bit modules.
- Windows DirectShow frame delivery through the CamJongUn shared-memory queue.
- Cross-platform platform-report contracts for Windows, macOS, Linux, and
  unsupported targets.
- GitHub Actions for Rust checks, native artifact packaging, and releases.
- Vendored OBS-derived source staged under `vendor/obs` with upstream-refresh
  contract tests.

## Not Done Yet

- macOS Camera Extension/DAL activation and frame transport behind the Rust
  adapter.
- Linux `/dev/video*` mapping persistence and frame transport behind the Rust
  adapter.
- Public macOS Camera Extension distribution without Apple Developer Program
  signing. Local/ad-hoc artifacts are possible; public ready-to-go distribution
  is still gated by Apple entitlements.
- Windows ARM64 DirectShow packaging.

## Verified

```sh
cargo test
```

Windows native package generation has also been validated locally with x64 and
x86 DirectShow modules.
