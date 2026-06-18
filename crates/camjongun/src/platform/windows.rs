use super::{artifact, PlatformReport};
use crate::{
    CjuResult, Controls, DeviceInfo, Error, Frame, PixelFormat, PlatformBackend, PlatformStream,
    ResultCode, VideoDesc,
};
use std::ffi::c_void;
use std::path::{Path, PathBuf};

const QUEUE_NAME: &str = "CamJongUnVirtualCamVideo";
const FRAME_HEADER_SIZE: usize = 32;
const PAGE_READWRITE: u32 = 0x04;
const FILE_MAP_ALL_ACCESS: u32 = 0x000F001F;
const ERROR_ALREADY_EXISTS: u32 = 183;
const SHARED_QUEUE_STATE_STARTING: u32 = 1;
const SHARED_QUEUE_STATE_READY: u32 = 2;
const SHARED_QUEUE_STATE_STOPPING: u32 = 3;

type Handle = *mut c_void;

#[repr(C)]
struct QueueHeader {
    write_idx: u32,
    read_idx: u32,
    state: u32,
    offsets: [u32; 3],
    queue_type: u32,
    cx: u32,
    cy: u32,
    interval: u64,
    reserved: [u32; 8],
}

#[repr(C)]
struct ShellExecuteInfoW {
    cb_size: u32,
    f_mask: u32,
    hwnd: Handle,
    lp_verb: *const u16,
    lp_file: *const u16,
    lp_parameters: *const u16,
    lp_directory: *const u16,
    n_show: i32,
    h_inst_app: Handle,
    lp_id_list: Handle,
    lp_class: *const u16,
    hkey_class: Handle,
    dw_hot_key: u32,
    h_icon_or_monitor: Handle,
    h_process: Handle,
}

#[link(name = "kernel32")]
extern "system" {
    fn CreateFileMappingW(
        file: Handle,
        attributes: *mut c_void,
        protect: u32,
        maximum_size_high: u32,
        maximum_size_low: u32,
        name: *const u16,
    ) -> Handle;
    fn MapViewOfFile(
        mapping: Handle,
        desired_access: u32,
        file_offset_high: u32,
        file_offset_low: u32,
        bytes_to_map: usize,
    ) -> *mut c_void;
    fn UnmapViewOfFile(base_address: *const c_void) -> i32;
    fn CloseHandle(object: Handle) -> i32;
    fn GetLastError() -> u32;
    fn WaitForSingleObject(handle: Handle, milliseconds: u32) -> u32;
    fn GetExitCodeProcess(handle: Handle, exit_code: *mut u32) -> i32;
}

#[link(name = "shell32")]
extern "system" {
    fn ShellExecuteExW(exec_info: *mut ShellExecuteInfoW) -> i32;
}

#[derive(Debug, Default)]
pub struct WindowsBackend;

impl PlatformBackend for WindowsBackend {
    fn device_install(&self, device: &DeviceInfo) -> CjuResult<()> {
        let paths = WindowsArtifacts::resolve()?;
        run_helper_elevated(
            &paths.helper,
            "directshow-install",
            &[&paths.module64, &paths.module32],
        )
        .map_err(|err| {
            Error::new(
                ResultCode::PermissionRequired,
                format!(
                    "failed to register DirectShow modules for '{}': {err}",
                    device.display_name
                ),
            )
        })
    }

    fn device_uninstall(&self, device: &DeviceInfo) -> CjuResult<()> {
        let paths = WindowsArtifacts::resolve()?;
        run_helper_elevated(
            &paths.helper,
            "directshow-uninstall",
            &[&paths.module64, &paths.module32],
        )
        .map_err(|err| {
            Error::new(
                ResultCode::PermissionRequired,
                format!(
                    "failed to unregister DirectShow modules for '{}': {err}",
                    device.display_name
                ),
            )
        })
    }

    fn stream_open(
        &self,
        _device: &DeviceInfo,
        video: VideoDesc,
    ) -> CjuResult<Box<dyn PlatformStream>> {
        Ok(Box::new(WindowsDirectShowStream::open(video)?))
    }
}

struct WindowsArtifacts {
    helper: PathBuf,
    module64: PathBuf,
    module32: PathBuf,
}

