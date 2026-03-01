//! Core types for the session store.

use serde::{Deserialize, Serialize};

/// Session status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Active,
    Archived,
    Distilled,
}

impl SessionStatus {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Archived => "archived",
            Self::Distilled => "distilled",
        }
    }
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Session type — classifies session lifecycle behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionType {
    Primary,
    Background,
    Ephemeral,
}

impl SessionType {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Background => "background",
            Self::Ephemeral => "ephemeral",
        }
    }

    /// Classify session type from key pattern.
    #[must_use]
    pub fn from_key(key: &str) -> Self {
        if key.contains("prosoche") {
            Self::Background
        } else if key.starts_with("ask:")
            || key.starts_with("spawn:")
            || key.starts_with("dispatch:")
            || key.starts_with("ephemeral:")
        {
            Self::Ephemeral
        } else {
            Self::Primary
        }
    }
}

impl std::fmt::Display for SessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Message role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
    ToolResult,
}

impl Role {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::ToolResult => "tool_result",
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A session record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub nous_id: String,
    pub session_key: String,
    pub parent_session_id: Option<String>,
    pub status: SessionStatus,
    pub model: Option<String>,
    pub token_count_estimate: i64,
    pub message_count: i64,
    pub last_input_tokens: i64,
    pub bootstrap_hash: Option<String>,
    pub distillation_count: i64,
    pub session_type: SessionType,
    pub last_distilled_at: Option<String>,
    pub computed_context_tokens: i64,
    pub thread_id: Option<String>,
    pub transport: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// A message record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub session_id: String,
    pub seq: i64,
    pub role: Role,
    pub content: String,
    pub tool_call_id: Option<String>,
    pub tool_name: Option<String>,
    pub token_estimate: i64,
    pub is_distilled: bool,
    pub created_at: String,
}

/// Usage record for a single turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub session_id: String,
    pub turn_seq: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub cache_read_tokens: i64,
    pub cache_write_tokens: i64,
    pub model: Option<String>,
}

/// Agent note — explicit agent-written context that survives distillation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNote {
    pub id: i64,
    pub session_id: String,
    pub nous_id: String,
    pub category: String,
    pub content: String,
    pub created_at: String,
}
