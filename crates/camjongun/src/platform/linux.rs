use super::{artifact, streaming_not_wired, PlatformReport};
use crate::{CjuResult, DeviceInfo, Error, PlatformBackend, PlatformStream, ResultCode, VideoDesc};
use std::process::Command;

#[derive(Debug, Default)]
pub struct LinuxBackend;

impl PlatformBackend for LinuxBackend {
    fn device_install(&self, device: &DeviceInfo) -> CjuResult<()> {
        install_v4l2loopback_device(device)
    }

    fn device_uninstall(&self, device: &DeviceInfo) -> CjuResult<()> {
        uninstall_v4l2loopback_device(device)
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

fn install_v4l2loopback_device(device: &DeviceInfo) -> CjuResult<()> {
    if command_success("v4l2loopback-ctl", &["--help"]) {
        let status = Command::new("pkexec")
            .arg("v4l2loopback-ctl")
            .arg("add")
            .arg("-n")
            .arg(&device.display_name)
            .arg("-x")
            .arg("1")
            .status();
        return command_result(
            status,
            &format!("created v4l2loopback device '{}'", device.display_name),
            "v4l2loopback-ctl add requires user/admin approval",
        );
    }

    if !v4l2loopback_available() {
        return Err(Error::new(
            ResultCode::PermissionRequired,
            "v4l2loopback is not available; install the distro package first, then rerun install",
        ));
    }

    let status = Command::new("pkexec")
        .arg("modprobe")
        .arg("v4l2loopback")
        .arg("exclusive_caps=1")
        .arg(format!("card_label={}", device.display_name))
        .status();
    command_result(
        status,
        &format!("loaded v4l2loopback for '{}'", device.display_name),
        "modprobe v4l2loopback requires user/admin approval",
    )
}

fn uninstall_v4l2loopback_device(device: &DeviceInfo) -> CjuResult<()> {
    if command_success("v4l2loopback-ctl", &["--help"]) {
        return Err(Error::new(
            ResultCode::Unsupported,
            format!(
                "automatic deletion needs the recorded /dev/video* path for '{}'; registry device-path mapping is the next Linux step",
                device.display_name
            ),
        ));
    }

    Err(Error::new(
        ResultCode::Unsupported,
        format!(
            "without v4l2loopback-ctl CamJongUn will not unload the whole module for '{}', because that could remove non-CamJongUn loopback cameras",
            device.display_name
        ),
    ))
}

fn command_success(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn command_result(
    status: std::io::Result<std::process::ExitStatus>,
    success_message: &str,
    permission_message: &str,
) -> CjuResult<()> {
    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(Error::new(
            ResultCode::PermissionRequired,
            format!("{permission_message}; command exited with {status}"),
        )),
        Err(err) => Err(Error::new(
            ResultCode::PermissionRequired,
            format!("{permission_message}; {err}"),
        )),
    }
    .map(|_| {
        let _ = success_message;
    })
}
