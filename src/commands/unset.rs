use clap::{Arg, Command};

use crate::{commands::var::write_vars, core::{error::MDMError, model::{CliCommand, CommandCtx}}};

const COMMAND_NAME: &str = "unset";
const COMMAND_ABOUT: &str = "Remove a variable from 'mdm/vars.yaml'";

const KEY_ARG_ID: &str = "key";

pub struct UnsetCommand {}

impl CliCommand for UnsetCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .args([
                Arg::new(KEY_ARG_ID)
                    .required(true)
                    .value_name("KEY")
                    .help("Name of the variable to remove"),
            ])
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.require_config()?;
        let provided_key = ctx.args.get_one::<String>(KEY_ARG_ID)
            .ok_or(MDMError::InvalidCommandState {
                reason: "Missing required KEY argument".into(),
                help: "Provide the key to remove, e.g.: 'mdm var unset MY_KEY'".into(),
            })?;

        let mut new_vars = config.vars.clone();
        if new_vars.shift_remove(provided_key).is_none() {
            return Err(MDMError::InvalidCommandState {
                reason: format!("'{}' key does not exist", provided_key),
                help: "Use 'mdm var list' to see the defined keys".into(),
            });
        }

        write_vars(config, &new_vars)?;
        Ok(())
    }
}
