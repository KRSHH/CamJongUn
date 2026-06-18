#!/usr/bin/env sh
set -eu

root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
stage="$root/artifacts/native/linux"

rm -rf "$stage"
mkdir -p "$stage/bin" "$stage/install" "$stage/docs"

cargo build -p camjongun-installer-helper --release
cp "$root/target/release/camjongun-installer-helper" "$stage/bin/"
cp "$root/native/linux/install-v4l2loopback.sh" "$stage/install/"
cp "$root/native/linux/README.md" "$stage/docs/"
cp "$root/artifacts-needed/linux/README.md" "$stage/docs/artifacts-needed-linux.md"

cat > "$stage/README.md" <<'EOF'
# CamJongUn Linux Native Package

This package contains the Linux installer helper and v4l2loopback install
script. The actual virtual camera device is provided by the distro's
v4l2loopback kernel module.

Use CamJongUn SDK/CLI install calls where possible; the shell script is included
for package maintainers and debugging.
EOF
