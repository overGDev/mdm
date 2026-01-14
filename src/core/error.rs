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

    #[error("Command aborted: '.mdm.conf' file not found. Run 'mdm init' first.")]
    MDMConfigNotFound,

    #[error("Command aborted: {reason}\n{help}")]
    InvalidCommandState { reason: String, help: String },

    #[error("{0}")]
    Other(String),
}

pub fn print_and_abort(e: MDMError) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}
