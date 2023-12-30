#![allow(dead_code, unused_variables)]
use core::panic;
use libc;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::{ffi, fs};
use tempfile::{tempdir, TempDir};

const DOCKER_EXPLORER: &str = "/usr/local/bin/docker-explorer";

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> ExitCode {
    let args = std::env::args().collect();
    let arguments = parse_args(args);
    // println!("Image: {}", &arguments.image);
    // println!(
    //     "Command: {} {}",
    //     &arguments.command,
    //     &arguments.command_arguments.join(" ")
    // );

    let bin_paths = vec![Path::new(DOCKER_EXPLORER)];
    let temp_dir = prepare_temp_dir(&bin_paths);
    // chroot into it
    let temp_dir_cstring = ffi::CString::new(temp_dir.path().to_str().unwrap()).unwrap();
    let chroot_result = unsafe { libc::chroot(temp_dir_cstring.as_ptr()) };
    if chroot_result != 0 {
        panic!("chroot failed with exit code {chroot_result}");
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

#[derive(Debug)]
struct Arguments {
    image: String,
    command: String,
    command_arguments: Vec<String>,
}

fn parse_args(args: Vec<String>) -> Arguments {
    let mut arguments = args.into_iter();
    let executable = arguments.next().unwrap();
    let docker_command = arguments
        .next()
        .expect("please provide a docker command to use");
    let image = arguments.next().expect("please provide an image to run");
    let command = arguments
        .next()
        .expect("please provide a command to run on your image");
    Arguments {
        image,
        command,
        command_arguments: arguments.collect(),
    }
}

/// Creates a temporary directory, creates /dev/null inside it, and copies the given binaries as
/// well
fn prepare_temp_dir(bin_paths: &Vec<&Path>) -> TempDir {
    let temp_dir = tempdir().expect("Failed to create temporary directory for the container");
    // create /dev/null inside temp dir
    fs::create_dir(temp_dir.path().join("dev")).expect("Failed to create dev/ inside temp dir");
    fs::write(temp_dir.path().join("dev/null"), b"")
        .expect("Failed to create dev/null inside temp dir");
    // copy binaries
    for bin_path in bin_paths {
        let p = temp_dir.path().join(
            bin_path
                .strip_prefix("/")
                .expect("Binary paths should be absolute"),
        );
        // create intermediary directories
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::copy(bin_path, p).expect(&format!("Unable to copy {:?} to temp dir", bin_path));
    }

    temp_dir
}
