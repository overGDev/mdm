use std::path::PathBuf;

use indexmap::IndexMap;

use crate::core::model::{PathsConfig, SchemaSection};

pub struct MDMConfig {
    pub root: PathBuf,
    pub paths: PathsConfig,
    pub schema: Option<Vec<SchemaSection>>,
    pub vars: IndexMap<String, String>
}