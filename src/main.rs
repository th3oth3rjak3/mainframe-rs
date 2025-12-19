mod auth;
mod authentication;
mod background_jobs;
mod database;
mod docs;
mod errors;
mod recipes;
mod roles;
mod services;
mod sessions;
mod shared_models;
mod token;
mod users;

use authentication::router as auth_router;
use axum::Router;
use database::Database;
use dotenvy::dotenv;
use recipes::router as recipe_router;
use roles::router as role_router;
use services::ServiceContainer;
use sessions::router as session_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::EnvFilter;
use users::router as user_router;
use utoipa_scalar::{Scalar, Servable};

use crate::{background_jobs::spawn_cleanup_task, docs::ApiDoc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    // Initialize DB and ServiceContainer
    let db = Database::new().await?;
    let container = ServiceContainer::new(db.pool.clone());
    let session_repo = container.session_repo();

    let app = Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::merge_modules()))
        .nest("/api/recipes", recipe_router())
        .nest("/api/users", user_router())
        .nest("/api/auth", auth_router())
        .nest("/api/sessions", session_router())
        .nest("/api/roles", role_router())
        .fallback_service(
            ServeDir::new("static").not_found_service(ServeFile::new("static/index.html")),
        )
        .with_state(container);

    let _cleanup_handle = spawn_cleanup_task(session_repo);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
