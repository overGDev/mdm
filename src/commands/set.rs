use clap::{Arg, ArgAction, Command};

use crate::core::{app::{ConfigFile, MDM_CONF_FOLDER_NAME}, error::MDMError, model::{CliCommand, CommandCtx}};

const COMMAND_NAME: &str = "set";
const COMMAND_ABOUT: &str = "Set the value of a variable inside of 'mdm/vars.yaml'";
const COMMAND_LONG_ABOUT: &str = "Sets the value for the specified key. If the key already exists, the command will abort to prevent accidental overwrites, unless the --force flag is used to update it.";

const KEY_ARG_ID: &str = "key";
const VALUE_ARG_ID: &str = "value";
const FORCE_FLAG_ID: &str = "force";

pub struct SetCommand {}

impl CliCommand for SetCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        return Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .long_about(COMMAND_LONG_ABOUT)
            .args([
                Arg::new(KEY_ARG_ID)
                    .required(true)
                    .value_name("KEY")
                    .help("name of keyword to associate with a value"),
                Arg::new(VALUE_ARG_ID)
                    .required(true)
                    .value_name("VALUE")
                    .help("value to replace a key with"),
                Arg::new(FORCE_FLAG_ID)
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .short('f')
                    .long("force")
                    .help("override the value of a variable already present"),
            ]);
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.config
            .ok_or(MDMError::InvalidCommandState {
                reason: "Failed to load config on a command that requires it.".into(),
                help: "Try running 'mdm check.".into(), 
            })?;

        let force = ctx.args.get_flag(FORCE_FLAG_ID);
        let provided_key = ctx.args.get_one::<String>(KEY_ARG_ID)
            .ok_or(MDMError::InvalidCommandState { 
                reason: "Missing required KEY argument.".into(),
                help: "Provide the key after the command, e.g.: 'mdm var set MY_KEY my_value'".into(),
            })?;
        let provided_value = ctx.args.get_one::<String>(VALUE_ARG_ID)
            .ok_or(MDMError::InvalidCommandState { 
                reason: "Missing required VALUE argument.".into(),
                help: "Provide a value for the key, e.g.: 'mdm var set MY_KEY my_value'".into(),
            })?;

        let mut new_vars = config.vars.clone();
        if new_vars.contains_key(provided_key) && !force {
            return Err(MDMError::InvalidCommandState {
                reason: format!("'{}' key exists already.", provided_key),
                help: "Use --force (-f) for command to override existing keys.".into()
            });
        }

        new_vars.insert(provided_key.clone(), provided_value.clone());
        let content = serde_yaml::to_string(&new_vars)
            .map_err(|e| MDMError::Parse(e))?;
        let vars_path = config.root
            .join(MDM_CONF_FOLDER_NAME)
            .join(ConfigFile::Vars.name());
        std::fs::write(&vars_path, content)
            .map_err(|e| MDMError::IO {
                source: e,
                path: vars_path,
            })?;
        Ok(())
    }
}