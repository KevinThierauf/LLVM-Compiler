use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, exit};

use log::error;

#[cfg(target_os = "windows")]
const LLC_PATH: &str = "llc.exe";
#[cfg(target_os = "windows")]
const LD_PATH: &str = "lld-link.exe";

#[cfg(target_os = "linux")]
const LLC_PATH: &str = "llc";
#[cfg(target_os = "linux")]
const LD_PATH: &str = "ld.lld";

#[cfg(target_os = "windows")]
pub fn getExecutableExtension() -> &'static str {
    return ".exe";
}

#[cfg(target_os = "linux")]
pub fn getExecutableExtension() -> &'static str {
    return "";
}

fn checkPath(path: &str, arg: impl AsRef<OsStr>) {
    if let Ok(output) = Command::new(path).arg(arg).output() {
        if !output.status.success() {
            error!("error when running {path}, program returned exit code {:?}", output.status);
            exit(3);
        }
    } else {
        error!("unable to find {path}, is it available on path?");
        exit(2);
    }
}

pub fn checkLinkerPath() {
    checkPath(LD_PATH, "-help");
    checkPath(LLC_PATH, "--help");
}

#[cfg(windows)]
fn link(entryName: &str, bitcodePath: impl AsRef<Path>, executablePath: impl AsRef<Path>) -> bool {
    return Command::new(LD_PATH)
        .arg(format!("/out:{}", executablePath.as_ref().as_os_str().to_str().unwrap()))
        .arg(format!("/entry:{entryName}"))
        .arg(format!("/defaultlib:lib/sdk/target/debug/sdk.dll.lib"))
        .arg("/subsystem:console")
        .arg(bitcodePath.as_ref().as_os_str())
        .status().expect(&format!("failed to run {LD_PATH}")).success();
}

#[cfg(linux)]
fn link(bitcodePath: impl AsRef<Path>, executablePath: impl AsRef<Path>) -> bool {
    return Command::new(LD_PATH)
        .arg("-o").arg(executablePath.as_ref())
        .arg(format!("entry={entryName}"))
        .arg(bitcodePath.as_ref().as_os_str())
        .status().expect(&format!("failed to run {LD_PATH}")).success();
}

pub(in super) fn linkExecutable(entryName: &str, bitcodePath: impl AsRef<Path>, executablePath: impl AsRef<Path>) {
    Command::new(LLC_PATH)
        .arg("-o").arg(executablePath.as_ref().as_os_str())
        .arg(bitcodePath.as_ref().as_os_str())
        .spawn().expect(&format!("failed to run {LLC_PATH}"));
    assert!(!entryName.is_empty());
    if !(link(entryName, bitcodePath, executablePath)) {
        error!("Linking failed");
        exit(5);
    }
}
