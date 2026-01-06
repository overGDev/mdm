use std::{env, fs::read_to_string, path::PathBuf};

use clap::{ArgMatches, Command};

use crate::{
    core::{
        error::MDMError,
        model::{CliCommand, DocumentConfig},
    },
    io::parse::document_config_from_yaml,
};

const SECTIONS_FOLDER_DEFAULT_NAME: &str = "sections";
pub const RESERVED_FOLDERS: [&str; 1] = [
    SECTIONS_FOLDER_DEFAULT_NAME,
];

const CONF_FILE_DEFAULT_NAME: &str = ".mdm.conf";
const SCHEMA_FILE_DEFAULT_NAME: &str = "schema.yaml";
const OUTPUT_FILE_DEFAULT_NAME: &str = "document.md";
const CONF_FILE_SAMPLE: &str = include_str!("../samples/.mdm.conf");
const SCHEMA_FILE_SAMPLE: &str = include_str!("../samples/schema.yaml");

pub const RESERVED_FILES: [(&str, &str); 3] = [
    (CONF_FILE_DEFAULT_NAME, CONF_FILE_SAMPLE),
    (SCHEMA_FILE_DEFAULT_NAME, SCHEMA_FILE_SAMPLE),
    (OUTPUT_FILE_DEFAULT_NAME, ""),
];

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
        let config_path = current_dir.join(CONF_FILE_DEFAULT_NAME);
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

pub fn load_config() -> Result<DocumentConfig, MDMError> {
    let Some(conf_root) = find_config_root() else {
        return Err(MDMError::MDMConfigNotFound)
    };
    let content = read_to_string(conf_root.join(CONF_FILE_DEFAULT_NAME)).map_err(|e| MDMError::IO {
        source: e,
        path: CONF_FILE_DEFAULT_NAME.into(),
    })?;
    let raw_config = document_config_from_yaml(&content)?;
    let config: DocumentConfig = raw_config
        .try_into()
        .map_err(|e: &str| MDMError::Other(e.to_string()))?;

    Ok(config)
}
