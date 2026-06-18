use camjongun::{
    make_device_path, make_platform_identity, DeviceCreateDesc, DeviceId, PixelFormat,
    ProducerPolicy, Runtime, RuntimeOptions, VideoDesc,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under CamJongUn/crates/camjongun")
        .to_path_buf()
}

fn read_to_string(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path.as_ref())
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.as_ref().display()))
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in
        fs::read_dir(root).unwrap_or_else(|err| panic!("failed to read {}: {err}", root.display()))
    {
        let entry = entry.expect("directory entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path);
        }
    }
}

fn tree_contains(root: &Path, needle: &str) -> bool {
    let mut files = Vec::new();
    collect_files(root, &mut files);

    files
        .into_iter()
        .filter(|path| {
            !path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    matches!(ext, "png" | "icns" | "tiff" | "exe" | "dll" | "lib" | "pdb")
                })
        })
        .any(|path| read_to_string(path).contains(needle))
}

fn temp_registry_path(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("camjongun-{name}-{stamp}.tsv"))
}

#[test]
fn vendor_obs_layout_has_expected_roots() {
    let root = repo_root();
    let expected = [
        "vendor/obs/platform/windows/directshow-module",
        "vendor/obs/platform/windows/obs-plugin",
        "vendor/obs/platform/macos/camera-extension",
        "vendor/obs/platform/macos/obs-plugin",
        "vendor/obs/platform/linux/obs-plugin/v4l2-output.c",
        "vendor/obs/shared/libobs",
        "vendor/obs/shared/obs-shared-memory-queue",
        "vendor/obs/shared/obs-tiny-nv12-scale",
        "vendor/obs/deps/libdshowcapture",
    ];

    for relative in expected {
        assert!(
            root.join(relative).exists(),
            "missing expected vendored OBS path: {relative}"
        );
    }
}

#[test]
fn upstream_obs_identity_markers_still_exist() {
    let vendor = repo_root().join("vendor/obs");
    let markers = [
        "obs-virtualcam.txt",
        "OBS Virtual Camera",
        "com.obsproject.obs-studio.mac-camera-extension",
        "virtualcam_output",
    ];

    for marker in markers {
        assert!(
            tree_contains(&vendor, marker),
            "upstream marker changed or disappeared; review adapter/template assumptions: {marker}"
        );
    }
}

#[test]
fn camjongun_artifact_docs_do_not_ship_obs_named_artifacts() {
    let root = repo_root();
    let docs = [
        "artifacts-needed/windows/README.md",
        "artifacts-needed/macos/README.md",
        "artifacts-needed/linux/README.md",
        "PACKAGING_CONTRACT.md",
    ];
    let forbidden = [
        "obs-virtualcam-module32.dll",
        "obs-virtualcam-module64.dll",
        "obs-virtualcam-module-arm64.dll",
        "obs-mac-virtualcam.plugin",
        "com.obsproject.obs-studio.mac-camera-extension.systemextension",
        "card_label='OBS Virtual Camera'",
    ];

    for doc in docs {
        let text = read_to_string(root.join(doc));
        for bad in forbidden {
            assert!(
                !text.contains(bad),
                "{doc} must not describe OBS-owned artifact as a CamJongUn shipped artifact: {bad}"
            );
        }
    }
}

#[test]
fn generated_camjongun_identities_do_not_reuse_obs_names() {
    let id = DeviceId::from_str_lossy("cju-contract");
    let platform_identity = make_platform_identity(&id);
    let device_path = make_device_path(&id);

    assert!(platform_identity.starts_with("camjongun.virtual-camera."));
    assert!(!platform_identity.to_lowercase().contains("obs"));
    assert!(!device_path.to_lowercase().contains("obs"));
}

