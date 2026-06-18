# Linux Packaging Notes

OBS does not ship a universal Linux camera driver in this source tree. The Linux
virtual camera path depends on `v4l2loopback`.

The product package should either install `v4l2loopback` for the target distro
or clearly fail with instructions when it is missing.

The OBS code attempts to load it with `pkexec modprobe` and then writes frames to
the available `/dev/video*` output device.
