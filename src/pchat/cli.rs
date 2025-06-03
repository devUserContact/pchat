use clap::Parser;

pub fn run() -> String {
    #[derive(Parser)]
    pub struct Args {
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
        pub prompt: Vec<String>,
    }
    let args = Args::parse();
    let prmt: String = args.prompt.join(" ");
    return prmt;
}
