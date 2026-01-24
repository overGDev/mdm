use crate::{
    commands::{BuildCommand, CheckCommand, InitCommand, SyncCommand, VarCommand}, core::{
        app::{config_files::load_config, subcommand_from_matches::subcommand_from_matches}, ext::CommandExt, model::{CliCommand, CommandCtx, ConfigLoader}
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
        Box::new(VarCommand {}),
        Box::new(BuildCommand {}),
    ];
    let app = clap::Command::new(APP_NAME)
        .about(APP_ABOUT)
        .long_about(APP_LONG_ABOUT)
        .load_subcommands(&subcommands);

    let matches = match app.get_cli_matches() {
        Ok(m) => m,
        Err(e) => e.print_and_abort(),
    };
    let (subcommand, args) = match subcommand_from_matches(matches, subcommands) {
        Ok(s) => s,
        Err(e) => e.print_and_abort(),
    };

    let config = if subcommand.requires_paths() {
        let loader = match YamlConfLoader::new() {
            Ok(loader) => Box::new(loader) as Box<dyn ConfigLoader>,
            Err(e) => e.print_and_abort(),
        };
        let loaded = match load_config(loader) {
            Ok(config) => config,
            Err(e) => e.print_and_abort(),
        };
        Some(loaded)
    } else {
        None
    };

    match subcommand.execute(CommandCtx { args, config }) {
        Err(e) => e.print_and_abort(),
        Ok(()) => (),
    };
}
