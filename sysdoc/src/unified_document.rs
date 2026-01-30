//! Unified document model for the transformation stage (Stage 2)
//!
//! This module defines the structures used after parsing source files
//! and aggregating them into a unified document structure ready for export.

use crate::source_model::{MarkdownSection, TableSource};
use std::path::PathBuf;

/// The unified document model ready for export
#[derive(Debug)]
pub struct UnifiedDocument {
    /// Document metadata
    pub metadata: DocumentMetadata,
    /// Root directory of the source
    pub root: PathBuf,
    /// Sorted sections of the document (from all markdown files)
    pub sections: Vec<MarkdownSection>,
    /// All tables used in the document
    pub tables: Vec<TableSource>,
}

impl UnifiedDocument {
    /// Create a new empty unified document
    ///
    /// # Parameters
    /// * `metadata` - Document metadata including title, owner, approver, etc.
    /// * `root` - Root directory path of the document source
    ///
    /// # Returns
    /// * `UnifiedDocument` - A new empty unified document with no sections or tables
    pub fn new(metadata: DocumentMetadata, root: PathBuf) -> Self {
        Self {
            metadata,
            root,
            sections: Vec::new(),
            tables: Vec::new(),
        }
    }

    /// Get the total number of tables
    ///
    /// # Returns
    /// * `usize` - Total number of tables in the document
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    /// Get the total number of sections
    ///
    /// # Returns
    /// * `usize` - Total number of sections in the document
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Get the total word count across all sections
    ///
    /// # Returns
    /// * `usize` - Total word count (currently counts content blocks, not actual words)
    pub fn word_count(&self) -> usize {
        // TODO: Implement proper word counting from MarkdownBlock content
        self.sections.iter().map(|s| s.content.len()).sum()
    }

    /// Get the total number of images
    ///
    /// # Returns
    /// * `usize` - Total number of images embedded in all sections
    pub fn image_count(&self) -> usize {
        use crate::source_model::MarkdownBlock;
        self.sections
            .iter()
            .flat_map(|s| &s.content)
            .filter(|block| matches!(block, MarkdownBlock::Image { .. }))
            .count()
    }
}

/// Document metadata
#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    /// System identifier (if any)
    pub system_id: Option<String>,
    /// Document unique identifier
    pub document_id: String,
    /// Document title
    pub title: String,
    /// Document subtitle (if any)
    pub subtitle: Option<String>,
    /// Document description (if any)
    pub description: Option<String>,
    /// Document type (SDD, SRS, etc.)
    pub doc_type: String,
    /// Standard/specification
    pub standard: String,
    /// Template used
    pub template: String,
    /// Document owner
    pub owner: Person,
    /// Document approver
    pub approver: Person,
    /// Version number (if any)
    pub version: Option<String>,
    /// Last modified date
    pub modified: Option<String>,
    /// Revision history entries extracted from git tags
    pub revision_history: Vec<RevisionHistoryEntry>,
    /// Optional protection/classification marking (e.g., "PC-PROTECTED//DESIGN")
    pub protection_mark: Option<String>,
    /// Optional path to a background image for the title page (used in PDF and HTML outputs)
    pub title_page_background: Option<String>,
    /// Heading color for PDF output as a hex color string (e.g., "#2B579A")
    pub heading_color: String,
}

/// Person information
#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub email: String,
}

/// A single entry in the revision history table
#[derive(Debug, Clone)]
pub struct RevisionHistoryEntry {
    /// Version identifier (typically the git tag name, e.g., "v1.0.0")
    pub version: String,
    /// Date of the revision in ISO 8601 format
    pub date: String,
    /// Description of the changes (from tag message or commit subject)
    pub description: String,
}

/// Format an ISO 8601 date string to display format (e.g., "6 Jul 2026")
///
/// # Parameters
/// * `iso_date` - ISO 8601 date string (e.g., "2026-07-06T12:34:56+00:00")
///
/// # Returns
/// * Formatted date string in "d Mon YYYY" format, or the original string if parsing fails
pub fn format_display_date(iso_date: &str) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    // ISO 8601 format: YYYY-MM-DDTHH:MM:SS...
    // We only need the date part before 'T'
    let date_part = iso_date.split('T').next().unwrap_or(iso_date);
    let parts: Vec<&str> = date_part.split('-').collect();

    if parts.len() >= 3 {
        if let (Ok(year), Ok(month), Ok(day)) = (
            parts[0].parse::<i32>(),
            parts[1].parse::<usize>(),
            parts[2].parse::<u32>(),
        ) {
            if (1..=12).contains(&month) {
                return format!("{} {} {}", day, MONTHS[month - 1], year);
            }
        }
    }

    // Return original if parsing fails
    iso_date.to_string()
}

