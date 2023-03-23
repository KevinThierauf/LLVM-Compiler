use std::path::Path;
use std::process::{Command, exit};
use log::error;

const LLC_PATH: &str = "llc.exe";
const LD_PATH: &str = "ld.lld.exe";

#[cfg(target_os = "windows")]
pub fn getExecutableExtension() -> &'static str {
    return ".exe";
}

#[cfg(target_os = "linux")]
pub fn getExecutableExtension() -> &'static str {
    return "";
}

fn checkPath(path: &str) {
    if let Ok(output) = Command::new(path).arg("--help").output() {
        if !output.status.success() {
            error!("error when running {path}, program returned exit code {:?}", output.status);
        }
    } else {
        error!("unable to find {path}, is it available on path?");
        exit(1);
    }
}

pub fn checkLinkerPath() {
    checkPath(LD_PATH);
    checkPath(LLC_PATH);
}

pub(in super) fn linkExecutable(bitcodePath: impl AsRef<Path>, executablePath: impl AsRef<Path>) {
    Command::new(LLC_PATH)
        .arg("-o").arg(executablePath.as_ref().as_os_str())
        .arg(bitcodePath.as_ref().as_os_str())
        .spawn().expect(&format!("failed to run {LLC_PATH}"));

    Command::new(LD_PATH)
        .arg("-o").arg(executablePath.as_ref().as_os_str())
        .arg(bitcodePath.as_ref().as_os_str())
        .spawn().expect(&format!("failed to run {LLC_PATH}"));
}
