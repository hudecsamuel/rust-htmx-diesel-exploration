use axum::Router;
use axum_login::{AuthManagerLayerBuilder, login_required};
use log::info;
use time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
// use serde::{Deserialize, Serialize};
use std::env;
// use tokio::signal;
use crate::{
    controllers::{home_controller, auth_controller},
    repositories::{postgres_store::PostgresStore, auth_backend::Backend},
};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

mod controllers;
mod db;
mod models;
mod repositories;
mod templates;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// Struct to hold the application state
#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("🚀 Server starting...");

    let app_environment = env::var("APP_ENVIRONMENT").unwrap_or("development".to_string());
    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("80".to_string());

    info!(
        "Server configured to accept connections on host {}...",
        app_host
    );
    info!(
        "Server configured to listen connections on port {}...",
        app_port
    );

    match app_environment.as_str() {
        "development" => {
            info!("Running in development mode");
        }
        "production" => {
            info!("Running in production mode");
        }
        _ => {
            info!("Running in development mode");
        }
    }

    let db_pool = PgPool::builder()
        .max_size(5)
        .build(ConnectionManager::<PgConnection>::new(
            env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        ))
        .expect("Failed to create pool.");

    let state = AppState { pool: db_pool.clone() };

    let session_store = PostgresStore::new(db_pool.clone());

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // Auth service.
    //
    // This combines the session layer with our backend to establish the auth
    // service which will provide the auth session as a request extension.
    let backend = Backend::new(db_pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .merge(home_controller::router())
        .merge(auth_controller::router())
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .layer(auth_layer)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// async fn shutdown_signal() {
//     let ctrl_c = async {
//         signal::ctrl_c()
//             .await
//             .expect("failed to install Ctrl+C handler");
//     };

//     #[cfg(unix)]
//     let terminate = async {
//         signal::unix::signal(signal::unix::SignalKind::terminate())
//             .expect("failed to install signal handler")
//             .recv()
//             .await;
//     };

//     #[cfg(not(unix))]
//     let terminate = std::future::pending::<()>();

//     tokio::select! {
//         _ = ctrl_c => {},
//         _ = terminate => {},
//     }

//     info!("signal received, starting graceful shutdown");
// }
