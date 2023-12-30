use crate::containerization::prepare_temp_dir;
use crate::parsing::arguments::parse_args;
use containerization::{chroot, unshare};
use core::panic;
use registry::authentication::authenticate;
use registry::layers::pull_layers;
use registry::manifest::get_manifest;
use std::io::{self, Write};
use std::process::{Command, ExitCode, Stdio};

mod containerization;
mod parsing;
mod registry;

const DOCKER_EXPLORER_PATH: &str = "/usr/local/bin/docker-explorer";
const SH_PATH: &str = "/bin/sh";

fn main() -> ExitCode {
    let args = std::env::args().collect();
    let arguments = parse_args(args);
    // println!("Image: {}", &arguments.image);
    // println!(
    //     "Command: {} {}",
    //     &arguments.command,
    //     &arguments.command_arguments.join(" ")
    // );

    // authenticate to the registry
    let auth = authenticate(&arguments.image).expect("Unable to authenticate to registry");
    // get manifest
    let manifest = get_manifest(&arguments.image, &auth).expect("Unable to get image manifest");
    // pull image layers
    let layers = pull_layers(&arguments.image, &manifest, &auth).expect("Unable to pull layers");

    // create temp dir for the container
    let bin_paths = [DOCKER_EXPLORER_PATH, SH_PATH];
    let temp_dir = prepare_temp_dir(&bin_paths, layers);

    // chroot into it
    let chroot_result = chroot(temp_dir.path().to_str().unwrap());
    if chroot_result != 0 {
        panic!("chroot failed with exit code {chroot_result}");
    };

    // create new pid namespace
    if unshare() != 0 {
        eprintln!("Failed to unshare");
    };

    let output = Command::new(arguments.command)
        .args(arguments.command_arguments)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    let exit_code = match output.status.code() {
        Some(code) => ExitCode::from(code as u8),
        None => ExitCode::from(0),
    };
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    exit_code
}
