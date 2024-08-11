use std::sync::Arc;
use std::fmt;
use serde_json::Value;
use tokio::sync::OnceCell;
use std::error::Error as StdError;
use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{request::Request, Content, Part, Role},
};
use std::error::Error;
use google_generative_ai_rs::v1::api::PostResult;
use log::info;
#[derive(Debug)]
pub enum GeminiError {
    Reqwest(reqwest::Error), // to handle reqwest-related errors
    InvalidApiKey(String),   // for custom application-specific errors
}

impl fmt::Display for GeminiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeminiError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            GeminiError::InvalidApiKey(key) => write!(f, "Invalid API key: {}", key),
        }
    }
}

impl StdError for GeminiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            GeminiError::Reqwest(err) => Some(err),
            GeminiError::InvalidApiKey(_) => None,
        }
    }
}

pub struct GeminiClient {
    client: Client,
}

impl GeminiClient {
    pub async fn new(api_key: String) -> Result<Self, GeminiError> {
        let client = Client::new(api_key);
        Ok(Self { client })
    }
    pub async fn request_text(&self, prompt: &str) -> Result<PostResult, Box<dyn Error>> {
        let txt_request = Request {
            contents: vec![Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(prompt.to_string()),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            }],
            tools: vec![],
            safety_settings: vec![],
            generation_config: None,

            system_instruction: None,
        };

        let response = self.client.post(30, &txt_request).await?;
        info!("{:#?}", response);
        Ok(response)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use tokio;
    use std::env;

    #[tokio::test]
    async fn test_request_text() -> Result<(), Box<dyn Error>> {
        // Fetch the API key from the environment variables and handle the case where it's not set
        let api_key = env::var("API_KEY").map_err(|_| {
            "API_KEY environment variable not set. Please set it before running the tests."
        })?;

        let client = GeminiClient::new(api_key).await?;

        let prompt = "What is the capital of France?";

        let response = client.request_text(prompt).await;

        // Check if the response is Ok
        assert!(response.is_ok());

        let response = response.unwrap();

        // Print the full response
        println!("Full response: {:?}", response);

        if let Some(gemini_response) = response.rest() {
            let default_value = "no answer from Gemini".to_string();
            let answer = gemini_response.candidates[0]
                .content
                .parts[0]
                .text
                .as_ref()
                .unwrap_or(&default_value);
            assert!(answer.contains("Paris"));
        } else {
            panic!("Expected a Rest response");
        }

        Ok(())
    }
}
