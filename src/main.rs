use std::net::SocketAddr;

use axum::{Router, routing::{post, get}, extract::State, response::IntoResponse};
use sqlx::{Row, SqlitePool};

use crate::errors::MailfrenError;

mod errors;
mod database;
mod schemas;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .nest("/v1", Router::new()
            .route("/frens", post(routes::create_fren))
            .route("/frens/:id", get(routes::get_fren))
            .route("/frens/:id/image.png", get(routes::greet_fren))
            .route("/frens/:id/greetings", get(routes::get_greetings))
        )
        .route("/", get(routes::homepage))
        .fallback(routes::fallback)
        .with_state(database::get_pool().await);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("could not set tcp listener on 127.0.0.1:3001");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap()
    
}
