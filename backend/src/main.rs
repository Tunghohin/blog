mod db;
mod handlers;
mod middleware;
mod models;

use atty::Stream;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use db::Db;
use middleware::auth::auth_middleware;

pub struct AppState {
    db: Db,
}

#[derive(Parser)]
#[command(name = "blog-backend")]
#[command(about = "Blog backend server with CLI tools")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the database and create admin user
    InitDb {
        /// Admin username
        #[arg(short, long)]
        username: Option<String>,
    },
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

    let cli = Cli::parse();

    // Handle CLI commands
    if let Some(Commands::InitDb { username }) = cli.command {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:./blog.db?mode=rwc".to_string());

        let db = db::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        db::init_db(&db)
            .await
            .expect("Failed to initialize database");

        // Create admin user
        let admin_username = username.unwrap_or_else(|| {
            print!("Enter admin username (default: admin): ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        });

        let admin_username = if admin_username.is_empty() {
            "admin".to_string()
        } else {
            admin_username
        };

        // 读取密码（支持 tty 和非 tty 环境)
        let password = if atty::is(Stream::Stdin) {
            // TTY 环境：隐藏密码输入
            rpassword::prompt_password("Enter admin password: ").unwrap()
        } else {
            // 非 TTY 环境：从 stdin 读取（用于脚本）
            let mut pwd = String::new();
            std::io::stdin().read_line(&mut pwd).unwrap();
            pwd.trim().to_string()
        };

        let password_confirm = if atty::is(Stream::Stdin) {
            rpassword::prompt_password("Confirm admin password: ").unwrap()
        } else {
            let mut pwd = String::new();
            std::io::stdin().read_line(&mut pwd).unwrap();
            pwd.trim().to_string()
        };

        if password != password_confirm {
            eprintln!("Passwords do not match!");
            std::process::exit(1);
        }

        if password.len() < 6 {
            eprintln!("Password must be at least 6 characters!");
            std::process::exit(1);
        }

        db::create_admin(&db, &admin_username, &password)
            .await
            .expect("Failed to create admin user");

        println!("Admin user '{}' created successfully!", admin_username);
        return;
    }

    // Start server
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./blog.db?mode=rwc".to_string());

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
        .route("/api/upload", post(handlers::upload_image))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // 静态文件服务（uploads 目录）
    let static_service = ServeDir::new("uploads");

    let app = public_routes
        .merge(protected_routes)
        .nest_service("/uploads", static_service)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    tracing::info!("Starting server on port 3001");
    axum::serve(listener, app).await.unwrap();
}
