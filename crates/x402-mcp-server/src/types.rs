// Type definitions for MCP tools
//
// Epic 8: Parameter types and response structures

use serde::{Deserialize, Serialize};

/// Common MCP error response
///
/// This type provides structured error responses for MCP tools.
/// Currently unused but will be utilized in Phase 2 for error translation.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct McpError {
    /// Error code (E3xxx, E4xxx, E5xxx)
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Actionable suggestion for fixing the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
    /// Link to documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_link: Option<String>,
    /// Additional context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
}

impl McpError {
    /// Create a new error
    ///
    /// Library API for Phase 2 error handling
    #[allow(dead_code)]
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            suggestion: None,
            docs_link: None,
            context: None,
        }
    }

    /// Add a suggestion to the error
    ///
    /// Library API for Phase 2 error handling
    #[allow(dead_code)]
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Add a documentation link
    ///
    /// Library API for Phase 2 error handling
    #[allow(dead_code)]
    pub fn with_docs_link(mut self, link: impl Into<String>) -> Self {
        self.docs_link = Some(link.into());
        self
    }

    /// Add context data
    ///
    /// Library API for Phase 2 error handling
    #[allow(dead_code)]
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = Some(context);
        self
    }
}

// Tool parameter types will be added as we implement tools in Phase 1-2
