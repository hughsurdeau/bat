use std::path::PathBuf;
use structopt::StructOpt;

pub mod commands;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Bev's Automation Tools",
    about = "A command line tool for csv automation"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub command: commands::Command,

    #[structopt(parse(from_os_str), short, long)]
    pub file_path: PathBuf,
}


