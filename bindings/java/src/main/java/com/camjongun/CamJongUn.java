package com.camjongun;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Structure;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.List;

public final class CamJongUn implements AutoCloseable {
    public static final class DeviceId extends Structure {
        public byte[] value = new byte[64];

        @Override
        protected List<String> getFieldOrder() {
            return List.of("value");
        }

        @Override
        public String toString() {
            int end = 0;
            while (end < value.length && value[end] != 0) end++;
            return new String(Arrays.copyOf(value, end), StandardCharsets.UTF_8);
        }
    }

    interface NativeApi extends Library {
        NativeApi INSTANCE = Native.load(
            System.getenv().getOrDefault("CAMJONGUN_FFI_PATH", "camjongun_ffi"),
            NativeApi.class
        );

        int cju_runtime_init();
        void cju_runtime_shutdown();
        int cju_camera_ensure(String displayName, DeviceId id);
        int cju_camera_rename(String displayName);
        int cju_camera_install();
        int cju_camera_uninstall();
        String cju_result_message(int code);
        String cju_last_error();
    }

    public CamJongUn() {
        check(NativeApi.INSTANCE.cju_runtime_init());
    }

    public DeviceId ensureCamera(String displayName) {
        DeviceId id = new DeviceId();
        check(NativeApi.INSTANCE.cju_camera_ensure(displayName, id));
        return id;
    }

    public void renameCamera(String displayName) {
        check(NativeApi.INSTANCE.cju_camera_rename(displayName));
    }

    public void installCamera() {
        check(NativeApi.INSTANCE.cju_camera_install());
    }

    public void uninstallCamera() {
        check(NativeApi.INSTANCE.cju_camera_uninstall());
    }

    @Override
    public void close() {
        NativeApi.INSTANCE.cju_runtime_shutdown();
    }

    private static void check(int code) {
        if (code == 0) return;
        String detail = NativeApi.INSTANCE.cju_last_error();
        if (detail == null || detail.isBlank()) {
            detail = NativeApi.INSTANCE.cju_result_message(code);
        }
        throw new IllegalStateException(detail);
    }
}
