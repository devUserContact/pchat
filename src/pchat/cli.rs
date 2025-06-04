use clap::{Arg, ArgAction, Command};

pub fn run() -> String {
    let cli = Command::new("pChat")
        .version("0.0.1")
        .author("Your Name <your.email@example.com>")
        .about("Terminal-based Ollama chat for your rust project 🦀🦙")
        .arg(
            Arg::new("prompt")
                .help("The prompt for the chat application")
                .short('p')
                .required(false)
                .num_args(1..)
                .action(ArgAction::Append)
                .value_delimiter(' '),
        )
        .get_matches();

    let prompt_args = cli
        .get_many::<String>("prompt")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let prmt: String = prompt_args.join(" ");
    prmt
}
