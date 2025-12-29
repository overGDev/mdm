use clap::ArgMatches;

use crate::core::model::DocumentConfig;

pub struct CommandCtx {
    pub args: ArgMatches,
    pub config: Option<DocumentConfig>,
}
