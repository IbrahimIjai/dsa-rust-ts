use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use sqlx::{Executor, Pool, Sqlite, sqlite};

#[tokio::main]
async fn main() {
    let pool = db().await;

    let app = app(pool);
    let listenier = tokio::net::TcpListener::bind("").await.unwrap();

    println!("Listening on http://{}", &listenier.local_addr().unwrap());

    axum::serve(listenier, app).await.unwrap();
}

async fn db() -> Pool<Sqlite> {
    let opt = sqlite::SqliteConnectOptions::new()
        .filename("test.db")
        .create_if_missing(true);
    let pool = sqlx::SqlitePool::connect_with(opt).await.unwrap();

    pool.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT
        )",
    )
    .await
    .unwrap();

    pool
}

async fn app(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/list", get(get_person_list))
        .route("/add_person", post(add_new_person))
        .route("/person/{id}", get(get_single_person))
        .route("/remove_person/{id}", delete(remove_person))
        .route("/update_person/id", patch(update_person))
        .with_state(pool)
}


//9:43