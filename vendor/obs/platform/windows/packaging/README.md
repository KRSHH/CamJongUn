# Windows Packaging Notes

OBS makes Windows virtual camera work by shipping an OBS-side plugin plus
DirectShow camera module DLLs.

The DirectShow module is the system-facing camera device. It must be registered
with `regsvr32`; otherwise other applications will not see the virtual camera.

Use the files in `../install-scripts/` as references for install and uninstall
behavior.
