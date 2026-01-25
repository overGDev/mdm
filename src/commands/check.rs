use clap::Command;
use crate::core::{error::MDMError, model::{CliCommand, CommandCtx}};

const COMMAND_NAME: &str = "check";
const COMMAND_ABOUT: &str = "Verify the current projects root dir.";

pub struct CheckCommand;

impl CliCommand for CheckCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.config
            .as_ref()
            .ok_or(MDMError::MDMConfigNotFound)?;

        println!("Configuration root found at: {}.", config.root.display());
        Ok(())
    }
}