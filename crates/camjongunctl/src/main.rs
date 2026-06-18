use camjongun::{platform::ArtifactStatus, CameraUpdate, Runtime, RuntimeOptions};
use std::env;

fn usage() {
    eprintln!("camjongunctl commands:");
    eprintln!("  show");
    eprintln!("  ensure <display-name>");
    eprintln!("  rename <display-name>");
    eprintln!("  delete");
    eprintln!("  install");
    eprintln!("  uninstall");
    eprintln!("  doctor");
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
        "show" => runtime.get_camera().map(|camera| {
            println!(
                "{}\t{}\tinstalled={}\tstreaming={}\t{}",
                camera.id,
                camera.display_name,
                yes_no(camera.installed),
                yes_no(camera.streaming),
                camera.platform_identity
            );
        }),
        "ensure" => {
            let Some(name) = args.next() else {
                usage();
                std::process::exit(1);
            };
            runtime
                .ensure_camera(name)
                .map(|camera| println!("{}", camera.id))
        }
        "rename" => {
            let Some(name) = args.next() else {
                usage();
                std::process::exit(1);
            };
            runtime
                .update_camera(CameraUpdate {
                    display_name: Some(name),
                    ..CameraUpdate::default()
                })
                .map(|_| ())
        }
        "delete" => runtime.delete_camera(),
        "install" => runtime.install_camera(),
        "uninstall" => runtime.uninstall_camera(),
        "doctor" => {
            let report = runtime.platform_report();
            println!("CamJongUn Rust runtime: ok");
            println!("Registry/API: single app-owned camera");
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
