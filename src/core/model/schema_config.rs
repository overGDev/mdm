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
    pub children: Vec<SchemaSection>,
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
            children: vec![],
        }
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
}
