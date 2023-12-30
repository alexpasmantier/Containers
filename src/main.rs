#[allow(dead_code)]
use std::process::{Command, Stdio};

use anyhow::Result;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args = std::env::args().collect();
    let arguments = parse_args(args);
    // println!("Image: {}", &arguments.image);
    // println!(
    //     "Command: {} {}",
    //     &arguments.command,
    //     &arguments.command_arguments.join(" ")
    // );

    Command::new(arguments.command)
        .args(arguments.command_arguments)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    Ok(())
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
