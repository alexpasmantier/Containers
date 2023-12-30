#[derive(Debug, Clone)]
pub struct Image {
    pub repository: String,
    pub tag: Option<String>,
}

#[derive(Debug)]
pub struct Arguments {
    pub image: Image,
    pub command: String,
    pub command_arguments: Vec<String>,
}

/// parses the command line arguments used to invoke this program
pub fn parse_args(args: Vec<String>) -> Arguments {
    let mut arguments = args.into_iter();
    let executable = arguments.next().unwrap();
    let docker_command = arguments
        .next()
        .expect("please provide a docker command to use");
    let image = arguments.next().expect("please provide an image to run");
    let mut image_parts = image.split("/");
    let repository = image_parts.next().expect("Invalid image").to_owned();
    let tag = image_parts.next().map(str::to_owned);
    let command = arguments
        .next()
        .expect("please provide a command to run on your image");
    Arguments {
        image: Image { repository, tag },
        command,
        command_arguments: arguments.collect(),
    }
}
