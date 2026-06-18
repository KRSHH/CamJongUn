use super::{artifact, streaming_not_wired, PlatformReport};
use crate::{CjuResult, DeviceInfo, Error, PlatformBackend, PlatformStream, ResultCode, VideoDesc};
use std::process::Command;

#[derive(Debug, Default)]
pub struct LinuxBackend;

impl PlatformBackend for LinuxBackend {
    fn device_install(&self, device: &DeviceInfo) -> CjuResult<()> {
        if !v4l2loopback_available() {
            return Err(Error::new(
                ResultCode::PermissionRequired,
                "v4l2loopback is not available; install the distro package or let the installer helper request elevation",
            ));
        }

        Err(Error::new(
            ResultCode::PermissionRequired,
            format!(
                "loading/configuring v4l2loopback for '{}' requires the packaged installer helper",
                device.display_name
            ),
        ))
    }

    fn device_uninstall(&self, device: &DeviceInfo) -> CjuResult<()> {
        Err(Error::new(
            ResultCode::PermissionRequired,
            format!(
                "removing CamJongUn-managed v4l2loopback device '{}' requires the installer helper",
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
        platform: "linux",
        summary: "CamJongUn uses v4l2loopback devices labeled and tracked through the CamJongUn registry.",
        artifacts: vec![artifact(
            "camjongun-installer-helper",
            "helper that loads/configures v4l2loopback for CamJongUn-managed devices",
            "camjongun-installer-helper",
        )],
    }
}

fn v4l2loopback_available() -> bool {
    Command::new("modinfo")
        .arg("v4l2loopback")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
