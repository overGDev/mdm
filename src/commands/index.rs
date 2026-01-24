use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};

use clap::Command;

use crate::core::model::SchemaSection;
use crate::core::{
    error::MDMError, model::{CliCommand, CommandCtx}
};

const COMMAND_NAME: &str = "index";
const COMMAND_ABOUT: &str = "Generate an automatic index file";
const COMMAND_LONG_ABOUT: &str = "Iterates over the schema to generate a Table of Contents and writes it to the section named 'index.md'.";

pub struct IndexCommand {}

impl IndexCommand {
    fn collect_indices(
        sections: &[SchemaSection],
        base_path: &Path,
        depth: usize,
        accumulator: &mut Vec<String>,
    ) -> Result<Option<PathBuf>, MDMError> {
        let mut found_index_path: Option<PathBuf> = None;

        for section in sections {
            let fs_name = section.get_fs_name();
            let current_path = base_path.join(&fs_name);

            if fs_name == "index.md" {
                found_index_path = Some(current_path.clone());
            } else {
                accumulator.push(section.get_section_index(depth));
            }

            let child_result = IndexCommand::collect_indices(
                &section.children, 
                &current_path, 
                depth + 1, 
                accumulator
            )?;

            // If a child branch found the index path, bubble it up
            if let Some(path) = child_result {
                found_index_path = Some(path);
            }
        }

        Ok(found_index_path)
    }
}

impl CliCommand for IndexCommand {
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
    }

    fn run(&self, ctx: CommandCtx) -> Result<(), MDMError> {
        let config = ctx.require_config()?;
        let schema = config.require_schema(
            "Schema not defined.",
            "Define a schema in your 'mdm/schema.yaml' file first.",
        )?;
        let sections_path = config.paths.sections.as_ref();
        let mut index_lines: Vec<String> = Vec::new();

        let target_path = IndexCommand::collect_indices(
            &schema, 
            &sections_path, 
            1, 
            &mut index_lines
        )?;

        if let Some(path) = target_path {
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
        } else {
            return Err(MDMError::InvalidCommandState { 
                reason: "No section with fs_name 'index.md' was found in the schema.".into(), 
                help: "Ensure one of your schema sections (usually the root or a dedicated section) is named/aliased to result in 'index.md'.".into() 
            });
        }
        Ok(())
    }
}