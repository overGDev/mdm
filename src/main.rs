use crate::{
    core::{
        app::{build_app, config_files::load_config, subcommand_from_matches::subcommand_from_matches}, ext::CommandExt, model::{CommandCtx, ConfigLoader}
    }, io::yaml_conf_loader::YamlConfLoader
};

pub mod commands;
pub mod core;
pub mod io;

fn main() {
    let subcommands = commands::all();
    let app = build_app(&subcommands);

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
