use clap::Command;

use crate::core::{error::MDMError, model::{CliCommand, CommandCtx}};

const COMMAND_NAME: &str = "list";
const COMMAND_ABOUT: &str = "List all variables set on 'mdm/config.yaml'.";

pub struct ListCommand {}

impl CliCommand for ListCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        return Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.require_config()?;
        for (var_name, var_value) in config.vars.iter() {
            println!("{}:\t{}", var_name, var_value);
        }
        Ok(())
    }
}