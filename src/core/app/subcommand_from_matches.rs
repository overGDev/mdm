use clap::{ArgMatches};

use crate::core::{error::MDMError, model::CliCommand};

pub fn subcommand_from_matches(
    matches: ArgMatches,
    subcommands: Vec<Box<dyn CliCommand>>,
) -> Result<(Box<dyn CliCommand>, ArgMatches), MDMError> {
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