using System;
using System.Runtime.InteropServices;
using System.Text;

namespace CamJongUn;

public sealed class CamJongUnRuntime : IDisposable
{
    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct DeviceId
    {
        public fixed byte Value[64];

        public override string ToString()
        {
            unsafe
            {
                fixed (byte* ptr = Value)
                {
                    var span = new ReadOnlySpan<byte>(ptr, 64);
                    var end = span.IndexOf((byte)0);
                    if (end >= 0) span = span[..end];
                    return Encoding.UTF8.GetString(span);
                }
            }
        }
    }

    public CamJongUnRuntime()
    {
        Check(Native.cju_runtime_init());
    }

    public DeviceId EnsureCamera(string displayName)
    {
        unsafe
        {
            DeviceId id = default;
            Check(Native.cju_camera_ensure(displayName, &id));
            return id;
        }
    }

    public void RenameCamera(string displayName) => Check(Native.cju_camera_rename(displayName));
    public void InstallCamera() => Check(Native.cju_camera_install());
    public void UninstallCamera() => Check(Native.cju_camera_uninstall());
    public void Dispose() => Native.cju_runtime_shutdown();

    private static void Check(int code)
    {
        if (code == 0) return;
        var error = Marshal.PtrToStringAnsi(Native.cju_last_error());
        if (string.IsNullOrEmpty(error))
            error = Marshal.PtrToStringAnsi(Native.cju_result_message(code));
        throw new InvalidOperationException(error ?? $"CamJongUn failed with code {code}");
    }

    private static unsafe class Native
    {
        private const string Library = "camjongun_ffi";

        [DllImport(Library)] public static extern int cju_runtime_init();
        [DllImport(Library)] public static extern void cju_runtime_shutdown();
        [DllImport(Library, CharSet = CharSet.Ansi)] public static extern int cju_camera_ensure(string displayName, DeviceId* id);
        [DllImport(Library, CharSet = CharSet.Ansi)] public static extern int cju_camera_rename(string displayName);
        [DllImport(Library)] public static extern int cju_camera_install();
        [DllImport(Library)] public static extern int cju_camera_uninstall();
        [DllImport(Library)] public static extern IntPtr cju_result_message(int code);
        [DllImport(Library)] public static extern IntPtr cju_last_error();
    }
}
