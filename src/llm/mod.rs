/*! # LLM Integration Module

This module provides integration with Google Gemini 1.5 Flash for:
- Markdown text improvement
- Command suggestions and corrections
- Code generation assistance

## Usage

Set the `GEMINI_API_KEY` environment variable:
```bash
export GEMINI_API_KEY="your_api_key_here"
```

This feature is optional and only enabled when the API key is set.
*/

#[cfg(feature = "llm")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "llm")]
#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[cfg(feature = "llm")]
#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[cfg(feature = "llm")]
#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[cfg(feature = "llm")]
#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[cfg(feature = "llm")]
#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[cfg(feature = "llm")]
#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[cfg(feature = "llm")]
#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: String,
}

/// Gemini LLM client
#[derive(Debug)]
pub struct GeminiClient {
    #[allow(dead_code)]
    api_key: String,
    #[cfg(feature = "llm")]
    client: reqwest::blocking::Client,
}

impl GeminiClient {
    /// Create a new Gemini client from environment variable
    pub fn from_env() -> Result<Self, String> {
        let api_key = std::env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY environment variable not set".to_string())?;

        #[cfg(feature = "llm")]
        {
            Ok(Self {
                api_key,
                client: reqwest::blocking::Client::new(),
            })
        }

        #[cfg(not(feature = "llm"))]
        {
            Ok(Self { api_key })
        }
    }

    /// Improve Markdown text with Gemini
    pub fn improve_markdown(&self, text: &str) -> Result<String, String> {
        #[cfg(feature = "llm")]
        {
            let prompt = format!(
                "以下のMarkdownテキストを改善してください。文法、スタイル、読みやすさを向上させますが、内容の意味は変えないでください:\n\n{}",
                text
            );
            self.call_gemini(&prompt)
        }

        #[cfg(not(feature = "llm"))]
        {
            let _ = text;  // Suppress unused warning
            Err("LLM feature is not enabled. Build with --features llm".to_string())
        }
    }

    /// Fix command with Gemini suggestions
    pub fn fix_command(&self, command: &str, error: Option<&str>) -> Result<String, String> {
        #[cfg(feature = "llm")]
        {
            let prompt = if let Some(err) = error {
                format!(
                    "次のコマンドがエラーを起こしました:\nコマンド: {}\nエラー: {}\n\n修正されたコマンドを提案してください（説明なしでコマンドのみ）:",
                    command, err
                )
            } else {
                format!(
                    "次のコマンドを改善してください:\n{}\n\n改善されたコマンドを提案してください（説明なしでコマンドのみ）:",
                    command
                )
            };
            self.call_gemini(&prompt)
        }

        #[cfg(not(feature = "llm"))]
        {
            let _ = (command, error);  // Suppress unused warnings
            Err("LLM feature is not enabled. Build with --features llm".to_string())
        }
    }

    #[cfg(feature = "llm")]
    fn call_gemini(&self, prompt: &str) -> Result<String, String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            self.api_key
        );

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "API request failed with status: {}",
                response.status()
            ));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        gemini_response
            .candidates
            .get(0)
            .and_then(|c| c.content.parts.get(0))
            .map(|p| p.text.clone())
            .ok_or_else(|| "No response from Gemini".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_client_creation() {
        // This test requires GEMINI_API_KEY to be set
        if std::env::var("GEMINI_API_KEY").is_ok() {
            assert!(GeminiClient::from_env().is_ok());
        }
    }
}
