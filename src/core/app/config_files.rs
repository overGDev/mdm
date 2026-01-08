use crate::core::{error::MDMError, model::{ConfigLoader, MDMConfig}};

pub enum ConfigFile {
    Schema,
    Paths,
    Vars,
}

impl ConfigFile {
    pub fn name(&self) -> &str {
        match self {
            Self::Schema => "schema.yaml",
            Self::Paths => "paths.yaml",
            Self::Vars => "vars.yaml",
        }
    }

    pub fn sample_content(&self) -> &str {
        match self {
            Self::Schema => include_str!("../../samples/schema.yaml"),
            Self::Paths => include_str!("../../samples/paths.yaml"),
            Self::Vars => include_str!("../../samples/vars.yaml"),
        }
    }

    pub fn all() -> [Self; 3] {
        [Self::Schema, Self::Paths, Self::Vars]
    }
}

pub const MDM_CONF_FOLDER_NAME: &str = "mdm";
pub const MDM_GIT_IGNORE_SAMPLE: &str = include_str!("../../samples/.gitignore");

pub fn load_config(conf_loader: Box<dyn ConfigLoader>) -> Result<MDMConfig, MDMError> {
    let config = conf_loader.load_config()?;

    Ok(config)
}