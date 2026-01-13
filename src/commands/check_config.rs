use clap::Command;
use crate::core::{error::MDMError, model::{CliCommand, CommandCtx}};

const COMMAND_NAME: &str = "check";

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
            .about("Verify if MDM configuration is valid and readable")
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.config
            .as_ref()
            .ok_or(MDMError::MDMConfigNotFound)?;

        println!("Configuration root found at: {}", config.root.display());
        Ok(())
    }
}