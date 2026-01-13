use clap::Command;

use crate::core::{error::MDMError, model::CommandCtx};

/// Declares helper for easy management of clap Commands as types.
pub trait CliCommand {
    fn name(&self) -> &str;
    fn requires_paths(&self) -> bool;
    /// Returns the instance of clap Command.
    fn build(&self) -> Command;
    /// Defines the custom functionality of a command.
    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError>;
    /// Orquestrates validation before running the command's custom functionality.
    fn execute(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        if self.requires_paths() && ctx.config.is_none() {
            panic!();
        }
        self.run(ctx)?;
        Ok(())
    }
}
