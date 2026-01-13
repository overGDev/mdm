use std::{collections::HashMap, path::PathBuf};

use crate::core::model::{PathsConfig, SchemaSection};

pub struct MDMConfig {
    pub root: PathBuf,
    pub paths: PathsConfig,
    pub schema: Option<Vec<SchemaSection>>,
    pub vars: HashMap<String, String>
}