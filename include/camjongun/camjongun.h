#ifndef CAMJONGUN_H
#define CAMJONGUN_H

#include <stdint.h>

#ifdef _WIN32
#  ifdef CAMJONGUN_STATIC
#    define CJU_API
#  else
#    define CJU_API __declspec(dllimport)
#  endif
#else
#  define CJU_API
#endif

#ifdef __cplusplus
extern "C" {
#endif

enum cju_result {
	CJU_RESULT_OK = 0,
	CJU_RESULT_NOT_INITIALIZED = 1,
	CJU_RESULT_ALREADY_RUNNING = 2,
	CJU_RESULT_NOT_RUNNING = 3,
	CJU_RESULT_PLATFORM_UNAVAILABLE = 4,
	CJU_RESULT_INVALID_ARGUMENT = 5,
	CJU_RESULT_BACKEND_ERROR = 6,
	CJU_RESULT_NOT_FOUND = 7,
	CJU_RESULT_ALREADY_EXISTS = 8,
	CJU_RESULT_PERMISSION_REQUIRED = 9,
	CJU_RESULT_BUFFER_TOO_SMALL = 10,
	CJU_RESULT_UNSUPPORTED = 11,
};

enum cju_pixel_format {
	CJU_PIXEL_FORMAT_NV12 = 0,
};

typedef struct cju_device_id {
	uint8_t value[64];
} cju_device_id;

typedef struct cju_video_desc {
	uint32_t width;
	uint32_t height;
	uint32_t fps_num;
	uint32_t fps_den;
	int format;
} cju_video_desc;

CJU_API int cju_runtime_init(void);
CJU_API void cju_runtime_shutdown(void);

/* Single app-owned virtual camera API. ensure returns the same camera for the
 * current app on later calls and updates its display name. */
CJU_API int cju_camera_ensure(const char *display_name, cju_device_id *out_id);
CJU_API int cju_camera_rename(const char *display_name);
CJU_API int cju_camera_install(void);
CJU_API int cju_camera_uninstall(void);

CJU_API const char *cju_result_message(int result);
CJU_API const char *cju_last_error(void);

#ifdef __cplusplus
}
#endif

#endif
