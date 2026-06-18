cmake_minimum_required(VERSION 3.24)
project(camjongun-directshow-module LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CAMJONGUN_ROOT "${CMAKE_CURRENT_LIST_DIR}/../../.." CACHE PATH "CamJongUn root")
set(CAMJONGUN_VIRTUALCAM_GUID "7f9f0d4f-9fb8-4dfb-a336-1f1d635f2d57" CACHE STRING "CamJongUn DirectShow CLSID")

if(NOT EXISTS "${CAMJONGUN_ROOT}/vendor/obs/deps/libdshowcapture/src/source/output-filter.hpp")
  message(FATAL_ERROR "libdshowcapture source is missing. Run scripts/fetch-native-deps.ps1 first.")
endif()

string(REPLACE "-" ";" GUID_VALS "${CAMJONGUN_VIRTUALCAM_GUID}")
list(GET GUID_VALS 0 GUID_VAL01)
list(GET GUID_VALS 1 GUID_VAL02)
list(GET GUID_VALS 2 GUID_VAL03)
list(GET GUID_VALS 3 GUID_VALS_DATA4)
list(GET GUID_VALS 4 GUID_VALS_DATA5)
string(SUBSTRING "${GUID_VALS_DATA4}" 0 2 GUID_VAL04)
string(SUBSTRING "${GUID_VALS_DATA4}" 2 2 GUID_VAL05)
string(SUBSTRING "${GUID_VALS_DATA5}" 0 2 GUID_VAL06)
string(SUBSTRING "${GUID_VALS_DATA5}" 2 2 GUID_VAL07)
string(SUBSTRING "${GUID_VALS_DATA5}" 4 2 GUID_VAL08)
string(SUBSTRING "${GUID_VALS_DATA5}" 6 2 GUID_VAL09)
string(SUBSTRING "${GUID_VALS_DATA5}" 8 2 GUID_VAL10)
string(SUBSTRING "${GUID_VALS_DATA5}" 10 2 GUID_VAL11)

set(GENERATED_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated")
file(MAKE_DIRECTORY "${GENERATED_DIR}")
configure_file(
  "${CAMJONGUN_ROOT}/vendor/obs/platform/windows/directshow-module/virtualcam-guid.h.in"
  "${GENERATED_DIR}/virtualcam-guid.h"
)

set(MODULE_SRC "${CAMJONGUN_ROOT}/vendor/obs/platform/windows/directshow-module")
set(SHARED_SRC "${CAMJONGUN_ROOT}/vendor/obs/shared")
set(LIBOBS_SRC "${CAMJONGUN_ROOT}/vendor/obs/shared/libobs")
set(DSHOW_SRC "${CAMJONGUN_ROOT}/vendor/obs/deps/libdshowcapture/src")

file(READ "${MODULE_SRC}/virtualcam-module.cpp" VIRTUALCAM_MODULE_CPP)
string(REPLACE "OBS Virtual Camera" "CamJongUn Virtual Camera" VIRTUALCAM_MODULE_CPP "${VIRTUALCAM_MODULE_CPP}")
string(REPLACE "obs-virtualcam-module64.dll" "camjongun-virtualcam-module64.dll" VIRTUALCAM_MODULE_CPP "${VIRTUALCAM_MODULE_CPP}")
string(REPLACE "obs-virtualcam-module32.dll" "camjongun-virtualcam-module32.dll" VIRTUALCAM_MODULE_CPP "${VIRTUALCAM_MODULE_CPP}")
file(WRITE "${GENERATED_DIR}/virtualcam-module.cpp" "${VIRTUALCAM_MODULE_CPP}")

file(READ "${SHARED_SRC}/obs-shared-memory-queue/shared-memory-queue.c" SHARED_MEMORY_QUEUE_C)
string(REPLACE "OBSVirtualCamVideo" "CamJongUnVirtualCamVideo" SHARED_MEMORY_QUEUE_C "${SHARED_MEMORY_QUEUE_C}")
file(WRITE "${GENERATED_DIR}/shared-memory-queue.c" "${SHARED_MEMORY_QUEUE_C}")

add_library(libdshowcapture STATIC
  "${DSHOW_SRC}/source/capture-filter.cpp"
  "${DSHOW_SRC}/source/device.cpp"
  "${DSHOW_SRC}/source/dshow-base.cpp"
  "${DSHOW_SRC}/source/dshow-demux.cpp"
  "${DSHOW_SRC}/source/dshow-encoded-device.cpp"
  "${DSHOW_SRC}/source/dshow-enum.cpp"
  "${DSHOW_SRC}/source/dshow-formats.cpp"
  "${DSHOW_SRC}/source/dshow-media-type.cpp"
  "${DSHOW_SRC}/source/dshowcapture.cpp"
  "${DSHOW_SRC}/source/dshowencode.cpp"
  "${DSHOW_SRC}/source/encoder.cpp"
  "${DSHOW_SRC}/source/external/IVideoCaptureFilter.h"
  "${DSHOW_SRC}/source/log.cpp"
  "${DSHOW_SRC}/source/output-filter.cpp"
  "${CMAKE_CURRENT_LIST_DIR}/vendor-stubs.cpp"
)
target_include_directories(libdshowcapture PUBLIC
  "${DSHOW_SRC}"
  "${DSHOW_SRC}/external/capture-device-support/Library"
)
target_compile_definitions(libdshowcapture PUBLIC _UP_WINDOWS=1 UNICODE _UNICODE)
target_compile_options(libdshowcapture PRIVATE /wd4005 /wd4018)

add_library(camjongun-virtualcam-module MODULE
  "${GENERATED_DIR}/virtualcam-module.cpp"
  "${MODULE_SRC}/virtualcam-filter.cpp"
  "${MODULE_SRC}/placeholder.cpp"
  "${MODULE_SRC}/sleepto.c"
  "${GENERATED_DIR}/shared-memory-queue.c"
  "${SHARED_SRC}/obs-tiny-nv12-scale/tiny-nv12-scale.c"
)
target_include_directories(camjongun-virtualcam-module PRIVATE
  "${GENERATED_DIR}"
  "${MODULE_SRC}"
  "${SHARED_SRC}/obs-shared-memory-queue"
  "${SHARED_SRC}/obs-tiny-nv12-scale"
  "${LIBOBS_SRC}"
  "${DSHOW_SRC}"
)
target_compile_definitions(camjongun-virtualcam-module PRIVATE VIRTUALCAM_AVAILABLE UNICODE _UNICODE)
target_link_libraries(camjongun-virtualcam-module PRIVATE libdshowcapture gdiplus strmiids winmm)

if(CMAKE_SIZEOF_VOID_P EQUAL 8)
  set(MODULE_OUTPUT_NAME "camjongun-virtualcam-module64")
  target_sources(camjongun-virtualcam-module PRIVATE "${CMAKE_CURRENT_LIST_DIR}/camjongun-virtualcam-module64.def")
else()
  set(MODULE_OUTPUT_NAME "camjongun-virtualcam-module32")
  target_sources(camjongun-virtualcam-module PRIVATE "${CMAKE_CURRENT_LIST_DIR}/camjongun-virtualcam-module32.def")
endif()

set_target_properties(camjongun-virtualcam-module PROPERTIES
  OUTPUT_NAME "${MODULE_OUTPUT_NAME}"
  PREFIX ""
)
