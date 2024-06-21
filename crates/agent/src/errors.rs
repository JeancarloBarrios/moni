use thiserror::Error;

#[derive(Error, Debug)]
pub enum GemineAgentError {
    #[error("attribute {0} missing")]
    AgentBuilderMissing(String),

    #[error("gcp auth error")]
    GCPAuth(gcp_auth::Error),
}
