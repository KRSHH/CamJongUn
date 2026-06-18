# C and C++ Consumers

Use the public header:

```c
#include <camjongun/camjongun.h>
```

Minimal flow:

```c
cju_device_id id;
int rc = cju_runtime_init();
if (rc == CJU_RESULT_OK)
    rc = cju_camera_ensure("My Virtual Camera", &id);
if (rc == CJU_RESULT_OK)
    rc = cju_camera_install();
cju_runtime_shutdown();
```

Link against `camjongun_ffi` from the release package `lib/` directory.
