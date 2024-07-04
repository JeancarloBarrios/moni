
use sqlx::postgres::PgPoolOptions;

mod router;
mod routes;
mod settings;
mod templates;
 mod documents;

 #[tokio::main]
async fn main() {
    // load settings
    let settings = settings::Settings::new().unwrap();

    println!("{:?}", settings.database.url.as_str());

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(settings.database.url.as_str())
        .await
        .unwrap();

    println!("{:?}", db);

    sqlx::migrate!().run(&db).await.unwrap();

    let app = router::init_router();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
