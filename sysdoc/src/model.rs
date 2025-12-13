//! Document model for representing parsed markdown documents

use pulldown_cmark::{Event, Parser, Tag};
use std::fmt;
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
    /// Parsed markdown events
    pub events: Vec<Event<'static>>,
    /// Image references found in the markdown
    pub images: Vec<ImageReference>,
    /// Table references found in the markdown (CSV files)
    pub tables: Vec<PathBuf>,
    /// Path to source file (for error reporting)
    #[allow(dead_code)]
    pub source_path: PathBuf,
}

impl Section {
    /// Parse the markdown content and extract references
    pub fn parse_content(&mut self) {
        let parser = Parser::new(&self.content);
        let mut events = Vec::new();
        let mut images = Vec::new();
        let mut tables = Vec::new();

        for event in parser {
            match &event {
                Event::Start(Tag::Image { dest_url, .. }) => {
                    let url = dest_url.to_string();
                    images.push(ImageReference {
                        url: url.clone(),
                        alt_text: String::new(), // Will be filled when we see the text
                    });
                }
                Event::Start(Tag::Link { dest_url, .. }) => {
                    let url = dest_url.to_string();
                    // Check if it's a CSV table reference
                    if url.ends_with(".csv") {
                        tables.push(PathBuf::from(url));
                    }
                }
                _ => {}
            }
            // Convert to 'static lifetime by cloning strings
            events.push(event.into_static());
        }

        self.events = events;
        self.images = images;
        self.tables = tables;
    }
}

/// Reference to an image in the markdown
#[derive(Debug, Clone)]
pub struct ImageReference {
    /// URL or path to the image
    #[allow(dead_code)]
    pub url: String,
    /// Alt text for the image
    #[allow(dead_code)]
    pub alt_text: String,
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
}

impl fmt::Display for SectionNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .parts
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(".");
        write!(f, "{}", s)
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
