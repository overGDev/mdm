use std::path::PathBuf;
use serde::Deserialize;

use crate::core::{app::MDM_CONF_FOLDER_NAME, model::Validable};

/// Canonical, direct representation of the contents of the 'mdm/paths.yaml' file.
/// Contains all the mdm paths that can be customized by the user.
#[derive(Debug, Deserialize)]
pub struct PathsConfig {
    pub sections: PathBuf,
    pub assets: PathBuf,
    pub output: PathBuf,
}

impl Validable for PathsConfig {
    fn validate(&self) -> Result<(), &'static str> {
        if self.sections.starts_with(MDM_CONF_FOLDER_NAME) {
            return Err("'sections_folder' cannot be nested inside of 'mdm' folder");
        }

        if self.output.starts_with(MDM_CONF_FOLDER_NAME) {
            return Err("'output_path' cannot be nested inside of 'mdm' folder");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_paths() -> PathsConfig {
        PathsConfig {
            sections: "sections".into(),
            assets: "assets".into(),
            output: "document.md".into(),
        }
    }

    #[test]
    fn reject_nested_sections() {
        let mut paths = mock_paths();
        paths.sections = PathBuf::from(MDM_CONF_FOLDER_NAME).join("sections");

        let result = paths.validate();

        assert!(result.is_err());
    }

    #[test]
    fn reject_nested_output() {
        let mut paths = mock_paths();
        paths.output = PathBuf::from(MDM_CONF_FOLDER_NAME).join("output.md");

        let result = paths.validate();

        assert!(result.is_err());
    }

    #[test]
    fn accept_valid_paths() {
        let paths = mock_paths();
        assert!(paths.validate().is_ok());
    }
}