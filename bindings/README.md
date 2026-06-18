# CamJongUn Language Bindings

CamJongUn uses the C ABI in `include/camjongun/camjongun.h` as the stable
boundary for non-Rust languages.

Current bindings are thin wrappers for the app-owned virtual camera lifecycle:

- initialize/shutdown runtime
- ensure or rename the app camera
- install or uninstall the camera integration
- surface CamJongUn error messages

Frame streaming is currently exposed through the Rust SDK on Windows. Add FFI
stream functions before claiming media/frame streaming support in non-Rust
bindings.

## Layout

- `c/`: CMake helper package for C and C++ consumers.
- `python/`: `ctypes` package.
- `node/`: Node.js wrapper using `ffi-napi`.
- `dotnet/`: C# P/Invoke wrapper.
- `go/`: cgo wrapper.
- `java/`: JNA wrapper.

## Native Library Loading

Each binding expects the CamJongUn C ABI library next to the application, in the
system library path, or pointed to by:

```text
CAMJONGUN_FFI_PATH
```

Expected library names:

- Windows: `camjongun_ffi.dll`
- macOS: `libcamjongun_ffi.dylib`
- Linux: `libcamjongun_ffi.so`
