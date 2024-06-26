use std::sync::Arc;

use gcp_auth::Token;

use crate::{errors::GemineAgentError, Content, CountTokensRequest, CountTokensResponse, Part};

static MODEL_NAME: &str = "gemini-pro";

pub struct GeminiAgent {
    project_id: String,
    location_id: String,
    api_endpoint: String,
    gcp_provider: Arc<dyn gcp_auth::TokenProvider>,
}

pub struct GeminiAgentBuilder {
    project_id: Option<String>,
    location_id: Option<String>,
    api_endpoint: Option<String>,
}

impl GeminiAgentBuilder {
    fn new() -> Self {
        Self {
            project_id: None,
            location_id: None,
            api_endpoint: None,
        }
    }

    pub fn project_id(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_string());
        self
    }

    pub fn location_id(mut self, location_id: &str) -> Self {
        self.location_id = Some(location_id.to_string());
        self
    }

    pub fn api_endpoint(mut self, api_endpoint: &str) -> Self {
        self.api_endpoint = Some(api_endpoint.to_string());
        self
    }

    pub async fn build(self) -> Result<GeminiAgent, GemineAgentError> {
        let provider = gcp_auth::provider()
            .await
            .map_err(GemineAgentError::GCPAuth)?;
        let project_id = self
            .project_id
            .ok_or(GemineAgentError::AgentBuilderMissing(
                "project_id".to_string(),
            ))?;
        let location_id = self
            .location_id
            .ok_or(GemineAgentError::AgentBuilderMissing(
                "location_id".to_string(),
            ))?;
        let api_endpoint = self
            .api_endpoint
            .ok_or(GemineAgentError::AgentBuilderMissing(
                "api_endpoint".to_string(),
            ))?;

        Ok(GeminiAgent {
            gcp_provider: provider,
            project_id,
            location_id,
            api_endpoint,
        })
    }
}

impl GeminiAgent {
    pub fn new() -> GeminiAgentBuilder {
        GeminiAgentBuilder::new()
    }

    async fn get_token(&mut self) -> Result<Arc<Token>, gcp_auth::Error> {
        let provider = &self.gcp_provider;
        let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
        let token = provider.token(scopes).await?;
        Ok(token)
    }

    fn get_url(&self) -> String {
        let project_id = &self.project_id;
        let location_id = &self.location_id;
        let api_endpoint = &self.api_endpoint;
        let endpoint_url = format!(
        "https://{api_endpoint}/v1beta1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:countTokens"
        );
        endpoint_url
    }

    async fn test_promt(&mut self) -> Result<(), GemineAgentError> {
        let token = self.get_token().await.map_err(GemineAgentError::GCPAuth)?;
        let prompt = "What is the airspeed of an unladen swallow?";

        let payload = CountTokensRequest {
            contents: Content {
                role: "user".to_string(),
                parts: vec![Part::Text(prompt.to_string())],
            },
        };

        let resp = reqwest::Client::new()
            .post(&self.get_url())
            .bearer_auth(token.as_str())
            .json(&payload)
            .send()
            .await
            .map_err(GemineAgentError::HTTPClient)?;

        let response = resp
            .json::<CountTokensResponse>()
            .await
            .map_err(GemineAgentError::HTTPClient)?;

        println!("{}", response.total_tokens);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gemini_agent_builder() {
        let result = GeminiAgent::new()
            .project_id("project_id")
            .location_id("location_id")
            .api_endpoint("api_endpoint")
            .build()
            .await;
        print!("test");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_gemini_agent_builder_missing_project_id() {
        let result = GeminiAgent::new()
            .location_id("location_id")
            .api_endpoint("api_endpoint")
            .build()
            .await;
        assert!(matches!(
            result,
            Err(GemineAgentError::AgentBuilderMissing(_))
        ));
    }

    #[tokio::test]
    async fn test_gemini_agent_builder_missing_location_id() {
        let result = GeminiAgent::new()
            .project_id("project_id")
            .api_endpoint("api_endpoint")
            .build()
            .await;
        assert!(matches!(
            result,
            Err(GemineAgentError::AgentBuilderMissing(_))
        ));
    }

    #[tokio::test]
    async fn test_gemini_agent_builder_missing_api_endpoint() {
        let result = GeminiAgent::new()
            .project_id("project_id")
            .location_id("location_id")
            .build()
            .await;
        assert!(matches!(
            result,
            Err(GemineAgentError::AgentBuilderMissing(_))
        ));
    }
}