/// Builder for constructing a UnifiedDocument from source models
pub struct DocumentBuilder {
    metadata: DocumentMetadata,
    root: PathBuf,
    sections: Vec<MarkdownSection>,
    tables: Vec<TableSource>,
}

impl DocumentBuilder {
    /// Create a new document builder
    ///
    /// # Parameters
    /// * `metadata` - Document metadata including title, owner, approver, etc.
    /// * `root` - Root directory path of the document source
    ///
    /// # Returns
    /// * `DocumentBuilder` - A new builder with empty sections and tables
    pub fn new(metadata: DocumentMetadata, root: PathBuf) -> Self {
        Self {
            metadata,
            root,
            sections: Vec::new(),
            tables: Vec::new(),
        }
    }

    /// Add a section to the document
    ///
    /// # Parameters
    /// * `section` - Markdown section to add
    pub fn add_section(&mut self, section: MarkdownSection) {
        self.sections.push(section);
    }

    /// Add a table to the document
    ///
    /// # Parameters
    /// * `table` - Table source to add
    pub fn add_table(&mut self, table: TableSource) {
        self.tables.push(table);
    }

    /// Build the unified document
    ///
    /// # Returns
    /// * `UnifiedDocument` - The completed unified document with all added content
    pub fn build(self) -> UnifiedDocument {
        UnifiedDocument {
            metadata: self.metadata,
            root: self.root,
            sections: self.sections,
            tables: self.tables,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_model::SectionNumber;

    fn test_metadata() -> DocumentMetadata {
        DocumentMetadata {
            system_id: None,
            document_id: "TEST-001".to_string(),
            title: "Test Document".to_string(),
            subtitle: None,
            description: None,
            doc_type: "SDD".to_string(),
            standard: "DI-IPSC-81435B".to_string(),
            template: "sdd-standard-v1".to_string(),
            owner: Person {
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            },
            approver: Person {
                name: "Jane Smith".to_string(),
                email: "jane@example.com".to_string(),
            },
            version: None,
            modified: None,
            revision_history: Vec::new(),
            protection_mark: None,
            title_page_background: None,
            heading_color: "#2B579A".to_string(),
        }
    }

    #[test]
    fn test_document_builder() {
        let section = MarkdownSection {
            heading_level: 1,
            heading_text: "Introduction".to_string(),
            section_number: SectionNumber::parse("1").unwrap(),
            line_number: 1,
            source_file: PathBuf::from("test.md"),
            content: vec![],
            metadata: None,
        };

        let mut builder = DocumentBuilder::new(test_metadata(), PathBuf::from("."));
        builder.add_section(section);
        let doc = builder.build();

        assert_eq!(doc.sections.len(), 1);
        assert_eq!(doc.sections[0].heading_text, "Introduction");
    }

    #[test]
    fn test_table_count() {
        let doc = UnifiedDocument::new(test_metadata(), PathBuf::from("."));
        assert_eq!(doc.table_count(), 0);
    }

    #[test]
    fn test_format_display_date_iso8601() {
        // Standard ISO 8601 format from git
        assert_eq!(
            format_display_date("2026-07-06T12:34:56+00:00"),
            "6 Jul 2026"
        );
        assert_eq!(
            format_display_date("2024-01-15T09:00:00+10:00"),
            "15 Jan 2024"
        );
        assert_eq!(
            format_display_date("2025-12-31T23:59:59-05:00"),
            "31 Dec 2025"
        );
    }

    #[test]
    fn test_format_display_date_date_only() {
        // Date only without time component
        assert_eq!(format_display_date("2026-07-06"), "6 Jul 2026");
        assert_eq!(format_display_date("2024-02-29"), "29 Feb 2024");
    }

    #[test]
    fn test_format_display_date_invalid_returns_original() {
        // Invalid formats should return original string
        assert_eq!(format_display_date("not a date"), "not a date");
        assert_eq!(format_display_date("2024-13-01"), "2024-13-01"); // Invalid month
        assert_eq!(format_display_date(""), "");
    }
}
