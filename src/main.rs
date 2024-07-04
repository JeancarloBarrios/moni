use sqlx::postgres::PgPoolOptions;

mod documents;
mod router;
mod routes;
mod settings;
mod templates;

#[tokio::main]
async fn main() {
    // load settings
    let settings = settings::Settings::new().unwrap();

    println!("{:?}", settings.database.url.as_str());

    // setup database
    let db = PgPoolOptions::new()
        .max_connections(settings.database.connections)
        .connect(settings.database.url.as_str())
        .await
        .unwrap();

    // run migrations
    sqlx::migrate!().run(&db).await.unwrap();

    let app = router::init_router();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
