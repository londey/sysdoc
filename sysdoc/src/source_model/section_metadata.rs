//! Section metadata for traceability
//!
//! This module defines metadata that can be embedded in markdown sections
//! using sysdoc code blocks to support requirements traceability.

use serde::Deserialize;

/// Metadata for a markdown section, parsed from `sysdoc` code blocks.
///
/// This struct is populated from TOML content within a fenced code block
/// with the `sysdoc` language identifier:
///
/// ```markdown
/// ```sysdoc
/// section_id = "REQ-001"
/// traced_ids = ["SRS-001", "SRS-002"]
/// ```
/// ```
///
/// The metadata enables traceability features like generating tables that map
/// section IDs to traced requirements and vice versa.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct SectionMetadata {
    /// Unique identifier for this section (e.g., "REQ-001", "SDD-3.2.1")
    pub section_id: Option<String>,

    /// List of IDs that this section traces to (e.g., requirements IDs)
    pub traced_ids: Option<Vec<String>>,

    /// If true, generate a table mapping section_ids to their traced_ids
    ///
    /// The generated table will have:
    /// - First column: section_id (sorted lexically)
    /// - Second column: comma-separated list of traced_ids (sorted lexically)
    pub generate_section_id_to_traced_ids_table: bool,

    /// If true, generate a table mapping traced_ids to section_ids that reference them
    ///
    /// The generated table will have:
    /// - First column: traced_id (deduplicated, sorted lexically)
    /// - Second column: comma-separated list of section_ids (sorted lexically)
    pub generate_traced_ids_to_section_ids_table: bool,
}

impl SectionMetadata {
    /// Parse metadata from TOML content
    ///
    /// # Parameters
    /// * `content` - TOML string to parse
    ///
    /// # Returns
    /// * `Ok(SectionMetadata)` - Successfully parsed metadata
    /// * `Err(toml::de::Error)` - Parse error
    pub fn parse(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    /// Check if this metadata has any traceability content
    pub fn has_traceability(&self) -> bool {
        self.section_id.is_some() || self.traced_ids.is_some()
    }

    /// Check if this metadata requests any table generation
    pub fn requests_table_generation(&self) -> bool {
        self.generate_section_id_to_traced_ids_table
            || self.generate_traced_ids_to_section_ids_table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let metadata = SectionMetadata::parse("").unwrap();
        assert_eq!(metadata.section_id, None);
        assert_eq!(metadata.traced_ids, None);
        assert!(!metadata.generate_section_id_to_traced_ids_table);
        assert!(!metadata.generate_traced_ids_to_section_ids_table);
    }

    #[test]
    fn test_parse_section_id_only() {
        let metadata = SectionMetadata::parse(r#"section_id = "REQ-001""#).unwrap();
        assert_eq!(metadata.section_id, Some("REQ-001".to_string()));
        assert_eq!(metadata.traced_ids, None);
    }

    #[test]
    fn test_parse_traced_ids() {
        let metadata = SectionMetadata::parse(r#"traced_ids = ["SRS-001", "SRS-002"]"#).unwrap();
        assert_eq!(
            metadata.traced_ids,
            Some(vec!["SRS-001".to_string(), "SRS-002".to_string()])
        );
    }

    #[test]
    fn test_parse_full_metadata() {
        let content = r#"
section_id = "SDD-3.2.1"
traced_ids = ["SRS-REQ-001", "SRS-REQ-002"]
generate_section_id_to_traced_ids_table = true
generate_traced_ids_to_section_ids_table = false
"#;
        let metadata = SectionMetadata::parse(content).unwrap();
        assert_eq!(metadata.section_id, Some("SDD-3.2.1".to_string()));
        assert_eq!(
            metadata.traced_ids,
            Some(vec!["SRS-REQ-001".to_string(), "SRS-REQ-002".to_string()])
        );
        assert!(metadata.generate_section_id_to_traced_ids_table);
        assert!(!metadata.generate_traced_ids_to_section_ids_table);
    }

    #[test]
    fn test_has_traceability() {
        let empty = SectionMetadata::default();
        assert!(!empty.has_traceability());

        let with_section_id = SectionMetadata {
            section_id: Some("REQ-001".to_string()),
            ..Default::default()
        };
        assert!(with_section_id.has_traceability());

        let with_traced_ids = SectionMetadata {
            traced_ids: Some(vec!["SRS-001".to_string()]),
            ..Default::default()
        };
        assert!(with_traced_ids.has_traceability());
    }

    #[test]
    fn test_requests_table_generation() {
        let empty = SectionMetadata::default();
        assert!(!empty.requests_table_generation());

        let with_forward = SectionMetadata {
            generate_section_id_to_traced_ids_table: true,
            ..Default::default()
        };
        assert!(with_forward.requests_table_generation());

        let with_reverse = SectionMetadata {
            generate_traced_ids_to_section_ids_table: true,
            ..Default::default()
        };
        assert!(with_reverse.requests_table_generation());
    }
}
