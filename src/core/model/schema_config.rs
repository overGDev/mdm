use serde::Deserialize;

use crate::core::{error::MDMError, model::Validable};

/// Helper structure that wraps all the schema sections defined at the 'mdm/schema.yaml' file.
#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct SchemaConfig {
    pub sections: Vec<RawSchemaSection>
}

/// Intermediate and recursive structure that handles optional fields and allows partial definition for better UX.
#[derive(Debug, Deserialize)]
pub struct RawSchemaSection {
    pub title: String,
    pub alias: Option<String>,
    pub custom_id: Option<String>,
    pub has_intro: Option<bool>,
    pub skip_after: Option<bool>,
    pub skip_title: Option<bool>,
    pub children: Option<Vec<RawSchemaSection>>,
}

/// The canonical representation of the schema configuration.
/// Guaranteed to be valid, with default values to simplify application's logic.
#[derive(Debug)]
pub struct SchemaSection {
    pub title: String,
    pub alias: Option<String>,
    pub custom_id: Option<String>,
    pub has_intro: bool,
    pub skip_after: bool,
    pub skip_title: bool,
    pub children: Vec<SchemaSection>,
}

impl SchemaSection {
    fn normalize(input: &mut String, separator: char) {
        let deunicoded = deunicode::deunicode(input).to_lowercase();
        let mut last_was_separator = true;
        input.clear();

        for c in deunicoded.chars() {
            if c.is_alphanumeric() {
                input.push(c);
                last_was_separator = false;
            } else if !last_was_separator && c != '.' {
                input.push(separator);
                last_was_separator = true;
            }
        }

        if input.ends_with(separator) {
            input.pop();
        }
    }

    pub fn is_leaf(&self) -> bool {
        return self.children.is_empty()
    }

    pub fn get_fs_name(&self) -> String {
        let mut fs_name = self.alias
            .as_deref()
            .unwrap_or(&self.title)
            .to_string();

        SchemaSection::normalize(&mut fs_name, '_');

        if self.is_leaf() {
            fs_name.push_str(".md");
        }

        fs_name
    }

    pub fn has_custom_id(&self) -> bool {
        match self.custom_id {
            Some(_) => true,
            None => false
        }
    }

    fn markdown_format_header(&self, depth: usize) -> String {
        let prefix = "#".repeat(depth);
        format!("{} {}", prefix, self.title)
    }

    fn kebab_case_title_id(&self) -> String {
        let mut title_id = self.custom_id
            .as_deref()
            .unwrap_or(&self.title)
            .to_string();
        SchemaSection::normalize(&mut title_id, '-');
        title_id
    }

    fn html_format_header(&self, depth: usize) -> String {
        let title_id = self.kebab_case_title_id();
        let open_tag = format!("<h{} id=\"{}\">", depth, title_id);
        let closing_tag = format!("</h{}>", depth.to_string());
        format!("{}{}{}", open_tag, self.title, closing_tag)
    }

    pub fn get_section_header(&self, depth: usize) -> String {
        if self.has_custom_id() {
            self.html_format_header(depth)
        } else {
            self.markdown_format_header(depth)
        }
    }

    pub fn get_section_index(&self, depth: usize) -> String {
        format!(
            "{}- [{}](#{})",
            "  ".repeat(depth - 1),
            self.title,
            self.kebab_case_title_id()
        )
    }
}

impl Validable for SchemaSection {
    fn validate(&self) -> Result<(), &'static str> {
        if self.title.trim().is_empty() {
            return Err("Node title cannot be empty");
        }

        if let Some(alias) = &self.alias {
            if alias.trim().is_empty() {
                return Err("Declared node alias cannot be empty string");
            }
        }

        if let Some(custom_id) = &self.custom_id {
            if custom_id.trim().is_empty() {
                return Err("Declared node id cannot be empty string");
            }
        }

        let is_leaf = self.children.is_empty();
        let has_intro = self.has_intro;
        if is_leaf && has_intro {
            return Err("Leaf document nodes cannot have intro sections");
        }

        Ok(())
    }
}

impl TryFrom<RawSchemaSection> for SchemaSection {
    type Error = MDMError;

