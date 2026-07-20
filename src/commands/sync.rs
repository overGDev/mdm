use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, Command};
use walkdir::WalkDir;

use crate::core::{error::MDMError, model::{CliCommand, CommandCtx, SchemaSection}};

const COMMAND_NAME: &str = "sync";
const COMMAND_ABOUT: &str = "Update the sections path contents based on 'mdm/schema.yaml'";
const COMMAND_LONG_ABOUT: &str = "Reads the contents of 'mdm/schema.yaml' and iterates over the defined structure, generating the corresponding directories and files while ensuring the usage of snake_case. If there are currently folder or files no longer present on the project's schema, these can be removed by using the --clean flag. An automatic index will be generated if a file named 'index.md' is found, unless the '--skip-index' flag is used";

const CLEAN_FLAG_ID: &str = "clean";
const SKIP_INDEX_FLAG_ID: &str = "skip-index";

pub struct SyncCommand {}

impl SyncCommand {
    #[cfg(unix)]
    fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
        std::os::unix::fs::symlink(target, link)
    }

    #[cfg(windows)]
    fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
        std::os::windows::fs::symlink_dir(target, link)
    }

    #[cfg(unix)]
    fn remove_symlink(link: &Path) -> std::io::Result<()> {
        std::fs::remove_file(link)
    }

    #[cfg(windows)]
    fn remove_symlink(link: &Path) -> std::io::Result<()> {
        std::fs::remove_dir(link)
    }

    /// Creates a symlink at 'link' pointing to 'target', replacing it first if one
    /// (possibly stale or dangling) already exists there, to keep 'sync' idempotent.
    fn ensure_symlink(target: &Path, link: &Path) -> Result<(), MDMError> {
        if std::fs::symlink_metadata(link).is_ok() {
            Self::remove_symlink(link).map_err(|e| MDMError::IO {
                source: e,
                path: link.to_path_buf(),
            })?;
        }

        Self::create_symlink(target, link).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                MDMError::SymlinkPermissionDenied { path: link.to_path_buf() }
            } else {
                MDMError::IO { source: e, path: link.to_path_buf() }
            }
        })
    }

    /// Mirrors the schema tree into the assets folder and links each section's
    /// mirrored assets directory back into the sections tree, so authors can
    /// reference their own section's images with a fixed, depth-independent path.
    fn sync_assets(
        admitted_sections_paths: &mut Vec<PathBuf>,
        admitted_asset_dirs: &mut Vec<PathBuf>,
        sections: &[SchemaSection],
        sections_base: &Path,
        assets_base: &Path,
    ) -> Result<(), MDMError> {
        for section in sections {
            let assets_path = assets_base.join(section.fs_stem());
            if !assets_path.exists() {
                std::fs::create_dir_all(&assets_path).map_err(|e| MDMError::IO {
                    source: e,
                    path: assets_path.clone(),
                })?;
            }
            admitted_asset_dirs.push(assets_path.clone());

            let link_parent = if section.is_leaf() {
                sections_base.to_path_buf()
            } else {
                sections_base.join(section.get_fs_name())
            };
            let link_path = link_parent.join(section.assets_link_name());

            Self::ensure_symlink(&assets_path, &link_path)?;
            admitted_sections_paths.push(link_path);

            if !section.is_leaf() {
                let child_sections_base = sections_base.join(section.get_fs_name());
                Self::sync_assets(
                    admitted_sections_paths,
                    admitted_asset_dirs,
                    &section.children,
                    &child_sections_base,
                    &assets_path,
                )?;
            }
        }
        Ok(())
    }

    /// Prints a warning for every mirrored assets directory no longer referenced by the
    /// schema, without deleting it — real images are never removed automatically.
    fn warn_orphaned_assets(assets_path: &Path, admitted_asset_dirs: &[PathBuf]) -> Result<(), MDMError> {
        if !assets_path.exists() {
            return Ok(());
        }
        for entry in WalkDir::new(assets_path).min_depth(1) {
            let dir_entry = entry.map_err(|e| MDMError::IO {
                source: e.into(),
                path: assets_path.to_path_buf(),
            })?;
            let path = dir_entry.path();
            if path.is_dir() && !admitted_asset_dirs.contains(&path.to_path_buf()) {
                println!(
                    "Warning: '{}' is no longer referenced by the schema; preserved, not deleted",
                    path.display()
                );
            }
        }
        Ok(())
    }

    fn collect_index(
        sections: &[SchemaSection],
        base_path: &Path,
        depth: usize,
        accumulator: &mut Vec<String>,
    ) -> Result<(), MDMError> {
        for section in sections {
            let fs_name = section.get_fs_name();
            let current_path = base_path.join(&fs_name);

            if fs_name != "index.md" {
                accumulator.push(section.get_section_index(depth));
            }

            Self::collect_index(
                &section.children, 
                &current_path, 
                depth + 1, 
                accumulator
            )?;
        }

        Ok(())
    }

    fn clean_sections(sections_path: &Path, admited_paths: Vec<PathBuf>) -> Result<(), MDMError> {
        Ok(for entry in WalkDir::new(sections_path) {
            let dir_entry = entry.map_err(|e| MDMError::IO {
                source: e.into(),
                path: sections_path.to_path_buf(),
            })?;

            let path = dir_entry.path();
            let is_symlink = dir_entry.path_is_symlink();
            if (path.is_file() || is_symlink) && !admited_paths.contains(&path.to_path_buf()) {
                println!("Removing: {}", path.display());
                if is_symlink {
                    Self::remove_symlink(path)
                } else {
                    std::fs::remove_file(path)
                }.map_err(|e| MDMError::IO {
                    source: e,
                    path: path.to_path_buf(),
                })?;
            }
        })
    }

    fn sync_sections(
        admited_paths: &mut Vec<PathBuf>,
        sections: &[SchemaSection],
        base_path: &Path,
    ) -> Result<Option<PathBuf>, MDMError> { 
        let mut found_index_path: Option<PathBuf> = None;

        for section in sections {
            let fs_name = section.get_fs_name();
            let current_path = base_path.join(&fs_name);

            if fs_name == "index.md" {
                found_index_path = Some(current_path.clone());
            }

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
                admited_paths.push(current_path.clone());
                continue;
            }

            if !current_path.exists() {
                std::fs::create_dir_all(&current_path)
                    .map_err(|e| MDMError::IO {
                        source: e,
                        path: current_path.clone(),
                    })?;
            }
            if section.has_intro {
                let intro_path = current_path.join("intro.md");
                let is_intro_present = std::fs::metadata(&intro_path)
                    .map(|m| m.is_file())
                    .unwrap_or(false);
                if !is_intro_present {
                    std::fs::write(&intro_path, "")
                    .map_err(|e| MDMError::IO {
                        source: e,
                        path: intro_path.clone(),
                    })?;
                }
                admited_paths.push(intro_path);
            }
            
            let child_result = Self::sync_sections(
                admited_paths,
                &section.children, 
                &current_path,
            )?;

            if let Some(path) = child_result {
                found_index_path = Some(path);
            }
        }
        Ok(found_index_path)
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
                    .help("Delete existing files non-present on current schema"),
                Arg::new(SKIP_INDEX_FLAG_ID)
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .long("skip-index")
                    .help("Avoid updating 'index.md' file"),
            ]);
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.require_config()?;
        let schema = config.require_schema(
            "Schema not defined",
            "Define a schema in your 'mdm/schema.yaml' file first",
        )?;
        let sections_path = config.paths.sections.as_ref();
        let assets_path = config.paths.assets.as_ref();

        let mut admited_paths= vec![];
        let index_path = Self::sync_sections(
            &mut admited_paths,
            schema,
            sections_path,
        )?;
        println!("Successfully updated sections folder at: {}", sections_path.display());

        let mut admitted_asset_dirs = vec![];
        Self::sync_assets(
            &mut admited_paths,
            &mut admitted_asset_dirs,
            schema,
            sections_path,
            assets_path,
        )?;
        println!("Successfully updated assets folder at: {}", assets_path.display());

        let clean = ctx.args.get_flag(CLEAN_FLAG_ID);
        if clean {
            Self::warn_orphaned_assets(assets_path, &admitted_asset_dirs)?;
            Self::clean_sections(sections_path, admited_paths)?;
        }

        let skip = ctx.args.get_flag(SKIP_INDEX_FLAG_ID);
        if let (Some(path), false) = (index_path, skip) {
            let mut index_lines: Vec<String> = Vec::new();
            Self::collect_index(
                &schema, 
                &sections_path, 
                1, 
                &mut index_lines
            )?;

            let output_file = match OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(&path) {
                    Ok(output) => output,
                    Err(e) => Err(MDMError::IO {
                        source: e,
                        path: config.paths.output.clone(),
                    })?
                };

            let mut writer = BufWriter::new(output_file);
            for line in index_lines {
                writeln!(writer, "{}", line)
                    .map_err(|e| MDMError::from_io(e, &path))?;
            }
            println!("Successfully updated index at: {}", path.display());
        }
        Ok(())
    }
}