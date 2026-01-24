use std::path::Path;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MDMError {
    #[error("Error: no subcommand provided. Use --help for options.")]
    NoSubcommandProvided,

    #[error("Error: unknown subcommand provided.")]
    UnknownSubcommand,

    #[error("Parsing error: {0}")]
    Parse(#[from] serde_yaml::Error),

    #[error("IO error ({path}): {source}")]
    IO {
        source: std::io::Error,
        path: std::path::PathBuf,
    },

    #[error("Command aborted: '.mdm.conf' file not found. Run 'mdm init' first.")]
    MDMConfigNotFound,

    #[error("Command aborted: {reason}\n{help}")]
    InvalidCommandState { reason: String, help: String },

    #[error("{0}")]
    Other(String),
}

impl MDMError {
    pub fn from_io(err: std::io::Error, path: &Path) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => MDMError::Other(format!("File not found: {:?}", path)),
            std::io::ErrorKind::PermissionDenied => MDMError::Other(format!("Access denied: {:?}", path)),
            _ => MDMError::IO {
                source: err,
                path: path.to_path_buf(),
            },
        }
    }

    pub fn print_and_abort(self) -> ! {
        eprintln!("{}", self);
        std::process::exit(1);
    }
}


