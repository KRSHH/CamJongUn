# macOS Native Backend

Goal: build CamJongUn-owned Camera Extension and optional legacy DAL bundles.

Future outputs:

- `com.camjongun.virtual-camera.systemextension`
- `camjongun-mac-virtualcam.plugin`

Rules:

- Use CamJongUn bundle IDs and Mach services.
- Generate Camera Extension device/source/sink UUIDs per camera.
- Keep signing, entitlement, and user approval behavior explicit.
- Do not install, update, or remove OBS camera bundles.

Without a paid Apple Developer Program account, CamJongUn can only provide
local/ad-hoc development artifacts and legacy DAL experiments. A public
ready-to-go macOS 13+ Camera Extension release requires Apple signing and the
required entitlements. See `NO_APPLE_DEVELOPER_ACCOUNT.md`.
