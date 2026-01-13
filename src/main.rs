use crate::{
    commands::{CheckCommand, InitCommand, SyncCommand}, core::{
        app::{config_files::load_config, get_subcommand::subcommand_from_input}, ext::CommandExt, model::{CliCommand, CommandCtx, ConfigLoader}
    }, io::yaml_conf_loader::YamlConfLoader
};

pub mod commands;
pub mod core;
pub mod io;

const APP_NAME: &str = "mdm";
const APP_ABOUT: &str = "Manage documentation projects using Docs-as-Code workflows";
const APP_LONG_ABOUT: &str = "MDM is a tool designed to manage documentation within version-controlled environments with Git. It streamlines the document lifecycle by providing a modular and flexible framework for handling the iterative growth of collaborative documents, such as technical documentation of services or applications.";

fn main() {
    let subcommands: Vec<Box<dyn CliCommand>> = vec![
        Box::new(InitCommand {}),
        Box::new(SyncCommand {}),
        Box::new(CheckCommand {}),
    ];
    let app = clap::Command::new(APP_NAME)
        .about(APP_ABOUT)
        .long_about(APP_LONG_ABOUT)
        .load_subcommands(&subcommands);

    let (subcommand, args) = subcommand_from_input(app, subcommands)
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });

    let config = if subcommand.requires_paths() {
        let loader = YamlConfLoader::new()
            .map(|instance| Box::new(instance) as Box<dyn ConfigLoader>)
            .unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });

        let loaded = load_config(loader)
            .unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });
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
