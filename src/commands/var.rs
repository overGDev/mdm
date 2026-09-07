use clap::Command;
use indexmap::IndexMap;

use crate::{commands::{GetCommand, ListCommand, SetCommand, UnsetCommand}, core::{app::{subcommand_from_matches::subcommand_from_matches, ConfigFile, MDM_CONF_FOLDER_NAME}, error::MDMError, ext::CommandExt, model::{CliCommand, CommandCtx, MDMConfig}}};

const COMMAND_NAME: &str = "var";
const COMMAND_ABOUT: &str = "Subcommand to access variable operations";

pub struct VarCommand {}

impl VarCommand {
    fn subcommands(&self) -> Vec<Box<dyn CliCommand>> {
        vec![
            Box::new(ListCommand {}),
            Box::new(GetCommand {}),
            Box::new(SetCommand {}),
            Box::new(UnsetCommand {}),
        ]
    }
}

/// Serializes 'vars' back into 'mdm/vars.yaml', shared by the 'set' and 'unset' subcommands.
pub(super) fn write_vars(config: &MDMConfig, vars: &IndexMap<String, String>) -> Result<(), MDMError> {
    let content = serde_yaml::to_string(vars)
        .map_err(|e| MDMError::Parse(e))?;
    let vars_path = config.root
        .join(MDM_CONF_FOLDER_NAME)
        .join(ConfigFile::Vars.name());
    std::fs::write(&vars_path, content)
        .map_err(|e| MDMError::IO {
            source: e,
            path: vars_path,
        })
}

impl CliCommand for VarCommand {
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