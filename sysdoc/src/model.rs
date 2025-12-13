//! Document model for representing parsed markdown documents

use std::path::PathBuf;

/// Represents the entire document being built
#[derive(Debug)]
pub struct Document {
    /// Root directory of the document source
    #[allow(dead_code)]
    pub root: PathBuf,
    /// Ordered sections of the document
    pub sections: Vec<Section>,
}

impl Document {
    /// Create a new empty document
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            sections: Vec::new(),
        }
    }
}

/// A section in the document (corresponds to a markdown file)
#[derive(Debug)]
pub struct Section {
    /// Section number parsed from filename (e.g., [1, 1] from "01.01_purpose.md")
    pub number: SectionNumber,
    /// Section title extracted from filename (e.g., "Purpose" from "01.01_purpose.md")
    pub title: String,
    /// Nesting level (0 = top level, 1 = first subsection, etc.)
    pub depth: usize,
    /// Raw markdown content
    pub content: String,
    /// Path to source file (for error reporting)
    #[allow(dead_code)]
    pub source_path: PathBuf,
}

/// Section number representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SectionNumber {
    /// Number components (e.g., [1, 2, 3] for "01.02.03")
    pub parts: Vec<u32>,
}

impl SectionNumber {
    /// Parse section number from filename prefix
    /// Examples: "01.01" -> [1, 1], "02.03.01" -> [2, 3, 1]
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Option<Vec<u32>> = s.split('.').map(|part| part.parse::<u32>().ok()).collect();

        parts.map(|parts| Self { parts })
    }

    /// Get the depth/nesting level (number of parts - 1)
    pub fn depth(&self) -> usize {
        self.parts.len().saturating_sub(1)
    }

    /// Format as string (e.g., "1.2.3")
    pub fn to_string(&self) -> String {
        self.parts
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_number_parse() {
        let num = SectionNumber::parse("01.01").unwrap();
        assert_eq!(num.parts, vec![1, 1]);
        assert_eq!(num.depth(), 1);

        let num = SectionNumber::parse("02.03.01").unwrap();
        assert_eq!(num.parts, vec![2, 3, 1]);
        assert_eq!(num.depth(), 2);
    }

    #[test]
    fn test_section_number_ordering() {
        let num1 = SectionNumber::parse("01.01").unwrap();
        let num2 = SectionNumber::parse("01.02").unwrap();
        let num3 = SectionNumber::parse("02.01").unwrap();

        assert!(num1 < num2);
        assert!(num2 < num3);
    }
}
