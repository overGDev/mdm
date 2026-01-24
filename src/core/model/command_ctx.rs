use clap::ArgMatches;

use crate::core::{error::MDMError, model::MDMConfig};

/// Wrapping structure for commands to take
/// Allows for flexibility on wether or not MDM is provided
pub struct CommandCtx {
    pub args: ArgMatches,
    pub config: Option<MDMConfig>,
}

impl CommandCtx {
    pub fn require_config(&self) -> Result<&MDMConfig, MDMError> {
        self.config.as_ref().ok_or_else(|| MDMError::InvalidCommandState {
            reason: "Failed to load config on a command that requires it.".into(),
            help: "Try running 'mdm check'.".into(),
        })
    }
}