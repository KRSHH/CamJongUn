package camjongun

/*
#cgo CFLAGS: -I../../include
#cgo windows LDFLAGS: -L../../target/release -lcamjongun_ffi
#cgo darwin LDFLAGS: -L../../target/release -lcamjongun_ffi
#cgo linux LDFLAGS: -L../../target/release -lcamjongun_ffi
#include "camjongun/camjongun.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"unsafe"
)

type DeviceID string

func Init() error {
	return check(C.cju_runtime_init())
}

func Shutdown() {
	C.cju_runtime_shutdown()
}

func EnsureCamera(displayName string) (DeviceID, error) {
	name := C.CString(displayName)
	defer C.free(unsafe.Pointer(name))
	var id C.cju_device_id
	if err := check(C.cju_camera_ensure(name, &id)); err != nil {
		return "", err
	}
	return DeviceID(C.GoString((*C.char)(unsafe.Pointer(&id.value[0])))), nil
}

func RenameCamera(displayName string) error {
	name := C.CString(displayName)
	defer C.free(unsafe.Pointer(name))
	return check(C.cju_camera_rename(name))
}

func InstallCamera() error {
	return check(C.cju_camera_install())
}

func UninstallCamera() error {
	return check(C.cju_camera_uninstall())
}

func check(code C.int) error {
	if code == 0 {
		return nil
	}
	if msg := C.cju_last_error(); msg != nil {
		return errors.New(C.GoString(msg))
	}
	return errors.New(C.GoString(C.cju_result_message(code)))
}
