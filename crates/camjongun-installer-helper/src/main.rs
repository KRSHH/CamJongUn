use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        usage();
        std::process::exit(1);
    };

    let result = match command.as_str() {
        "directshow-install" => directshow(args.collect(), false),
        "directshow-uninstall" => directshow(args.collect(), true),
        _ => {
            usage();
            std::process::exit(1);
        }
    };

    if let Err(err) = result {
        eprintln!("camjongun-installer-helper: {err}");
        std::process::exit(1);
    }
}

fn directshow(paths: Vec<String>, unregister: bool) -> Result<(), String> {
    if paths.len() != 2 {
        return Err(
            "usage: camjongun-installer-helper directshow-install|directshow-uninstall <module64.dll> <module32.dll>"
                .to_string(),
        );
    }

    let module64 = PathBuf::from(&paths[0]);
    let module32 = PathBuf::from(&paths[1]);
    if !module64.exists() {
        return Err(format!(
            "missing 64-bit DirectShow module: {}",
            module64.display()
        ));
    }
    if !module32.exists() {
        return Err(format!(
            "missing 32-bit DirectShow module: {}",
            module32.display()
        ));
    }

    run_regsvr32(
        &PathBuf::from(r"C:\Windows\System32\regsvr32.exe"),
        &module64,
        unregister,
    )?;
    run_regsvr32(
        &PathBuf::from(r"C:\Windows\SysWOW64\regsvr32.exe"),
        &module32,
        unregister,
    )
}

fn run_regsvr32(regsvr32: &Path, module: &Path, unregister: bool) -> Result<(), String> {
    if !regsvr32.exists() {
        return Err(format!(
            "missing Windows registry server: {}",
            regsvr32.display()
        ));
    }

    let mut command = Command::new(regsvr32);
    command.arg("/s");
    if unregister {
        command.arg("/u");
    }
    command.arg(module);

    let status = command
        .status()
        .map_err(|err| format!("failed to run {}: {err}", regsvr32.display()))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "{} failed for {} with status {}",
            regsvr32.display(),
            module.display(),
            status
        ))
    }
}

fn usage() {
    eprintln!("camjongun-installer-helper commands:");
    eprintln!("  directshow-install <camjongun-virtualcam-module64.dll> <camjongun-virtualcam-module32.dll>");
    eprintln!("  directshow-uninstall <camjongun-virtualcam-module64.dll> <camjongun-virtualcam-module32.dll>");
}
