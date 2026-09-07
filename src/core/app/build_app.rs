use clap::Command;

use crate::core::{ext::CommandExt, model::CliCommand};

pub const APP_NAME: &str = "mdm";
pub const APP_ABOUT: &str = "Manage documentation projects using Docs-as-Code workflows";
pub const APP_LONG_ABOUT: &str = "MDM is a tool designed to manage documentation within version-controlled environments with Git. It streamlines the document lifecycle by providing a modular and flexible framework for handling the iterative growth of collaborative documents, such as technical documentation of services or applications";

/// Builds the full clap command tree from the given top-level subcommands. Shared by
/// 'main' (to parse argv) and the 'completions' command (to generate scripts from the
/// exact same tree), so the two can never drift apart.
pub fn build_app(subcommands: &[Box<dyn CliCommand>]) -> Command {
    Command::new(APP_NAME)
        .about(APP_ABOUT)
        .long_about(APP_LONG_ABOUT)
        .version(env!("CARGO_PKG_VERSION"))
        .load_subcommands(subcommands)
}
