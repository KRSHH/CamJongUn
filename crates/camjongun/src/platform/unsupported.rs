use super::PlatformReport;
use crate::{CjuResult, DeviceInfo, Error, PlatformBackend, PlatformStream, ResultCode, VideoDesc};

#[derive(Debug, Default)]
pub struct UnsupportedBackend;

impl PlatformBackend for UnsupportedBackend {
    fn device_install(&self, _device: &DeviceInfo) -> CjuResult<()> {
        Err(Error::new(
            ResultCode::PlatformUnavailable,
            "unsupported platform",
        ))
    }

    fn device_uninstall(&self, _device: &DeviceInfo) -> CjuResult<()> {
        Err(Error::new(
            ResultCode::PlatformUnavailable,
            "unsupported platform",
        ))
    }

    fn stream_open(
        &self,
        _device: &DeviceInfo,
        _video: VideoDesc,
    ) -> CjuResult<Box<dyn PlatformStream>> {
        Err(Error::new(
            ResultCode::PlatformUnavailable,
            "unsupported platform",
        ))
    }
}

pub fn report() -> PlatformReport {
    PlatformReport {
        platform: "unsupported",
        summary: "This platform does not have a CamJongUn backend.",
        artifacts: Vec::new(),
    }
}
