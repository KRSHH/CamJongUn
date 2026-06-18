use super::{artifact, missing_artifacts_error, streaming_not_wired, PlatformReport};
use crate::{CjuResult, DeviceInfo, Error, PlatformBackend, PlatformStream, ResultCode, VideoDesc};

#[derive(Debug, Default)]
pub struct MacosBackend;

impl PlatformBackend for MacosBackend {
    fn device_install(&self, device: &DeviceInfo) -> CjuResult<()> {
        let report = report();
        if let Some(err) = missing_artifacts_error(&report) {
            return Err(err);
        }

        Err(Error::new(
            ResultCode::PermissionRequired,
            format!(
                "Camera Extension activation for '{}' requires macOS user approval",
                device.display_name
            ),
        ))
    }

    fn device_uninstall(&self, device: &DeviceInfo) -> CjuResult<()> {
        let report = report();
        if let Some(err) = missing_artifacts_error(&report) {
            return Err(err);
        }

        Err(Error::new(
            ResultCode::PermissionRequired,
            format!(
                "Camera Extension removal for '{}' requires the packaged CamJongUn helper",
                device.display_name
            ),
        ))
    }

    fn stream_open(
        &self,
        device: &DeviceInfo,
        video: VideoDesc,
    ) -> CjuResult<Box<dyn PlatformStream>> {
        streaming_not_wired(device, video)
    }
}

pub fn report() -> PlatformReport {
    PlatformReport {
        platform: "macos",
        summary: "CamJongUn must ship signed Camera Extension/DAL bundles with CamJongUn bundle IDs and UUIDs.",
        artifacts: vec![
            artifact(
                "com.camjongun.virtual-camera.systemextension",
                "macOS 13+ Camera Extension bundle",
                "macos/com.camjongun.virtual-camera.systemextension",
            ),
            artifact(
                "camjongun-mac-virtualcam.plugin",
                "legacy DAL plugin bundle for older supported macOS versions",
                "macos/camjongun-mac-virtualcam.plugin",
            ),
            artifact(
                "camjongun-installer-helper",
                "helper that activates/deactivates CamJongUn camera components only",
                "camjongun-installer-helper",
            ),
        ],
    }
}
