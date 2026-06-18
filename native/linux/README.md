# Linux Native Backend

Goal: manage CamJongUn-owned `v4l2loopback` camera devices through the helper
and registry.

Rules:

- Require or install `v4l2loopback` through packaging/helper flow.
- Use CamJongUn camera labels.
- Record `/dev/video*` mappings in the CamJongUn registry.
- Do not reuse OBS labels or config paths.

Linux does not ship a universal camera driver from this source tree; the system
kernel module provides the actual virtual camera device.

Day-one behavior:

- `camjongun-installer-helper install <device-id>` calls the Linux backend.
- The backend uses `v4l2loopback-ctl add -n <name> -x 1` through `pkexec` when
  available.
- Otherwise it falls back to `pkexec modprobe v4l2loopback exclusive_caps=1
  card_label=<name>`.
- Automatic per-device uninstall requires storing the exact `/dev/video*`
  mapping; until that mapping is implemented, CamJongUn refuses to unload the
  whole module because that could remove cameras it does not own.
