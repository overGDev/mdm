use clap::{Arg, ArgAction, Command};

use crate::core::{error::MDMError, model::{CliCommand, CommandCtx}};

const COMMAND_NAME: &str = "update";
const COMMAND_ABOUT: &str = "Update mdm to the latest release";
const COMMAND_LONG_ABOUT: &str = "Runs the 'mdm-update' companion program installed alongside mdm by the install script, which checks GitHub Releases and replaces the current binary if a newer version is available";

const ARGS_ID: &str = "args";

#[cfg(windows)]
const UPDATER_BIN_NAME: &str = "mdm-update.exe";
#[cfg(not(windows))]
const UPDATER_BIN_NAME: &str = "mdm-update";

pub struct UpdateCommand {}

impl UpdateCommand {
    fn updater_path() -> Result<std::path::PathBuf, MDMError> {
        let current_exe = std::env::current_exe()
            .map_err(|e| MDMError::Other(format!("Failed to determine current executable path: {}", e)))?;
        let install_dir = current_exe.parent()
            .ok_or_else(|| MDMError::Other("Failed to determine install directory".into()))?;
        Ok(install_dir.join(UPDATER_BIN_NAME))
    }
}

impl CliCommand for UpdateCommand {
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
                Arg::new(ARGS_ID)
                    .required(false)
                    .action(ArgAction::Append)
                    .num_args(0..)
                    .allow_hyphen_values(true)
                    .trailing_var_arg(true)
                    .help("Extra flags forwarded as-is to 'mdm-update'"),
            ])
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let updater_path = Self::updater_path()?;
        let passthrough_args: Vec<&String> = ctx.args
            .get_many::<String>(ARGS_ID)
            .map(|values| values.collect())
            .unwrap_or_default();

        match std::process::Command::new(&updater_path)
            .args(passthrough_args)
            .status() {
            Ok(exit_status) if exit_status.success() => Ok(()),
            Ok(exit_status) => Err(MDMError::InvalidCommandState {
                reason: "mdm-update exited with a failure".into(),
                help: format!("It exited with status: {}", exit_status),
            }),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(MDMError::InvalidCommandState {
                reason: format!("'{}' was not found next to the current mdm binary", UPDATER_BIN_NAME),
                help: "The updater ships alongside mdm since v1.3.0. Re-run the install script to get it, or your installation may not support self-updating (e.g. a package manager install).".into(),
            }),
            Err(error) => Err(MDMError::IO {
                source: error,
                path: updater_path,
            }),
        }
    }
}
