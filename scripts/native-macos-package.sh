#!/usr/bin/env sh
set -eu

root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
stage="$root/artifacts/native/macos"

rm -rf "$stage"
mkdir -p "$stage/bin" "$stage/docs" "$stage/source"

cargo build -p camjongun-installer-helper --release
cp "$root/target/release/camjongun-installer-helper" "$stage/bin/"
cp "$root/native/macos/README.md" "$stage/docs/"
cp "$root/native/macos/NO_APPLE_DEVELOPER_ACCOUNT.md" "$stage/docs/"
cp "$root/artifacts-needed/macos/README.md" "$stage/docs/artifacts-needed-macos.md"

cp -R "$root/vendor/obs/platform/macos/camera-extension" "$stage/source/"
cp -R "$root/vendor/obs/platform/macos/dal-plugin" "$stage/source/"
cp -R "$root/vendor/obs/platform/macos/obs-plugin" "$stage/source/"
cp -R "$root/vendor/obs/platform/macos/common" "$stage/source/"

cat > "$stage/README.md" <<'EOF'
# CamJongUn macOS Native Package

This package contains the macOS helper, native source, and no-paid-account
constraints. Public ready-to-go Camera Extension distribution requires Apple
signing/entitlements owned by the distributor.
EOF
