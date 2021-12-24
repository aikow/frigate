use clap::Parser;
use log::{debug, error, info, trace, warn};

#[derive(Parser, Debug)]
#[clap(name = "Frigate")]
#[clap(author = "Aiko Wessels <aiko.wessels@gmail.com>")]
#[clap(version = "0.1")]
#[clap(about = "Carefully ships your dotfiles from one computer to the next.")]
struct Args {
    /// Name of the config file.
    #[clap(short, long)]
    config_file: String,

    /// The base directory for the config files.
    #[clap(short, long)]
    base_dir: String,

}

pub fn start() {
    let args = Args::parse();

    println!("{}", args.config_file);
    println!("{}", args.base_dir);
}