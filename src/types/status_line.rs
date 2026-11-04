//! Maps to: CC `types/statusLine.ts` / the runtime shape built by
//! `components/StatusLine.tsx#buildStatusLineCommandInput`.

use crate::types::message::TokenUsage;
use serde::{Deserialize, Serialize};

/// Maps to: CC `StatusLineCommandInput` (stdin JSON for `statusLine.command`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusLineCommandInput {
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_name: Option<String>,
    pub transcript_path: String,
    pub cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_mode: Option<String>,
    pub model: StatusLineModel,
    pub workspace: StatusLineWorkspace,
    pub version: String,
    pub output_style: StatusLineOutputStyle,
    pub cost: StatusLineCost,
    pub context_window: StatusLineContextWindow,
    pub exceeds_200k_tokens: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<StatusLineRateLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vim: Option<StatusLineVim>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<StatusLineAgent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<StatusLineRemote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worktree: Option<StatusLineWorktree>,
    // Fields newer CC builds send (observed on CC 2.1.316) that status line
    // tools such as ccline read. Optional so older consumers are unaffected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<StatusLineEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<StatusLineThinking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache: Option<StatusLinePromptCache>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineEffort {
    pub level: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineThinking {
    pub enabled: bool,
}

/// Prompt-cache health for the main conversation thread, in the shape CC
/// 2.1.316 emits. Cometix fills the subset it can observe from API usage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusLinePromptCache {
    pub warm: bool,
    pub caching_observed: bool,
    pub ttl: String,
    pub expires_at: Option<i64>,
    pub requests: u64,
    pub misses: u64,
    pub hit_ratio: Option<f64>,
    pub cache_write_tokens: u64,
    pub last_miss_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineModel {
    pub id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineWorkspace {
    pub current_dir: String,
    pub project_dir: String,
    pub added_dirs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineOutputStyle {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusLineCost {
    pub total_cost_usd: f64,
    pub total_duration_ms: u64,
    pub total_api_duration_ms: u64,
    pub total_lines_added: u64,
    pub total_lines_removed: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusLineContextWindow {
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub context_window_size: i64,
    /// Token usage from the last API call; `null` when no assistant usage yet.
    pub current_usage: Option<TokenUsage>,
    pub used_percentage: Option<u8>,
    pub remaining_percentage: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusLineRateLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub five_hour: Option<StatusLineRateLimitWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seven_day: Option<StatusLineRateLimitWindow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusLineRateLimitWindow {
    pub used_percentage: f64,
    pub resets_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineVim {
    pub mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineAgent {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineRemote {
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusLineWorktree {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    pub original_cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_branch: Option<String>,
}
