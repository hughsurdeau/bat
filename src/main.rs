#![allow(unused)]

mod cli;

use std::path::PathBuf;
use clap::{Arg, App, Parser};
use structopt::StructOpt;
use std::fs;
use cli::commands as commands;

fn main() {
    let args = cli::CommandLineArgs::from_args();
    let command = args.command;
    println!("{:#?}", command);
    match command {
        commands::Command::Add {text} => add(text, args.file_path),
        commands::Command::Create {text} => create(text, args.file_path),
    }

}

fn create(text: String, file_path: PathBuf) {
    let path_string = file_path.into_os_string().into_string().unwrap();
    fs::write(path_string, text).expect("Unable to write file");
}

fn add(text: String, file_path: PathBuf) {
    let path_string = file_path.into_os_string().into_string().unwrap();
    fs::write(path_string, text).expect("Unable to write file");
}
