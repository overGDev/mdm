use clap::Command;

use crate::{commands::{IndexCommand, SectionsCommand}, core::{app::subcommand_from_matches::subcommand_from_matches, error::MDMError, ext::CommandExt, model::{CliCommand, CommandCtx}}};

const COMMAND_NAME: &str = "sync";
const COMMAND_ABOUT: &str = "Update folders and files to'mdm/schema.yaml' contents";

pub struct SyncCommand {}

impl SyncCommand {
    fn subcommands(&self) -> Vec<Box<dyn CliCommand>> {
        vec![
            Box::new(SectionsCommand {}),
            Box::new(IndexCommand {}),
        ]
    }
}

impl CliCommand for SyncCommand {
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