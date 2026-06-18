use camjongun::{
    platform::ArtifactStatus, DeviceCreateDesc, DeviceId, PixelFormat, ProducerPolicy, Runtime,
    RuntimeOptions, VideoDesc,
};
use std::env;

fn usage() {
    eprintln!("camjongunctl commands:");
    eprintln!("  list");
    eprintln!("  create <display-name>");
    eprintln!("  delete <device-id>");
    eprintln!("  install <device-id>");
    eprintln!("  uninstall <device-id>");
    eprintln!("  doctor");
}

fn parse_id(value: &str) -> DeviceId {
    DeviceId::from_str_lossy(value)
}

fn main() {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        usage();
        std::process::exit(1);
    };

    let runtime = match Runtime::new(RuntimeOptions::default()) {
        Ok(runtime) => runtime,
        Err(err) => {
            eprintln!("camjongunctl: {err}");
            std::process::exit(1);
        }
    };

    let result = match command.as_str() {
        "list" => match runtime.list_devices() {
            Ok(devices) => {
                for device in devices {
                    println!(
                        "{}\t{}\tinstalled={}\tstreaming={}\t{}",
                        device.id,
                        device.display_name,
                        yes_no(device.installed),
                        yes_no(device.streaming),
                        device.platform_identity
                    );
                }
                Ok(())
            }
            Err(err) => Err(err),
        },
        "create" => {
            let Some(name) = args.next() else {
                usage();
                std::process::exit(1);
            };
            let id = runtime.create_device(DeviceCreateDesc {
                display_name: name,
                owner_app: Some("camjongunctl".to_string()),
                preferred_video: VideoDesc {
                    width: 1920,
                    height: 1080,
                    fps_num: 30,
                    fps_den: 1,
                    format: PixelFormat::Nv12,
                },
                producer_policy: ProducerPolicy::RejectSecond,
            });
            match id {
                Ok(id) => {
                    println!("{id}");
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
        "delete" => {
            let Some(id) = args.next() else {
                usage();
                std::process::exit(1);
            };
            runtime.delete_device(parse_id(&id))
        }
        "install" => {
            let Some(id) = args.next() else {
                usage();
                std::process::exit(1);
            };
            runtime.install_device(parse_id(&id))
        }
        "uninstall" => {
            let Some(id) = args.next() else {
                usage();
                std::process::exit(1);
            };
            runtime.uninstall_device(parse_id(&id))
        }
        "doctor" => {
            let report = runtime.platform_report();
            println!("CamJongUn Rust runtime: ok");
            println!("Registry/API: ok");
            println!("Platform: {}", report.platform);
            println!("{}", report.summary);
            println!("OBS conflict policy: use CamJongUn-specific identities only");
            for artifact in report.artifacts {
                match artifact.status {
                    ArtifactStatus::Present(path) => {
                        println!(
                            "present\t{}\t{}\t{}",
                            artifact.name,
                            artifact.purpose,
                            path.display()
                        );
                    }
                    ArtifactStatus::Missing(path) => {
                        println!(
                            "missing\t{}\t{}\t{}",
                            artifact.name,
                            artifact.purpose,
                            path.display()
                        );
                    }
                    ArtifactStatus::NotApplicable => {
                        println!("n/a\t{}\t{}", artifact.name, artifact.purpose);
                    }
                }
            }
            Ok(())
        }
        _ => {
            usage();
            std::process::exit(1);
        }
    };

    if let Err(err) = result {
        eprintln!("camjongunctl: {err}");
        std::process::exit(err.code as i32);
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}
