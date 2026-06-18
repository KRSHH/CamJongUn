use super::{artifact, missing_artifacts_error, streaming_not_wired, PlatformReport};
use crate::{CjuResult, DeviceInfo, Error, PlatformBackend, PlatformStream, ResultCode, VideoDesc};
use std::process::Command;

#[derive(Debug, Default)]
pub struct WindowsBackend;

impl PlatformBackend for WindowsBackend {
    fn device_install(&self, device: &DeviceInfo) -> CjuResult<()> {
        let report = report();
        if let Some(err) = missing_artifacts_error(&report) {
            return Err(err);
        }

        Err(Error::new(
            ResultCode::PermissionRequired,
            format!(
                "DirectShow registration for '{}' requires the packaged CamJongUn module installer helper",
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
                "DirectShow unregistration for '{}' requires the packaged CamJongUn module installer helper",
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
        platform: "windows",
        summary: "CamJongUn must ship 32-bit and 64-bit DirectShow modules with CamJongUn CLSIDs and names.",
        artifacts: vec![
            artifact(
                "camjongun-virtualcam-module64.dll",
                "64-bit DirectShow virtual camera filter",
                "windows/camjongun-virtualcam-module64.dll",
            ),
            artifact(
                "camjongun-virtualcam-module32.dll",
                "32-bit DirectShow virtual camera filter for 32-bit clients",
                "windows/camjongun-virtualcam-module32.dll",
            ),
            artifact(
                "camjongun-installer-helper.exe",
                "privileged helper that registers/unregisters CamJongUn filters only",
                "camjongun-installer-helper.exe",
            ),
        ],
    }
}

#[allow(dead_code)]
fn run_regsvr32(module_path: &std::path::Path, unregister: bool) -> CjuResult<()> {
    let mut command = Command::new("regsvr32.exe");
    command.arg("/s");
    if unregister {
        command.arg("/u");
    } else {
        command.arg("/i");
    }
    command.arg(module_path);

    let status = command
        .status()
        .map_err(|err| Error::new(ResultCode::BackendError, err.to_string()))?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::new(
            ResultCode::BackendError,
            format!("regsvr32 failed for {}", module_path.display()),
        ))
    }
}