    fn try_from(raw: RawSchemaSection) -> Result<SchemaSection, Self::Error> {
        let title = raw.title;
        let alias = raw.alias.filter(|s| !s.trim().is_empty());
        let custom_id = raw.custom_id.filter(|s| !s.trim().is_empty());
        let has_intro = raw.has_intro.unwrap_or(false);
        let skip_after = raw.skip_after.unwrap_or(true);
        let skip_title = raw.skip_title.unwrap_or(false);

        let mut children = Vec::new();
        for raw_child in raw.children.unwrap_or_default() {
            let processed_child = SchemaSection::try_from(raw_child)?;
            children.push(processed_child);
        }

        let node = SchemaSection {
            title,
            alias,
            custom_id,
            has_intro,
            skip_after,
            skip_title,
            children,
        };

        node.validate()
            .map_err(|e| MDMError::Other(e.to_string()))?;
        
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mock_node() -> SchemaSection {
        SchemaSection {
            title: "Test".to_string(),
            alias: None,
            custom_id: None,
            has_intro: false,
            skip_after: false,
            skip_title: false,
            children: vec![],
        }
    }

    #[test]
    fn normalization_replaces_runes() {
        let mut node = mock_node();
        
        node.title = "Helló Wörld".to_string();
        assert_eq!(node.get_fs_name(), "hello_world.md");
    }

    #[test]
    fn normalization_fixes_erratic_casing() {
        let mut node = mock_node();
        
        node.title = "HeLLo woRlD".to_string();
        assert_eq!(node.get_fs_name(), "hello_world.md");
    }

    #[test]
    fn normalization_fixes_erratic_spacing() {
        let mut node = mock_node();
        
        node.title = "        hello   world   ".to_string();
        assert_eq!(node.get_fs_name(), "hello_world.md");
    }

    #[test]
    fn reject_empty_title() {
        let mut bad_node = mock_node();
        bad_node.title = String::from("   ");

        let result = bad_node.validate();

        assert!(result.is_err(), "Node title cannot ever be empty",);
    }

    #[test]
    fn reject_defined_empty_alias() {
        let mut bad_node = mock_node();
        bad_node.alias = Some(String::from("   "));

        let result = bad_node.validate();

        assert!(
            result.is_err(),
            "Defined aliases should not be allowed to be empty",
        );
    }

    #[test]
    fn reject_defined_empty_custom_id() {
        let mut bad_node = mock_node();
        bad_node.custom_id = Some(String::from("   "));

        let result = bad_node.validate();

        assert!(
            result.is_err(),
            "Defined custom ids should not be allowed to be empty",
        );
    }

    #[test]
    fn leaf_cannot_have_intro() {
        let mut bad_node = mock_node();
        bad_node.has_intro = true;

        let result = bad_node.validate();

        assert!(result.is_err(), "Leaf nodes with intros should fail");
    }
    #[test]
    fn parent_can_have_intro() {
        let child_node = mock_node();
        let mut parent_node = mock_node();
        parent_node.has_intro = true;
        parent_node.children = vec![child_node];

        let result = parent_node.validate();

        assert!(result.is_ok(), "Parent nodes with intros should be valid");
    }

    #[test]
    fn depth_changes_header_level() {
        let node = mock_node();
        let mut custom_id_node = mock_node();
        custom_id_node.custom_id = Some(String::from("custom"));

        let test_1 = node.get_section_header(2);
        let test_2 = custom_id_node.get_section_header(4);
        let expected_1 = "## Test";
        let expected_2 = "<h4 id=\"custom\">Test</h4>";

        assert!(
            test_1.eq(expected_1) && test_2.eq(expected_2)
        )
    }

    #[test]
    fn nesting_indents_index() {
        let node = mock_node();

        let result_1 = node.get_section_index(1);
        let result_2 = node.get_section_index(3);

        assert!(
            result_1.eq("- [Test](#test)") && result_2.eq("    - [Test](#test)"),
            "Parent nodes with intros should be valid"
        );
    }

    #[test]
    fn custom_id_appears_on_index() {
        let mut complicated_node = mock_node();
        complicated_node.custom_id = Some(String::from("my very smart id"));

        let result_1 = complicated_node.get_section_index(1);

        assert!(
            result_1.eq("- [Test](#my-very-smart-id)"),
            "Parent nodes with intros should be valid"
        );
    }
}