impl WindowsArtifacts {
    fn resolve() -> CjuResult<Self> {
        Ok(Self {
            helper: find_artifact("camjongun-installer-helper.exe")?,
            module64: find_artifact("camjongun-virtualcam-module64.dll")?,
            module32: find_artifact("camjongun-virtualcam-module32.dll")?,
        })
    }
}

struct WindowsDirectShowStream {
    handle: Handle,
    view: *mut u8,
    width: u32,
    height: u32,
    offsets: [usize; 3],
    closed: bool,
}

unsafe impl Send for WindowsDirectShowStream {}

impl WindowsDirectShowStream {
    fn open(video: VideoDesc) -> CjuResult<Self> {
        if video.format != PixelFormat::Nv12 {
            return Err(Error::new(
                ResultCode::Unsupported,
                "Windows DirectShow stream expects NV12 frames",
            ));
        }

        let frame_size = video.width as usize * video.height as usize * 3 / 2;
        let mut size = align32(std::mem::size_of::<QueueHeader>());
        let mut offsets = [0usize; 3];
        for offset in &mut offsets {
            *offset = size;
            size = align32(size + frame_size + FRAME_HEADER_SIZE);
        }

        let name = wide_null(QUEUE_NAME);
        let handle = unsafe {
            CreateFileMappingW(
                (-1isize) as Handle,
                std::ptr::null_mut(),
                PAGE_READWRITE,
                0,
                size as u32,
                name.as_ptr(),
            )
        };
        if handle.is_null() {
            return Err(Error::new(
                ResultCode::BackendError,
                "CreateFileMappingW failed",
            ));
        }
        if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
            unsafe {
                CloseHandle(handle);
            }
            return Err(Error::new(
                ResultCode::AlreadyRunning,
                "CamJongUn DirectShow queue is already active",
            ));
        }

        let view = unsafe { MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, 0) } as *mut u8;
        if view.is_null() {
            unsafe {
                CloseHandle(handle);
            }
            return Err(Error::new(ResultCode::BackendError, "MapViewOfFile failed"));
        }

        let fps_den = video.fps_den.max(1) as u64;
        let fps_num = video.fps_num.max(1) as u64;
        let interval = 10_000_000u64.saturating_mul(fps_den) / fps_num;
        let header = QueueHeader {
            write_idx: 0,
            read_idx: 0,
            state: SHARED_QUEUE_STATE_STARTING,
            offsets: [offsets[0] as u32, offsets[1] as u32, offsets[2] as u32],
            queue_type: 0,
            cx: video.width,
            cy: video.height,
            interval,
            reserved: [0; 8],
        };
        unsafe {
            std::ptr::write(view as *mut QueueHeader, header);
        }

        Ok(Self {
            handle,
            view,
            width: video.width,
            height: video.height,
            offsets,
            closed: false,
        })
    }

    fn write_nv12(&mut self, nv12: &[u8], timestamp_ns: u64) -> CjuResult<()> {
        let expected = self.width as usize * self.height as usize * 3 / 2;
        if nv12.len() != expected {
            return Err(Error::new(
                ResultCode::InvalidArgument,
                format!(
                    "frame size mismatch: got {}, expected {}",
                    nv12.len(),
                    expected
                ),
            ));
        }

        let timestamp_100ns = timestamp_ns / 100;
        unsafe {
            let header = &mut *(self.view as *mut QueueHeader);
            let inc = header.write_idx.wrapping_add(1);
            header.write_idx = inc;
            let idx = (inc % 3) as usize;
            let frame_base = self.view.add(self.offsets[idx]);
            std::ptr::write(frame_base as *mut u64, timestamp_100ns);
            std::ptr::copy_nonoverlapping(
                nv12.as_ptr(),
                frame_base.add(FRAME_HEADER_SIZE),
                nv12.len(),
            );
            header.read_idx = inc;
            header.state = SHARED_QUEUE_STATE_READY;
        }

        Ok(())
    }
}

impl PlatformStream for WindowsDirectShowStream {
    fn push_frame(&mut self, frame: &Frame<'_>) -> CjuResult<()> {
        self.write_nv12(frame.planes[0], frame.timestamp_ns)
    }

    fn update_controls(&mut self, _controls: &Controls) -> CjuResult<()> {
        Ok(())
    }

