//! Configuration schema for tracey
//!
//! Config lives at `.config/tracey/config.kdl` relative to the project root.

use facet::Facet;

/// Root configuration for tracey
#[derive(Debug, Facet)]
pub struct Config {
    /// Specifications to track coverage against
    pub specs: Vec<SpecConfig>,
}

/// Configuration for a single specification
#[derive(Debug, Facet)]
pub struct SpecConfig {
    /// Name of the spec (for display purposes)
    pub name: String,

    /// URL to the spec's _rules.json manifest
    /// e.g., "https://rapace.dev/_rules.json"
    pub rules_url: String,

    /// Glob patterns for Rust files to scan
    /// Defaults to ["**/*.rs"] if not specified
    #[facet(default)]
    pub include: Vec<String>,

    /// Glob patterns to exclude
    #[facet(default)]
    pub exclude: Vec<String>,
}

impl Default for SpecConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            rules_url: String::new(),
            include: vec!["**/*.rs".to_string()],
            exclude: vec!["target/**".to_string()],
        }
    }
}
