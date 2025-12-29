use std::process;

use crate::{
    commands::InitCommand,
    core::{
        load_config,
        subcommand_from_input,
        ext::CommandExt,
        model::{CliCommand, CommandCtx},
    },
};

pub mod commands;
pub mod core;
pub mod io;

const APP_NAME: &str = "mdm";
const APP_ABOUT: &str = "CLI application that allows for better management of docs as code";
const APP_LONG_ABOUT: &str = "";

fn main() {
    let subcommands: Vec<Box<dyn CliCommand>> = vec![Box::new(InitCommand {})];
    let app = clap::Command::new(APP_NAME)
        .about(APP_ABOUT)
        .long_about(APP_LONG_ABOUT)
        .arg_required_else_help(true)
        .load_subcommands(&subcommands);

    let (subcommand, args) = subcommand_from_input(app, subcommands)
        .map_err(|e| {
            eprintln!("{}", e);
            process::exit(1);
        })
        .unwrap();

    let config = if subcommand.requires_config() {
        let loaded = load_config()
            .map_err(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            })
            .unwrap();
        Some(loaded)
    } else {
        None
    };

    match subcommand.execute(CommandCtx { args, config }) {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(()) => {
            std::process::exit(0);
        }
    };
}
