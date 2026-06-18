#!/usr/bin/env sh
set -eu

label="${1:-CamJongUn Virtual Camera}"

if command -v v4l2loopback-ctl >/dev/null 2>&1; then
  exec pkexec v4l2loopback-ctl add -n "$label" -x 1
fi

if ! modinfo v4l2loopback >/dev/null 2>&1; then
  echo "v4l2loopback is not installed. Install the distro package first." >&2
  exit 9
fi

exec pkexec modprobe v4l2loopback exclusive_caps=1 "card_label=$label"
