use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::core::model::Validable;

/// Direct mapping of the project's configuration file on mdm.conf
/// Acts as the entry point for the deserialization process.
#[derive(Debug, Deserialize)]
pub struct RawDocumentConfig {
    pub paths: PathsConfig,
    pub vars: Option<HashMap<String, String>>,
}

/// Helper structure to group path-related settings in the mdm.conf file.
/// Ensures a logical organization of input and output locations.
#[derive(Debug, Deserialize)]
pub struct PathsConfig {
    pub schema_path: PathBuf,
    pub sections_folder: PathBuf,
    pub assets_folder: PathBuf,
    pub output_path: PathBuf,
}

/// The final project configuration data structure used by the core logic.
pub struct DocumentConfig {
    pub schema_path: PathBuf,
    pub sections_folder: PathBuf,
    pub output_path: PathBuf,
    pub vars: HashMap<String, String>,
}

impl Validable for DocumentConfig {
    fn validate(&self) -> Result<(), &'static str> {
        if self.schema_path.starts_with(&self.sections_folder) {
            return Err("mdm.conf: 'schema_path' cannot be nested inside of 'sections_folder'");
        }

        if self.output_path.starts_with(&self.sections_folder) {
            return Err("mdm.conf: 'output_path' cannot be nested inside of 'sections_folder'");
        }

        Ok(())
    }
}

impl TryFrom<RawDocumentConfig> for DocumentConfig {
    type Error = &'static str;

    fn try_from(raw: RawDocumentConfig) -> Result<Self, Self::Error> {
        let schema = raw.paths.schema_path;
        let sections = raw.paths.sections_folder;
        let output = raw.paths.output_path;
        let vars = raw.vars.unwrap_or_default();

        let config = DocumentConfig {
            schema_path: schema,
            sections_folder: sections,
            output_path: output,
            vars: vars,
        };

        config.validate()?;
        Ok(config)
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::core::model::{DocumentConfig, Validable};

    fn mock_config() -> DocumentConfig {
        DocumentConfig {
            schema_path: "schema.yaml".into(),
            sections_folder: "sections".into(),
            output_path: "document.md".into(),
            vars: HashMap::new()
        }
    }

    #[test]
    fn reject_nested_schema() {
        let mut config = mock_config();
        config.schema_path = "sections/schema_path.yaml".into();

        let result = config.validate();

        assert!(
            result.is_err(),
            "Document schema file shouldn't be nested inside sections folder"
        )
    }

    #[test]
    fn reject_nested_output() {
        let mut config = mock_config();
        config.output_path = "sections/document.md".into();

        let result = config.validate();

        assert!(
            result.is_err(),
            "Document output file shouldn't be nested inside sections folder"
        )
    }
}