    fn close(&mut self) -> CjuResult<()> {
        if self.closed {
            return Ok(());
        }

        unsafe {
            if !self.view.is_null() {
                let header = &mut *(self.view as *mut QueueHeader);
                header.state = SHARED_QUEUE_STATE_STOPPING;
                UnmapViewOfFile(self.view as *const c_void);
                self.view = std::ptr::null_mut();
            }
            if !self.handle.is_null() {
                CloseHandle(self.handle);
                self.handle = std::ptr::null_mut();
            }
        }
        self.closed = true;
        Ok(())
    }
}

impl Drop for WindowsDirectShowStream {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

pub fn report() -> PlatformReport {
    PlatformReport {
        platform: "windows",
        summary: "CamJongUn ships one elevated installer-helper flow that registers both DirectShow modules and a Rust shared-memory producer that does not depend on UI focus.",
        artifacts: vec![
            artifact(
                "camjongun-virtualcam-module64.dll",
                "64-bit DirectShow virtual camera filter",
                "bin/camjongun-virtualcam-module64.dll",
            ),
            artifact(
                "camjongun-virtualcam-module32.dll",
                "32-bit DirectShow virtual camera filter for 32-bit clients",
                "bin/camjongun-virtualcam-module32.dll",
            ),
            artifact(
                "camjongun-installer-helper.exe",
                "single privileged helper that registers/unregisters CamJongUn filters only",
                "bin/camjongun-installer-helper.exe",
            ),
        ],
    }
}

fn find_artifact(name: &str) -> CjuResult<PathBuf> {
    let roots = artifact_roots();
    for root in roots {
        for relative in [
            name.to_string(),
            format!("bin/{name}"),
            format!("windows/{name}"),
        ] {
            let path = root.join(relative);
            if path.exists() {
                return Ok(path);
            }
        }
    }
    Err(Error::new(
        ResultCode::BackendError,
        format!("missing CamJongUn platform artifact: {name}"),
    ))
}

fn artifact_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(path) = std::env::var_os("CAMJONGUN_ARTIFACT_DIR") {
        roots.push(PathBuf::from(path));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            roots.push(parent.to_path_buf());
            roots.push(parent.join("windows"));
            roots.push(parent.join("native").join("windows"));
        }
    }
    roots.push(PathBuf::from("."));
    roots
}

fn run_helper_elevated(helper: &Path, command: &str, modules: &[&Path]) -> Result<(), String> {
    let params = format!(
        "{} {}",
        command,
        modules
            .iter()
            .map(|path| quote_arg(&path.display().to_string()))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let verb = wide_null("runas");
    let file = wide_null(&helper.display().to_string());
    let parameters = wide_null(&params);
    let directory_text = helper
        .parent()
        .map(|path| path.display().to_string())
        .unwrap_or_default();
    let directory = wide_null(&directory_text);
    let mut exec = ShellExecuteInfoW {
        cb_size: std::mem::size_of::<ShellExecuteInfoW>() as u32,
        f_mask: 0x0000_0040,
        hwnd: std::ptr::null_mut(),
        lp_verb: verb.as_ptr(),
        lp_file: file.as_ptr(),
        lp_parameters: parameters.as_ptr(),
        lp_directory: directory.as_ptr(),
        n_show: 0,
        h_inst_app: std::ptr::null_mut(),
        lp_id_list: std::ptr::null_mut(),
        lp_class: std::ptr::null(),
        hkey_class: std::ptr::null_mut(),
        dw_hot_key: 0,
        h_icon_or_monitor: std::ptr::null_mut(),
        h_process: std::ptr::null_mut(),
    };

    let ok = unsafe { ShellExecuteExW(&mut exec) };
    if ok == 0 {
        return Err("UAC approval was cancelled or helper launch failed".to_string());
    }
    if exec.h_process.is_null() {
        return Err("helper process handle was not returned".to_string());
    }

    unsafe {
        WaitForSingleObject(exec.h_process, u32::MAX);
        let mut exit_code = 1u32;
        let got_exit = GetExitCodeProcess(exec.h_process, &mut exit_code);
        CloseHandle(exec.h_process);
        if got_exit == 0 {
            return Err("failed to read helper exit code".to_string());
        }
        if exit_code == 0 {
            Ok(())
        } else {
            Err(format!("helper exited with code {exit_code}"))
        }
    }
}

fn quote_arg(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\\\""))
}

fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn align32(value: usize) -> usize {
    (value + 31) & !31
}
