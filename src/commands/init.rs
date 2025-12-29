use clap::{Command};

use crate::core::{
    error::MDMError,
    model::{CliCommand, CommandCtx},
};

const COMMAND_NAME: &str = "init";
const COMMAND_ABOUT: &str = "generates the main folder structure for the document";
const COMMAND_LONG_ABOUT: &str = "";

pub struct InitCommand {}

impl CliCommand for InitCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_config(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        return Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .long_about(COMMAND_LONG_ABOUT)
    }

    fn run(&self, _ctx: &CommandCtx) -> Result<(), MDMError> {
        println!("init command");
        Ok(())
    }
}
