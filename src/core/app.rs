use std::{env, fs::read_to_string, path::PathBuf};

use clap::{ArgMatches, Command};

use crate::{
    core::{
        error::MDMError,
        model::{CliCommand, PathsConfig},
    },
    io::parse::document_config_from_yaml,
};

const SCHEMA_FILE_NAME: &str = "schema.yaml";
const PATHS_FILE_NAME: &str = "paths.yaml";
const VARS_FILE_NAME: &str = "vars.yaml";
const SCHEMA_FILE_SAMPLE: &str = include_str!("../samples/schema.yaml");
const PATHS_FILE_SAMPLE: &str = include_str!("../samples/paths.yaml");
const VARS_FILE_SAMPLE: &str = include_str!("../samples/vars.yaml");

pub const MDM_CONF_FOLDER_NAME: &str = "mdm";
pub const MDM_CONF_FILES: [(&str, &str); 3] = [
    (SCHEMA_FILE_NAME, SCHEMA_FILE_SAMPLE),
    (PATHS_FILE_NAME, PATHS_FILE_SAMPLE),
    (VARS_FILE_NAME, VARS_FILE_SAMPLE),
];

pub const MDM_GIT_IGNORE_SAMPLE: &str = include_str!("../samples/.gitignore");

pub fn subcommand_from_input(
    app: Command,
    subcommands: Vec<Box<dyn CliCommand>>,
) -> Result<(Box<dyn CliCommand>, ArgMatches), MDMError> {
    let matches = app.get_matches();

    let (subcommand_name, subcommand_args) = match matches.subcommand() {
        Some(s) => s,
        None => {
            return Err(MDMError::NoSubcommandProvided);
        }
    };

    subcommands
        .into_iter()
        .find(|c| c.name() == subcommand_name)
        .ok_or(MDMError::UnknownSubcommand)
        .map(|s| (s, subcommand_args.clone()))
}

fn find_config_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;
    loop {
        let config_path = current_dir.join(PATHS_FILE_NAME);
        if config_path.exists() {
            return Some(current_dir);
        }
        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => break,
        }
    }
    None
}

pub fn load_config() -> Result<PathsConfig, MDMError> {
    let Some(conf_root) = find_config_root() else {
        return Err(MDMError::MDMConfigNotFound)
    };
    let content = read_to_string(conf_root.join(PATHS_FILE_NAME)).map_err(|e| MDMError::IO {
        source: e,
        path: PATHS_FILE_NAME.into(),
    })?;
    let config = document_config_from_yaml(&content)?;

    Ok(config)
}
