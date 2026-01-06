use serde_norway::Error;

use crate::core::model::{PathsConfig, RawSchemaConfig};

// I will make this match the other one by providing the core model 'NodeConfig' later
pub fn node_config_from_yaml(content: &str) -> Result<RawSchemaConfig, Error> {
    let raw: RawSchemaConfig = serde_norway::from_str(content)?;
    Ok(raw)
}

pub fn document_config_from_yaml(content: &str) -> Result<PathsConfig, Error> {
    let raw: PathsConfig = serde_norway::from_str(content)?;
    Ok(raw)
}
