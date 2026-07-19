use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::{Component, Path, PathBuf};

use clap::Command;
use indexmap::IndexMap;
use regex::{Captures, Regex};

use crate::core::model::SchemaSection;
use crate::core::{
    error::MDMError, model::{CliCommand, CommandCtx}
};

const COMMAND_NAME: &str = "build";
const COMMAND_ABOUT: &str = "Combine the sections into the output file";
const COMMAND_LONG_ABOUT: &str = "Iterates over all the sections folder path recursively and appends their contents to the output file on the specified order";

const PAGE_BREAK: &str = "<div style=\"page-break-after: always;\"></div>";

pub struct BuildCommand {}

impl BuildCommand {
    /// Computes the relative path from 'from_dir' to 'to', assuming both are absolute.
    fn relative_path(from_dir: &Path, to: &Path) -> PathBuf {
        let from_components: Vec<Component> = from_dir.components().collect();
        let to_components: Vec<Component> = to.components().collect();
        let common_len = from_components.iter()
            .zip(to_components.iter())
            .take_while(|(a, b)| a == b)
            .count();

        let mut result = PathBuf::new();
        for _ in common_len..from_components.len() {
            result.push("..");
        }
        for component in &to_components[common_len..] {
            result.push(component.as_os_str());
        }
        result
    }

    /// Rewrites this section's own fixed assets-link prefix (e.g. 'mobile_version.assets/'
    /// or 'assets/') found in markdown/HTML image references into the real path relative
    /// to the output file, since the built document no longer sits next to the symlink.
    fn rewrite_asset_references(content: &str, link_name: &str, relative_target: &Path) -> Result<String, MDMError> {
        let escaped_prefix = regex::escape(&format!("{}/", link_name));
        let pattern = format!(r#"(\]\(|src="|src=')({})"#, escaped_prefix);
        let re = Regex::new(&pattern)?;

        let mut replacement = relative_target.to_string_lossy().replace('\\', "/");
        replacement.push('/');

        Ok(re.replace_all(content, |caps: &Captures| {
            format!("{}{}", &caps[1], replacement)
        }).into_owned())
    }

    /// Substitutes every '{{var_name}}' placeholder with its value from 'mdm/vars.yaml'.
    /// Fails the build if a referenced variable isn't defined, rather than shipping a
    /// document with a literal unresolved placeholder in it.
    fn substitute_variables(content: &str, vars: &IndexMap<String, String>, path: &Path) -> Result<String, MDMError> {
        let re = Regex::new(r"\{\{\s*([A-Za-z0-9_]+)\s*\}\}")?;
        let mut error = None;

        let result = re.replace_all(content, |caps: &Captures| {
            let var_name = &caps[1];
            match vars.get(var_name) {
                Some(value) => value.clone(),
                None => {
                    if error.is_none() {
                        error = Some(MDMError::UndefinedVariable {
                            var_name: var_name.to_string(),
                            path: path.to_path_buf(),
                        });
                    }
                    caps[0].to_string()
                }
            }
        }).into_owned();

        match error {
            Some(e) => Err(e),
            None => Ok(result),
        }
    }

    fn sync_sections(
        writer: &mut BufWriter<File>,
        sections: &[SchemaSection],
        base_path: &Path,
        assets_base: &Path,
        output_dir: &Path,
        vars: &IndexMap<String, String>,
        depth: usize,
    ) -> Result<(), MDMError> {
        for section in sections {
            let current_path: PathBuf = base_path.join(section.get_fs_name());
            let assets_path = assets_base.join(section.fs_stem());
            if !current_path.exists() {
                let reason = format!(
                    "'{}' was not found inside the sections folder",
                    current_path.to_string_lossy(),
                );
                return Err(MDMError::InvalidCommandState {
                    reason,
                    help: "Try running 'mdm sync' to create missing files".into(),
                });
            }

            if !section.skip_title {
                let header = section.get_section_header(depth);
                writeln!(writer, "{}\n", header)
                    .map_err(|err| MDMError::from_io(err, &current_path))?;
            }

            let read_path = if section.is_leaf() {
                Some(current_path.clone())
            } else if section.has_intro {
                Some(current_path.join("intro.md"))
            } else {
                None
            };

            if let Some(path) = read_path {
                let content = std::fs::read_to_string(&path)
                    .map_err(|err| MDMError::from_io(err, &path))?;
                let relative_assets = Self::relative_path(output_dir, &assets_path);
                let asset_resolved = Self::rewrite_asset_references(
                    &content,
                    &section.assets_link_name(),
                    &relative_assets,
                )?;
                let processed_content = Self::substitute_variables(&asset_resolved, vars, &path)?;

                writer.write_all(processed_content.as_bytes())
                    .map_err(|err| MDMError::Other(err.to_string()))?;
                writer.write_all(b"\n\n")
                    .map_err(|err| MDMError::Other(err.to_string()))?;

                if section.skip_after {
                    writeln!(writer, "{}", PAGE_BREAK)
                        .map_err(|err| MDMError::from_io(err, &path))?;
                    writer.write_all(b"\n")
                        .map_err(|err| MDMError::Other(err.to_string()))?;
                }
            };

            BuildCommand::sync_sections(
                writer, &section.children, &current_path, &assets_path, output_dir, vars, depth + 1
            )?;
        }
        Ok(())
    }
}

impl CliCommand for BuildCommand {
    fn name(&self) -> &str {
        COMMAND_NAME
    }

    fn requires_paths(&self) -> bool {
        true
    }

    fn build(&self) -> Command {
        return Command::new(COMMAND_NAME)
            .about(COMMAND_ABOUT)
            .long_about(COMMAND_LONG_ABOUT);
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.require_config()?;
        let schema = config.require_schema(
            "Cannot build output file without a defined schema",
            "Define a schema and ensure files are generated by running 'mdm sync sections'"
        )?;
        let sections_path = config.paths.sections.as_ref();
        let assets_path = config.paths.assets.as_ref();
        let output_dir = config.paths.output.parent().unwrap_or_else(|| Path::new("."));
        let output_file = match OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(&config.paths.output) {
                Ok(output) => output,
                Err(e) => Err(MDMError::IO {
                    source: e,
                    path: config.paths.output.clone(),
                })?
            };
        let mut writer = BufWriter::new(output_file);
        BuildCommand::sync_sections(
            &mut writer, &schema, &sections_path, &assets_path, output_dir, &config.vars, 1
        )?;
        println!("Successfully combined document at '{}'", config.paths.output.display());
        Ok(())
    }
}