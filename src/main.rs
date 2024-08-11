#![allow(dead_code)]
mod data_sources;
mod documents;
mod models;
mod router;
mod routes;
mod settings;
mod templates;

use std::sync::Arc;
use gemini::client::GeminiClient;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
    vector_db: VectorDB,
    gemini_client: Arc<GeminiClient>,
}

#[derive(Clone)]
struct VectorDB {
    key: String,
    url: String,
}
async fn initialize_gemini(api_key: String) -> Arc<GeminiClient> {
    match GeminiClient::new(api_key).await {
        Ok(client) => Arc::new(client),
        Err(e) => {
            panic!("Failed to initialize GeminiClient: {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    // load settings
    let settings = settings::Settings::new().unwrap();

    println!("{:?}", settings);

    // setup database
    let db = PgPoolOptions::new()
        .max_connections(settings.database.connections)
        .connect(settings.database.url.as_str())
        .await
        .unwrap();

    // run migrations
    sqlx::migrate!().run(&db).await.unwrap();
    let v_db = VectorDB {
        key: settings.firebase_config.key,
        url: settings.firebase_config.url,
    };
    let gemini_client = initialize_gemini(settings.gemini_config.api_key).await;
    let app_state = Arc::new(AppState {
        pg_pool: db,
        vector_db: v_db,
        gemini_client,
    });

    let app = router::init_router(app_state);
    // run it
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", settings.server.host, settings.server.port))
            .await
            .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
