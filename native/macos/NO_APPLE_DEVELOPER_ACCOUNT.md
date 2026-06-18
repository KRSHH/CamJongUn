# macOS Without Apple Developer Program

CamJongUn can build local/ad-hoc macOS artifacts, but a public ready-to-go
Camera Extension package is not possible without Apple's required signing and
entitlement flow.

What works without the paid account:

- Build the Rust SDK, CLI, FFI, and installer helper on macOS.
- Build local development bundles for inspection.
- Experiment with legacy DAL placement on machines/configurations that still
  load unsigned or ad-hoc local DAL plugins.

What does not become a clean public release without the paid account:

- A notarized/signed macOS 13+ Camera Extension that installs and activates for
  other users like OBS's release build.
- Distribution with the required Camera Extension/System Extension
  entitlements under a Developer ID.

CamJongUn release packages therefore include macOS SDK/helper artifacts and
native source/contracts on day one. The actual distributable Camera Extension
must be built by a signer that owns the required Apple entitlement.
