mod database;
mod errors;
mod recipes;
mod services;
mod users;

use axum::Router;
use database::Database;
use dotenvy::dotenv;
use recipes::router as recipe_router;
use services::ServiceContainer;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;
use users::router as user_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize DB and ServiceContainer
    let db = Database::new().await?;
    let container = Arc::new(ServiceContainer::new(db.pool.clone()));

    let app = Router::new()
        .nest("/api/recipes", recipe_router())
        .nest("/api/users", user_router())
        .fallback_service(ServeDir::new("frontend"))
        .with_state(container.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
