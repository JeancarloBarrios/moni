use std::sync::Arc;

use gcp_auth::Token;
use serde::{Deserialize, Serialize};

use crate::{errors::GemineAgentError, Content, CountTokensRequest, CountTokensResponse, Part};

static MODEL_NAME: &str = "gemini-pro";

pub struct GeminiAgent {
    gcp_generative_language_api_key: String,
    project_id: String,
    location_id: String,
    api_endpoint: String,
    gcp_provider: Arc<dyn gcp_auth::TokenProvider>,
}

pub struct GeminiAgentBuilder {
    gcp_generative_language_api_key: Option<String>,
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
            gcp_generative_language_api_key: None,
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

    pub fn gcp_generative_language_api_key(
        mut self,
        gcp_generative_language_api_key: &str,
    ) -> Self {
        self.gcp_generative_language_api_key = Some(gcp_generative_language_api_key.to_string());
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

        let gcp_generative_language_api_key =
            self.gcp_generative_language_api_key
                .ok_or(GemineAgentError::AgentBuilderMissing(
                    "gcp_generative_language_api_key".to_string(),
                ))?;

        Ok(GeminiAgent {
            gcp_provider: provider,
            project_id,
            location_id,
            api_endpoint,
            gcp_generative_language_api_key,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedingRequest {
    model: String,
    content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embedings {
    values: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedingResponse {
    embedding: Embedings,
}

pub struct EmbedingRequestBuilder {
    model: Option<String>,
    role: Option<String>,
    parts: Option<Vec<Part>>,
}

impl EmbedingRequestBuilder {
    fn new() -> Self {
        Self {
            model: None,
            role: None,
            parts: None,
        }
    }

    pub fn model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn role(mut self, role: &str) -> Self {
        self.role = Some(role.to_string());
        self
    }

    pub fn parts(mut self, parts: Vec<Part>) -> Self {
        self.parts = Some(parts);
        self
    }

    pub fn add_part(mut self, part: Part) -> Self {
        let mut parts = self.parts.unwrap_or_default();
        parts.push(part);
        self.parts = Some(parts);
        self
    }

    pub fn build(self) -> Result<EmbedingRequest, GemineAgentError> {
        let model = self
            .model
            .ok_or(GemineAgentError::AgentBuilderMissing("model".to_string()))?;
        let role = self
            .role
            .ok_or(GemineAgentError::AgentBuilderMissing("role".to_string()))?;
        let parts = self
            .parts
            .ok_or(GemineAgentError::AgentBuilderMissing("parts".to_string()))?;

        Ok(EmbedingRequest {
            model,
            content: Content { role, parts },
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

    pub async fn gen_embedings(
        &self,
        request: EmbedingRequest,
    ) -> Result<Embedings, GemineAgentError> {
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/text-embedding-004:embedContent?key={}", self.gcp_generative_language_api_key);
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(GemineAgentError::HTTPClient)?;

        let response = resp
            .json::<EmbedingResponse>()
            .await
            .map_err(GemineAgentError::HTTPClient)?;
        Ok(response.embedding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gemini_embeding() {
        println!("test-------------------------");
        let prompt = "";
        let model = "text-embedding-004";
        let api_key = "AIzaSyCcC8YZE4ksQsf52ra2jeDshr7m0oWGxM8";
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/text-embedding-004:embedContent?key={}", api_key);

        let payload = EmbedingRequest {
            model: model.to_string(),
            content: Content {
                role: "test".to_string(),
                parts: vec![Part::Text(prompt.to_string())],
            },
        };
        let resp = reqwest::Client::new()
            .post(url)
            .json(&payload)
            .send()
            .await
            .unwrap();

        let _response = resp.json::<EmbedingResponse>().await.unwrap();

        println!("test-------------------------");
    }

    #[tokio::test]
    async fn test_gen_embedings() {
        let agent = GeminiAgent::new()
            .project_id("test")
            .location_id("us-central1")
            .api_endpoint("generativelanguage.googleapis.com")
            .gcp_generative_language_api_key("AIzaSyCcC8YZE4ksQsf52ra2jeDshr7m0oWGxM8")
            .build()
            .await
            .unwrap();
        let embedings = agent
            .gen_embedings(EmbedingRequest {
                model: "text-embedding-004".to_string(),
                content: Content {
                    role: "test".to_string(),
                    parts: vec![Part::Text("some sort of promt".to_string())],
                },
            })
            .await
            .unwrap();
        println!("test-------------------------");
        println!("{:?}", embedings);
        println!("test-------------------------");
    }
}
