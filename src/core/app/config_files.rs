use std::path::PathBuf;

use crate::core::{error::MDMError, model::{ConfigLoader, MDMConfig}};

pub enum ConfigFile {
    Schema,
    Paths,
    Vars,
    Workflow,
    ClaudeMd,
    PreCommitHook,
}

impl ConfigFile {
    pub fn name(&self) -> &str {
        match self {
            Self::Schema => "schema.yaml",
            Self::Paths => "paths.yaml",
            Self::Vars => "vars.yaml",
            Self::Workflow => "mdm-build.yml",
            Self::ClaudeMd => "CLAUDE.md",
            Self::PreCommitHook => "pre-commit",
        }
    }

    /// Path to this file, relative to the project's root (i.e. the parent of 'mdm/').
    pub fn relative_path(&self) -> PathBuf {
        match self {
            Self::Schema | Self::Paths | Self::Vars => {
                PathBuf::from(MDM_CONF_FOLDER_NAME).join(self.name())
            }
            Self::Workflow => PathBuf::from(".github/workflows").join(self.name()),
            Self::ClaudeMd => PathBuf::from(self.name()),
            Self::PreCommitHook => PathBuf::from(".githooks").join(self.name()),
        }
    }

    pub fn sample_content(&self) -> &str {
        match self {
            Self::Schema => include_str!("../../samples/schema.yaml"),
            Self::Paths => include_str!("../../samples/paths.yaml"),
            Self::Vars => include_str!("../../samples/vars.yaml"),
            Self::Workflow => include_str!("../../samples/mdm-build.yml"),
            Self::ClaudeMd => include_str!("../../samples/CLAUDE.md"),
            Self::PreCommitHook => include_str!("../../samples/pre-commit"),
        }
    }

    pub fn all() -> [Self; 6] {
        [Self::Schema, Self::Paths, Self::Vars, Self::Workflow, Self::ClaudeMd, Self::PreCommitHook]
    }
}

pub const MDM_CONF_FOLDER_NAME: &str = "mdm";
pub const MDM_GIT_IGNORE_SAMPLE: &str = include_str!("../../samples/.gitignore");

pub fn load_config(conf_loader: Box<dyn ConfigLoader>) -> Result<MDMConfig, MDMError> {
    let config = conf_loader.load_config()?;

    Ok(config)
}