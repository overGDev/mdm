use std::io;

use clap::{Arg, Command};
use clap_complete::Shell;

use crate::core::{app::{build_app, APP_NAME}, error::MDMError, model::{CliCommand, CommandCtx}};

const COMMAND_NAME: &str = "completions";
const COMMAND_ABOUT: &str = "Print a shell completion script for mdm";
const COMMAND_LONG_ABOUT: &str = "Prints a completion script for the given shell to stdout. Save it wherever your shell loads completions from, e.g.: 'mdm completions zsh > ~/.zsh/completions/_mdm'";

const SHELL_ARG_ID: &str = "shell";

pub struct CompletionsCommand {}

impl CliCommand for CompletionsCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        false
    }

    fn build(&self) -> Command {
        Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .long_about(COMMAND_LONG_ABOUT)
            .args([
                Arg::new(SHELL_ARG_ID)
                    .required(true)
                    .value_parser(clap::value_parser!(Shell))
                    .value_name("SHELL")
                    .help("Shell to generate the completion script for"),
            ])
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let shell = *ctx.args.get_one::<Shell>(SHELL_ARG_ID)
            .ok_or(MDMError::InvalidCommandState {
                reason: "Missing required SHELL argument".into(),
                help: "Provide a shell, e.g.: 'mdm completions zsh'".into(),
            })?;

        let mut app = build_app(&crate::commands::all());
        clap_complete::generate(shell, &mut app, APP_NAME, &mut io::stdout());
        Ok(())
    }
}
