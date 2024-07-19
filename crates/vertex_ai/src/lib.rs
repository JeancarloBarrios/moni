pub mod client;
pub mod data_store;
pub mod error;
use std::{collections::HashMap, sync::Arc};

use serde_json::Value;

use error::VertexError;
use gcp_auth::TokenProvider;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

const BASE_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";
