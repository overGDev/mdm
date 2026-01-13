use clap::{ArgMatches, Command};

use crate::core::{error::MDMError, model::CliCommand};

pub fn subcommand_from_input(
    app: Command,
    subcommands: Vec<Box<dyn CliCommand>>,
) -> Result<(Box<dyn CliCommand>, ArgMatches), MDMError> {
    let matches = app.try_get_matches().map_err(|e| {
        match e.kind() {
            clap::error::ErrorKind::InvalidSubcommand => MDMError::UnknownSubcommand,
            _ => {
                e.exit();
            }
        }
    })?;

    let (subcommand_name, subcommand_args) = match matches.subcommand() {
        Some(s) => s,
        None => {
            return Err(MDMError::NoSubcommandProvided);
        }
    };

    subcommands
        .into_iter()
        .find(|c| c.name() == subcommand_name)
        .ok_or(MDMError::UnknownSubcommand)
        .map(|s| (s, subcommand_args.clone()))
}