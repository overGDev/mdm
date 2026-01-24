use std::path::PathBuf;

use indexmap::IndexMap;

use crate::core::{error::MDMError, model::{PathsConfig, SchemaSection}};

pub struct MDMConfig {
    pub root: PathBuf,
    pub paths: PathsConfig,
    pub schema: Option<Vec<SchemaSection>>,
    pub vars: IndexMap<String, String>
}

impl MDMConfig {
    pub fn require_schema(&self, reason: &str, help: &str) -> Result<&Vec<SchemaSection>, MDMError> {
        self.schema.as_ref().ok_or_else(|| MDMError::InvalidCommandState {
            reason: reason.into(),
            help: help.into(),
        })
    }
}