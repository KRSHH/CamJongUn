use camjongun::{
    DeviceCreateDesc, DeviceId, DeviceUpdateDesc, ProducerPolicy, ResultCode, Runtime,
    RuntimeOptions, VideoDesc,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::sync::{Mutex, OnceLock};

static RUNTIME: OnceLock<Mutex<Option<Runtime>>> = OnceLock::new();
static LAST_ERROR: OnceLock<Mutex<CString>> = OnceLock::new();

#[repr(C)]
pub struct CjuDeviceId {
    value: [u8; camjongun::ID_MAX],
}

#[repr(C)]
pub struct CjuVideoDesc {
    width: u32,
    height: u32,
    fps_num: u32,
    fps_den: u32,
    format: c_int,
}

#[no_mangle]
pub extern "C" fn cju_runtime_init() -> c_int {
    let runtime = match Runtime::new(RuntimeOptions::default()) {
        Ok(runtime) => runtime,
        Err(err) => return set_error_and_code(err),
    };
    let cell = RUNTIME.get_or_init(|| Mutex::new(None));
    *cell.lock().expect("runtime lock poisoned") = Some(runtime);
    ResultCode::Ok as c_int
}

#[no_mangle]
pub extern "C" fn cju_runtime_shutdown() {
    if let Some(cell) = RUNTIME.get() {
        *cell.lock().expect("runtime lock poisoned") = None;
    }
}

#[no_mangle]
pub unsafe extern "C" fn cju_device_create(
    display_name: *const c_char,
    out_id: *mut CjuDeviceId,
) -> c_int {
    if display_name.is_null() || out_id.is_null() {
        return ResultCode::InvalidArgument as c_int;
    }
    let name = match CStr::from_ptr(display_name).to_str() {
        Ok(value) => value.to_string(),
        Err(_) => return ResultCode::InvalidArgument as c_int,
    };

    with_runtime(|runtime| {
        runtime.create_device(DeviceCreateDesc {
            display_name: name,
            owner_app: Some("camjongun-ffi".to_string()),
            preferred_video: VideoDesc::default(),
            producer_policy: ProducerPolicy::RejectSecond,
        })
    })
    .map(|id| {
        ptr::write(out_id, CjuDeviceId { value: id.0 });
        ResultCode::Ok as c_int
    })
    .unwrap_or_else(set_error_and_code)
}

#[no_mangle]
pub unsafe extern "C" fn cju_camera_ensure(
    display_name: *const c_char,
    out_id: *mut CjuDeviceId,
) -> c_int {
    cju_device_create(display_name, out_id)
}

#[no_mangle]
pub unsafe extern "C" fn cju_camera_rename(display_name: *const c_char) -> c_int {
    if display_name.is_null() {
        return ResultCode::InvalidArgument as c_int;
    }
    let name = match CStr::from_ptr(display_name).to_str() {
        Ok(value) => value.to_string(),
        Err(_) => return ResultCode::InvalidArgument as c_int,
    };

    with_runtime(|runtime| {
        runtime.update_camera(DeviceUpdateDesc {
            display_name: Some(name),
            ..DeviceUpdateDesc::default()
        })
    })
    .map(|_| ResultCode::Ok as c_int)
    .unwrap_or_else(set_error_and_code)
}

#[no_mangle]
pub extern "C" fn cju_camera_install() -> c_int {
    with_runtime(|runtime| runtime.install_camera())
        .map(|_| ResultCode::Ok as c_int)
        .unwrap_or_else(set_error_and_code)
}

#[no_mangle]
pub extern "C" fn cju_camera_uninstall() -> c_int {
    with_runtime(|runtime| runtime.uninstall_camera())
        .map(|_| ResultCode::Ok as c_int)
        .unwrap_or_else(set_error_and_code)
}

#[no_mangle]
pub unsafe extern "C" fn cju_device_delete(id: CjuDeviceId) -> c_int {
    with_runtime(|runtime| runtime.delete_device(DeviceId(id.value)))
        .map(|_| ResultCode::Ok as c_int)
        .unwrap_or_else(set_error_and_code)
}

#[no_mangle]
pub extern "C" fn cju_result_message(code: c_int) -> *const c_char {
    let code = match code {
        0 => ResultCode::Ok,
        1 => ResultCode::NotInitialized,
        2 => ResultCode::AlreadyRunning,
        3 => ResultCode::NotRunning,
        4 => ResultCode::PlatformUnavailable,
        5 => ResultCode::InvalidArgument,
        6 => ResultCode::BackendError,
        7 => ResultCode::NotFound,
        8 => ResultCode::AlreadyExists,
        9 => ResultCode::PermissionRequired,
        10 => ResultCode::BufferTooSmall,
        11 => ResultCode::Unsupported,
        _ => ResultCode::BackendError,
    };
    cstring_static(code.message())
}

#[no_mangle]
pub extern "C" fn cju_last_error() -> *const c_char {
    LAST_ERROR
        .get_or_init(|| Mutex::new(CString::new("").unwrap()))
        .lock()
        .expect("error lock poisoned")
        .as_ptr()
}

fn with_runtime<T>(f: impl FnOnce(&Runtime) -> camjongun::CjuResult<T>) -> camjongun::CjuResult<T> {
    let cell = RUNTIME
        .get()
        .ok_or_else(|| camjongun::Error::new(ResultCode::NotInitialized, ""))?;
    let guard = cell.lock().expect("runtime lock poisoned");
    let runtime = guard
        .as_ref()
        .ok_or_else(|| camjongun::Error::new(ResultCode::NotInitialized, ""))?;
    f(runtime)
}

fn set_error_and_code(err: camjongun::Error) -> c_int {
    let text =
        CString::new(err.to_string()).unwrap_or_else(|_| CString::new("CamJongUn error").unwrap());
    *LAST_ERROR
        .get_or_init(|| Mutex::new(CString::new("").unwrap()))
        .lock()
        .expect("error lock poisoned") = text;
    err.code as c_int
}

fn cstring_static(value: &'static str) -> *const c_char {
    match value {
        "ok" => c"ok".as_ptr(),
        "CamJongUn runtime is not initialized" => c"CamJongUn runtime is not initialized".as_ptr(),
        "stream is already running" => c"stream is already running".as_ptr(),
        "stream is not running" => c"stream is not running".as_ptr(),
        "platform backend is unavailable" => c"platform backend is unavailable".as_ptr(),
        "invalid argument" => c"invalid argument".as_ptr(),
        "backend error" => c"backend error".as_ptr(),
        "device not found" => c"device not found".as_ptr(),
        "device already exists" => c"device already exists".as_ptr(),
        "permission or user approval is required" => {
            c"permission or user approval is required".as_ptr()
        }
        "buffer is too small" => c"buffer is too small".as_ptr(),
        "operation is unsupported" => c"operation is unsupported".as_ptr(),
        _ => c"unknown CamJongUn result".as_ptr(),
    }
}
