use clap::ArgMatches;

use crate::core::model::PathsConfig;

pub struct CommandCtx {
    pub args: ArgMatches,
    pub paths_config: Option<PathsConfig>,
}
