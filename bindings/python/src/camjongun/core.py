from __future__ import annotations

import ctypes
import os
import platform
from dataclasses import dataclass
from pathlib import Path


class DeviceId(ctypes.Structure):
    _fields_ = [("value", ctypes.c_uint8 * 64)]

    def text(self) -> str:
        raw = bytes(self.value)
        return raw.split(b"\0", 1)[0].decode("utf-8", errors="replace")


@dataclass
class CamJongUnError(RuntimeError):
    code: int
    detail: str

    def __str__(self) -> str:
        return f"{self.detail} ({self.code})"


def _default_library_name() -> str:
    system = platform.system()
    if system == "Windows":
        return "camjongun_ffi.dll"
    if system == "Darwin":
        return "libcamjongun_ffi.dylib"
    return "libcamjongun_ffi.so"


def _load_library(path: str | os.PathLike[str] | None = None) -> ctypes.CDLL:
    chosen = path or os.environ.get("CAMJONGUN_FFI_PATH") or _default_library_name()
    lib = ctypes.CDLL(str(chosen))
    lib.cju_runtime_init.restype = ctypes.c_int
    lib.cju_runtime_shutdown.restype = None
    lib.cju_camera_ensure.argtypes = [ctypes.c_char_p, ctypes.POINTER(DeviceId)]
    lib.cju_camera_ensure.restype = ctypes.c_int
    lib.cju_camera_rename.argtypes = [ctypes.c_char_p]
    lib.cju_camera_rename.restype = ctypes.c_int
    lib.cju_camera_install.restype = ctypes.c_int
    lib.cju_camera_uninstall.restype = ctypes.c_int
    lib.cju_result_message.argtypes = [ctypes.c_int]
    lib.cju_result_message.restype = ctypes.c_char_p
    lib.cju_last_error.restype = ctypes.c_char_p
    return lib


class CamJongUn:
    def __init__(self, library_path: str | Path | None = None) -> None:
        self._lib = _load_library(library_path)

    def init(self) -> None:
        self._check(self._lib.cju_runtime_init())

    def shutdown(self) -> None:
        self._lib.cju_runtime_shutdown()

    def ensure_camera(self, display_name: str) -> DeviceId:
        device_id = DeviceId()
        rc = self._lib.cju_camera_ensure(display_name.encode("utf-8"), ctypes.byref(device_id))
        self._check(rc)
        return device_id

    def rename_camera(self, display_name: str) -> None:
        self._check(self._lib.cju_camera_rename(display_name.encode("utf-8")))

    def install_camera(self) -> None:
        self._check(self._lib.cju_camera_install())

    def uninstall_camera(self) -> None:
        self._check(self._lib.cju_camera_uninstall())

    def _check(self, code: int) -> None:
        if code == 0:
            return
        detail = self._lib.cju_last_error()
        if detail:
            text = detail.decode("utf-8", errors="replace")
        else:
            text = self._lib.cju_result_message(code).decode("utf-8", errors="replace")
        raise CamJongUnError(code, text)

    def __enter__(self) -> "CamJongUn":
        self.init()
        return self

    def __exit__(self, *_exc: object) -> None:
        self.shutdown()
