use std::path::PathBuf;
use serde::Deserialize;
use crate::core::{MDM_CONF_FOLDER_NAME, model::Validable};

/// Canonical, direct representation of the contents of the 'mdm/paths.yaml' file.
/// Contains all the mdm paths that can be customized by the user.
#[derive(Debug, Deserialize)]
pub struct PathsConfig {
    pub sections_folder: PathBuf,
    pub assets_folder: PathBuf,
    pub output_path: PathBuf,
}

impl Validable for PathsConfig {
    fn validate(&self) -> Result<(), &'static str> {
        if self.sections_folder.starts_with(MDM_CONF_FOLDER_NAME) {
            return Err("'sections_folder' cannot be nested inside of 'mdm' folder");
        }

        if self.output_path.starts_with(MDM_CONF_FOLDER_NAME) {
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
            sections_folder: "sections".into(),
            assets_folder: "assets".into(),
            output_path: "document.md".into(),
        }
    }

    #[test]
    fn reject_nested_sections() {
        let mut paths = mock_paths();
        paths.sections_folder = PathBuf::from(MDM_CONF_FOLDER_NAME).join("sections");

        let result = paths.validate();

        assert!(result.is_err());
    }

    #[test]
    fn reject_nested_output() {
        let mut paths = mock_paths();
        paths.output_path = PathBuf::from(MDM_CONF_FOLDER_NAME).join("output.md");

        let result = paths.validate();

        assert!(result.is_err());
    }

    #[test]
    fn accept_valid_paths() {
        let paths = mock_paths();
        assert!(paths.validate().is_ok());
    }
}