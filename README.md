# CamJongUn

CamJongUn is a staging bundle for the OBS virtual camera implementation across
Windows, macOS, and Linux.

This folder is not a complete camera driver SDK yet. It collects the source,
templates, packaging references, and dependency code OBS uses, plus a Rust-first
SDK layer that will wrap those backends without turning them into a messy fork.

## What Is Included

- `crates/camjongun`: CamJongUn's Rust SDK layer. This is the canonical
  developer-facing implementation.
- `crates/camjongunctl`: Rust CLI for create/list/delete/install/uninstall.
- `crates/camjongun-installer-helper`: Rust helper shell for future privileged
  install/uninstall work.
- `crates/camjongun-ffi`: Rust-built C ABI bridge for non-Rust consumers.
- `include/camjongun/camjongun.h`: C ABI header shipped with release packages.
- `vendor/obs/platform/windows`: OBS Windows virtual camera output and DirectShow
  camera module source.
- `vendor/obs/platform/macos`: OBS macOS virtual camera plugin, legacy DAL
  plugin, modern Camera Extension, and shared Mach protocol source.
- `vendor/obs/platform/linux`: OBS Linux V4L2 plugin source, including virtual
  camera output support.
- `vendor/obs/shared/libobs`: OBS core engine code required by the virtual
  camera outputs.
- `vendor/obs/shared/obs-shared-memory-queue`: shared memory video queue used by
  Windows.
- `vendor/obs/shared/obs-tiny-nv12-scale`: small NV12 scaling/conversion helper.
- `vendor/obs/deps/libdshowcapture`: DirectShow support dependency used on
  Windows.
- `artifacts-needed`: notes for binaries and system-facing artifacts that must
  be produced and shipped later.
- `upstream-maintenance`: rules for upgrading the OBS-derived code without
  turning CamJongUn into a messy fork.
- `patches`: ledger for any unavoidable patches to OBS-derived files.

## Important Reality Check

OBS virtual camera is not one portable backend. Each platform has its own OS
camera integration, and OBS connects them through the `libobs` output API.

All platform backends expose/register the output id:

```text
virtualcam_output
```

A future CamJongUn SDK should hide the platform setup from developers. A
developer using the library should not have to manually register DLLs, approve
extensions, or figure out V4L2 devices.

The intended architecture is:

```text
developer app -> Rust CamJongUn API -> platform adapter -> OBS-derived backend
```

Keep OBS-derived files mostly pristine under `vendor/obs/`. Put CamJongUn
behavior in Rust crates, and document unavoidable OBS patches in `patches/`.

The SDK scaffold now includes:

- a multi-camera Rust API
- a Rust C ABI crate for non-Rust callers
- a persistent CamJongUn device registry
- stream/device management entry points
- `camjongunctl` for create/list/delete/install/uninstall/doctor
- `camjongun-installer-helper` as the future privileged install bridge
- `identity-rules.md` for OBS conflict avoidance
- `PACKAGING_CONTRACT.md` for artifact layout and adapter rules
- upstream contract tests for OBS refresh safety
- GitHub Actions workflows for cross-platform Rust contracts and native artifact
  contract checks
- GitHub Actions release packaging for Rust SDK, C ABI, CLI, helper, headers,
  and docs

## Platform Shipping Requirements

Windows needs built DirectShow module DLLs and registration with `regsvr32`.
The OBS-side plugin alone is not enough for apps like browsers, Zoom, or
Discord to see a camera device.

macOS needs bundled and signed plugin/system-extension artifacts, bundle IDs,
UUIDs, entitlements, `/Applications` placement behavior, and user approval
handling.

Linux cannot be fully self-contained from this OBS source alone because the
camera device comes from the external `v4l2loopback` kernel module. Packaging
must install or require that module per distro.

## Current Status

This now builds a standalone Rust CamJongUn SDK scaffold and tools. Device
create/list/delete is implemented through the CamJongUn registry. Platform
adapters now exist and report required artifacts/privilege needs. They do not
fake OS camera registration or streaming; Windows, macOS, and Linux frame
delivery still needs to be wired behind the Rust wrapper.

## CI

When CamJongUn is used as its own repository, GitHub Actions in `.github/`
checks the Rust SDK and upstream contracts on Windows, macOS, and Linux. Native
artifact workflows currently validate contracts and document expected outputs;
they will become real native builds as the platform backends are implemented.

## Releases

Release automation lives in `.github/workflows/release.yml`. It builds and tests
the Rust workspace on Windows, macOS, and Linux, then packages the SDK, C ABI
library, CLI, installer-helper shell, headers, and docs. See `RELEASE.md` for
the push/tag flow.
