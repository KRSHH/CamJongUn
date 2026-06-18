//! Rust-first CamJongUn SDK layer.
//!
//! This crate owns the device registry, public SDK model, conflict-free
//! identities, CLI/helper orchestration, and platform adapter boundary. It does
//! not modify vendored OBS-derived source directly.

use std::env;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod platform;

pub const ID_MAX: usize = 64;
pub const NAME_MAX: usize = 128;
pub const PATH_MAX: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ResultCode {
    Ok = 0,
    NotInitialized = 1,
    AlreadyRunning = 2,
    NotRunning = 3,
    PlatformUnavailable = 4,
    InvalidArgument = 5,
    BackendError = 6,
    NotFound = 7,
    AlreadyExists = 8,
    PermissionRequired = 9,
    BufferTooSmall = 10,
    Unsupported = 11,
}

impl ResultCode {
    pub fn message(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::NotInitialized => "CamJongUn runtime is not initialized",
            Self::AlreadyRunning => "stream is already running",
            Self::NotRunning => "stream is not running",
            Self::PlatformUnavailable => "platform backend is unavailable",
            Self::InvalidArgument => "invalid argument",
            Self::BackendError => "backend error",
            Self::NotFound => "device not found",
            Self::AlreadyExists => "device already exists",
            Self::PermissionRequired => "permission or user approval is required",
            Self::BufferTooSmall => "buffer is too small",
            Self::Unsupported => "operation is unsupported",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub code: ResultCode,
    pub detail: String,
}

impl Error {
    pub fn new(code: ResultCode, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.detail.is_empty() {
            write!(f, "{}", self.code.message())
        } else {
            write!(f, "{}: {}", self.code.message(), self.detail)
        }
    }
}

impl std::error::Error for Error {}

pub type CjuResult<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PixelFormat {
    Nv12 = 0,
    Yuy2 = 1,
    Bgra = 2,
}

impl Default for PixelFormat {
    fn default() -> Self {
        Self::Nv12
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FitMode {
    Fit = 0,
    Fill = 1,
    Stretch = 2,
}

impl Default for FitMode {
    fn default() -> Self {
        Self::Fit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Rotation {
    R0 = 0,
    R90 = 90,
    R180 = 180,
    R270 = 270,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::R0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProducerPolicy {
    RejectSecond = 0,
    Takeover = 1,
}

impl Default for ProducerPolicy {
    fn default() -> Self {
        Self::RejectSecond
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VideoDesc {
    pub width: u32,
    pub height: u32,
    pub fps_num: u32,
    pub fps_den: u32,
    pub format: PixelFormat,
}

impl Default for VideoDesc {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps_num: 30,
            fps_den: 1,
            format: PixelFormat::Nv12,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId(pub [u8; ID_MAX]);

impl DeviceId {
    pub fn new_generated() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let process = std::process::id();
        let text = format!("cju-{nanos:032x}-{process:08x}");
        Self::from_str_lossy(&text)
    }

    pub fn from_str_lossy(value: &str) -> Self {
        let mut id = [0u8; ID_MAX];
        let bytes = value.as_bytes();
        let count = bytes.len().min(ID_MAX - 1);
        id[..count].copy_from_slice(&bytes[..count]);
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        let end = self.0.iter().position(|b| *b == 0).unwrap_or(ID_MAX);
        std::str::from_utf8(&self.0[..end]).unwrap_or("")
    }

    pub fn is_empty(&self) -> bool {
        self.0[0] == 0
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeOptions {
    pub app_name: String,
    pub registry_path: Option<PathBuf>,
    pub auto_install_helper: bool,
}

impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            app_name: "CamJongUn".to_string(),
            registry_path: None,
            auto_install_helper: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceCreateDesc {
    pub display_name: String,
    pub owner_app: Option<String>,
    pub preferred_video: VideoDesc,
    pub producer_policy: ProducerPolicy,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: DeviceId,
    pub display_name: String,
    pub owner_app: String,
    pub platform_identity: String,
    pub device_path: String,
    pub preferred_video: VideoDesc,
    pub producer_policy: ProducerPolicy,
    pub enabled: bool,
    pub installed: bool,
    pub visible_to_os: bool,
    pub streaming: bool,
    pub last_frame_time_ns: u64,
    pub last_error: String,
}

#[derive(Debug, Clone)]
pub struct Controls {
    pub enabled: bool,
    pub mirror: bool,
    pub flip: bool,
    pub rotation: Rotation,
    pub fit_mode: FitMode,
    pub background_rgba: u32,
    pub placeholder_frame: Option<Vec<u8>>,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            enabled: true,
            mirror: false,
            flip: false,
            rotation: Rotation::R0,
            fit_mode: FitMode::Fit,
            background_rgba: 0x000000ff,
            placeholder_frame: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Frame<'a> {
    pub planes: [&'a [u8]; 4],
    pub linesize: [u32; 4],
    pub timestamp_ns: u64,
}

pub trait PlatformBackend: Send + Sync {
    fn device_install(&self, device: &DeviceInfo) -> CjuResult<()>;
    fn device_uninstall(&self, device: &DeviceInfo) -> CjuResult<()>;
    fn stream_open(
        &self,
        device: &DeviceInfo,
        video: VideoDesc,
    ) -> CjuResult<Box<dyn PlatformStream>>;
}

pub trait PlatformStream: Send {
    fn push_frame(&mut self, frame: &Frame<'_>) -> CjuResult<()>;
    fn update_controls(&mut self, controls: &Controls) -> CjuResult<()>;
    fn close(&mut self) -> CjuResult<()>;
}

pub struct Runtime {
    app_name: String,
    registry: Registry,
    backend: Box<dyn PlatformBackend>,
}

impl Runtime {
    pub fn new(options: RuntimeOptions) -> CjuResult<Self> {
        let registry_path = options.registry_path.unwrap_or_else(default_registry_path);
        Ok(Self {
            app_name: options.app_name,
            registry: Registry::new(registry_path),
            backend: platform::default_backend(),
        })
    }

    pub fn with_backend(
        options: RuntimeOptions,
        backend: Box<dyn PlatformBackend>,
    ) -> CjuResult<Self> {
        let registry_path = options.registry_path.unwrap_or_else(default_registry_path);
        Ok(Self {
            app_name: options.app_name,
            registry: Registry::new(registry_path),
            backend,
        })
    }

    pub fn create_device(&self, desc: DeviceCreateDesc) -> CjuResult<DeviceId> {
        if desc.display_name.trim().is_empty() {
            return Err(Error::new(
                ResultCode::InvalidArgument,
                "display name is required",
            ));
        }

        let id = DeviceId::new_generated();
        let mut device = DeviceInfo {
            id,
            display_name: desc.display_name,
            owner_app: desc.owner_app.unwrap_or_else(|| self.app_name.clone()),
            platform_identity: make_platform_identity(&id),
            device_path: make_device_path(&id),
            preferred_video: desc.preferred_video,
            producer_policy: desc.producer_policy,
            enabled: true,
            installed: false,
            visible_to_os: false,
            streaming: false,
            last_frame_time_ns: 0,
            last_error: String::new(),
        };
        normalize_video(&mut device.preferred_video);
        self.registry.upsert(&device)?;
        Ok(id)
    }

    pub fn delete_device(&self, id: DeviceId) -> CjuResult<()> {
        let device = self.registry.get(id)?;
        if device.installed {
            let _ = self.uninstall_device(id);
        }
        self.registry.delete(id)
    }

    pub fn list_devices(&self) -> CjuResult<Vec<DeviceInfo>> {
        self.registry.load()
    }

    pub fn platform_report(&self) -> platform::PlatformReport {
        platform::platform_report()
    }

    pub fn get_device(&self, id: DeviceId) -> CjuResult<DeviceInfo> {
        self.registry.get(id)
    }

    pub fn install_device(&self, id: DeviceId) -> CjuResult<()> {
        let mut device = self.registry.get(id)?;
        match self.backend.device_install(&device) {
            Ok(()) => {
                device.installed = true;
                device.visible_to_os = true;
                device.last_error.clear();
                self.registry.upsert(&device)
            }
            Err(err) => {
                device.last_error = err.to_string();
                let _ = self.registry.upsert(&device);
                Err(err)
            }
        }
    }

    pub fn uninstall_device(&self, id: DeviceId) -> CjuResult<()> {
        let mut device = self.registry.get(id)?;
        match self.backend.device_uninstall(&device) {
            Ok(())
            | Err(Error {
                code: ResultCode::PlatformUnavailable,
                ..
            }) => {
                device.installed = false;
                device.visible_to_os = false;
                device.streaming = false;
                self.registry.upsert(&device)
            }
            Err(err) => {
                device.last_error = err.to_string();
                let _ = self.registry.upsert(&device);
                Err(err)
            }
        }
    }

    pub fn open_stream(&self, id: DeviceId, mut video: VideoDesc) -> CjuResult<Stream<'_>> {
        normalize_video(&mut video);
        let mut device = self.registry.get(id)?;
        if !device.enabled {
            return Err(Error::new(ResultCode::Unsupported, "device is disabled"));
        }
        if device.streaming && device.producer_policy == ProducerPolicy::RejectSecond {
            return Err(Error::new(
                ResultCode::AlreadyRunning,
                "device already has an active producer",
            ));
        }

        let platform = self.backend.stream_open(&device, video)?;
        device.streaming = true;
        device.preferred_video = video;
        device.last_error.clear();
        self.registry.upsert(&device)?;

        Ok(Stream {
            runtime: self,
            device_id: id,
            platform,
            open: true,
        })
    }
}

pub struct Stream<'a> {
    runtime: &'a Runtime,
    device_id: DeviceId,
    platform: Box<dyn PlatformStream>,
    open: bool,
}

impl Stream<'_> {
    pub fn push_frame(&mut self, frame: &Frame<'_>) -> CjuResult<()> {
        if !self.open {
            return Err(Error::new(ResultCode::NotRunning, "stream is closed"));
        }
        self.platform.push_frame(frame)?;
        let mut device = self.runtime.registry.get(self.device_id)?;
        device.last_frame_time_ns = frame.timestamp_ns;
        self.runtime.registry.upsert(&device)?;
        Ok(())
    }

    pub fn update_controls(&mut self, controls: &Controls) -> CjuResult<()> {
        if !self.open {
            return Err(Error::new(ResultCode::NotRunning, "stream is closed"));
        }
        self.platform.update_controls(controls)
    }

    pub fn close(mut self) -> CjuResult<()> {
        self.close_inner()
    }

    fn close_inner(&mut self) -> CjuResult<()> {
        if !self.open {
            return Ok(());
        }
        let result = self.platform.close();
        if result.is_ok() {
            let mut device = self.runtime.registry.get(self.device_id)?;
            device.streaming = false;
            self.runtime.registry.upsert(&device)?;
            self.open = false;
        }
        result
    }
}

impl Drop for Stream<'_> {
    fn drop(&mut self) {
        let _ = self.close_inner();
    }
}

#[derive(Debug, Clone)]
pub struct Registry {
    path: PathBuf,
}

impl Registry {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> CjuResult<Vec<DeviceInfo>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let mut text = String::new();
        fs::File::open(&self.path)
            .map_err(|err| Error::new(ResultCode::BackendError, err.to_string()))?
            .read_to_string(&mut text)
            .map_err(|err| Error::new(ResultCode::BackendError, err.to_string()))?;

        let mut devices = Vec::new();
        for line in text.lines() {
            if let Some(device) = deserialize_device(line) {
                devices.push(device);
            }
        }
        Ok(devices)
    }

    pub fn save(&self, devices: &[DeviceInfo]) -> CjuResult<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| Error::new(ResultCode::BackendError, err.to_string()))?;
        }

        let mut file = fs::File::create(&self.path)
            .map_err(|err| Error::new(ResultCode::BackendError, err.to_string()))?;
        for device in devices {
            writeln!(file, "{}", serialize_device(device))
                .map_err(|err| Error::new(ResultCode::BackendError, err.to_string()))?;
        }
        Ok(())
    }

    pub fn get(&self, id: DeviceId) -> CjuResult<DeviceInfo> {
        self.load()?
            .into_iter()
            .find(|device| device.id == id)
            .ok_or_else(|| Error::new(ResultCode::NotFound, id.to_string()))
    }

    pub fn upsert(&self, device: &DeviceInfo) -> CjuResult<()> {
        let mut devices = self.load()?;
        if let Some(existing) = devices.iter_mut().find(|item| item.id == device.id) {
            *existing = device.clone();
        } else {
            devices.push(device.clone());
        }
        self.save(&devices)
    }

    pub fn delete(&self, id: DeviceId) -> CjuResult<()> {
        let mut devices = self.load()?;
        let before = devices.len();
        devices.retain(|device| device.id != id);
        if devices.len() == before {
            return Err(Error::new(ResultCode::NotFound, id.to_string()));
        }
        self.save(&devices)
    }
}

pub fn make_platform_identity(id: &DeviceId) -> String {
    format!("camjongun.virtual-camera.{}", id)
}

pub fn make_device_path(id: &DeviceId) -> String {
    if cfg!(windows) {
        format!("directshow:{}", id)
    } else if cfg!(target_os = "macos") {
        format!("cmio:{}", id)
    } else {
        format!("v4l2loopback:{}", id)
    }
}

pub fn default_registry_path() -> PathBuf {
    if cfg!(windows) {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("CamJongUn")
            .join("devices.tsv")
    } else {
        env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state")))
            .unwrap_or_else(|| PathBuf::from("."))
            .join("camjongun")
            .join("devices.tsv")
    }
}

fn normalize_video(video: &mut VideoDesc) {
    if video.width == 0 {
        video.width = 1920;
    }
    if video.height == 0 {
        video.height = 1080;
    }
    if video.fps_num == 0 {
        video.fps_num = 30;
    }
    if video.fps_den == 0 {
        video.fps_den = 1;
    }
}

fn serialize_device(device: &DeviceInfo) -> String {
    [
        device.id.to_string(),
        escape(&device.display_name),
        escape(&device.owner_app),
        escape(&device.platform_identity),
        escape(&device.device_path),
        device.preferred_video.width.to_string(),
        device.preferred_video.height.to_string(),
        device.preferred_video.fps_num.to_string(),
        device.preferred_video.fps_den.to_string(),
        (device.preferred_video.format as i32).to_string(),
        (device.producer_policy as i32).to_string(),
        bool_field(device.enabled),
        bool_field(device.installed),
        bool_field(device.visible_to_os),
        bool_field(device.streaming),
        device.last_frame_time_ns.to_string(),
        escape(&device.last_error),
    ]
    .join("\t")
}

fn deserialize_device(line: &str) -> Option<DeviceInfo> {
    let fields: Vec<&str> = line.split('\t').collect();
    if fields.len() < 17 {
        return None;
    }
    Some(DeviceInfo {
        id: DeviceId::from_str_lossy(fields[0]),
        display_name: unescape(fields[1]),
        owner_app: unescape(fields[2]),
        platform_identity: unescape(fields[3]),
        device_path: unescape(fields[4]),
        preferred_video: VideoDesc {
            width: fields[5].parse().ok()?,
            height: fields[6].parse().ok()?,
            fps_num: fields[7].parse().ok()?,
            fps_den: fields[8].parse().ok()?,
            format: parse_pixel_format(fields[9]),
        },
        producer_policy: parse_producer_policy(fields[10]),
        enabled: fields[11] == "1",
        installed: fields[12] == "1",
        visible_to_os: fields[13] == "1",
        streaming: fields[14] == "1",
        last_frame_time_ns: fields[15].parse().ok()?,
        last_error: unescape(fields[16]),
    })
}

fn parse_pixel_format(value: &str) -> PixelFormat {
    match value {
        "1" => PixelFormat::Yuy2,
        "2" => PixelFormat::Bgra,
        _ => PixelFormat::Nv12,
    }
}

fn parse_producer_policy(value: &str) -> ProducerPolicy {
    match value {
        "1" => ProducerPolicy::Takeover,
        _ => ProducerPolicy::RejectSecond,
    }
}

fn bool_field(value: bool) -> String {
    if value {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_round_trip_preserves_identity() {
        let id = DeviceId::from_str_lossy("cju-test");
        let device = DeviceInfo {
            id,
            display_name: "Cam\tOne".to_string(),
            owner_app: "test".to_string(),
            platform_identity: make_platform_identity(&id),
            device_path: make_device_path(&id),
            preferred_video: VideoDesc::default(),
            producer_policy: ProducerPolicy::RejectSecond,
            enabled: true,
            installed: false,
            visible_to_os: false,
            streaming: false,
            last_frame_time_ns: 7,
            last_error: "none".to_string(),
        };
        let line = serialize_device(&device);
        let parsed = deserialize_device(&line).unwrap();
        assert_eq!(parsed.id, id);
        assert_eq!(parsed.display_name, "Cam\tOne");
        assert!(parsed
            .platform_identity
            .starts_with("camjongun.virtual-camera."));
    }

    #[test]
    fn default_runtime_uses_platform_report() {
        let runtime = Runtime::new(RuntimeOptions::default()).unwrap();
        let report = runtime.platform_report();
        assert!(!report.platform.is_empty());
        assert!(!report.summary.contains("OBS"));
    }

    #[test]
    fn generated_device_never_uses_obs_identity() {
        let id = DeviceId::from_str_lossy("cju-clean");
        assert!(!make_platform_identity(&id).contains("obs"));
        assert!(!make_device_path(&id).contains("obs"));
    }
}
