use clap::ArgMatches;

use crate::core::model::MDMConfig;

/// Wrapping structure for commands to take
/// Allows for flexibility on wether or not MDM is provided
pub struct CommandCtx {
    pub args: ArgMatches,
    pub config: Option<MDMConfig>,
}