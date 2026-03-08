mod db;
mod handlers;
mod middleware;
mod models;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use db::Db;
use middleware::auth::auth_middleware;

pub struct AppState {
    db: Db,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./blog.db?mode=rwc".to_string());

    let db = db::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    db::init_db(&db)
        .await
        .expect("Failed to initialize database");

    let state = Arc::new(AppState { db });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 公开路由
    let public_routes = Router::new()
        .route("/api/posts", get(handlers::list_posts))
        .route("/api/posts/:id", get(handlers::get_post))
        .route("/api/posts/:id/comments", get(handlers::list_comments))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/register", post(handlers::register));

    // 需要认证的路由
    let protected_routes = Router::new()
        .route("/api/posts", post(handlers::create_post))
        .route("/api/posts/:id", put(handlers::update_post))
        .route("/api/posts/:id", delete(handlers::delete_post))
        .route("/api/posts/:id/comments", post(handlers::create_comment))
        .route("/api/comments/:id", delete(handlers::delete_comment))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware));

    let app = public_routes
        .merge(protected_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    tracing::info!("Starting server on port 3001");
    axum::serve(listener, app).await.unwrap();
}
