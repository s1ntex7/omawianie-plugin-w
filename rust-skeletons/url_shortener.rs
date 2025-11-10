// ============================================================================
// URL SHORTENER
// ============================================================================
// Priority: P2 (Nice to have)
// Tech: HTTP client + clipboard_manager
// Features: Shorten URLs using your domain, copy to clipboard
// Estimated time: 10 hours (requires API setup)
// ============================================================================

use crate::clipboard_manager::ClipboardManager;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortenedUrl {
    pub short_code: String,
    pub short_url: String,
    pub long_url: String,
    pub created_at: i64,
    pub clicks: u32,
}

pub struct UrlShortener {
    clipboard_manager: ClipboardManager,
    api_endpoint: String,
    api_key: String,
}

impl UrlShortener {
    pub fn new(clipboard_manager: ClipboardManager, api_endpoint: String, api_key: String) -> Self {
        Self {
            clipboard_manager,
            api_endpoint,
            api_key,
        }
    }

    /// Shorten URL and copy to clipboard
    pub async fn shorten_and_copy(&self, long_url: &str) -> Result<ShortenedUrl, String> {
        let shortened = self.shorten(long_url).await?;
        self.clipboard_manager.paste_text(&shortened.short_url)?;
        Ok(shortened)
    }

    /// Shorten URL using API
    async fn shorten(&self, long_url: &str) -> Result<ShortenedUrl, String> {
        // TODO: Implement API call
        //
        // Options:
        // 1. Self-hosted (YOURLS, Polr, Shlink)
        // 2. Cloud API (Bitly, TinyURL, Rebrandly)
        //
        // Example with reqwest:
        // ```rust
        // let client = reqwest::Client::new();
        // let response = client.post(&self.api_endpoint)
        //     .header("Authorization", format!("Bearer {}", self.api_key))
        //     .json(&serde_json::json!({ "long_url": long_url }))
        //     .send()
        //     .await?;
        // let data: ShortenedUrl = response.json().await?;
        // ```

        Err("Not implemented - requires API setup".to_string())
    }

    /// Get shortened URL stats
    pub async fn get_stats(&self, short_code: &str) -> Result<ShortenedUrl, String> {
        // TODO: Query API for stats (clicks, created_at, etc.)
        Err("Not implemented".to_string())
    }
}

// ============================================================================
// INTEGRATION: See clipboard_manager.rs for similar patterns
// ============================================================================
