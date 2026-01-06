use serde_norway::Error;

use crate::core::model::{RawDocumentConfig, RawNodeConfig};

pub fn node_config_from_yaml(content: &str) -> Result<RawNodeConfig, Error> {
    let raw: RawNodeConfig = serde_norway::from_str(content)?;
    Ok(raw)
}

pub fn document_config_from_yaml(content: &str) -> Result<RawDocumentConfig, Error> {
    let raw: RawDocumentConfig = serde_norway::from_str(content)?;
    Ok(raw)
}
