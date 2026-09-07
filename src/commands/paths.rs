use clap::Command;

use crate::{commands::PathsSetCommand, core::{app::subcommand_from_matches::subcommand_from_matches, error::MDMError, ext::CommandExt, model::{CliCommand, CommandCtx}}};

const COMMAND_NAME: &str = "paths";
const COMMAND_ABOUT: &str = "Subcommand to access path configuration operations";

pub struct PathsCommand {}

impl PathsCommand {
    fn subcommands(&self) -> Vec<Box<dyn CliCommand>> {
        vec![
            Box::new(PathsSetCommand {}),
        ]
    }
}

impl CliCommand for PathsCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        return Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .load_subcommands(&self.subcommands())

    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let (subcommand, args) = subcommand_from_matches(
            ctx.args,
            self.subcommands()
        )?;
        let sub_ctx = CommandCtx {
            args,
            config: ctx.config,
        };
        subcommand.execute(sub_ctx)?;
        Ok(())
    }
}
