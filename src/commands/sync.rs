use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, Command};
use walkdir::WalkDir;

use crate::core::{error::MDMError, model::{CliCommand, CommandCtx, SchemaSection}};

const COMMAND_NAME: &str = "sync";
const COMMAND_ABOUT: &str = "Syncronize the sections folder";
const COMMAND_LONG_ABOUT: &str = "Sets up the necessary directory structure and generates default configuration files for the project.It creates all required reserved files in the specified workspace. If these files already exist, the command will abort to prevent data loss, unless the --force flag is used to overwrite them.";

const CLEAN_FLAG_ID: &str = "clean";

pub struct SyncCommand {}

impl SyncCommand {
    fn sync_sections(
        admited_paths: &mut Vec<PathBuf>,
        sections: &[SchemaSection],
        base_path: &Path,
    ) -> Result<(), MDMError> { 
        for section in sections {
            let current_path = base_path.join(section.get_fs_name());

            if section.is_leaf() {
                if !current_path.exists() {
                    if let Some(parent) = current_path.parent() {
                        std::fs::create_dir_all(parent)
                            .map_err(|e| MDMError::IO {
                                source: e,
                                path: parent.to_path_buf(),
                            })?;
                    }
                    std::fs::write(&current_path, "")
                        .map_err(|e| MDMError::IO {
                            source: e,
                            path: current_path.clone(),
                        })?;
                }
                admited_paths.push(current_path);
            } else {
                if !current_path.exists() {
                    std::fs::create_dir_all(&current_path)
                        .map_err(|e| MDMError::IO {
                            source: e,
                            path: current_path.clone(),
                        })?;
                }
                let children_slice = &section.children;
                SyncCommand::sync_sections(
                    admited_paths,
                    children_slice, 
                    &current_path,
                )?;
            }
        }
        Ok(())
    }
}

impl CliCommand for SyncCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        return Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .long_about(COMMAND_LONG_ABOUT)
            .args([
                Arg::new(CLEAN_FLAG_ID)
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .short('c')
                    .long("clean")
                    .help("delete existing files non-present on current schema"),
            ]);
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.config
            .ok_or(MDMError::InvalidCommandState {
                reason: "Failed to load config on a command that requires it.".into(),
                help: "Try running 'mdm check.".into(), 
            })?;

        let schema = config.schema
            .ok_or(MDMError::InvalidCommandState {
                reason: "Unable to execute sync command without a loaded schema".into(),
                help: "Generate a document schema inside of 'mdm/schema.yaml' first.".into(), 
            })?;

        let mut admited_paths= vec![];
        SyncCommand::sync_sections(
            &mut admited_paths,
            &schema, 
            &config.paths.sections
        )?;

        let clean = ctx.args.get_flag(CLEAN_FLAG_ID);
        if !clean {
            return Ok(())
        }

        for entry in WalkDir::new(&config.paths.sections) {
            let dir_entry = entry.map_err(|e| MDMError::IO {
                source: e.into(),
                path: config.paths.sections.clone(),
            })?;
            
            let path = dir_entry.path();
            if path.is_file() && !admited_paths.contains(&path.to_path_buf()) {
                println!("Clear: {}", path.display());
                std::fs::remove_file(path)
                    .map_err(|e| MDMError::IO {
                        source: e,
                        path: path.into(),
                    })?;
            }
        }

        Ok(())
    }
}