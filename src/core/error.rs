use thiserror::Error;

#[derive(Error, Debug)]
pub enum MDMError {
    #[error("Error: no subcommand provided. Use --help for options.")]
    NoSubcommandProvided,

    #[error("Error: unknown subcommand provided.")]
    UnknownSubcommand,

    #[error("Parsing error: {0}")]
    Parse(#[from] serde_norway::Error),

    #[error("IO error ({path}): {source}")]
    IO {
        source: std::io::Error,
        path: std::path::PathBuf,
    },

    #[error("Command aborted: '.mdm.conf' file not found.\nRun 'mdm init' first.")]
    MDMConfigNotFound,

    #[error("Command aborted: {reason}\n{help}")]
    InvalidCommandState { reason: String, help: String },

    #[error("Error: {0}")]
    Other(String),
}

impl MDMError {
    pub fn abort(reason: &str, help: &str) -> MDMError {
        return MDMError::InvalidCommandState {
            reason: reason.into(),
            help: help.into(),
        };
    }
}
