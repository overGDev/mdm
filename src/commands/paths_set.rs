use std::path::{Path, PathBuf};

use clap::{Arg, Command};

use crate::core::{app::ConfigFile, error::MDMError, model::{CliCommand, CommandCtx, PathsConfig, Validable}};

const COMMAND_NAME: &str = "set";
const COMMAND_ABOUT: &str = "Change one of the paths mdm uses, defined in 'mdm/paths.yaml'";
const COMMAND_LONG_ABOUT: &str = "Updates the given path in 'mdm/paths.yaml'. When changing 'sections', the matching entries in '.gitignore' are rewritten to match. When changing 'output', '.gitignore', '.github/workflows/mdm-build.yml', and '.githooks/pre-commit' (whichever are present) are rewritten to match";

const KEY_ARG_ID: &str = "key";
const VALUE_ARG_ID: &str = "value";

const SECTIONS_KEY: &str = "sections";
const ASSETS_KEY: &str = "assets";
const OUTPUT_KEY: &str = "output";

pub struct PathsSetCommand {}

impl PathsSetCommand {
    /// Reads a file's contents, returning 'None' rather than erroring when it's absent:
    /// the '.gitignore' and CI workflow are optional companions to 'paths.yaml', either of
    /// which the user may have removed or skipped generating (e.g. via 'mdm init --no-cicd').
    fn read_optional(path: &Path) -> Result<Option<String>, MDMError> {
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(Some(content)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(MDMError::IO { source: e, path: path.to_path_buf() }),
        }
    }

    /// Replaces the data lines following a '# mdm:<marker>' comment with 'replacement',
    /// so the templated portions of '.gitignore'/the CI workflow can be regenerated
    /// without having to know their previous contents. Any comment lines wrapping the
    /// marker's explanation onto further lines are skipped over, not overwritten.
    fn splice_after_marker(content: &str, marker: &str, replacement: &[String]) -> Option<String> {
        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        let marker_idx = lines.iter().position(|l| l.trim_start().starts_with(marker))?;

        let mut start = marker_idx + 1;
        while start < lines.len() && lines[start].trim_start().starts_with('#') {
            start += 1;
        }
        let end = start + replacement.len();
        if end > lines.len() {
            return None;
        }
        lines[start..end].clone_from_slice(replacement);

        let mut result = lines.join("\n");
        result.push('\n');
        Some(result)
    }

    /// Applies 'splice_after_marker' to the file at 'path' in place, if the file and the
    /// marker are both present. Silently does nothing otherwise, since these companion
    /// files are optional and may not carry the marker if hand-edited.
    fn patch_marked_file(path: &Path, marker: &str, replacement: Vec<String>) -> Result<(), MDMError> {
        let Some(content) = Self::read_optional(path)? else {
            return Ok(());
        };
        let Some(patched) = Self::splice_after_marker(&content, marker, &replacement) else {
            return Ok(());
        };
        std::fs::write(path, patched)
            .map_err(|e| MDMError::IO { source: e, path: path.to_path_buf() })
    }

    fn relative_to_root(path: &Path, root: &Path) -> PathBuf {
        path.strip_prefix(root).unwrap_or(path).to_path_buf()
    }

    fn to_slash_string(path: &Path) -> String {
        path.to_string_lossy().replace('\\', "/")
    }
}

impl CliCommand for PathsSetCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .long_about(COMMAND_LONG_ABOUT)
            .args([
                Arg::new(KEY_ARG_ID)
                    .required(true)
                    .value_parser([SECTIONS_KEY, ASSETS_KEY, OUTPUT_KEY])
                    .value_name("KEY")
                    .help("Which path to change"),
                Arg::new(VALUE_ARG_ID)
                    .required(true)
                    .value_name("VALUE")
                    .help("New path, relative to the project's root"),
            ])
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.require_config()?;

        let key = ctx.args.get_one::<String>(KEY_ARG_ID)
            .ok_or(MDMError::InvalidCommandState {
                reason: "Missing required KEY argument".into(),
                help: "Provide the path to change, e.g.: 'mdm paths set output out/document.md'".into(),
            })?;
        let new_value = ctx.args.get_one::<String>(VALUE_ARG_ID)
            .ok_or(MDMError::InvalidCommandState {
                reason: "Missing required VALUE argument".into(),
                help: "Provide a new value for the path, e.g.: 'mdm paths set output out/document.md'".into(),
            })?;

        let mut sections = Self::relative_to_root(&config.paths.sections, &config.root);
        let mut assets = Self::relative_to_root(&config.paths.assets, &config.root);
        let mut output = Self::relative_to_root(&config.paths.output, &config.root);
        match key.as_str() {
            SECTIONS_KEY => sections = PathBuf::from(new_value),
            ASSETS_KEY => assets = PathBuf::from(new_value),
            OUTPUT_KEY => output = PathBuf::from(new_value),
            _ => unreachable!("clap restricts KEY to a known set of values"),
        }

        let new_paths = PathsConfig { sections, assets, output };
        new_paths.validate().map_err(|e| MDMError::InvalidCommandState {
            reason: e.into(),
            help: "Fix the provided value and try again".into(),
        })?;

        let old_abs = match key.as_str() {
            SECTIONS_KEY => &config.paths.sections,
            ASSETS_KEY => &config.paths.assets,
            OUTPUT_KEY => &config.paths.output,
            _ => unreachable!("clap restricts KEY to a known set of values"),
        };
        let new_abs = config.root.join(new_value);
        if old_abs.exists() && *old_abs != new_abs {
            if let Some(parent) = new_abs.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| MDMError::IO { source: e, path: parent.to_path_buf() })?;
            }
            std::fs::rename(old_abs, &new_abs)
                .map_err(|e| MDMError::IO { source: e, path: old_abs.clone() })?;
            println!("Moved '{}' -> '{}'", old_abs.display(), new_abs.display());
        }

        let content = serde_yaml::to_string(&new_paths)
            .map_err(|e| MDMError::Parse(e))?;
        let paths_yaml = config.root.join(ConfigFile::Paths.relative_path());
        std::fs::write(&paths_yaml, content)
            .map_err(|e| MDMError::IO { source: e, path: paths_yaml })?;
        println!("Updated 'mdm/paths.yaml': {} -> {}", key, new_value);

        if key == SECTIONS_KEY {
            Self::patch_marked_file(
                &config.root.join(".gitignore"),
                "# mdm:sections-glob",
                vec![
                    format!("{}/**/*.assets", new_value),
                    format!("{}/**/assets", new_value),
                ],
            )?;
        }

        if key == OUTPUT_KEY {
            Self::patch_marked_file(
                &config.root.join(".gitignore"),
                "# mdm:output-path",
                vec![new_value.clone()],
            )?;
            Self::patch_marked_file(
                &config.root.join(ConfigFile::Workflow.relative_path()),
                "# mdm:output-path",
                vec![format!("  MDM_OUTPUT_PATH: \"{}\"", Self::to_slash_string(Path::new(new_value)))],
            )?;
            Self::patch_marked_file(
                &config.root.join(ConfigFile::PreCommitHook.relative_path()),
                "# mdm:output-path",
                vec![format!("OUTPUT_FILE=\"{}\"", Self::to_slash_string(Path::new(new_value)))],
            )?;
        }

        Ok(())
    }
}
