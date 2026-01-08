use std::collections::HashMap;

use crate::core::model::{PathsConfig, SchemaSection};

pub struct MDMConfig {
    pub paths: PathsConfig,
    pub schema: Option<Vec<SchemaSection>>,
    pub vars: HashMap<String, String>
}