pub mod auth;
pub mod posts;

pub use auth::{login, register};
pub use posts::{list_posts, get_post, create_post, update_post, delete_post};
