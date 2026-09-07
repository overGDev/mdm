pub mod init;
pub mod sync;
pub mod check;
pub mod var;
pub mod list;
pub mod set;
pub mod get;
pub mod unset;
pub mod paths;
pub mod paths_set;
pub mod build;
pub mod update;
pub mod completions;

pub use init::InitCommand;
pub use sync::SyncCommand;
pub use check::CheckCommand;
pub use var::VarCommand;
pub use list::ListCommand;
pub use set::SetCommand;
pub use get::GetCommand;
pub use unset::UnsetCommand;
pub use paths::PathsCommand;
pub use paths_set::PathsSetCommand;
pub use build::BuildCommand;
pub use update::UpdateCommand;
pub use completions::CompletionsCommand;

use crate::core::model::CliCommand;

/// All top-level subcommands mdm exposes, shared by 'main' (to build the CLI) and the
/// 'completions' command (to generate scripts from the exact same command tree).
pub fn all() -> Vec<Box<dyn CliCommand>> {
    vec![
        Box::new(InitCommand {}),
        Box::new(SyncCommand {}),
        Box::new(CheckCommand {}),
        Box::new(VarCommand {}),
        Box::new(PathsCommand {}),
        Box::new(BuildCommand {}),
        Box::new(UpdateCommand {}),
        Box::new(CompletionsCommand {}),
    ]
}