use camjongun::{DeviceId, Runtime, RuntimeOptions};
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        usage();
        std::process::exit(1);
    };
    let Some(id) = args.next() else {
        usage();
        std::process::exit(1);
    };

    let runtime = match Runtime::new(RuntimeOptions::default()) {
        Ok(runtime) => runtime,
        Err(err) => {
            eprintln!("camjongun-installer-helper: {err}");
            std::process::exit(1);
        }
    };

    let id = DeviceId::from_str_lossy(&id);
    let result = match command.as_str() {
        "install" => runtime.install_device(id),
        "uninstall" => runtime.uninstall_device(id),
        _ => {
            usage();
            std::process::exit(1);
        }
    };

    if let Err(err) = result {
        eprintln!("camjongun-installer-helper: {err}");
        std::process::exit(err.code as i32);
    }
}

fn usage() {
    eprintln!("usage: camjongun-installer-helper install|uninstall <device-id>");
}
