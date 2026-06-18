# CamJongUn

CamJongUn is a staging bundle for the OBS virtual camera implementation across
Windows, macOS, and Linux.

This folder is not a complete camera driver SDK yet. It collects the source,
templates, packaging references, and dependency code OBS uses, plus a Rust-first
SDK layer that will wrap those backends without turning them into a messy fork.

## What Is Included

- `crates/camjongun`: CamJongUn's Rust SDK layer. This is the canonical
  developer-facing implementation.
- `crates/camjongunctl`: Rust CLI for show/ensure/rename/delete/install/uninstall.
- `crates/camjongun-installer-helper`: Rust privileged helper used by the SDK to
  register/unregister CamJongUn-owned native artifacts.
- `crates/camjongun-ffi`: Rust-built C ABI bridge for non-Rust consumers.
- `include/camjongun/camjongun.h`: C ABI header shipped with release packages.
- `bindings`: package-ready wrappers for C/C++, Python, Node.js, .NET, Go, and
  Java/JNA built on top of the C ABI.
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

CamJongUn should hide platform setup from developers. A developer using the
library should not have to manually register DLLs, approve extensions, or figure
out V4L2 devices.

The intended architecture is:

```text
developer app -> Rust CamJongUn API -> platform adapter -> OBS-derived backend
```

Keep OBS-derived files mostly pristine under `vendor/obs/`. Put CamJongUn
behavior in Rust crates, and document unavoidable OBS patches in `patches/`.

The SDK scaffold now includes:

- a single app-owned virtual camera Rust API
- a Rust C ABI crate for non-Rust callers
- thin language bindings for the common native/desktop app ecosystems
- a persistent CamJongUn device registry
- camera naming/editing, install/uninstall, and stream management entry points
- `camjongunctl` for show/ensure/rename/delete/install/uninstall/doctor
- `camjongun-installer-helper` as the privileged install bridge
- `identity-rules.md` for OBS conflict avoidance
- `PACKAGING_CONTRACT.md` for artifact layout and adapter rules
- upstream contract tests for OBS refresh safety
- GitHub Actions workflows for cross-platform Rust contracts and native artifact
  contract checks
- GitHub Actions release packaging for Rust SDK, C ABI, CLI, helper, headers,
  language bindings, and docs

## Platform Shipping Requirements

Windows needs built DirectShow module DLLs. Developer apps should call the SDK
camera install API; the SDK launches the CamJongUn helper once with UAC and the
helper silently registers both 64-bit and 32-bit modules. Developer apps should
not spawn PowerShell, call `regsvr32`, or copy DLLs manually.

macOS needs bundled and signed plugin/system-extension artifacts, bundle IDs,
UUIDs, entitlements, `/Applications` placement behavior, and user approval
handling.

Linux cannot be fully self-contained from this OBS source alone because the
camera device comes from the external `v4l2loopback` kernel module. Packaging
must install or require that module per distro.

## Current Status

This now builds a standalone Rust CamJongUn SDK scaffold and tools. The runtime
owns one virtual camera per developer app/owner. Repeated create/ensure calls
update that app camera instead of accumulating OS camera devices. Windows
install/uninstall and DirectShow shared-memory frame delivery are wired through
the Rust adapter and packaged helper. macOS and Linux still expose artifact and
privilege contracts while their frame delivery remains backend work.

## CI

When CamJongUn is used as its own repository, GitHub Actions in `.github/`
checks the Rust SDK and upstream contracts on Windows, macOS, and Linux. Native
artifact workflows build/package platform outputs where possible:

- Windows: DirectShow module build scaffold, helper, and package output.
- Linux: helper plus `v4l2loopback` install support package.
- macOS: helper, native source/support package, and no-paid-account constraints.

macOS public Camera Extension distribution still requires Apple signing and the
required entitlements. Without that, CamJongUn can only ship local/ad-hoc
development artifacts for macOS.

## Releases

Release automation lives in `.github/workflows/release.yml`. It builds and tests
the Rust workspace on Windows, macOS, and Linux, then packages the SDK, C ABI
library, CLI, installer-helper shell, headers, and docs. See `RELEASE.md` for
the push/tag flow.
