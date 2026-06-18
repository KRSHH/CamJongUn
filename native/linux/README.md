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
