use clap::{ArgMatches, Command, error::ErrorKind};

use crate::core::{error::MDMError, model::CliCommand};

pub trait CommandExt {
    fn get_cli_matches(self) -> Result<ArgMatches, MDMError>;
    fn load_subcommands(self, subs: &[Box<dyn CliCommand>]) -> Self;
}

impl CommandExt for Command {
    fn get_cli_matches(self) -> Result<ArgMatches, MDMError> {
        let matches = self.try_get_matches()
            .map_err(|e| {
                match e.kind() {
                    // clap represents --help/--version as an "error" carrying the text to
                    // print, not a real failure. e.exit() prints to the correct stream
                    // (stdout) and exits 0, matching normal CLI conventions.
                    ErrorKind::DisplayHelp
                    | ErrorKind::DisplayVersion
                    | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => e.exit(),
                    ErrorKind::InvalidSubcommand => MDMError::UnknownSubcommand,
                    _ => {
                        MDMError::Other(e.to_string())
                    }
                }
            })?;
        Ok(matches)
    }

    fn load_subcommands(mut self, subs: &[Box<dyn CliCommand>]) -> Self {
        for cmd in subs {
            self = self.subcommand(cmd.build());
        }
        self
    }
}
