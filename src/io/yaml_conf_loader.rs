use std::{env, path::PathBuf};

use indexmap::IndexMap;
use serde::de::DeserializeOwned;

use crate::core::{app::{ConfigFile, MDM_CONF_FOLDER_NAME}, error::MDMError, model::{ConfigLoader, MDMConfig, PathsConfig, SchemaSection, schema_config::SchemaConfig}};

pub struct YamlConfLoader {
    pub base_path: PathBuf,
}

impl YamlConfLoader {
    fn find_config_root() -> Result<PathBuf, MDMError> {
        let mut current_dir = env::current_dir()
            .map_err(|_| MDMError::Other(
                "error: Failed to determine current workdir".into()
            ))?;
        loop {
            let config_path = current_dir.join(MDM_CONF_FOLDER_NAME);
            if config_path.exists() {
                return Ok(current_dir);
            }
            match current_dir.parent() {
                Some(parent) => current_dir = parent.to_path_buf(),
                None => break,
            }
        }
        Err(MDMError::MDMConfigNotFound)
    }

    pub fn new() -> Result<Self, MDMError> {
        let root = Self::find_config_root()?;
        Ok(Self { base_path: root })
    }

    pub fn config_from_file<T: DeserializeOwned>(&self, file_name: &str) -> Result<T, MDMError> {
        let full_path = self.base_path
            .join(MDM_CONF_FOLDER_NAME)
            .join(file_name);
        
        let content = std::fs::read_to_string(&full_path)
            .map_err(|e| MDMError::IO { 
                source: e,
                path: full_path,
            })?;

        let config = serde_yaml::from_str(&content)
            .map_err(|e| MDMError::Parse(e))?;

        Ok(config)
    }
}

impl ConfigLoader for YamlConfLoader {
    fn load_config(&self) -> Result<MDMConfig, MDMError> {
        let vars: IndexMap<String, String> = self.config_from_file(
            ConfigFile::Vars.name()
        )?;
        
        let mut paths: PathsConfig = self.config_from_file(
            ConfigFile::Paths.name()
        )?;
        paths.establish_root(&self.base_path);

        let raw_schema: Option<SchemaConfig> = self.config_from_file(
            ConfigFile::Schema.name()
        )?;
        let schema = raw_schema
            .map(|raw| {
                raw.sections
                    .into_iter()
                    .map(|section| section.try_into())
                    .collect::<Result<Vec<SchemaSection>, MDMError>>()
            }).transpose()?;

        let root = self.base_path.clone();
        Ok(MDMConfig {
            root,
            paths,
            schema,
            vars,
        })
    }
}