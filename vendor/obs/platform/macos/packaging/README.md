# macOS Packaging Notes

OBS supports macOS virtual camera through a modern Camera Extension on macOS 13+
and a legacy DAL plugin path for older supported systems.

The source here is not enough by itself. The final product must handle signing,
entitlements, bundle IDs, UUIDs, app bundle placement, user approval, and
extension activation.

`macos-helpers.cmake` is copied here as the reference for OBS bundling behavior.
