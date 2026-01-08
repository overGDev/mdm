use std::{collections::HashMap, env, path::PathBuf};

use serde::de::DeserializeOwned;

use crate::core::{app::{ConfigFile, MDM_CONF_FOLDER_NAME}, error::MDMError, model::{ConfigLoader, MDMConfig, PathsConfig, SchemaSection, schema_config::SchemaConfig}};

pub struct YamlConfLoader {
    base_path: PathBuf,
}

fn find_config_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().ok()?;
    loop {
        let config_path = current_dir.join(MDM_CONF_FOLDER_NAME);
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

impl YamlConfLoader {
    pub fn new() -> Option<Self> {
        let root = find_config_root()?;
        Some(Self { base_path: root })
    }

    fn get_full_path(&self, file_name: &str) -> PathBuf {
        self.base_path.join(MDM_CONF_FOLDER_NAME).join(file_name)
    }

    pub fn config_from_file<T: DeserializeOwned>(&self, file_name: &str) -> Result<T, MDMError> {
        let full_path = self.get_full_path(file_name);
        
        let content = std::fs::read_to_string(&full_path)
            .map_err(|e| MDMError::IO { 
                source: e,
                path: full_path,
            })?;

        let config = serde_norway::from_str(&content)
            .map_err(|e| MDMError::Parse(e))?;

        Ok(config)
    }
}

impl ConfigLoader for YamlConfLoader {
    fn load_config(&self) -> Result<MDMConfig, MDMError> {
        let paths: PathsConfig = self.config_from_file(
            ConfigFile::Paths.name()
        )?;
        let raw_schema: Option<SchemaConfig> = self.config_from_file(ConfigFile::Schema.name()).ok();
        let vars: HashMap<String, String> = self.config_from_file(
            ConfigFile::Vars.name()
        )?;

        let schema = raw_schema
            .map(|raw| {
                raw.sections
                    .into_iter()
                    .map(|section| section.try_into())
                    .collect::<Result<Vec<SchemaSection>, MDMError>>()
            }).transpose()?;
            

        Ok(MDMConfig {
            paths,
            schema,
            vars,
        })
    }
}