#[test]
fn registry_create_list_get_delete_round_trip_is_stable() {
    let registry_path = temp_registry_path("round-trip");
    let runtime = Runtime::new(RuntimeOptions {
        app_name: "contract-test".to_string(),
        registry_path: Some(registry_path.clone()),
        auto_install_helper: false,
    })
    .expect("runtime should initialize");

    let id = runtime
        .create_device(DeviceCreateDesc {
            display_name: "Contract Camera".to_string(),
            owner_app: Some("contract-test".to_string()),
            preferred_video: VideoDesc {
                width: 1280,
                height: 720,
                fps_num: 60,
                fps_den: 1,
                format: PixelFormat::Nv12,
            },
            producer_policy: ProducerPolicy::RejectSecond,
        })
        .expect("device should be created");

    let listed = runtime
        .list_devices()
        .expect("registry should list devices");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, id);
    assert_eq!(listed[0].display_name, "Contract Camera");
    assert!(listed[0]
        .platform_identity
        .starts_with("camjongun.virtual-camera."));

    let fetched = runtime
        .get_device(id)
        .expect("created device should be fetchable");
    assert_eq!(fetched.preferred_video.width, 1280);
    assert_eq!(fetched.preferred_video.height, 720);

    runtime.delete_device(id).expect("device should delete");
    assert!(runtime
        .list_devices()
        .expect("registry should list after delete")
        .is_empty());

    let _ = fs::remove_file(registry_path);
}

#[test]
fn registry_ignores_malformed_lines_and_keeps_valid_rows() {
    let registry_path = temp_registry_path("compat");
    let valid_id = "cju-compat";
    let valid_row = [
        valid_id,
        "Compat Camera",
        "contract-test",
        "camjongun.virtual-camera.cju-compat",
        "directshow:cju-compat",
        "640",
        "480",
        "30",
        "1",
        "0",
        "0",
        "1",
        "0",
        "0",
        "0",
        "123",
        "",
        "future-extra-field",
    ]
    .join("\t");

    fs::write(&registry_path, format!("bad row\n{valid_row}\n"))
        .expect("registry fixture should write");

    let runtime = Runtime::new(RuntimeOptions {
        app_name: "contract-test".to_string(),
        registry_path: Some(registry_path.clone()),
        auto_install_helper: false,
    })
    .expect("runtime should initialize");

    let devices = runtime
        .list_devices()
        .expect("registry should parse valid rows");
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].id, DeviceId::from_str_lossy(valid_id));
    assert_eq!(devices[0].last_frame_time_ns, 123);

    let _ = fs::remove_file(registry_path);
}

#[test]
fn platform_reports_use_camjongun_artifact_names() {
    let runtime = Runtime::new(RuntimeOptions::default()).expect("runtime should initialize");
    let report = runtime.platform_report();
    let names = report
        .artifacts
        .iter()
        .map(|artifact| artifact.name)
        .collect::<Vec<_>>()
        .join("\n");

    if cfg!(target_os = "windows") {
        assert!(names.contains("camjongun-virtualcam-module64.dll"));
        assert!(names.contains("camjongun-virtualcam-module32.dll"));
        assert!(!names.contains("obs-virtualcam-module"));
    } else if cfg!(target_os = "macos") {
        assert!(names.contains("com.camjongun.virtual-camera.systemextension"));
        assert!(names.contains("camjongun-mac-virtualcam.plugin"));
        assert!(!names.contains("com.obsproject"));
    } else if cfg!(unix) {
        assert!(report.summary.contains("v4l2loopback"));
        assert!(names.contains("camjongun-installer-helper"));
    }
}

#[test]
fn platform_adapter_sources_keep_expected_cross_platform_names() {
    let root = repo_root();
    let windows = read_to_string(root.join("crates/camjongun/src/platform/windows.rs"));
    let macos = read_to_string(root.join("crates/camjongun/src/platform/macos.rs"));
    let linux = read_to_string(root.join("crates/camjongun/src/platform/linux.rs"));

    assert!(windows.contains("camjongun-virtualcam-module64.dll"));
    assert!(windows.contains("camjongun-virtualcam-module32.dll"));
    assert!(macos.contains("com.camjongun.virtual-camera.systemextension"));
    assert!(macos.contains("camjongun-mac-virtualcam.plugin"));
    assert!(linux.contains("v4l2loopback"));
}
