use crate::{CjuResult, DeviceInfo, Error, PlatformBackend, PlatformStream, ResultCode, VideoDesc};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ArtifactStatus {
    Present(PathBuf),
    Missing(PathBuf),
    NotApplicable,
}

#[derive(Debug, Clone)]
pub struct PlatformArtifact {
    pub name: &'static str,
    pub purpose: &'static str,
    pub status: ArtifactStatus,
}

#[derive(Debug, Clone)]
pub struct PlatformReport {
    pub platform: &'static str,
    pub summary: &'static str,
    pub artifacts: Vec<PlatformArtifact>,
}

pub fn default_backend() -> Box<dyn PlatformBackend> {
    #[cfg(target_os = "windows")]
    {
        return Box::<windows::WindowsBackend>::default();
    }

    #[cfg(target_os = "macos")]
    {
        return Box::<macos::MacosBackend>::default();
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return Box::<linux::LinuxBackend>::default();
    }

    #[allow(unreachable_code)]
    Box::<unsupported::UnsupportedBackend>::default()
}

pub fn platform_report() -> PlatformReport {
    #[cfg(target_os = "windows")]
    {
        return windows::report();
    }

    #[cfg(target_os = "macos")]
    {
        return macos::report();
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return linux::report();
    }

    #[allow(unreachable_code)]
    unsupported::report()
}

fn artifact_dir() -> PathBuf {
    std::env::var_os("CAMJONGUN_ARTIFACT_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::current_exe()
                .ok()
                .and_then(|path| path.parent().map(PathBuf::from))
        })
        .unwrap_or_else(|| PathBuf::from("."))
}

fn artifact(name: &'static str, purpose: &'static str, relative_path: &str) -> PlatformArtifact {
    let path = artifact_dir().join(relative_path);
    let status = if path.exists() {
        ArtifactStatus::Present(path)
    } else {
        ArtifactStatus::Missing(path)
    };
    PlatformArtifact {
        name,
        purpose,
        status,
    }
}

#[allow(dead_code)]
fn missing_artifacts_error(report: &PlatformReport) -> Option<Error> {
    let missing: Vec<&str> = report
        .artifacts
        .iter()
        .filter_map(|artifact| match artifact.status {
            ArtifactStatus::Missing(_) => Some(artifact.name),
            _ => None,
        })
        .collect();

    if missing.is_empty() {
        None
    } else {
        Some(Error::new(
            ResultCode::BackendError,
            format!(
                "missing CamJongUn platform artifacts: {}",
                missing.join(", ")
            ),
        ))
    }
}

#[allow(dead_code)]
fn streaming_not_wired(
    _device: &DeviceInfo,
    _video: VideoDesc,
) -> CjuResult<Box<dyn PlatformStream>> {
    Err(Error::new(
        ResultCode::PlatformUnavailable,
        "platform install artifacts can be validated, but frame streaming is not wired yet",
    ))
}

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;

mod unsupported;
