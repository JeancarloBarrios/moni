#![allow(dead_code)]
mod data_sources;
mod documents;
mod models;
mod router;
mod routes;
mod settings;
mod templates;

use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
    vector_db: VectorDB,
}

#[derive(Clone)]
struct VectorDB {
    key: String,
    url: String,
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

    let app_state = Arc::new(AppState {
        pg_pool: db,
        vector_db: v_db,
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
