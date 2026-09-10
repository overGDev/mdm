use std::path::Path;
use std::io::Write;

use clap::{Arg, ArgAction, Command, ValueHint};

use crate::core::{
    app::{ConfigFile, MDM_GIT_IGNORE_SAMPLE}, error::MDMError, model::{CliCommand, CommandCtx}
};

const COMMAND_NAME: &str = "init";
const COMMAND_ABOUT: &str = "Initialize a new document project workspace";
const COMMAND_LONG_ABOUT: &str = "Sets up the necessary directory structure and generates default configuration files for the project, including a '.github/workflows/mdm-build.yml' CI workflow that rebuilds the document on every push to 'main' or 'develop' (skip it with --no-cicd). It creates all required reserved files in the specified workspace. If these files already exist, the command will abort to prevent data loss, unless the --force flag is used to overwrite them";

const WORKDIR_ARG_ID: &str = "workdir";
const FORCE_FLAG_ID: &str = "force";
const NO_CICD_FLAG_ID: &str = "no-cicd";

pub struct InitCommand {}

impl InitCommand {
    fn execute_git_init(workdir: &Path) -> Result<(), MDMError> {
        match std::process::Command::new("git")
            .arg("init")
            .current_dir(workdir)
            .status() {
            Ok(exit_status) if exit_status.success() => Ok(()),
            Ok(_) => {
                return Err(MDMError::InvalidCommandState {
                    reason: "Git initialization failed".into(),
                    help: "Ensure you have permissions to write in the target directory".into(),
                })
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Err(MDMError::InvalidCommandState {
                    reason: "Git command not found".into(),
                    help: "Please install Git and ensure it is available in your PATH".into(),
                })
            }
            Err(error) => {
                return Err(MDMError::IO {
                    source: error,
                    path: workdir.to_path_buf(),
                })
            }
        }
    }

    fn add_to_gitignore(workdir: &Path) -> Result<(), MDMError> {
        let gitignore_path = workdir.join(".gitignore");
        let is_empty = std::fs::metadata(&gitignore_path)
            .map(|m| m.len() == 0)
            .unwrap_or(true);
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&gitignore_path)
            .map_err(|e| MDMError::IO {
                source: e,
                path: gitignore_path.clone(),
            })?;

        let content = if is_empty {
            format!("{}", MDM_GIT_IGNORE_SAMPLE)
        } else {
            format!("\n# MDM\n{}", MDM_GIT_IGNORE_SAMPLE)
        };

        write!(file, "{}", content).map_err(|e| MDMError::IO {
            source: e,
            path: gitignore_path,
        })?;

        Ok(())
    }
}

impl CliCommand for InitCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        false
    }

    fn build(&self) -> Command {
        return Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .long_about(COMMAND_LONG_ABOUT)
            .args([
                Arg::new(WORKDIR_ARG_ID)
                    .required(false)
                    .default_value(".")
                    .value_hint(ValueHint::DirPath)
                    .value_name("WORKDIR")
                    .help("Project's initialization directory"),
                Arg::new(FORCE_FLAG_ID)
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .short('f')
                    .long("force")
                    .help("Override existing 'mdm' folder config files if present"),
                Arg::new(NO_CICD_FLAG_ID)
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .long("no-cicd")
                    .help("Skip generating the '.github/workflows/mdm-build.yml' CI workflow"),
            ]);
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let workdir = Path::new(
            ctx.args.get_one::<String>(WORKDIR_ARG_ID).unwrap()
        );

        let force = ctx.args.get_flag(FORCE_FLAG_ID);
        let no_cicd = ctx.args.get_flag(NO_CICD_FLAG_ID);
        for file in ConfigFile::all().into_iter() {
            if no_cicd && matches!(file, ConfigFile::Workflow) {
                continue;
            }

            let path = workdir.join(file.relative_path());
            if path.exists() && !force {
                return Err(MDMError::InvalidCommandState {
                    reason: format!("'{}' file exists already", file.name()),
                    help: "Use --force (-f) for command to override existing files".into()
                });
            }

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| MDMError::IO {
                        source: e,
                        path: parent.to_path_buf(),
                    })?;
            }
            std::fs::write(&path, file.sample_content())
                .map_err(|e| MDMError::IO {
                    source: e,
                    path: path,
                })?;
        }

        let git_folder = workdir.join(".git");
        let is_git_initialized = std::fs::metadata(&git_folder)
            .map(|m| m.is_dir())
            .unwrap_or(false);
        
        if !is_git_initialized {
            InitCommand::execute_git_init(workdir)?;
        }
        InitCommand::add_to_gitignore(workdir)?;
        Ok(())
    }
}