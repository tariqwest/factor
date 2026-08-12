//! Response types for Agent Mode query suggestions (active AI prompt chips).
//!
//! Factor keeps only the wire/DTO shapes used by BYOP active_ai parsing — the
//! cloud `GenerateAMQuerySuggestions` endpoint is not used.

use serde::{Deserialize, Serialize};

use crate::ai::agent::FileLocations;

/// Top-level response type for GenerateAMQuerySuggestions (BYOP-parsed).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateAMQuerySuggestionsResponse {
    pub id: String,
    pub suggestion: Option<Suggestion>,
}

/// Represents a particular type of suggestion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Suggestion {
    Simple(SimpleQuery),
    Coding(CodingQuery),
}

/// Simple suggestion structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimpleQuery {
    pub query: String,
    /// If the query is a complex task, we should use Dispatch to create a plan first.
    pub should_plan_task: bool,
}

impl GenerateAMQuerySuggestionsResponse {
    /// Check whether the response contains a valid code delegation.
    pub fn is_valid_code_delegation(&self) -> bool {
        matches!(&self.suggestion, Some(Suggestion::Coding(coding_query)) if !coding_query.files.is_empty())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodingQuery {
    pub files: Vec<GeneratedFileLocations>,
    pub query: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeneratedFileLocations {
    pub file_name: String,
    pub line_numbers: Option<Vec<usize>>,
}

impl From<GeneratedFileLocations> for FileLocations {
    fn from(value: GeneratedFileLocations) -> Self {
        Self {
            name: value.file_name,
            // Line numbers discarded for now (same as Zap).
            lines: Vec::new(),
        }
    }
}
